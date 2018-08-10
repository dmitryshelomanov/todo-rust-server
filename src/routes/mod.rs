use actix_web::{http, App, HttpRequest, Responder};
mod todo;

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
        .resource("/get", |r| r.f(todo::get_todos))
}
