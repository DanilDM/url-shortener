use axum_auto_openapi::Router;

pub fn all() -> Router<crate::state::App> {
    Router::new()
        .nest("/auth", crate::auth::routes())
}