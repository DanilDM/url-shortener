use axum::{Extension, Json};

use super::super::dto;

pub async fn it(
    Extension(user): Extension<dto::UserResponse>,
) -> Json<dto::UserResponse> {
    Json(user)
}