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
use std::sync::{Arc, Mutex};

mod app_state;
mod db;
mod middleware;
mod models;
mod responses;
mod routes;
mod schema;

pub fn create_server() -> HttpServer<impl IntoHttpHandler> {
    let server_creator = move || {
        let database = db::establish_connection();

        let state = app_state::AppState::new(Arc::new(Mutex::new(database)));
        let app = App::with_state(state);

        routes::with(app)
    };

    server::new(server_creator).bind("127.0.0.1:9000").unwrap()
}
