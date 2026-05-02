#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use uuid::Uuid;

    use crate::application::models::transfer::TransferModel;
    use crate::application::usecases::transfer::TransferUseCase;
    use crate::domain::dto::transfer::TransferDto;
    use crate::domain::repositories::transfer::MockTransferRepository;

    #[tokio::test]
    async fn test_create_transfer_success() {
        // Arrange
        let mut mock_repo = MockTransferRepository::new();
        let user_id = Uuid::new_v4();
        let transaction_id = Uuid::new_v4();
        let amount = Decimal::from(1000);
        let currency = "THB".to_string();

        mock_repo.expect_create_transaction().returning(move |dto| {
            let user_id = dto.user_id;
            let amount = dto.amount;
            let currency = dto.currency.clone();
            Box::pin(async move {
                Ok(TransferDto {
                    id: transaction_id,
                    user_id,
                    amount,
                    currency,
                    status: "pending".to_string(),
                    created_at: chrono::Utc::now(),
                    updated_at: None,
                })
            })
        });

        let usecase = TransferUseCase::new(std::sync::Arc::new(mock_repo));
        let transfer_model = TransferModel {
            amount,
            currency: currency.clone(),
        };

        // Act
        let result = usecase.create_transfer(user_id, transfer_model).await;

        // Assert
        assert!(result.is_ok());
        let transfer_result = result.unwrap();
        assert_eq!(transfer_result.transaction_id, transaction_id.to_string());
        assert_eq!(transfer_result.user_id, user_id.to_string());
        assert_eq!(transfer_result.amount, amount);
        assert_eq!(transfer_result.currency, currency);
    }

    #[tokio::test]
    async fn test_create_transfer_zero_amount() {
        // Arrange
        let mut mock_repo = MockTransferRepository::new();
        let user_id = Uuid::new_v4();
        let amount = Decimal::ZERO;
        let currency = "THB".to_string();
        let currency_for_mock = currency.clone();

        mock_repo.expect_create_transaction().return_once(move |_| {
            Box::pin(async move { Ok(TransferDto::new(Uuid::new_v4(), amount, currency_for_mock)) })
        });

        let usecase = TransferUseCase::new(std::sync::Arc::new(mock_repo));
        let transfer_model = TransferModel { amount, currency };

        // Act
        let result = usecase.create_transfer(user_id, transfer_model).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_transfer_negative_amount() {
        // Arrange
        let mut mock_repo = MockTransferRepository::new();
        let user_id = Uuid::new_v4();
        let amount = Decimal::from(-100);
        let currency = "THB".to_string();
        let currency_for_mock = currency.clone();

        mock_repo.expect_create_transaction().return_once(move |_| {
            Box::pin(async move { Ok(TransferDto::new(Uuid::new_v4(), amount, currency_for_mock)) })
        });

        let usecase = TransferUseCase::new(std::sync::Arc::new(mock_repo));
        let transfer_model = TransferModel { amount, currency };

        // Act - The business logic should handle negative amounts
        let result = usecase.create_transfer(user_id, transfer_model).await;

        // Assert - Currently, this will succeed. You may want to add validation
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_transfer_large_amount() {
        // Arrange
        let mut mock_repo = MockTransferRepository::new();
        let user_id = Uuid::new_v4();
        let amount = Decimal::from(999999999);
        let currency = "THB".to_string();
        let currency_for_mock = currency.clone();

        mock_repo.expect_create_transaction().return_once(move |_| {
            Box::pin(async move { Ok(TransferDto::new(Uuid::new_v4(), amount, currency_for_mock)) })
        });

        let usecase = TransferUseCase::new(std::sync::Arc::new(mock_repo));
        let transfer_model = TransferModel { amount, currency };

        // Act
        let result = usecase.create_transfer(user_id, transfer_model).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_transfer_different_currencies() {
        // Arrange
        let user_id = Uuid::new_v4();
        let currencies = vec!["THB", "USD", "EUR", "JPY"];

        for currency in currencies {
            let mut mock_repo = MockTransferRepository::new();
            mock_repo.expect_create_transaction().returning(|_| {
                Box::pin(async move {
                    Ok(TransferDto::new(
                        Uuid::new_v4(),
                        Decimal::from(1000),
                        "THB".to_string(),
                    ))
                })
            });

            let usecase = TransferUseCase::new(std::sync::Arc::new(mock_repo));
            let transfer_model = TransferModel {
                amount: Decimal::from(1000),
                currency: currency.to_string(),
            };

            // Act
            let result = usecase
                .create_transfer(user_id, transfer_model.clone())
                .await;

            // Assert
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_create_transfer_repository_error() {
        // Arrange
        let mut mock_repo = MockTransferRepository::new();
        let user_id = Uuid::new_v4();
        let amount = Decimal::from(1000);
        let currency = "THB".to_string();

        mock_repo
            .expect_create_transaction()
            .returning(|_| Box::pin(async move { Err(anyhow::anyhow!("Database error")) }));

        let usecase = TransferUseCase::new(std::sync::Arc::new(mock_repo));
        let transfer_model = TransferModel { amount, currency };

        // Act
        let result = usecase.create_transfer(user_id, transfer_model).await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database error"));
    }

    #[tokio::test]
    async fn test_create_transfer_default_currency() {
        // Arrange
        let mut mock_repo = MockTransferRepository::new();
        let user_id = Uuid::new_v4();
        let amount = Decimal::from(1000);

        mock_repo.expect_create_transaction().returning(move |_| {
            Box::pin(async move { Ok(TransferDto::new(Uuid::new_v4(), amount, "THB".to_string())) })
        });

        let usecase = TransferUseCase::new(std::sync::Arc::new(mock_repo));
        let transfer_model = TransferModel {
            amount,
            currency: "THB".to_string(), // Default currency
        };

        // Act
        let result = usecase.create_transfer(user_id, transfer_model).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().currency, "THB");
    }
}
