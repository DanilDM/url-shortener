use axum::{Json, extract::State, http::StatusCode, response::IntoResponse as _};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

use crate::util::error;
use super::super::model;

pub async fn it(
    State(state): State<crate::state::App>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    let token = match headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
    {
        Some(token) if !token.is_empty() => token,
        _ => {
            return error(
                StatusCode::UNAUTHORIZED,
                "Missing or invalid authorization token".to_string(),
            );
        }
    };

    let claims = match decode::<model::Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(data) => data.claims,
        Err(_) => {
            return error(
                StatusCode::UNAUTHORIZED,
                "Invalid or expired token".to_string(),
            );
        }
    };

    let user_id = match uuid::Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return error(
                StatusCode::UNAUTHORIZED,
                "Invalid token".to_string(),
            );
        }
    };

    let result = sqlx::query_as!(
        model::UserResponse,
        "SELECT id, username FROM users WHERE id = $1",
        user_id,
    )
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(user)) => Json(user).into_response(),
        Ok(None) => error(
            StatusCode::UNAUTHORIZED,
            "User no longer exists".to_string(),
        ),
        Err(_) => error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not verify user".to_string(),
        ),
    }
}