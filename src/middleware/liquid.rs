use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

use actix_web::{
    error::ErrorInternalServerError,
    Responder, HttpRequest, HttpResponse, Error, Result,
    http::StatusCode,
    middleware::{ Middleware, Response },
};
use liquid::{ Parser, ParserBuilder, Object, compiler::FilesystemInclude };

use failure;

pub struct Liquid {
    path: PathBuf,
    parser: Parser,
}

impl Liquid {
    pub fn new<P>(path: P) -> Self
        where P: Into<PathBuf>
    {
        let path = path.into();

        let parser = ParserBuilder::with_liquid()
            .include_source(Box::new(FilesystemInclude::new(&path)))
            .build();

        Self {
            path: path,
            parser: parser,
        }
    }

    fn render(&self, tmpl: Template) -> Result<String, failure::Error> {
        let mut file = File::open(self.path.join(tmpl.tmpl))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(self.parser.parse(&buf)?.render(&tmpl.globals)?)
    }
}

impl<S> Middleware<S> for Liquid {
    fn response(&self, req: &HttpRequest<S>, mut resp: HttpResponse) -> Result<Response> {
        // If there is a template object we want to parse and render that template.
        if let Some(tmpl) = req.extensions_mut().remove::<Template>() {
            use actix_web::http::header::CONTENT_TYPE;
            let render = self.render(tmpl).map_err(ErrorInternalServerError)?;
            resp.headers_mut().insert(CONTENT_TYPE, "text/html; charset=UTF-8".parse().unwrap());
            resp.set_body(render);
        }

        Ok(Response::Done(resp))
    }
}

pub struct Template {
    pub tmpl: String,
    pub globals: Object,
}

impl Template {
    pub fn render<T, O>(tmpl: T, globals: O) -> Self
        where T: Into<String>,
              O: Into<Option<Object>>,
    {
        Template {
            tmpl: tmpl.into(),
            globals: globals.into().unwrap_or_else(|| Object::new()),
        }
    }
}

impl Responder for Template {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        // Add the template to the request so middleware can add to it.
        req.extensions_mut().insert(self);
        Ok(HttpResponse::new(StatusCode::OK))
    }
}

