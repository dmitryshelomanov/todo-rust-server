extern crate actix_web;
use actix_web::{server, App, HttpRequest, Responder};


fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    let app = || {
        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/{name}", |r| r.f(greet))
    };

    server::new(app)
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
