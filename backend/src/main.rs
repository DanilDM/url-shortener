mod db;
mod state;
mod cors;

use std::{env::var, net::SocketAddr};
use axum::routing::get;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = var("DATABASE_URL")?;
    let pool = db::create_pool(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = state::AppState { db: pool };

    let app = axum::Router::new()
        .route(
            "/api/v1/hello", // temporary route
            get("hello from axum"))
        .layer(cors::cors_layer()?)
        .with_state(state);

    let host = var("HOST")?;
    let port: u16 = var("PORT")?.parse()?;
    let addr: SocketAddr = format!("{host}:{port}").parse()?;

    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}