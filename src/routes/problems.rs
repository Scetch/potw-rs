use actix_web::{
    Result, App, Responder, Path, State,
    error::{
        ErrorInternalServerError,
        ErrorNotFound,
    }
};
use diesel::prelude::*;
use liquid::{ Object, Value };

use ::{
    AppState,
    db::{
        models::{ Language, Problem },
        schema::{ language, problem },
    },
    middleware::Template,
};

pub fn configure(app: App<AppState>) -> App<AppState> {
    app.scope("/problems", |s| {
            s.resource("/", |r| r.with(index))
                .resource("/{id}/", |r| r.with(problem))
        })
}

fn index(state: State<AppState>) -> Result<impl Responder> {
    let problems = problem::table
        .load::<Problem>(&state.db)
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(|prob| Value::Object(prob.to_liquid(false)));

    let mut obj = Object::new();
    obj.insert("problems".into(), Value::array(problems));
    Ok(Template::render("problems/index.liquid", obj))
}

fn problem((state, id): (State<AppState>, Path<i32>)) -> Result<impl Responder> {
    let problem = problem::table
        .filter(problem::id.eq(*id))
        .first::<Problem>(&state.db)
        .optional()
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("No problem found."))?;

    let languages = language::table
        .load::<Language>(&state.db)
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(|lang| Value::Object(lang.to_liquid()));

    let mut obj = Object::new();
    obj.insert("problem".into(), Value::Object(problem.to_liquid(true)));
    obj.insert("languages".into(), Value::array(languages));
    Ok(Template::render("problems/problem.liquid", obj))
}
