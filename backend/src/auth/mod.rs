mod handler {
    pub mod register;
    pub mod login;
    pub mod me;
}
mod model;

use axum_auto_openapi::{
    Router, routing::{get, post},
};
use crate::util::ErrorResponse;

pub fn routes() -> Router<crate::state::App> {
    Router::new()
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
        )
        .route(
            "/me", 
            get(handler::me::it)
            .response::<model::UserResponse>(200)
            .response::<ErrorResponse>(401)
            .response::<ErrorResponse>(500)
        )
}