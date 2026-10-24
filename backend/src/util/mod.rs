pub mod cors;
pub mod password;

use axum::response::IntoResponse;

pub fn error(status: axum::http::StatusCode, message: String) 
-> axum::response::Response {
    (status, axum::Json(ErrorResponse { error: message })).into_response()
}

#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct ErrorResponse {
    error: String,
}