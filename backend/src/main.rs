mod db;
mod state;
mod route;
mod util;
mod cron;

mod auth;

use std::{env::var, net::SocketAddr};

const OPENAPI_PATH: &str = "/api/v1/openapi_docs";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = var("DATABASE_URL")?;
    let pool = db::create_pool(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    cron::run_all(pool.clone());

    let jwt_secret = std::sync::Arc::from(var("JWT_SECRET")?);

    let state = state::App { db: pool, jwt_secret };

    let (app, _) = axum_auto_openapi::Router::new()
        .nest("/api/v1", route::all(state.clone()))
        .layer(util::cors::layer()?)
        .with_state(state)
        .info(axum_auto_openapi::Info::new("URL Shortener", "0.1.0"))
        .openapi_json_route(OPENAPI_PATH)
        .swagger_ui("/api/v1/swagger", OPENAPI_PATH)
        .finish();

    let host = var("HOST")?;
    let port: u16 = var("PORT")?.parse()?;
    let addr: SocketAddr = format!("{host}:{port}").parse()?;

    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}