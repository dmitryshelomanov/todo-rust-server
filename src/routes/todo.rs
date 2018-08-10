use actix_web::{HttpRequest, Json, Path};
use db;
use diesel;
use diesel::prelude::*;
use models::{AsChangesetTodo, InsertableTodo, Session, Todo};
use responses::errors::ApiError;
use responses::response::{ApiJson, ApiResponse};
use schema::todos::dsl;

#[derive(Deserialize, Debug)]
pub struct RequestTodo {
    title: String,
}

pub fn add_todo(todo: Json<RequestTodo>) -> Result<ApiJson<&'static str>, ApiError> {
    let conn = db::establish_connection();
    let _ = diesel::insert_into(dsl::todos)
        .values(InsertableTodo {
            title: todo.title.clone(),
            checked: false,
            user_id: 2,
        })
        .execute(&conn)
        .map_err(|error| ApiError::DbError(error.to_string()))?;

    Ok(ApiResponse::new("success"))
}

pub fn get_todos((req, session): (HttpRequest, Session)) -> Result<ApiJson<Vec<Todo>>, ApiError> {
    let conn = db::establish_connection();
    let todos = dsl::todos
        .filter(dsl::user_id.eq(session.user_id))
        .load::<Todo>(&conn)
        .map_err(|error| ApiError::DbError(error.to_string()))?;

    Ok(ApiResponse::new(todos))
}

#[derive(Deserialize, Debug)]
pub struct ReqPath {
    id: i32,
}

pub fn update_todo(
    (path, todo, req, session): (Path<ReqPath>, Json<AsChangesetTodo>, HttpRequest, Session),
) -> Result<ApiJson<&'static str>, ApiError> {
    let conn = db::establish_connection();
    let target = dsl::todos
        .filter(dsl::id.eq(path.id))
        .filter(dsl::user_id.eq(session.user_id));
    let _ = diesel::update(target)
        .set(&AsChangesetTodo {
            title: todo.title.clone(),
            checked: todo.checked.clone(),
        })
        .execute(&conn)
        .map_err(|error| ApiError::DbError(error.to_string()))?;

    Ok(ApiResponse::new("success"))
}

pub fn delete_todo(
    (path, req, session): (Path<ReqPath>, HttpRequest, Session),
) -> Result<ApiJson<&'static str>, ApiError> {
    let conn = db::establish_connection();
    let target = dsl::todos
        .filter(dsl::id.eq(path.id))
        .filter(dsl::user_id.eq(session.user_id));
    let _ = diesel::delete(target)
        .execute(&conn)
        .map_err(|error| ApiError::DbError(error.to_string()))?;

    Ok(ApiResponse::new("success"))
}
