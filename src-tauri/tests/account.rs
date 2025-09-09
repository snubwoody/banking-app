use banking_app_lib::{AccountService, Error};
use chrono::NaiveDate;
use sqlx::SqlitePool;

#[sqlx::test]
async fn create_account(pool: SqlitePool) -> Result<(), Error> {
    let accounts = AccountService::from_pool(pool.clone()).await;
    let account_type = &accounts.get_account_types().await?[0].id;
    let id = accounts.create_account("Name", account_type, 240).await?;
    let row = sqlx::query!("SELECT * FROM accounts WHERE id=$1", id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.starting_balance, 240);
    assert_eq!(&row.name, "Name");
    Ok(())
}

#[sqlx::test]
async fn delete_account(pool: SqlitePool) -> Result<(), Error> {
    let accounts = AccountService::from_pool(pool.clone()).await;
    let account_type = &accounts.get_account_types().await?[0].id;
    let id = accounts.create_account("Name", account_type, 200).await?;
    let rows = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(rows.len(), 1);
    accounts.delete_account(&id).await?;
    let rows = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert!(rows.is_empty());
    Ok(())
}

#[sqlx::test]
async fn get_accounts(pool: SqlitePool) -> Result<(), Error> {
    let accounts = AccountService::from_pool(pool.clone()).await;
    let account_type = &accounts.get_account_types().await?[0].id;
    accounts.create_account("Name", account_type, 0).await?;
    accounts.create_account("Name", account_type, 0).await?;
    accounts.create_account("Name", account_type, 0).await?;
    let all_accounts = accounts.get_accounts().await?;
    assert_eq!(all_accounts.len(), 3);
    Ok(())
}

#[sqlx::test]
async fn add_category(pool: SqlitePool) -> Result<(), Error> {
    let accounts = AccountService::from_pool(pool.clone()).await;
    let category = accounts.add_category("My category").await?;
    let row = sqlx::query!("SELECT * FROM categories WHERE id = $1", category.id)
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.title, "My category");
    Ok(())
}

#[sqlx::test]
async fn no_duplicate_categories(pool: SqlitePool) -> Result<(), Error> {
    let accounts = AccountService::from_pool(pool.clone()).await;
    accounts.add_category("My category").await?;
    let result = accounts.add_category("My category").await;
    assert!(result.is_err());
    Ok(())
}

#[sqlx::test]
async fn get_categories(pool: SqlitePool) -> Result<(), Error> {
    let accounts = AccountService::from_pool(pool.clone()).await;
    let length = accounts.get_categories().await?.len();
    accounts.add_category("1").await?;
    accounts.add_category("2").await?;
    accounts.add_category("3").await?;
    let categories = accounts.get_categories().await?;
    assert_eq!(categories.len(), length + 3);
    Ok(())
}

#[sqlx::test]
async fn add_transaction(pool: SqlitePool) -> Result<(), Error> {
    let accounts = AccountService::from_pool(pool.clone()).await;
    let account_type = &accounts.get_account_types().await?[0].id;
    let id = accounts.create_account("", account_type, 2).await?;
    let category = &accounts.get_categories().await?[0];
    let date = NaiveDate::parse_from_str("2025-10-04", "%Y-%m-%d").unwrap();

    let transaction_id = accounts
        .add_transaction(100, &id, &category.id, date)
        .await?;
    let transaction = sqlx::query!("SELECT * FROM transactions WHERE id=$1", transaction_id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(transaction.amount, 100);
    assert_eq!(transaction.category, category.id);
    assert_eq!(transaction.account, id);
    Ok(())
}

#[sqlx::test]
async fn get_transactions(pool: SqlitePool) -> Result<(), Error> {
    let accounts = AccountService::from_pool(pool.clone()).await;
    let account_type = &accounts.get_account_types().await?[0].id;
    let id = &accounts.create_account("", account_type, 20).await?;
    let category = &accounts.get_categories().await?[0];
    let date = NaiveDate::parse_from_str("2025-10-04", "%Y-%m-%d").unwrap();

    accounts
        .add_transaction(100, id, &category.id, date)
        .await?;
    accounts
        .add_transaction(500, id, &category.id, date)
        .await?;
    let transactions = accounts.get_transactions(id).await?;
    assert_eq!(transactions.len(), 2);
    Ok(())
}
