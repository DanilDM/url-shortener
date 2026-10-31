use axum::{Extension, Json};

use super::super::model;

pub async fn it(
    Extension(user): Extension<model::UserResponse>,
) -> Json<model::UserResponse> {
    Json(user)
}