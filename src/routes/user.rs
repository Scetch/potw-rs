use actix_web::{
    Result,
    error::{
        ErrorBadRequest,
        ErrorInternalServerError,
        ErrorNotFound,
    },
    App, Responder, State, Query, HttpResponse, Path,
    middleware::session::Session,
};
use diesel::{ self, prelude::*, sql_types::Text };
use liquid::{ Object, Value };

use ::{
    AppState,
    db::{
        models::User,
        schema::{ oauth, user, problem, solution, language },
    },
    oauth::GProfile,
    middleware::Template,
};

pub fn configure(app: App<AppState>) -> App<AppState> {
    app.resource("/login", |r| r.with(login))
        .resource("/authorize", |r| r.with(authorize))
        .resource("/logout", |r| r.with(logout))
        .resource("/user/{sid}/", |r| r.with(user))
}

fn login(req: (State<AppState>, Session)) -> Result<impl Responder> {
    let (state, session) = req;

    let (url, csrf) = state.auth.authorize_url();
    session.set("csrf", csrf)?;

    Ok(HttpResponse::Found()
        .header("location", url.as_str())
        .finish())
}

#[derive(Deserialize)]
struct AuthorizeQuery {
    code: String,
    state: String,
}

fn authorize(req: (State<AppState>, Session, Query<AuthorizeQuery>)) -> Result<impl Responder> {
    let (app_state, session, query) = req;
    let db = &app_state.db;
    let AuthorizeQuery { code, state } = query.into_inner();

    // We want to make sure the session csrf state matches the one that we
    // are recieving.
    match session.get::<String>("csrf")? {
        Some(ref csrf) if csrf == state.as_str() => {
            // If the state matches we will then exchange the code we recieve
            // for the users google profile.
            let GProfile { id, email, .. } = app_state.auth.get_profile(code)?;

            // We will then try to get this users id from their Google id.
            let uid = db.transaction::<_, diesel::result::Error, _>(|| {
                    let get_uid = oauth::table
                        .select(oauth::uid)
                        .filter(oauth::gid.eq(&id));

                    if let Some(uid) = get_uid.first::<i32>(db).optional()? {
                        Ok(uid)
                    } else {
                        // If the Google id wasn't found in the database we want
                        // to create a new user and insert it into the database.
                        let sid = email.split('@').next().unwrap();

                        diesel::insert_into(user::table)
                            .values((user::sid.eq(sid), user::admin.eq(false)))
                            .execute(db)?;

                        user::table
                            .select((id.as_sql::<Text>(), user::id))
                            .insert_into(oauth::table)
                            .into_columns((oauth::gid, oauth::uid))
                            .execute(db)?;

                        get_uid.first::<i32>(db)
                    }
                })
                .map_err(ErrorInternalServerError)?;

            // Set up the session so we don't have to log in again right after.
            session.remove("csrf");
            session.set("uid", uid)?;

            // We'll redirect the user back to the main page.
            Ok(HttpResponse::Found()
                .header("location", "/")
                .finish())
        }
        _ => Err(ErrorBadRequest("Invalid csrf state.")),
    }
}

fn logout(session: Session) -> impl Responder {
    session.remove("uid");

    HttpResponse::Found()
        .header("location", "/")
        .finish()
}

fn user((state, user): (State<AppState>, Path<String>)) -> Result<impl Responder> {
    let user = user::table
        .filter(user::sid.eq(user.as_str()))
        .first::<User>(&state.db)
        .optional()
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("User not found."))?;

    let solutions = solution::table
        .filter(solution::uid.eq(user.id))
        .inner_join(language::table)
        .inner_join(problem::table)
        .select((solution::id, problem::name, language::name))
        .load::<(i32, String, String)>(&state.db)
        .map_err(ErrorInternalServerError)?
        .into_iter()
        .map(|(id, name, language)| {
            let mut obj = Object::new();
            obj.insert("id".into(), Value::scalar(id));
            obj.insert("name".into(), Value::scalar(name));
            obj.insert("language".into(), Value::scalar(language));
            Value::Object(obj)
        });

    let mut user = user.to_liquid();
    user.insert("solutions".into(), Value::array(solutions));

    let mut obj = Object::new();
    obj.insert("user".into(), Value::Object(user));

    Ok(Template::render("user.liquid", obj))
}
