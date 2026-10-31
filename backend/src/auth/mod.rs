mod handler {
    pub mod register;
    pub mod login;
    pub mod me;
}
mod model;
pub mod middleware;

use axum_auto_openapi::{
    Router, routing::{get, post},
};

use crate::util::ErrorResponse;

#[macro_export]
macro_rules! openapi_auth_codes {
    ($mr:expr) => {
        $mr.response::<ErrorResponse>(401)
           .response::<ErrorResponse>(500)
    };
}

pub fn routes(state: crate::state::App) -> Router<crate::state::App> {
    let public = Router::new()
        .route(
            "/register",
            post(handler::register::it)
                .response::<model::UserResponse>(201)
                .response::<ErrorResponse>(409)
                .response::<ErrorResponse>(500)
        )
        .route(
            "/login",
            post(handler::login::it)
                .response::<model::LoginResponse>(200)
                .response::<ErrorResponse>(401)
                .response::<ErrorResponse>(500)
        );

    let protected = Router::new()
        .route(
            "/me",
            openapi_auth_codes!(
                get(handler::me::it)
                .response::<model::UserResponse>(200)
            )
        )
        .route_layer(
            axum::middleware::from_fn_with_state(
                state,
                middleware::authenticated,
            )
        );

    public.merge(protected)
}