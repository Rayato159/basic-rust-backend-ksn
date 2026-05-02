use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tiberius::{Query, Row};
use tokio::sync::Mutex;

use crate::{
    domain::dto::transfer::TransferDto,
    domain::repositories::view_transactions::ViewTransactionsRepository,
    infrastructure::database::MSSQLClient,
};

pub struct ViewTransactionsMSSQL {
    db_client: Arc<Mutex<MSSQLClient>>,
}

impl ViewTransactionsMSSQL {
    pub fn new(db_client: Arc<Mutex<MSSQLClient>>) -> Self {
        Self { db_client }
    }
}

#[async_trait]
impl ViewTransactionsRepository for ViewTransactionsMSSQL {
    async fn get_transactions_by_user_id(&self, user_id: uuid::Uuid) -> Result<Vec<TransferDto>> {
        let mut client = self.db_client.lock().await;

        let sql = "
            SELECT id, user_id, amount, currency, status, created_at, updated_at
            FROM transactions
            WHERE user_id = @p1
            ORDER BY created_at DESC
        ";

        let mut query = Query::new(sql);
        query.bind(&user_id);

        let stream = query
            .query(&mut *client)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to query transactions: {}", e))?;

        let mut transactions = Vec::new();

        let rows = stream
            .into_results()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to fetch transaction results: {}", e))?;

        for row_result in &rows {
            for row in row_result {
                let transaction = self.row_to_transfer_dto(row)?;
                transactions.push(transaction);
            }
        }

        Ok(transactions)
    }
}

impl ViewTransactionsMSSQL {
    fn row_to_transfer_dto(&self, row: &Row) -> Result<TransferDto> {
        let id: uuid::Uuid = row
            .get::<uuid::Uuid, _>("id")
            .ok_or_else(|| anyhow::anyhow!("Missing id field"))?;

        let user_id: uuid::Uuid = row
            .get::<uuid::Uuid, _>("user_id")
            .ok_or_else(|| anyhow::anyhow!("Missing user_id field"))?;

        // Get amount as Decimal (tiberius with rust_decimal feature handles the conversion)
        let amount: rust_decimal::Decimal = row
            .get("amount")
            .ok_or_else(|| anyhow::anyhow!("Missing amount field"))?;

        let created_at: chrono::NaiveDateTime =
            row.get::<chrono::NaiveDateTime, _>("created_at")
                .ok_or_else(|| anyhow::anyhow!("Missing created_at field"))?;

        let currency: String = row
            .get::<&str, _>("currency")
            .ok_or_else(|| anyhow::anyhow!("Missing currency field"))?
            .to_string();

        let status: String = row
            .get::<&str, _>("status")
            .ok_or_else(|| anyhow::anyhow!("Missing status field"))?
            .to_string();

        let updated_at: Option<chrono::NaiveDateTime> =
            row.get::<chrono::NaiveDateTime, _>("updated_at");

        Ok(TransferDto {
            id,
            user_id,
            amount,
            currency,
            status,
            created_at: chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                created_at,
                chrono::Utc,
            ),
            updated_at: updated_at.map(|dt| {
                chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
            }),
        })
    }
}
