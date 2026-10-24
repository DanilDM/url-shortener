mod handler {
    pub mod register;
    pub mod login;
    pub mod me;
}
mod model;

use axum::{
    Router, routing::{get, post},
};

pub fn routes() -> Router<crate::state::App> {
    Router::new()
        .route(
            "/register", 
            post(handler::register::it)
        )
        .route(
            "/login", 
            post(handler::login::it)
        )
        .route(
            "/me", 
            get(handler::me::it)
        )
}