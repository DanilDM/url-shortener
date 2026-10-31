use axum_auto_openapi::Router;

pub fn all(state: crate::state::App) -> Router<crate::state::App> {
    Router::new()
        .nest("/auth", crate::auth::routes(state))
}