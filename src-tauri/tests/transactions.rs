use banking_app_lib::{AccountService, AppState, Result, service::TransactionService};
use chrono::NaiveDate;
use sqlx::SqlitePool;

#[sqlx::test]
async fn add_expense(pool: SqlitePool) -> Result<()> {
    let state = AppState::from_pool(pool.clone());
    let transactions = TransactionService::new(state);

    let accounts = AccountService::new().await?;
    let category_id = &accounts.get_categories().await?[0].id;
    let account_type = &accounts.get_account_types().await?[0].id;
    let account = accounts.create_account("", account_type, 0.0).await?;

    let date = NaiveDate::parse_from_str("2025-10-04", "%Y-%m-%d").unwrap();
    transactions
        .add_expense(240.24, &account, category_id, date)
        .await;
    Ok(())
}
