use axum::{Extension, Json, extract::State, http::StatusCode, response::Response};
use chrono::Utc;

use crate::util::error;
use super::super::model;

const MAX_ATTEMPTS: usize = 5;
const CODE_LEN: usize = 8;

pub async fn it(
    State(state): State<crate::state::App>,
    Extension(user): Extension<crate::auth::model::UserResponse>,
    Json(req): Json<model::NewRequest>,
) -> Result<(StatusCode, Json<model::NewResponse>), Response> {
    let long_url; 
    match crate::util::url::validate(&req.url) {
        Ok(url) => long_url = url.to_string(),
        Err((sc, msg)) => return Err(error(sc, msg)),
    }

    if let Some(exp) = req.expires_at {
        if exp <= Utc::now() {
            return Err(error(
                StatusCode::UNPROCESSABLE_ENTITY,
                "expires_at must be in the future".to_string(),
            ));
        }
    }

    for attempt in 0..MAX_ATTEMPTS {
        let input = if attempt == 0 {
            format!("{}:{}", user.id, long_url)
        } else {
            format!("{}:{}#{}", user.id, long_url, attempt)
        };
        let code; 
        match super::super::generate::short_code(&input, CODE_LEN) {
            Ok(res) => code = res,
            Err(e) => return Err(error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Short url generation failed: {}", e)
            )),
        }

        let result = sqlx::query_as!( 
            model::NewResponse,
            r#"
            INSERT INTO short_links (user_id, code, original_url, expires_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, code, original_url, created_at, expires_at
            "#,
            user.id,
            code,
            long_url,
            req.expires_at
        )
        .fetch_one(&state.db)
        .await;

        fn is_code_collision(err: &sqlx::Error) -> bool {
            if let sqlx::Error::Database(db) = err {
                db.is_unique_violation()
                    && db.constraint() == Some("short_links_code_unique_idx")
            } else { false }
        }

        match result {
            Ok(resp) => {
                return Ok((StatusCode::CREATED, Json(resp)));
            }
            Err(e) if is_code_collision(&e) => continue, // retry with a new salt
            Err(_) => return Err(error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database failure".to_string()
            )),
        }
    }

    Err(error(
        StatusCode::UNPROCESSABLE_ENTITY,
        "Short url generation failed: max salting attempts exceeded".to_string()
    ))
}