use actix_web::{http, App, FromRequest, HttpRequest};
use middleware::HandleAuth;
use models::Session;
mod auth;
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

pub fn with(app: App<()>) -> App<()> {
    app.scope("/api", |api| {
        api.middleware(HandleAuth)
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
    }).scope("/api", |api| {
        api.resource("/register", |r| {
            r.method(http::Method::POST).with(auth::register)
        })
    })
}
