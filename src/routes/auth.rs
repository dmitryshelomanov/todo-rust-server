use actix_web::Json;
use app_state::Req;
use diesel;
use diesel::prelude::*;
use models::InsertableUser;
use responses::errors::ApiError;
use responses::response::{ApiJson, ApiResponse};
use schema::users::dsl;

#[derive(Deserialize)]
pub struct RequestAuth {
    login: String,
    password: String,
}

pub fn register((req, user): (Req, Json<RequestAuth>)) -> Result<ApiJson<&'static str>, ApiError> {
    let conn = req.state().db.lock().unwrap();

    diesel::insert_into(dsl::users)
        .values(InsertableUser {
            login: user.login.clone(),
            password: user.password.clone(),
        })
        .execute(&*conn)
        .map_err(|error| ApiError::DbError(error.to_string()))?;

    Ok(ApiResponse::new("success"))
}
