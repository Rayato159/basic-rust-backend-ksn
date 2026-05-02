use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

use crate::domain::entities::users::User;

#[async_trait]
#[automock]
pub trait LoginRepository {
    async fn get_user(&self, username: &str) -> Result<Option<User>>;
}
