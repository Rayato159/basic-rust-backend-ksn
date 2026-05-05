use std::sync::Arc;
use tracing::{debug, info};
use uuid::Uuid;

#[allow(unused_imports)]
use crate::{
    application::models::view_transactions::TransactionInfo,
    domain::entities::transactions::Transaction,
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
        let transaction_entities = self
            .view_transactions_repository
            .get_transactions_by_user_id(path_user_id)
            .await?;

        // Convert Entity to TransactionInfo
        let transactions: Vec<TransactionInfo> = transaction_entities
            .into_iter()
            .map(|entity| entity.into())
            .collect();

        info!(
            user_id = %path_user_id,
            count = transactions.len(),
            "Successfully fetched transactions"
        );

        Ok(transactions)
    }

    pub async fn view_transaction_by_id(
        &self,
        authenticated_user_id: Uuid,
        transaction_id: Uuid,
    ) -> Result<TransactionInfo> {
        info!(
            user_id = %authenticated_user_id,
            transaction_id = %transaction_id,
            "Fetching transaction by ID"
        );
        debug!(
            user_id = %authenticated_user_id,
            transaction_id = %transaction_id,
            "Starting transaction retrieval by ID"
        );

        // Get transaction by ID
        let transaction_entity = self
            .view_transactions_repository
            .get_transaction_by_id(transaction_id)
            .await?;

        // Check if transaction exists
        let transaction_entity = transaction_entity
            .ok_or_else(|| anyhow::anyhow!("Transaction with id {} not found", transaction_id))?;

        // Validate that the authenticated user_id matches the transaction's user_id
        if authenticated_user_id != transaction_entity.user_id {
            return Err(anyhow::anyhow!(
                "Access denied: authenticated user_id does not match transaction owner"
            ));
        }

        // Convert Entity to TransactionInfo
        let transaction_info: TransactionInfo = transaction_entity.into();

        info!(
            user_id = %authenticated_user_id,
            transaction_id = %transaction_id,
            "Successfully fetched transaction by ID"
        );

        Ok(transaction_info)
    }
}
