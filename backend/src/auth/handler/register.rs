use axum::{Json, extract::State, http::StatusCode, response::Response};

use crate::util::error;
use super::super::model;

pub async fn it(
    State(state): State<crate::state::App>,
    Json(payload): Json<model::RegisterRequest>,
) -> Response {
    let username = payload.username;
    let password = payload.password;

    let password_hash = match tokio::task::spawn_blocking(move || {
        crate::util::password::hash(password)
    }).await {
        Ok(Ok(hash)) => hash,
        _ => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not create account".to_string(),
            );
        }
    };

    let result = sqlx::query_as!(
        model::UserResponse,
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        RETURNING id, username
        "#,
        username,
        password_hash,
    )
    .fetch_one(&state.db)
    .await;

    use axum::response::IntoResponse;
    match result {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),

        Err(sqlx::Error::Database(db_error))
            if db_error.code().as_deref() == Some("23505") =>
        { error(
            StatusCode::CONFLICT,
            "An account with this email already exists".to_string(),
        ) }

        Err(_) => error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not create account".to_string(),
        ),
    }
}