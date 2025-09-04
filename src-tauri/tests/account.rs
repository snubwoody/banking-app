use banking_app_lib::{db::{Account, Transaction}, AccountService};
use sqlx::SqlitePool;

#[tokio::test]
async fn create_account() {
    let accounts = AccountService::new().await;
    let account = accounts.create_account("Name").await;
    dbg!(&account);
    let url = "../data.db";
    let pool = sqlx::SqlitePool::connect(&url).await.unwrap();
    let row: Account = sqlx::query_as("SELECT * FROM accounts WHERE id=$1")
        .bind(account.id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(account.id, row.id);
    assert_eq!(account.name, row.name);
}

#[tokio::test]
async fn delete_account() {
    let accounts = AccountService::new().await;
    let account = accounts.create_account("Name").await;
    accounts.delete_account(account.id).await;
}


#[sqlx::test(migrations="../migrations")]
async fn add_transaction(pool: SqlitePool) {
    let accounts = AccountService::from_pool(pool.clone()).await;
    let account = accounts.create_account("Name").await;
    let transaction = accounts.add_transaction(account.id, 10_000).await;
    let row: Transaction = sqlx::query_as("SELECT * FROM transactions WHERE id=$1")
        .bind(transaction.id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(row.amount,10_000);
}
