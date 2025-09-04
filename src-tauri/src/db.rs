use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub account: Uuid,
    pub amount: i32,
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

    pub async fn from_pool(pool: SqlitePool) -> Self {
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

    /// Create a new transaction.
    pub async fn add_transaction(&self, account_id: Uuid, amount: i32) -> Transaction{
        let transaction: Transaction = sqlx::query_as(
            "INSERT INTO transactions(id,amount,account) 
            VALUES($1,$2,$3) 
            RETURNING *",
        )
        .bind(Uuid::new_v4())
        .bind(amount)
        .bind(account_id)
        .fetch_one(&self.pool)
        .await
        .unwrap();

        transaction
    }

    /// Delete a transactions.
    pub async fn delete_transaction(&self, id: Uuid) -> Transaction{
        let transaction: Transaction = sqlx::query_as(
            "DELETE FROM transactions WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .unwrap();

        transaction
    }

    /// Get all transactions belonging to a particular account.
    pub async fn get_transactions(&self, account_id: Uuid) -> Vec<Transaction>{
        let transactions: Vec<Transaction> = sqlx::query_as(
            "SELECT * FROM transactions WHERE account = $1"
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .unwrap();

        transactions
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
    id: Uuid,
) -> Result<(), ()> {
    accounts.delete_account(id).await;
    tracing::info!("Deleted account: {id}");
    Ok(())
}

#[tauri::command]
pub async fn add_transaction(
    accounts: tauri::State<'_, AccountService>,
    account: Uuid,
    amount: i32
) -> Result<(), ()> {
    accounts.add_transaction(account,amount).await;
    Ok(())
}


#[tauri::command]
pub async fn get_transactions(
    accounts: tauri::State<'_, AccountService>,
    account: Uuid,
) -> Result<Vec<Transaction>, ()> {
    let transactions = accounts.get_transactions(account).await;
    Ok(transactions)
}
