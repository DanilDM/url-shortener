#[derive(Clone)]
pub struct App {
    pub db: sqlx::PgPool,
    pub jwt_secret: std::sync::Arc<str>,
}