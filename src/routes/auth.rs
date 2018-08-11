use actix_web::Json;
use db;
use diesel;
use diesel::prelude::*;
use models::InsertableUser;
use responses::errors::ApiError;
use responses::response::{ApiJson, ApiResponse};
use schema::users::dsl;

#[derive(Deserialize)]
pub struct RequestLogin {
    login: String,
    password: String,
}

pub fn register(user: Json<RequestLogin>) -> Result<ApiJson<&'static str>, ApiError> {
    let conn = db::establish_connection();
    let _ = diesel::insert_into(dsl::users)
        .values(InsertableUser {
            login: user.login.clone(),
            password: user.password.clone(),
        })
        .execute(&conn)
        .map_err(|error| ApiError::DbError(error.to_string()))?;

    Ok(ApiResponse::new("success"))
}
