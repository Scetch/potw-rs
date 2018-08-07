extern crate actix_web;
extern crate comrak;
#[macro_use] extern crate diesel;
extern crate env_logger;
extern crate failure;
extern crate liquid;
extern crate oauth2;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;

use actix_web::{
    App, Responder,
    fs::StaticFiles,
    http::StatusCode,
    middleware::{
        Logger, ErrorHandlers, Response,
        session::{ SessionStorage, CookieSessionBackend },
    },
};
use diesel::{ SqliteConnection, Connection };

use middleware::{ CurrentUser, Liquid, Template };

mod db;
mod middleware;
mod oauth;
mod routes;

pub struct AppState {
    pub db: SqliteConnection,
    pub auth: oauth::OAuth2,
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    actix_web::server::new(|| {
            let db = SqliteConnection::establish("database.db")
                .expect("Couldn't connect to Sqlite database.");

            App::with_state({
                    AppState {
                        db: db,
                        auth: oauth::OAuth2::new(),
                    }
                })
                .middleware(Logger::default())
                .middleware(SessionStorage::new(CookieSessionBackend::private(&[0; 32]).secure(false)))
                .middleware(CurrentUser)
                .middleware({
                    ErrorHandlers::new()
                        .handler(StatusCode::NOT_FOUND, |req, _| {
                            Template::render("404.liquid", None)
                                .respond_to(req)
                                .map(|resp| Response::Done(resp))
                        })
                })
                .middleware(Liquid::new("./templates/"))
                .configure(routes::configure)
                .handler("/static", StaticFiles::new("./static/").unwrap())
        })
        .bind("192.168.1.144:80")
        .unwrap()
        .run();
}
