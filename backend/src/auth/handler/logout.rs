use axum::{Extension, extract::State, http::StatusCode};

pub async fn it(
    State(state): State<crate::state::App>,
    Extension(claims): Extension<super::super::model::Claims>,
) -> Result<StatusCode, axum::response::Response> {
    sqlx::query!(
        "INSERT INTO revoked_tokens (jti, expires_at)
         VALUES ($1, to_timestamp($2::float8))
         ON CONFLICT DO NOTHING",
        claims.jti,
        claims.exp as f64,
    )
    .execute(&state.db)
    .await
    .map_err(|_| crate::util::error(StatusCode::INTERNAL_SERVER_ERROR, "Logout failed".to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}