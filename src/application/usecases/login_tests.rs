#[cfg(test)]
mod tests {
    use chrono::Utc;
    use mockall::predicate::*;
    use uuid::Uuid;

    use crate::application::models::login::LoginModel;
    use crate::application::usecases::login::LoginUseCase;
    use crate::domain::entities::users::User;
    use crate::domain::repositories::login::MockLoginRepository;

    #[tokio::test]
    async fn test_login_success() {
        // Arrange
        let mut mock_repo = MockLoginRepository::new();
        let user_id = Uuid::new_v4();
        let username = "testuser".to_string();
        let plain_password = "correct_password".to_string();
        let hashed_password =
            crate::infrastructure::argon2_hashing::hash(plain_password.clone()).unwrap();
        let username_for_mock = username.clone();
        let username_for_with = username.clone();

        mock_repo
            .expect_get_user()
            .with(eq(username_for_with))
            .return_once(move |_| {
                Box::pin(async move {
                    Ok(Some(User {
                        id: user_id,
                        username: username_for_mock,
                        password: hashed_password,
                        created_at: Utc::now(),
                        updated_at: None,
                    }))
                })
            });

        let usecase = LoginUseCase::new(std::sync::Arc::new(mock_repo));

        let login_model = LoginModel {
            username: username.clone(),
            password: plain_password.clone(),
        };

        // Act
        let result = usecase
            .login(login_model, "test_secret".to_string(), "24h".to_string())
            .await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        // Arrange
        let mut mock_repo = MockLoginRepository::new();
        let username = "nonexistent_user".to_string();

        mock_repo
            .expect_get_user()
            .with(eq(username.clone()))
            .returning(|_| Box::pin(async move { Ok(None) }));

        let usecase = LoginUseCase::new(std::sync::Arc::new(mock_repo));

        let login_model = LoginModel {
            username: username.clone(),
            password: "any_password".to_string(),
        };

        // Act
        let result = usecase
            .login(login_model, "test_secret".to_string(), "24h".to_string())
            .await;

        // Assert
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid username or password")
        );
    }
}
