// use actix_web::http::{header, HttpTryFrom};
use actix_web::middleware::{Middleware, Started};
use actix_web::{error, HttpRequest, Result};
use db;
use diesel::prelude::*;
use models::Session;
use responses::errors::ApiError;
use schema::sessions::dsl;

pub struct HandleAuth;

const TOKEN_KEY_NAME: &str = "x-token";

impl<S> Middleware<S> for HandleAuth {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let headers = req.headers();

        if !headers.contains_key(TOKEN_KEY_NAME) {
            req.extensions_mut().insert(Session { user_id: None });
        } else {
            let header_token = req.headers().get(TOKEN_KEY_NAME).unwrap();
            let token = header_token.to_str().unwrap();
            let conn = db::establish_connection();
            let user_id: i32 = dsl::sessions
                .select(dsl::user_id)
                .filter(dsl::token.eq(token))
                .first(&conn)
                .map_err(|error| error::Error::from(ApiError::DbError(error.to_string())))?;

            req.extensions_mut().insert(Session {
                user_id: Some(user_id),
            });
        }

        Ok(Started::Done)
    }
}
