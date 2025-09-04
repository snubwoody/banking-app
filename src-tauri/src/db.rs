use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
}

pub struct AccountService {
    pool: SqlitePool,
}

impl AccountService {
    pub async fn new() -> Self {
        let url = "../data.db";
        let pool = sqlx::SqlitePool::connect(&url).await.unwrap();
        Self { pool }
    }

    pub async fn create_account(&self, name: &str) -> Account {
        let id = Uuid::new_v4();

        let account = sqlx::query_as("INSERT INTO accounts(id,name) VALUES($1,$2) RETURNING *")
            .bind(id)
            .bind(name)
            .fetch_one(&self.pool)
            .await
            .unwrap();


        account
    }

    /// Get all the accounts
    pub async fn get_accounts(&self) -> Vec<Account> {
        let accounts: Vec<Account> = sqlx::query_as("SELECT * FROM accounts")
            .fetch_all(&self.pool)
            .await
            .unwrap();

        accounts
    }

    /// Delete an account.
    pub async fn delete_account(&self, id: Uuid) {
        sqlx::query("DELETE FROM accounts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .unwrap();
    }
}

#[tauri::command]
pub async fn create_account(
    accounts: tauri::State<'_, AccountService>,
    name: String,
) -> Result<Account, ()> {
    let account = accounts.create_account(&name).await;
    tracing::info!("Created new account: {name}");
    Ok(account)
}

#[tauri::command]
pub async fn fetch_accounts(
    accounts: tauri::State<'_, AccountService>,
) -> Result<Vec<Account>, ()> {
    let accounts = accounts.get_accounts().await;
    Ok(accounts)
}

#[tauri::command]
pub async fn delete_account(
    accounts: tauri::State<'_, AccountService>,
    id: Uuid
) -> Result<(),()>{
    accounts.delete_account(id).await;
    tracing::info!("Deleted account: {id}");
    Ok(())
}
