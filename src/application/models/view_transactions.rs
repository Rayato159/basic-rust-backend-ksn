use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::dto::transfer::TransferDto;
use crate::domain::entities::transactions::Transaction;

/// Result model for viewing all transactions of a user
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ViewTransactionsResult {
    pub user_id: String,
    pub transactions: Vec<TransactionInfo>,
}

/// Individual transaction information
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TransactionInfo {
    pub transaction_id: String,
    #[schema(value_type = f64)]
    pub amount: Decimal,
    pub currency: String,
    pub status: String,
    #[schema(value_type = String)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = Option<String>)]
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<TransferDto> for TransactionInfo {
    fn from(dto: TransferDto) -> Self {
        Self {
            transaction_id: dto.id.to_string(),
            amount: dto.amount,
            currency: dto.currency,
            status: dto.status,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

impl From<Transaction> for TransactionInfo {
    fn from(entity: Transaction) -> Self {
        Self {
            transaction_id: entity.id.to_string(),
            amount: entity.amount,
            currency: entity.currency,
            status: entity.status,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
