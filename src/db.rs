use std::env;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;


pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
