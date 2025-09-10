use banking_app_lib::{AccountService, AppState, Result, service::TransactionService};
use chrono::NaiveDate;
use sqlx::SqlitePool;

#[sqlx::test]
async fn expense_is_negative(pool: SqlitePool) -> Result<()> {
    let state = AppState::from_pool(pool.clone());
    let transactions = TransactionService::new(state);

    let accounts = AccountService::from_pool(pool).await;
    let categories = &accounts.get_categories().await?;
    let account_type = &accounts.get_account_types().await?[0].id;
    let account = accounts.create_account("", account_type, 0.0).await?;
    let category_id = &categories[0].id;
    let date = NaiveDate::parse_from_str("2025-10-04", "%Y-%m-%d").unwrap();

    let id = transactions
        .add_expense(240.24, &account, category_id, date)
        .await?;

    let transaction = transactions.get_transaction(&id).await?;
    assert_eq!(transaction.amount, -240.24);
    assert_eq!(transaction.date, date);
    Ok(())
}

#[sqlx::test]
async fn income_is_positive(pool: SqlitePool) -> Result<()> {
    let state = AppState::from_pool(pool.clone());
    let transactions = TransactionService::new(state);

    let accounts = AccountService::from_pool(pool).await;
    let categories = &accounts.get_categories().await?;
    let account_type = &accounts.get_account_types().await?[0].id;
    let account = accounts.create_account("", account_type, 0.0).await?;
    let category_id = &categories[0].id;
    let date = NaiveDate::parse_from_str("2025-10-04", "%Y-%m-%d").unwrap();

    let id = transactions
        .add_income(240.24, &account, category_id, date)
        .await?;

    let transaction = transactions.get_transaction(&id).await?;
    assert_eq!(transaction.amount, 240.24);
    assert_eq!(transaction.date, date);
    Ok(())
}

#[sqlx::test]
async fn get_transaction(pool: SqlitePool) -> Result<()> {
    let state = AppState::from_pool(pool.clone());
    let transactions = TransactionService::new(state);

    let accounts = AccountService::from_pool(pool).await;
    let categories = &accounts.get_categories().await?;
    let account_type = &accounts.get_account_types().await?[0].id;
    let account = accounts.create_account("", account_type, 0.0).await?;
    let category_id = &categories[0].id;
    let date = NaiveDate::parse_from_str("2025-10-04", "%Y-%m-%d").unwrap();
    let transaction_id = accounts
        .add_transaction(24, &account, category_id, date)
        .await?;

    transactions.get_transaction(&transaction_id).await?;
    Ok(())
}
