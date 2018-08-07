use actix_web::{
    Result,
    error::{ ErrorInternalServerError, ErrorNotFound },
    Scope, Responder, State, Path, Form,
    HttpResponse,
};
use diesel::{ self, prelude::* };
use liquid::{ Object, Value };

use ::{
    AppState,
    db::{
        models::Language,
        schema::language,
    },
    middleware::Template,
};

pub fn configure(scope: Scope<AppState>) -> Scope<AppState> {
    scope.resource("/create", |r| {
            r.get().with(create);
            r.post().with(create_form);
        })
        .resource("/{id}/edit", |r| {
            r.get().with(edit);
            r.post().with(edit_form);
        })
        .resource("/{id}/delete", |r| r.with(delete))
        .resource("/{id}/delete/confirm", |r| r.with(delete_confirm))
}

#[derive(Deserialize)]
struct LanguageForm {
    name: String,
}

fn create(_state: State<AppState>) -> impl Responder {
    Template::render("admin/language.liquid", None)
}

fn create_form((state, form): (State<AppState>, Form<LanguageForm>)) -> Result<impl Responder> {
    let LanguageForm { name } = form.into_inner();

    diesel::insert_into(language::table)
        .values(language::name.eq(name))
        .execute(&state.db)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Found().header("location", "/admin/").finish())
}


fn edit((state, id): (State<AppState>, Path<i32>)) -> Result<impl Responder> {
    let language = language::table
        .filter(language::id.eq(*id))
        .first::<Language>(&state.db)
        .optional()
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("No problem found."))?;

    let mut obj = Object::new();
    obj.insert("language".into(), Value::Object(language.to_liquid()));
    Ok(Template::render("admin/language.liquid", obj))
}

fn edit_form((state, id, form): (State<AppState>, Path<i32>, Form<LanguageForm>)) -> Result<impl Responder> {
    let LanguageForm { name } = form.into_inner();

    diesel::update(language::table.filter(language::id.eq(*id)))
        .set(language::name.eq(name))
        .execute(&state.db)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Found().header("location", "/admin/").finish())
}

fn delete((state, id): (State<AppState>, Path<i32>)) -> Result<impl Responder> {
    let language = language::table
        .filter(language::id.eq(*id))
        .first::<Language>(&state.db)
        .optional()
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("No problem found."))?;

    let mut obj = Object::new();
    obj.insert("confirmation".into(), Value::scalar(format!("Are you sure you want to delete {}?", language.name)));
    obj.insert("url".into(), Value::scalar(format!("/admin/languages/{}/delete/confirm", language.id)));
    Ok(Template::render("confirm.liquid", obj))
}

fn delete_confirm((state, id): (State<AppState>, Path<i32>)) -> Result<impl Responder> {
    diesel::delete(language::table.filter(language::id.eq(*id)))
        .execute(&state.db)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Found().header("location", "/admin/").finish())
}
