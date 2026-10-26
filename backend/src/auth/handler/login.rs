use axum::{Json, extract::State, http::StatusCode, response::IntoResponse as _};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::util::error;
use super::super::model;

pub async fn it(
    State(state): State<crate::state::App>,
    Json(payload): Json<model::LoginRequest>,
) -> axum::response::Response {
    struct UserCredentials 
        { id            : uuid::Uuid
        , username      : String
        , password_hash : String
        }

    let result = sqlx::query_as!(
        UserCredentials,
        r#"
        SELECT id, username, password_hash
        FROM users
        WHERE username = $1
        "#,
        &payload.username
    )
    .fetch_optional(&state.db)
    .await;

    let user = match result {
        Ok(Some(user)) => user,
        Ok(None) => {
            return error(
                StatusCode::UNAUTHORIZED,
                "Invalid email or password".to_string(),
            );
        }
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Login failed".to_string(),
            );
        }
    };

    let password = payload.password;

    let valid = match tokio::task::spawn_blocking(move || {
        crate::util::password::verify(&password, &user.password_hash)
    }).await {
        Ok(valid) => valid,
        Err(e) => {
            eprintln!("In /auth/login handler, during password verification: {e}");
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Login failed, something is wrong with the async runtime".to_string(),
            );
        }
    };

    if !valid {
        return error(
            StatusCode::UNAUTHORIZED,
            "Invalid email or password".to_string(),
        );
    }

    let expires_in = match std::env::var("JWT_EXP_MINUTES") {
        Ok(value) => match value.parse::<u64>() {
            Ok(minutes) if minutes > 0 && minutes <= 1440 => minutes * 60,
            _ => {
                eprintln!("Invalid JWT_EXP_MINUTES value");
                return error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Invalid authentication configuration".to_string(),
                );
            },
        },
        Err(_) => {
            eprintln!("Invalid JWT_EXP_MINUTES value");
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Invalid authentication configuration".to_string(),
            );
        },
    };

    let now = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => {
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Login failed".to_string(),
            );
        }
    };

    let claims = model::Claims {
        sub: user.id.to_string(),
        exp: (now + expires_in) as usize,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    ) {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Failed token creation in auth/login handle: {e}");
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not issue access token".to_string(),
            );
        }
    };

    Json(model::LoginResponse {
        access_token: token,
        token_type: "Bearer",
        expires_in,
        user: model::UserResponse {
            id: user.id,
            username: user.username,
        },
    }).into_response()
}