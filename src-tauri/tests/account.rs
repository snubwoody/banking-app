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

#[sqlx::test]
async fn get_accounts(pool: SqlitePool) -> Result<(),Error>{
    let accounts = AccountService::from_pool(pool.clone()).await;
    accounts.create_account("Name").await?;
    accounts.create_account("Name").await?;
    accounts.create_account("Name").await?;
    let all_accounts = accounts.get_accounts().await?;
    assert_eq!(all_accounts.len(),3);
    Ok(())
}

#[sqlx::test]
async fn add_category(pool: SqlitePool) -> Result<(),Error>{
    let accounts = AccountService::from_pool(pool.clone()).await;
    let category = accounts.add_category("My category").await?;
    let row = sqlx::query!("SELECT * FROM categories WHERE id = $1",category.id)
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.title,"My category");
    Ok(())
}

#[sqlx::test]
async fn no_duplicate_categories(pool: SqlitePool) -> Result<(),Error>{
    let accounts = AccountService::from_pool(pool.clone()).await;
    accounts.add_category("My category").await?;
    let result = accounts.add_category("My category").await;
    assert!(result.is_err());
    Ok(())
}

#[sqlx::test]
async fn get_categories(pool: SqlitePool) -> Result<(),Error>{
    let accounts = AccountService::from_pool(pool.clone()).await;
    let length = accounts.get_categories().await?.len();
    accounts.add_category("1").await?;
    accounts.add_category("2").await?;
    accounts.add_category("3").await?;
    let categories = accounts.get_categories().await?;
    assert_eq!(categories.len(),length+3);
    Ok(())
}
