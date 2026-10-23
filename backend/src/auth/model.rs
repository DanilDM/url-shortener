use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, sqlx::prelude::FromRow)]
// #[derive(sqlx::prelude::FromRow)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
}