use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use crate::domain::dto::transfer::TransferDto;

#[async_trait]
#[automock]
pub trait ViewTransactionsRepository {
    async fn get_transactions_by_user_id(&self, user_id: Uuid) -> Result<Vec<TransferDto>>;
}
