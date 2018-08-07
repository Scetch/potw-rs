use actix_web::{
    Result, error::ErrorInternalServerError,
    HttpRequest, HttpResponse,
    middleware::{
        Middleware, Started, Response,
        session::RequestSession,
    },
};
use diesel::prelude::*;
use liquid::Value;

use ::{
    AppState,
    db::{ schema::user, models::User },
    middleware::Template,
};

/// Gets the user object from the database and adds it to the current request
/// if it exists. Will also add user data to the template if it exists.
pub struct CurrentUser;

impl Middleware<AppState> for CurrentUser {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {
        // If the session has a uid we will try to load the user model
        // into the request.
        if let Some(uid) = req.session().get::<i32>("uid")? {
            let user = user::table
                .filter(user::id.eq(uid))
                .first::<User>(&req.state().db)
                .optional()
                .map_err(ErrorInternalServerError)?;

            // If the user doesn't exist in the database for this uid an error
            // has occured with the session and we should clear it. If not we'll
            // add the user to the session.
            if let Some(user) = user {
                req.extensions_mut().insert(user);
            } else {
                req.session().remove("uid");
            }
        }

        Ok(Started::Done)
    }

    fn response(&self, req: &HttpRequest<AppState>, resp: HttpResponse) -> Result<Response> {
        // If there is a template we'll add the user data to the template
        // here so we don't have to do it each request.
        if req.extensions().get::<Template>().is_some() {
            // Add the user data to the current template if a user exists.
            let obj = req.extensions()
                .get::<User>()
                .map(|usr| usr.clone().to_liquid());

            if let Some(obj) = obj {
                // We already checked if a template exists so it's fine for us to unwrap.
                req.extensions_mut()
                    .get_mut::<Template>()
                    .unwrap()
                    .globals
                    .insert("current_user".into(), Value::Object(obj));
            }
        }

        Ok(Response::Done(resp))
    }
}
