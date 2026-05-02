use crate::domain::dto::register::RegisterDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize, Clone, ToSchema)]
pub struct RegisterModel {
    #[validate(length(min = 6))]
    #[schema(example = "ksn_user")]
    pub username: String,
    #[validate(length(min = 6))]
    #[schema(example = "password1234")]
    pub password: String,
}

impl From<RegisterModel> for RegisterDto {
    fn from(model: RegisterModel) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            username: model.username,
            password: model.password,
            created_at: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResult {
    #[schema(value_type = String, example = "550e8400-e29b-41d4-a716-446655440000")]
    pub user_id: Uuid,
}
