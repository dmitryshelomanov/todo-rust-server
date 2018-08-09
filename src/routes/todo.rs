use actix_web::{Json, HttpRequest, Path};
use diesel::prelude::*;
use schema::todos::dsl;
use models::{Todo, FormTodo};
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
		.values(FormTodo {
            title: Some(todo.title.clone()),
            checked: Some(false),
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

pub fn update_todo((path, todo): (Path<ReqPath>, Json<FormTodo>)) -> Result<ApiJson<&'static str>, ApiError> {
	let conn = db::establish_connection();
	let _ = diesel::update(dsl::todos.find(path.id))
		.set(&FormTodo {
			title: todo.title.clone(),
			checked: todo.checked.clone(),
		})
		.execute(&conn)
		.map_err(|_| ApiError::InternalError)?;

	Ok(ApiResponse::new("sucess"))
}

pub fn delete_todo(path: Path<ReqPath>) -> Result<ApiJson<&'static str>, ApiError> {
	let conn = db::establish_connection();
	let _ = diesel::delete(dsl::todos.find(path.id))
		.execute(&conn)
		.map_err(|_| ApiError::InternalError)?;

	Ok(ApiResponse::new("sucess"))
}
