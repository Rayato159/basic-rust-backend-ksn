use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tiberius::Query;
use tokio::sync::Mutex;

use crate::{
    domain::dto::transfer::TransferDto, domain::repositories::transfer::TransferRepository,
    infrastructure::database::MSSQLClient,
};

pub struct TransferMSSQL {
    db_client: Arc<Mutex<MSSQLClient>>,
}

impl TransferMSSQL {
    pub fn new(db_client: Arc<Mutex<MSSQLClient>>) -> Self {
        Self { db_client }
    }
}

#[async_trait]
impl TransferRepository for TransferMSSQL {
    async fn create_transaction(&self, dto: &TransferDto) -> Result<TransferDto> {
        let mut client = self.db_client.lock().await;

        let sql = "
            INSERT INTO transactions (id, user_id, amount, currency, status, created_at, updated_at)
            VALUES (@p1, @p2, @p3, @p4, @p5, @p6, @p7)
        ";

        let mut query = Query::new(sql);

        query.bind(&dto.id);
        query.bind(&dto.user_id);
        // Bind Decimal as string - SQL Server will convert it to DECIMAL
        let amount_str = dto.amount.to_string();
        query.bind(&amount_str);
        query.bind(dto.currency.as_str());
        query.bind(dto.status.as_str());
        // Convert chrono DateTime to NaiveDateTime for tiberius
        query.bind(dto.created_at.naive_utc());
        // Convert Option<DateTime<Utc>> to Option<NaiveDateTime> for tiberius
        let updated_at_naive = dto.updated_at.map(|dt| dt.naive_utc());
        query.bind(updated_at_naive);

        query
            .execute(&mut *client)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to insert transaction: {}", e))?;

        // Return the inserted DTO
        Ok(dto.clone())
    }
}
