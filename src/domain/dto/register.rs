use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct RegisterDto {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}
