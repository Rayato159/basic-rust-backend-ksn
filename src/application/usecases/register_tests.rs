#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::application::models::register::RegisterModel;
    use crate::application::usecases::register::RegisterUseCase;
    use crate::domain::repositories::register::MockRegisterRepository;

    #[tokio::test]
    async fn test_register_success() {
        // Arrange
        let mut mock_repo = MockRegisterRepository::new();
        let username = "newuser".to_string();
        let password = "password123".to_string();

        let user_id = Uuid::new_v4();
        mock_repo
            .expect_register()
            .returning(move |_| Box::pin(async move { Ok(user_id) }));

        let usecase = RegisterUseCase::new(std::sync::Arc::new(mock_repo));

        let register_model = RegisterModel {
            username: username.clone(),
            password: password.clone(),
        };

        // Act
        let result = usecase.register(register_model).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_existing_user() {
        // Arrange
        let mut mock_repo = MockRegisterRepository::new();
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_register()
            .returning(move |_| Box::pin(async move { Ok(user_id) }));

        let usecase = RegisterUseCase::new(std::sync::Arc::new(mock_repo));

        let register_model = RegisterModel {
            username: "existinguser".to_string(),
            password: "password123".to_string(),
        };

        // Act
        let result = usecase.register(register_model).await;

        // Assert
        assert!(result.is_ok());
    }
}
