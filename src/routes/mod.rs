use actix_web::{http, App, FromRequest, HttpRequest, Result};
use models::Session;
use middleware::HandleAuth;
mod todo;
mod auth;

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

pub fn with(app: App<()>) -> App<()> {
    app.scope("/api", |scope| {
        scope
        .middleware(HandleAuth)
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
    })
    .resource("/register", |r| {
        r.method(http::Method::POST).with(auth::register)
    })
}
