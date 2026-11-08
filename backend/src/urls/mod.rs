mod generate;
mod model;
mod handler {
    pub mod new;
}

use axum_auto_openapi::{Router, routing::post};

use crate::openapi_auth_codes;
use crate::util::ErrorResponse;

pub fn routes(state: crate::state::App) -> Router<crate::state::App> {
    Router::new()
        .route(
            "/new",
            openapi_auth_codes!(
                post(handler::new::it)
                .response::<model::NewResponse>(201)
                .response::<ErrorResponse>(422)
            )
        )
        .route_layer(
            axum::middleware::from_fn_with_state(
                state,
                crate::auth::middleware::authenticated,
            )
        )
}