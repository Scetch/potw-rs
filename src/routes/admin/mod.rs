use actix_web::{
    Result, App, Responder, State,
    error::ErrorInternalServerError,
};
use diesel::prelude::*;
use liquid::{ Object, Value };

use ::{
    AppState,
    db::{
        models::{ User, Problem, Language },
        schema::{ user, problem, language },
    },
    middleware::{ Admin, Template },
};

mod languages;
mod problems;

pub fn configure(app: App<AppState>) -> App<AppState> {
    app.scope("/admin", |s| {
            s.middleware(Admin)
                .resource("/", |r| r.with(index))
                .nested("/languages", |s| self::languages::configure(s))
                .nested("/problems", |s| self::problems::configure(s))
        })
}

fn index(state: State<AppState>) -> Result<impl Responder> {
    let users = user::table
        .load::<User>(&state.db)
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(|user| Value::Object(user.to_liquid()));

    let problems = problem::table
        .load::<Problem>(&state.db)
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(|prob| Value::Object(prob.to_liquid(false)));

    let languages = language::table
        .load::<Language>(&state.db)
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(|lang| Value::Object(lang.to_liquid()));

    let mut obj = Object::new();
    obj.insert("users".into(), Value::array(users));
    obj.insert("problems".into(), Value::array(problems));
    obj.insert("languages".into(), Value::array(languages));

    Ok(Template::render("admin/index.liquid", obj))
}
