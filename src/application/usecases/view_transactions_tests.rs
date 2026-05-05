#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    use rust_decimal::Decimal;
    use uuid::Uuid;

    use crate::application::usecases::view_transactions::ViewTransactionsUseCase;
    use crate::domain::entities::transactions::Transaction;
    use crate::domain::repositories::view_transactions::MockViewTransactionsRepository;
    use chrono::Utc;

    #[tokio::test]
    async fn test_view_transactions_single_transaction() {
        // Arrange
        let mut mock_repo = MockViewTransactionsRepository::new();
        let user_id = Uuid::new_v4();
        let authenticated_user_id = user_id; // Same user

        let transactions = vec![Transaction {
            id: Uuid::new_v4(),
            user_id,
            amount: Decimal::from(1500),
            currency: "EUR".to_string(),
            status: "completed".to_string(),
            created_at: Utc::now(),
            updated_at: None,
        }];

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

        let transactions: Vec<Transaction> = (0..100)
            .map(|_| Transaction {
                id: Uuid::new_v4(),
                user_id,
                amount: Decimal::from(100),
                currency: "THB".to_string(),
                status: "pending".to_string(),
                created_at: Utc::now(),
                updated_at: None,
            })
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
}
