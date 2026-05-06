use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::dto::transfer::TransferDto;

/// Request model for creating a transfer
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TransferModel {
    #[schema(value_type = f64, example = 1000.50)]
    pub amount: Decimal,

    #[serde(default = "default_currency")]
    #[schema(example = "THB")]
    pub currency: String,
}

impl TransferModel {
    pub fn to_dto(&self, user_id: Uuid) -> TransferDto {
        TransferDto::new(user_id, self.amount, self.currency.clone())
    }
}

/// Default currency is THB
fn default_currency() -> String {
    "THB".to_string()
}

/// Result model for transfer operation
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct TransferResult {
    pub transaction_id: String,
    pub user_id: String,
    #[schema(value_type = f64)]
    pub amount: Decimal,
    pub currency: String,
    pub status: String,
    pub message: String,
}

impl From<TransferDto> for TransferResult {
    fn from(dto: TransferDto) -> Self {
        Self {
            transaction_id: dto.id.to_string(),
            user_id: dto.user_id.to_string(),
            amount: dto.amount,
            currency: dto.currency,
            status: dto.status,
            message: "Transaction recorded successfully".to_string(),
        }
    }
}
