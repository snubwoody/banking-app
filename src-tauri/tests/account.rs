use banking_app_lib::{db::{Account}, AccountService, Error};
use sqlx::SqlitePool;

#[sqlx::test]
async fn create_account(pool: SqlitePool) -> Result<(),Error>{
    let accounts = AccountService::from_pool(pool.clone()).await;
    let account = accounts.create_account("Name").await?;
    let row: Account = sqlx::query_as("SELECT * FROM accounts WHERE id=$1")
        .bind(account.id)
        .fetch_one(&pool)
        .await?;

    assert_eq!(account.id, row.id);
    assert_eq!(account.name, row.name);
    Ok(())
}


#[sqlx::test]
async fn delete_account(pool: SqlitePool) -> Result<(),Error>{
    let accounts = AccountService::from_pool(pool.clone()).await;
    let account = accounts.create_account("Name").await?;
    let rows = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert_eq!(rows.len(),1);
    accounts.delete_account(account.id).await?;
    let rows = sqlx::query!("SELECT * FROM accounts")
        .fetch_all(&pool)
        .await?;
    assert!(rows.is_empty());
    Ok(())
}