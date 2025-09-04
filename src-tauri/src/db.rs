use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};
use crate::Error;

#[derive(Debug, FromRow, Serialize, Deserialize,Default)]
pub struct Account {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize,Default)]
pub struct Transaction {
    pub id: i64,
    pub account: Account,
    pub amount: i32,
    pub category: Category
}

#[derive(Debug,FromRow,Serialize,Deserialize,Default)]
pub struct Category{
    id: i32,
    title: String
}

pub struct AccountService {
    pool: SqlitePool,
}

impl AccountService {
    pub async fn new() -> Self {
        let url = "data.db";
        let pool = sqlx::SqlitePool::connect(&url).await.unwrap();
        Self { pool }
    }

    pub async fn from_pool(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_account(&self, name: &str) -> Result<Account,Error> {
        let account: Account = sqlx::query_as("INSERT INTO accounts(name) VALUES($1) RETURNING *")
            .bind(name)
            .fetch_one(&self.pool)
            .await?;

        Ok(account)
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
    pub async fn delete_account(&self, id: i64) -> Result<(),crate::Error> {
        sqlx::query("DELETE FROM accounts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn add_category(&self, title: &str){
        // sqlx::query_file!("queries/add_category.sql",title)
    }

    /// Create a new transaction.
    pub async fn add_transaction(&self, amount: i32,account: i64, category: i64) -> i64{
        let row = sqlx::query_file!(
            "queries/add_transaction.sql",
            amount,
            account,
            category
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        row.id
    }

    /// Delete a transactions.
    pub async fn delete_transaction(&self, id: i64){
        sqlx::query!(
            "DELETE FROM transactions WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    /// Get all transactions belonging to a particular account.
    pub async fn get_transactions(&self, account_id: i64) -> Vec<Transaction>{
        let rows = sqlx::query_file!("queries/get_all_transactions.sql")
        .fetch_all(&self.pool)
        .await
        .unwrap();
        dbg!(rows);

        vec![]
    }
}

#[tauri::command]
pub async fn create_account(
    accounts: tauri::State<'_, AccountService>,
    name: String,
) -> Result<Account, ()> {
    let account = accounts.create_account(&name).await.unwrap();
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
    id: i64,
) -> Result<(), ()> {
    accounts.delete_account(id).await;
    tracing::info!("Deleted account: {id}");
    Ok(())
}

#[tauri::command]
pub async fn add_transaction(
    accounts: tauri::State<'_, AccountService>,
    amount: i32,
    account: i64,
    category: i64,
) -> Result<(), ()> {
    accounts.add_transaction(amount,account,category).await;
    Ok(())
}


#[tauri::command]
pub async fn get_transactions(
    accounts: tauri::State<'_, AccountService>,
    account: i64,
) -> Result<Vec<Transaction>, ()> {
    let transactions = accounts.get_transactions(account).await;
    Ok(transactions)
}
