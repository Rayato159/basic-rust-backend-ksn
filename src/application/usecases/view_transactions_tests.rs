#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    use rust_decimal::Decimal;
    use uuid::Uuid;

    use crate::application::usecases::view_transactions::ViewTransactionsUseCase;
    use crate::domain::dto::transfer::TransferDto;
    use crate::domain::repositories::view_transactions::MockViewTransactionsRepository;

    #[tokio::test]
    async fn test_view_transactions_success() {
        // Arrange
        let mut mock_repo = MockViewTransactionsRepository::new();
        let user_id = Uuid::new_v4();
        let authenticated_user_id = user_id; // Same user

        let transactions = vec![
            TransferDto::new(user_id, Decimal::from(1000), "THB".to_string()),
            TransferDto::new(user_id, Decimal::from(2000), "THB".to_string()),
            TransferDto::new(user_id, Decimal::from(500), "USD".to_string()),
        ];

        mock_repo
            .expect_get_transactions_by_user_id()
            .with(eq(user_id))
            .return_once(move |_| Box::pin(async move { Ok(transactions) }));

        let usecase = ViewTransactionsUseCase::new(std::sync::Arc::new(mock_repo));

        // Act
        let result = usecase
            .view_transactions(authenticated_user_id, user_id)
            .await;

        // Assert
        assert!(result.is_ok());
        let transactions_result = result.unwrap();
        assert_eq!(transactions_result.len(), 3);
    }

    #[tokio::test]
    async fn test_view_transactions_access_denied() {
        // Arrange
        let mock_repo = MockViewTransactionsRepository::new();
        let user_id = Uuid::new_v4();
        let authenticated_user_id = Uuid::new_v4(); // Different user

        let usecase = ViewTransactionsUseCase::new(std::sync::Arc::new(mock_repo));

        // Act
        let result = usecase
            .view_transactions(authenticated_user_id, user_id)
            .await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Access denied"));
    }

    #[tokio::test]
    async fn test_view_transactions_empty_list() {
        // Arrange
        let mut mock_repo = MockViewTransactionsRepository::new();
        let user_id = Uuid::new_v4();
        let authenticated_user_id = user_id; // Same user

        mock_repo
            .expect_get_transactions_by_user_id()
            .with(eq(user_id))
            .returning(|_| Box::pin(async move { Ok(vec![]) }));

        let usecase = ViewTransactionsUseCase::new(std::sync::Arc::new(mock_repo));

        // Act
        let result = usecase
            .view_transactions(authenticated_user_id, user_id)
            .await;

        // Assert
        assert!(result.is_ok());
        let transactions_result = result.unwrap();
        assert_eq!(transactions_result.len(), 0);
    }

    #[tokio::test]
    async fn test_view_transactions_single_transaction() {
        // Arrange
        let mut mock_repo = MockViewTransactionsRepository::new();
        let user_id = Uuid::new_v4();
        let authenticated_user_id = user_id; // Same user

        let transactions = vec![TransferDto::new(
            user_id,
            Decimal::from(1500),
            "EUR".to_string(),
        )];

        mock_repo
            .expect_get_transactions_by_user_id()
            .with(eq(user_id))
            .return_once(move |_| Box::pin(async move { Ok(transactions) }));

        let usecase = ViewTransactionsUseCase::new(std::sync::Arc::new(mock_repo));

        // Act
        let result = usecase
            .view_transactions(authenticated_user_id, user_id)
            .await;

        // Assert
        assert!(result.is_ok());
        let transactions_result = result.unwrap();
        assert_eq!(transactions_result.len(), 1);
    }

    #[tokio::test]
    async fn test_view_transactions_many_transactions() {
        // Arrange
        let mut mock_repo = MockViewTransactionsRepository::new();
        let user_id = Uuid::new_v4();
        let authenticated_user_id = user_id; // Same user

        let transactions: Vec<TransferDto> = (0..100)
            .map(|_| TransferDto::new(user_id, Decimal::from(100), "THB".to_string()))
            .collect();

        mock_repo
            .expect_get_transactions_by_user_id()
            .with(eq(user_id))
            .return_once(move |_| Box::pin(async move { Ok(transactions) }));

        let usecase = ViewTransactionsUseCase::new(std::sync::Arc::new(mock_repo));

        // Act
        let result = usecase
            .view_transactions(authenticated_user_id, user_id)
            .await;

        // Assert
        assert!(result.is_ok());
        let transactions_result = result.unwrap();
        assert_eq!(transactions_result.len(), 100);
    }

    #[tokio::test]
    async fn test_view_transactions_repository_error() {
        // Arrange
        let mut mock_repo = MockViewTransactionsRepository::new();
        let user_id = Uuid::new_v4();
        let authenticated_user_id = user_id; // Same user

        mock_repo
            .expect_get_transactions_by_user_id()
            .with(eq(user_id))
            .returning(|_| Box::pin(async move { Err(anyhow::anyhow!("Database error")) }));

        let usecase = ViewTransactionsUseCase::new(std::sync::Arc::new(mock_repo));

        // Act
        let result = usecase
            .view_transactions(authenticated_user_id, user_id)
            .await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }

    #[tokio::test]
    async fn test_view_transactions_different_amounts() {
        // Arrange
        let mut mock_repo = MockViewTransactionsRepository::new();
        let user_id = Uuid::new_v4();
        let authenticated_user_id = user_id; // Same user

        let transactions = vec![
            TransferDto::new(user_id, Decimal::from(0), "THB".to_string()),
            TransferDto::new(user_id, Decimal::from(1), "THB".to_string()),
            TransferDto::new(user_id, Decimal::from(1000000), "THB".to_string()),
        ];

        mock_repo
            .expect_get_transactions_by_user_id()
            .with(eq(user_id))
            .return_once(move |_| Box::pin(async move { Ok(transactions) }));

        let usecase = ViewTransactionsUseCase::new(std::sync::Arc::new(mock_repo));

        // Act
        let result = usecase
            .view_transactions(authenticated_user_id, user_id)
            .await;

        // Assert
        assert!(result.is_ok());
        let transactions_result = result.unwrap();
        assert_eq!(transactions_result.len(), 3);
    }

    #[tokio::test]
    async fn test_view_transactions_same_user_id_uuids() {
        // Arrange
        let mut mock_repo = MockViewTransactionsRepository::new();
        let user_id = Uuid::new_v4();
        let authenticated_user_id = user_id; // Same Uuid instance

        mock_repo
            .expect_get_transactions_by_user_id()
            .with(eq(user_id))
            .returning(|_| Box::pin(async move { Ok(vec![]) }));

        let usecase = ViewTransactionsUseCase::new(std::sync::Arc::new(mock_repo));

        // Act
        let result = usecase
            .view_transactions(authenticated_user_id, user_id)
            .await;

        // Assert
        assert!(result.is_ok());
    }
}
