#[macro_use]
extern crate diesel;
extern crate actix_web;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use actix_web::{server, App};

pub mod db;
pub mod models;
mod routes;
mod schema;

pub fn create_server() {
    let server_creator = move || {
        let app = App::new();

        routes::with(app)
    };

    server::new(server_creator)
        .bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
