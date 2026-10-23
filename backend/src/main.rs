mod db;
mod state;
mod route;
mod util;

mod auth;

use std::{env::var, net::SocketAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = var("DATABASE_URL")?;
    let pool = db::create_pool(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let jwt_secret = var("JWT_SECRET")?;

    let state = state::App { db: pool, jwt_secret };

    let app = axum::Router::new()
        .nest("/api/v1", route::all())
        .layer(util::cors::layer()?)
        .with_state(state);

    let host = var("HOST")?;
    let port: u16 = var("PORT")?.parse()?;
    let addr: SocketAddr = format!("{host}:{port}").parse()?;

    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}