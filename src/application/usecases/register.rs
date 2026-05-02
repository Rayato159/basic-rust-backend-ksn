use std::sync::Arc;

use crate::{
    application::models::register::{RegisterModel, RegisterResult},
    domain::repositories::register::RegisterRepository,
    infrastructure::argon2_hashing,
};
use anyhow::Result;

pub struct RegisterUseCase<T>
where
    T: RegisterRepository + Send + Sync + 'static,
{
    register_repository: Arc<T>,
}

impl<T> RegisterUseCase<T>
where
    T: RegisterRepository + Send + Sync + 'static,
{
    pub fn new(register_repository: Arc<T>) -> Self {
        Self {
            register_repository,
        }
    }

    pub async fn register(&self, mut register_model: RegisterModel) -> Result<RegisterResult> {
        let hashed_password = argon2_hashing::hash(register_model.password.clone())?;

        register_model.password = hashed_password;

        let user_id = self
            .register_repository
            .register(register_model.into())
            .await?;

        Ok(RegisterResult { user_id })
    }
}
