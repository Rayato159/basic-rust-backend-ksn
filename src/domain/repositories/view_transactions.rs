use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use crate::domain::entities::transactions::Transaction;

#[async_trait]
#[automock]
pub trait ViewTransactionsRepository {
    async fn get_transactions_by_user_id(&self, user_id: Uuid) -> Result<Vec<Transaction>>;
    async fn get_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>>;
}
