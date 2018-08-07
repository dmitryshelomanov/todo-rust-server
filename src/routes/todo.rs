use actix_web::{Responder, Json, HttpResponse};
use diesel::prelude::*;
use schema::todos::dsl;
use models::{NewTodo};
use routes::ApiResponse;
use diesel;
use db;


#[derive(Deserialize, Debug)]
pub struct RequestTodo {
	title: String,
}

pub fn add_todo(todo: Json<RequestTodo>) -> Result<Json<ApiResponse>, Json<ApiResponse>> {
	let conn = db::establish_connection();

	let result = diesel::insert_into(dsl::todos)
		.values(NewTodo::new(&todo.title))
		.execute(&conn);

	match result {
		Ok(_) => Ok(Json(ApiResponse::new("sucess"))),
		Err(_) => Err(Json(ApiResponse::new("fail"))),
	}
}
