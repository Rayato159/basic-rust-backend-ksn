use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Data Transfer Object for Transfer/Transaction
/// Represents a transaction record in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl TransferDto {
    /// Create a new TransferDto with initial values
    pub fn new(user_id: Uuid, amount: Decimal, currency: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            amount,
            currency,
            status: "pending".to_string(),
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    /// Update the status of the transfer
    pub fn update_status(mut self, status: String) -> Self {
        self.status = status;
        self.updated_at = Some(Utc::now());
        self
    }
}
