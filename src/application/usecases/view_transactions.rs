use std::sync::Arc;
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
    application::models::view_transactions::TransactionInfo,
    domain::repositories::view_transactions::ViewTransactionsRepository,
};
use anyhow::Result;

pub struct ViewTransactionsUseCase<T>
where
    T: ViewTransactionsRepository + Send + Sync + 'static,
{
    view_transactions_repository: Arc<T>,
}

impl<T> ViewTransactionsUseCase<T>
where
    T: ViewTransactionsRepository + Send + Sync + 'static,
{
    pub fn new(view_transactions_repository: Arc<T>) -> Self {
        Self {
            view_transactions_repository,
        }
    }

    pub async fn view_transactions(
        &self,
        authenticated_user_id: Uuid,
        path_user_id: Uuid,
    ) -> Result<Vec<TransactionInfo>> {
        // Validate that the authenticated user_id matches the path user_id
        if authenticated_user_id != path_user_id {
            return Err(anyhow::anyhow!(
                "Access denied: authenticated user_id does not match requested user_id"
            ));
        }

        info!(user_id = %path_user_id, "Fetching transactions for user");
        debug!(user_id = %path_user_id, "Starting transaction retrieval");

        // Call repository to get all transactions for the user
        let transaction_dtos = self
            .view_transactions_repository
            .get_transactions_by_user_id(path_user_id)
            .await?;

        // Convert DTOs to TransactionInfo
        let transactions: Vec<TransactionInfo> =
            transaction_dtos.into_iter().map(|dto| dto.into()).collect();

        info!(
            user_id = %path_user_id,
            count = transactions.len(),
            "Successfully fetched transactions"
        );

        Ok(transactions)
    }
}
