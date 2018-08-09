use actix_web::{Json, HttpRequest, Path};
use diesel::prelude::*;
use schema::todos::dsl;
use models::{NewTodo, Todo, UpdatedTodo};
use routes::{ApiError, ApiResponse, ApiJson};
use diesel;
use db;


#[derive(Deserialize, Debug)]
pub struct RequestTodo {
	title: String,
}

pub fn add_todo(todo: Json<RequestTodo>) -> Result<ApiJson<&'static str>, ApiError> {
	let conn = db::establish_connection();
	let _ = diesel::insert_into(dsl::todos)
		.values(NewTodo {
            title: &todo.title,
            checked: &false,
		})
		.execute(&conn)
		.map_err(|_| ApiError::InternalError)?;

	Ok(ApiResponse::new("sucess"))
}

pub fn get_todos(_req: &HttpRequest) -> Result<ApiJson<Vec<Todo>>, ApiError> {
	let conn = db::establish_connection();
	let todos = dsl::todos.load::<Todo>(&conn)
		.map_err(|_| ApiError::InternalError)?;

	Ok(ApiResponse::new(todos))
}

#[derive(Deserialize, Debug)]
pub struct ReqPath {
	id: i32
}

pub fn update_todo((rt, todo): (Path<ReqPath>, Json<UpdatedTodo>)) -> Result<ApiJson<&'static str>, ApiError> {
	let conn = db::establish_connection();
	let _ = diesel::update(dsl::todos.find(rt.id))
		.set(&UpdatedTodo {
			title: todo.title.clone(),
			checked: todo.checked.clone(),
		})
		.execute(&conn)
		.map_err(|_| ApiError::InternalError)?;

	Ok(ApiResponse::new("sucess"))
}
