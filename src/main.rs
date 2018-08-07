extern crate todo_rust_server;
extern crate dotenv;

use dotenv::dotenv;
use todo_rust_server::{create_server};


fn main() {
    dotenv().ok();

    create_server()
}
