use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, sqlx::prelude::FromRow, JsonSchema)]
// #[derive(sqlx::prelude::FromRow)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: u64,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}