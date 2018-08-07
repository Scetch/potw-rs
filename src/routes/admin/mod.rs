use actix_web::{
    Result, App, Responder, State, Path, HttpResponse,
    error::ErrorInternalServerError,
};
use diesel::{ self, prelude::* };
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
                .resource("/promote/{id}", |r| r.with(promote))
                .resource("/demote/{id}", |r| r.with(demote))
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

fn promote((state, id): (State<AppState>, Path<i32>)) -> Result<impl Responder> {
    diesel::update(user::table.filter(user::id.eq(*id)))
        .set(user::admin.eq(true))
        .execute(&state.db)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Found().header("location", "/admin/").finish())
}

fn demote((state, id): (State<AppState>, Path<i32>)) -> Result<impl Responder> {
    diesel::update(user::table.filter(user::id.eq(*id)))
        .set(user::admin.eq(false))
        .execute(&state.db)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Found().header("location", "/admin/").finish())
}
