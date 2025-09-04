use banking_app_lib::{db::Account, AccountService};

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
