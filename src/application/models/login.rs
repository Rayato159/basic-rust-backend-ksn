use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::dto::login::LoginDto;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct LoginModel {
    #[schema(example = "ksn_user")]
    pub username: String,
    #[schema(example = "password1234")]
    pub password: String,
}

impl From<LoginDto> for LoginModel {
    fn from(dto: LoginDto) -> Self {
        Self {
            username: dto.username,
            password: dto.password,
        }
    }
}
