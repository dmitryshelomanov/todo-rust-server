use actix_web::{dev, http, App, FromRequest, HttpRequest, Responder, Result};
use models::Session;
mod todo;

impl<S> FromRequest<S> for Session {
    type Config = ();
    type Result = Self;

    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        let session = req.extensions().get::<Session>().unwrap().clone();

        Session {
            user_id: session.user_id,
        }
    }
}

fn index(_req: &HttpRequest) -> impl Responder {
    "index"
}

pub fn with(app: App<()>) -> App<()> {
    app.resource("/", |r| r.f(index))
        .resource("/add", |r| {
            r.method(http::Method::POST).with(todo::add_todo)
        })
        .resource("/update/{id}", |r| {
            r.method(http::Method::POST).with(todo::update_todo)
        })
        .resource("/delete/{id}", |r| {
            r.method(http::Method::POST).with(todo::delete_todo)
        })
        .resource("/get", |r| {
            r.method(http::Method::GET).with(todo::get_todos)
        })
}
