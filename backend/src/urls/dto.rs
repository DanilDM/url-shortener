use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::util;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NewRequest {
    pub url: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, JsonSchema, sqlx::FromRow)]
pub struct Link {
    pub id: Uuid,
    pub code: String,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub click_count: i64,
    pub last_clicked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AllRequest {
    pub active: Option<bool>, // filters for either active, inactive, or both
    pub query: util::dto::PageQuery,
}

pub type AllResponse = util::dto::Paginated<Link>;