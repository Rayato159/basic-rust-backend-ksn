use std::sync::Arc;
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
    application::models::transfer::{TransferModel, TransferResult},
    domain::repositories::transfer::TransferRepository,
};
use anyhow::Result;

pub struct TransferUseCase<T>
where
    T: TransferRepository + Send + Sync + 'static,
{
    transfer_repository: Arc<T>,
}

impl<T> TransferUseCase<T>
where
    T: TransferRepository + Send + Sync + 'static,
{
    pub fn new(transfer_repository: Arc<T>) -> Self {
        Self {
            transfer_repository,
        }
    }

    pub async fn create_transfer(
        &self,
        user_id: Uuid,
        transfer_model: TransferModel,
    ) -> Result<TransferResult> {
        // Log user_id before processing
        info!(user_id = %user_id, "Starting transfer transaction creation");
        debug!(user_id = %user_id, amount = %transfer_model.amount, currency = %transfer_model.currency, "Transfer details");

        // Convert model to DTO
        let transfer_dto = transfer_model.to_dto(user_id);

        // Call repository to insert transaction
        let inserted_dto = self
            .transfer_repository
            .create_transaction(&transfer_dto)
            .await?;

        // Log user_id after processing
        info!(user_id = %user_id, transaction_id = %inserted_dto.id, "Transfer transaction created successfully");
        debug!(user_id = %user_id, transaction_id = %inserted_dto.id, status = %inserted_dto.status, "Transfer completed with status");

        // Convert to result
        let result = TransferResult::from(inserted_dto);

        Ok(result)
    }
}
