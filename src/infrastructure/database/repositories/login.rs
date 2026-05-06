use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tiberius::Query;
use tokio::sync::Mutex;

use crate::{
    domain::{entities::users::User, repositories::login::LoginRepository},
    infrastructure::database::MSSQLClient,
};

pub struct LoginMSSQL {
    db_client: Arc<Mutex<MSSQLClient>>,
}

impl LoginMSSQL {
    pub fn new(db_client: Arc<Mutex<MSSQLClient>>) -> Self {
        Self { db_client }
    }
}

#[async_trait]
impl LoginRepository for LoginMSSQL {
    async fn get_user(&self, username: &str) -> Result<Option<User>> {
        let mut client = self.db_client.lock().await;

        let sql = "
            SELECT id, username, password, created_at, updated_at
            FROM users
            WHERE username = @p1
        ";

        let mut query = Query::new(sql);

        query.bind(username);

        let row = query
            .query(&mut *client)
            .await
            .map_err(|e| anyhow::anyhow!("Query failed: {}", e))?
            .into_row()
            .await
            .map_err(|e| anyhow::anyhow!("Row fetch failed: {}", e))?;

        match row {
            Some(row) => {
                let id: uuid::Uuid = row
                    .get(0)
                    .ok_or_else(|| anyhow::anyhow!("Missing id field"))?;
                let username: &str = row
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("Missing username field"))?;
                let password: &str = row
                    .get(2)
                    .ok_or_else(|| anyhow::anyhow!("Missing password field"))?;
                let created_at: chrono::DateTime<chrono::Utc> = row
                    .get(3)
                    .ok_or_else(|| anyhow::anyhow!("Missing created_at field"))?;
                let updated_at: Option<chrono::DateTime<chrono::Utc>> = row.get(4);

                let user = User {
                    id,
                    username: username.to_string(),
                    password: password.to_string(),
                    created_at,
                    updated_at,
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }
}
