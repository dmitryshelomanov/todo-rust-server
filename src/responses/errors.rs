use actix_web::{error, http, HttpResponse};

#[derive(Fail, Debug)]
pub enum ApiError {
    #[fail(display = "An internal error occurred. Please try again later.")]
    InternalError,
    #[fail(display = "DieselError. Please try again later.")]
    DbError(String),
    #[fail(display = "reauest not auth")]
    Unauthorized,
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ApiError::InternalError => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
            ApiError::Unauthorized => HttpResponse::new(http::StatusCode::UNAUTHORIZED),
            ApiError::DbError(ref error) => {
                println!("Db error {}", &error);
                HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
