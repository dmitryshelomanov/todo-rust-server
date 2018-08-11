#[macro_use]
extern crate diesel;
extern crate actix_web;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use actix_web::{
    server::{self, HttpServer, IntoHttpHandler},
    App,
};

pub mod db;
mod middleware;
pub mod models;
mod responses;
mod routes;
mod schema;

pub fn create_server() -> HttpServer<impl IntoHttpHandler> {
    let server_creator = move || {
        let app = App::new();

        routes::with(app)
    };

    server::new(server_creator).bind("127.0.0.1:8080").unwrap()
}
