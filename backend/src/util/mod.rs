pub mod cors;
pub mod password;
pub mod url;
pub mod dto;

use axum::{http::StatusCode, response::{IntoResponse, Response}};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub fn error(status: axum::http::StatusCode, message: String) 
-> axum::response::Response {
    (status, axum::Json(ErrorResponse { error: message })).into_response()
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub enum Order { Ascending, Descending }

pub fn internal_errors_that_should_not_happen(e: sqlx::Error) -> Response {
    error( match e {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        sqlx::Error::Io(_) |
        sqlx::Error::PoolTimedOut |
        sqlx::Error::PoolClosed => StatusCode::SERVICE_UNAVAILABLE,
        // Database errors
        sqlx::Error::Database(database_error) => {
            match database_error.code().as_deref() {
                Some("23505") |
                Some("23503") => StatusCode::CONFLICT,
                Some("23502") |
                Some("23514") => StatusCode::UNPROCESSABLE_ENTITY,
                Some("40001") |
                Some("40P01") => StatusCode::CONFLICT,
                Some("08006") => StatusCode::SERVICE_UNAVAILABLE,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }, "Something went wrong".to_string())
}