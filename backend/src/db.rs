use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn create_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await
}