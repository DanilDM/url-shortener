use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::{state, util::error};
use super::model;

pub async fn authenticated(
    State(state): State<state::App>,
    mut request: Request,
    next: Next,
) -> Response {
    // Extract bearer token
    let token = match request
        .headers()
        .get(header::AUTHORIZATION)
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

    // Validate JWT
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

    // Parse user ID
    let user_id = match uuid::Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return error(
                StatusCode::UNAUTHORIZED,
                "Invalid token".to_string(),
            );
        }
    };

    // Fetch user
    let user = match sqlx::query_as!(
        model::UserResponse,
        "SELECT id, username FROM users WHERE id = $1",
        user_id,
    )
    .fetch_optional(&state.db)
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return error(
                StatusCode::UNAUTHORIZED,
                "User no longer exists".to_string(),
            );
        }
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not verify user".to_string(),
            );
        }
    };

    request.extensions_mut().insert(user);

    next.run(request).await
}