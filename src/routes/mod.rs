use actix_web::{App, Responder, HttpRequest, http, error};

mod todo;

use actix_web::*;


#[derive(Fail, Debug, Serialize)]
#[fail(display="ApiResponse")]
pub struct ApiResponse {
   status: &'static str
}

impl error::ResponseError for ApiResponse {}

impl ApiResponse {
	fn new(status: &'static str) -> ApiResponse {
		ApiResponse { status }
	}
}

fn index(_req: &HttpRequest) -> impl Responder {
	"Index"
}

pub fn with(app: App<()>) -> App<()> {
	app.resource("/", |r| r.f(index))
		.resource("/add", |r| {
			r.method(http::Method::POST).with(todo::add_todo)
		})
}
