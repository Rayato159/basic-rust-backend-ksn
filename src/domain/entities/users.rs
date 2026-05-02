use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
