use actix_web::HttpRequest;
use diesel::mysql::MysqlConnection;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub db: Arc<Mutex<MysqlConnection>>,
}

impl AppState {
    pub fn new(db: Arc<Mutex<MysqlConnection>>) -> AppState {
        AppState { db }
    }
}

pub type Req = HttpRequest<AppState>;
