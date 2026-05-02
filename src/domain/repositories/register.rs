use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use uuid::Uuid;

use crate::domain::dto::register::RegisterDto;

#[async_trait]
#[automock]
pub trait RegisterRepository {
    async fn register(&self, register_dto: RegisterDto) -> Result<Uuid>;
}
