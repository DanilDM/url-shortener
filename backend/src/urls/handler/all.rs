use axum::{Extension, Json, extract::State, http::StatusCode, response::Response};
use crate::util::{self, Order, dto::Paginated};

use super::super::dto;

pub async fn it(
    State(state): State<crate::state::App>,
    Extension(user): Extension<crate::auth::dto::UserResponse>,
    Json(req): Json<dto::AllRequest>,
) -> Result<(StatusCode, Json<dto::AllResponse>), Response> {
    let page = req.query.page();
    let per_page = req.query.per_page();
    let descending = matches!(req.query.order, Order::Descending);

    let limit = per_page as i64;
    let offset = req.query.offset() as i64;

    let items = sqlx::query_as!(
        dto::Link,
        r#"
            SELECT id, code, original_url, created_at, expires_at, click_count, last_clicked_at
            FROM short_links
            WHERE user_id = $5 AND (
                $1::BOOLEAN IS NULL
                OR CASE
                    WHEN $1::BOOLEAN = TRUE
                        THEN expires_at IS NOT NULL AND expires_at <= NOW()
                    ELSE
                        expires_at IS NULL OR expires_at > NOW()
                END )
            ORDER BY
                CASE WHEN $2::BOOLEAN THEN created_at END DESC,
                CASE WHEN NOT $2::BOOLEAN THEN created_at END ASC,
                id ASC
            LIMIT $3 OFFSET $4
        "#,
        req.active,
        descending,
        limit,
        offset,
        user.id,
    )
    .fetch_all(&state.db);

    let total = sqlx::query_scalar!(
        r#"
            SELECT COUNT(*) AS "count!"
            FROM short_links
            WHERE user_id = $2 AND (
                $1::BOOLEAN IS NULL
                OR CASE
                    WHEN $1::BOOLEAN = TRUE
                        THEN expires_at IS NOT NULL AND expires_at <= NOW()
                    ELSE
                        expires_at IS NULL OR expires_at > NOW()
                END )
        "#,
        req.active,
        user.id,
    )
    .fetch_one(&state.db);

    match tokio::try_join!(items, total) {
        Ok((items, total)) => 
            Ok((
                StatusCode::OK,
                Json(Paginated::new(
                    items,
                    page,
                    per_page,
                    total as u64,
                )),
            )),
        Err(e) => Err( util::internal_errors_that_should_not_happen(e) ),
    }
}