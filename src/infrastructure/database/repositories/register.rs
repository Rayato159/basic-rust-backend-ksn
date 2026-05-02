use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tiberius::Query;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    domain::{dto::register::RegisterDto, repositories::register::RegisterRepository},
    infrastructure::database::MSSQLClient,
};

pub struct RegisterMSSQL {
    db_client: Arc<Mutex<MSSQLClient>>,
}

impl RegisterMSSQL {
    pub fn new(db_client: Arc<Mutex<MSSQLClient>>) -> Self {
        Self { db_client }
    }
}

#[async_trait]
impl RegisterRepository for RegisterMSSQL {
    async fn register(&self, register_dto: RegisterDto) -> Result<Uuid> {
        let mut client = self.db_client.lock().await;

        let sql = "
            INSERT INTO users (id, username, password, created_at)
            VALUES (@p1, @p2, @p3, @p4)
        ";

        let mut query = Query::new(sql);

        query.bind(register_dto.id);
        query.bind(register_dto.username);
        query.bind(register_dto.password);
        query.bind(register_dto.created_at);

        query
            .execute(&mut *client)
            .await
            .map_err(|e| anyhow::anyhow!("Insert failed: {}", e))?;

        Ok(register_dto.id)
    }
}
