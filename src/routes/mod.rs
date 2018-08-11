use actix_web::{http, App, FromRequest, HttpRequest};

use app_state::AppState;
use middleware::HandleAuth;
use models::Session;
mod auth;
mod todo;

impl FromRequest<AppState> for Session {
    type Config = ();
    type Result = Self;

    fn from_request(req: &HttpRequest<AppState>, _: &Self::Config) -> Self::Result {
        let session = req.extensions().get::<Session>().unwrap().clone();

        Session {
            user_id: session.user_id,
        }
    }
}

pub fn with(app: App<AppState>) -> App<AppState> {
    app.scope("/api", |api| {
        api
        .resource("/register", |r| {
            r
            .method(http::Method::POST).with(auth::register)
        })
        // .middleware(HandleAuth)
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
}
