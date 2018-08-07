use actix_web::{
    Result, HttpRequest, HttpResponse,
    middleware::{ Middleware, Started },
};

use ::db::models::User;

/// Make sure the user is an admin before allowing the request.
/// If they aren't an admin we'll redirect them to the main page.
pub struct Admin;

impl<S> Middleware<S> for Admin {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let is_admin = req.extensions()
            .get::<User>()
            .map(|usr| usr.admin)
            .unwrap_or(false);

        if is_admin {
            Ok(Started::Done)
        } else {
            let resp = HttpResponse::Found()
                .header("location", "/")
                .finish();

            Ok(Started::Response(resp))
        }
    }
}
