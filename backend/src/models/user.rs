use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)] // パスワードハッシュはAPIレスポンスに絶対含めない
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}