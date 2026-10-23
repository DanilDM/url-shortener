mod handler {
    pub mod register;
    pub mod login;
}
mod model;

use axum::{
    routing::post,
    Router,
};

pub fn routes() -> Router<crate::state::App> {
    Router::new()
        .route(
            "/register", 
            post(handler::register::it)
        )
        // .route(
        //     "/login", 
        //     post(handler::login::it)
        // )
}