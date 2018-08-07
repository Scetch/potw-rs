use actix_web::{
    Result, App, Responder, State, Path,
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

pub fn configure(app: App<AppState>) -> App<AppState> {
    app.scope("/admin", |s| {
            s.middleware(Admin)
                .resource("/", |r| r.with(index))
                .nested("/problems/", |s| {
                    s.resource("/create", |r| r.with(problem))
                        .resource("/{}/edit", |r| r.with(problem))
                        .resource("/{}/delete", |r| r.with(problem))
                })
                .resource("/language/{id}/", |r| r.with(language))
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

fn problem(state: State<AppState>) -> impl Responder {
    Template::render("admin/problem.liquid", None)
}

fn language(state: State<AppState>) -> impl Responder {
    Template::render("admin/language.liquid", None)
}

/*
fn problem_new(state: State<AppState>) -> impl Responder {

    Template::render("admin/problem.liquid", None)
}

fn language_new(state: State<AppState>) -> impl Responder {

    Template::render("admin/language.liquid", None)
}
*/

/*
fn problem_index(state: State<AppState>) -> Result<impl Responder> {
    use ::db::{
        models::Problem,
        schema::problem::dsl::problem,
    };

    let problems = problem.load::<Problem>(&state.db)
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(|Problem { id, name, description }| {
            let mut obj = Object::new();
            obj.insert("id".to_string(), Value::scalar(id));
            obj.insert("name".to_string(), Value::scalar(name));
            obj.insert("description".to_string(), Value::scalar(description));
            Value::Object(obj)
        });

    let mut obj = Object::new();
    obj.insert("problems".to_string(), Value::array(problems));

    Ok(Template::render("admin/problems/index.liquid", obj))
}

#[derive(Deserialize)]
pub struct ProblemForm {
    name: String,
    description: String,
}

fn problem_create_post((state, form): (State<AppState>, Form<ProblemForm>)) -> Result<impl Responder> {
    use diesel;
    use ::db::schema::problem::dsl as problem;

    let mut obj = Object::new();

    if form.name.is_empty() || form.description.is_empty() {
        obj.insert("error".to_string(), Value::scalar("Fields can not be blank."));
    } else {
        diesel::insert_into(problem::problem)
            .values(&(problem::name.eq(&form.name), problem::description.eq(&form.description)))
            .execute(&state.db)
            .map_err(ErrorInternalServerError)?;

        obj.insert("success".to_string(), Value::scalar("Successfully added!"));
    }

    Ok(Template::render("admin/problems/create.liquid", obj))
}

fn problem_create_get(_state: State<AppState>) -> impl Responder {
    Template::render("admin/problems/create.liquid", None)
}
*/
