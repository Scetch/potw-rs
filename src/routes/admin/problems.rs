use actix_web::{
    Result, Responder, State, Path, Scope, Form, HttpResponse,
    error::{ ErrorInternalServerError, ErrorNotFound },
};
use diesel::{ self, prelude::* };
use liquid::{ Object, Value };
use ::{
    AppState,
    db::{ models::Problem, schema::problem, },
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

#[derive(Debug, Deserialize)]
struct ProblemForm {
    name: String,
    description: String,
}

fn create(_: State<AppState>) -> impl Responder {
    Template::render("admin/problem.liquid", None)
}

fn create_form((state, form): (State<AppState>, Form<ProblemForm>)) -> Result<impl Responder> {
    let ProblemForm { name, description } = form.into_inner();

    diesel::insert_into(problem::table)
        .values((problem::name.eq(name), problem::description.eq(description)))
        .execute(&state.db)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Found().header("location", "/admin/").finish())
}

fn edit((state, id): (State<AppState>, Path<i32>)) -> Result<impl Responder> {
    let problem = problem::table
        .filter(problem::id.eq(*id))
        .first::<Problem>(&state.db)
        .optional()
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("No problem found."))?;

    let mut obj = Object::new();
    obj.insert("problem".into(), Value::Object(problem.to_liquid(false)));
    Ok(Template::render("admin/problem.liquid", obj))
}

fn edit_form((state, id, form): (State<AppState>, Path<i32>, Form<ProblemForm>)) -> Result<impl Responder> {
    let ProblemForm { name, description } = form.into_inner();

    diesel::update(problem::table.filter(problem::id.eq(*id)))
        .set((problem::name.eq(name), problem::description.eq(description)))
        .execute(&state.db)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Found().header("location", "/admin/").finish())
}


fn delete((state, id): (State<AppState>, Path<i32>)) -> Result<impl Responder> {
    let problem = problem::table
        .filter(problem::id.eq(*id))
        .first::<Problem>(&state.db)
        .optional()
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("No problem found."))?;

    let mut obj = Object::new();
    obj.insert("confirmation".into(), Value::scalar(format!("Are you sure you want to delete {}?", problem.name)));
    obj.insert("url".into(), Value::scalar(format!("/admin/problems/{}/delete/confirm", problem.id)));
    Ok(Template::render("confirm.liquid", obj))
}

fn delete_confirm((state, id): (State<AppState>, Path<i32>)) -> Result<impl Responder> {
    diesel::delete(problem::table.filter(problem::id.eq(*id)))
        .execute(&state.db)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Found().header("location", "/admin/").finish())
}
