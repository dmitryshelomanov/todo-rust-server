use actix_web::{error, http, App, HttpRequest, Responder};

mod todo;

use actix_web::*;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    data: T,
}

pub type ApiJson<T> = Json<ApiResponse<T>>;

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> ApiJson<T> {
        Json(ApiResponse { data })
    }
}

#[derive(Fail, Debug)]
pub enum ApiError {
    #[fail(display = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ApiError::InternalError => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

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
