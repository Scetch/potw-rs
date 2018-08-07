use actix_web::{
    Result, error::ErrorInternalServerError,
    App, Responder, HttpRequest, State,
};
use diesel::prelude::*;
use liquid::{ Object, Value };

use ::{
    AppState,
    db::{ models::Problem, schema::problem },
    middleware::Template,
};

mod admin;
mod user;
mod problems;
mod leaderboard;

pub fn configure(app: App<AppState>) -> App<AppState> {
    app.resource("/", |r| r.with(index))
        .configure(self::admin::configure)
        .configure(self::user::configure)
        .configure(self::problems::configure)
        .configure(self::leaderboard::configure)
        .default_resource(|r| r.with(not_found))
}

fn index(state: State<AppState>) -> Result<impl Responder> {
    let prob = problem::table
        .first::<Problem>(&state.db)
        .optional()
        .map_err(ErrorInternalServerError)?
        .map(|prob| prob.to_liquid(true));

    let mut obj = Object::new();

    if let Some(prob) = prob {
        obj.insert("problem".into(), Value::Object(prob));
    }

    Ok(Template::render("index.liquid", obj))
}

fn not_found<S>(_: HttpRequest<S>) -> impl Responder {
    Template::render("404.liquid", None)
}
