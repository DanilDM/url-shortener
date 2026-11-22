mod generate;
mod dto;
mod handler {
    pub mod new;
    pub mod all;
}

use axum_auto_openapi::{Router, routing::{get, post}};

use crate::openapi_auth_codes;
use crate::util::ErrorResponse;

pub fn routes(state: crate::state::App) -> Router<crate::state::App> {
    Router::new()
        .route(
            "/new",
            openapi_auth_codes!(
                post(handler::new::it)
                .response::<dto::Link>(201)
                .response::<ErrorResponse>(422)
            )
        )
        .route(
            "/all",
            openapi_auth_codes!(
                get(handler::all::it)
                .response::<dto::AllResponse>(200)
            )
        )
        .route_layer(
            axum::middleware::from_fn_with_state(
                state,
                crate::auth::middleware::authenticated,
            )
        )
}