use axum::Router;

pub fn all() -> Router<crate::state::App> {
    Router::new()
        .nest("/auth", crate::auth::routes())
}