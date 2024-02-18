use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
#[derive(Deserialize, Serialize, Debug, FromRow)]
#[allow(non_snake_case)]
pub struct ReviewModel {
    pub id: Uuid,
    pub comment: String,
    pub rating: i32,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
