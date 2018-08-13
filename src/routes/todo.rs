use actix_web::{Json, Path};
use app_state::Req;
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

pub fn add_todo(
    (req, todo, session): (Req, Json<RequestTodo>, Session),
) -> Result<ApiJson<&'static str>, ApiError> {
    if session.is_auth() {
        let conn = req.state().db.lock().unwrap();
        let user_id = session.user_id.unwrap();
        diesel::insert_into(dsl::todos)
            .values(InsertableTodo {
                title: todo.title.clone(),
                checked: false,
                user_id,
            })
            .execute(&*conn)
            .map_err(|error| ApiError::DbError(error.to_string()))?;

        Ok(ApiResponse::new("success"))
    } else {
        Err(ApiError::Unauthorized)
    }
}

pub fn get_todos((req, session): (Req, Session)) -> Result<ApiJson<Vec<Todo>>, ApiError> {
    if session.is_auth() {
        let conn = req.state().db.lock().unwrap();
        let user_id = session.user_id.unwrap();
        let todos: Vec<Todo> = dsl::todos
            .filter(dsl::user_id.eq(user_id))
            .load(&*conn)
            .map_err(|error| ApiError::DbError(error.to_string()))?;

        Ok(ApiResponse::new(todos))
    } else {
        Err(ApiError::Unauthorized)
    }
}

#[derive(Deserialize, Debug)]
pub struct ReqPath {
    id: i32,
}

pub fn update_todo(
    (req, path, todo, session): (Req, Path<ReqPath>, Json<AsChangesetTodo>, Session),
) -> Result<ApiJson<&'static str>, ApiError> {
    if session.is_auth() {
        let conn = req.state().db.lock().unwrap();
        let user_id = session.user_id.unwrap();
        let target = dsl::todos
            .filter(dsl::id.eq(path.id))
            .filter(dsl::user_id.eq(user_id));

        diesel::update(target)
            .set(&AsChangesetTodo {
                title: todo.title.clone(),
                checked: todo.checked.clone(),
            })
            .execute(&*conn)
            .map_err(|error| ApiError::DbError(error.to_string()))?;

        Ok(ApiResponse::new("success"))
    } else {
        Err(ApiError::Unauthorized)
    }
}

pub fn delete_todo(
    (req, path, session): (Req, Path<ReqPath>, Session),
) -> Result<ApiJson<&'static str>, ApiError> {
    if session.is_auth() {
        let conn = req.state().db.lock().unwrap();
        let user_id = session.user_id.unwrap();
        let target = dsl::todos
            .filter(dsl::id.eq(path.id))
            .filter(dsl::user_id.eq(user_id));

        diesel::delete(target)
            .execute(&*conn)
            .map_err(|error| ApiError::DbError(error.to_string()))?;

        Ok(ApiResponse::new("success"))
    } else {
        Err(ApiError::Unauthorized)
    }
}
