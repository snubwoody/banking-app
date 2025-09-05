use std::str::FromStr;

use crate::Error;
use chrono::{Date, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};

#[derive(Debug, FromRow, Serialize, Deserialize, Default)]
pub struct Account {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Default)]
pub struct Transaction {
    pub id: i64,
    pub account: Account,
    pub amount: i64,
    pub category: Category,
    pub date: NaiveDate
}

#[derive(Debug, FromRow, Serialize, Deserialize, Default)]
pub struct Category {
    pub id: i64,
    pub title: String,
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

    pub async fn create_account(&self, name: &str) -> Result<Account, Error> {
        let account: Account = sqlx::query_as("INSERT INTO accounts(name) VALUES($1) RETURNING *")
            .bind(name)
            .fetch_one(&self.pool)
            .await?;

        Ok(account)
    }

    /// Get all the accounts.
    pub async fn get_accounts(&self) -> Result<Vec<Account>, Error> {
        let accounts: Vec<Account> = sqlx::query_as("SELECT * FROM accounts")
            .fetch_all(&self.pool)
            .await?;
        Ok(accounts)
    }

    /// Delete an account.
    pub async fn delete_account(&self, id: i64) -> Result<(), crate::Error> {
        sqlx::query("DELETE FROM accounts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn add_category(&self, title: &str) -> Result<Category, Error> {
        let category = sqlx::query_file_as!(Category, "queries/add_category.sql", title)
            .fetch_one(&self.pool)
            .await?;

        Ok(category)
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>, Error> {
        let categories = sqlx::query_file_as!(Category, "queries/get_categories.sql")
            .fetch_all(&self.pool)
            .await?;

        Ok(categories)
    }

    /// Create a new transaction.
    pub async fn add_transaction(
        &self,
        amount: i32,
        account: i64,
        category: i64,
        date: NaiveDate,
    ) -> Result<i64, Error> {
        let row = sqlx::query_file!(
            "queries/add_transaction.sql",
            amount,
            account,
            category,
            date
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.id)
    }

    /// Delete a transactions.
    pub async fn delete_transaction(&self, id: i64) {
        sqlx::query!("DELETE FROM transactions WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    /// Get all transactions belonging to a particular account.
    pub async fn get_transactions(&self, account_id: i64) -> Result<Vec<Transaction>,Error> {
        let rows = sqlx::query_file!("queries/get_transactions.sql",account_id)
            .fetch_all(&self.pool)
            .await?;


        let transactions: Vec<Transaction> = rows.into_iter().map(|row|{
            let category = Category{
                id: row.category_id,
                title: row.category_title
            };

            let account = Account{
                id: row.account_id,
                name: row.account_name
            };

            // FIXME
            let date = NaiveDate::from_str(&row.date).unwrap();
            Transaction{
                id: row.id,
                date,
                amount: row.amount,
                account,
                category,
            }
        }).collect();

        Ok(transactions)
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
    let accounts = accounts.get_accounts().await.unwrap();
    Ok(accounts)
}

#[tauri::command]
pub async fn delete_account(accounts: tauri::State<'_, AccountService>, id: i64) -> Result<(), ()> {
    accounts.delete_account(id).await.unwrap();
    tracing::info!("Deleted account: {id}");
    Ok(())
}

#[tauri::command]
pub async fn add_transaction(
    accounts: tauri::State<'_, AccountService>,
    amount: i32,
    account: i64,
    category: i64,
    date: NaiveDate,
) -> Result<(), ()> {
    accounts
        .add_transaction(amount, account, category, date)
        .await
        .unwrap();
    Ok(())
}

#[tauri::command]
pub async fn get_transactions(
    accounts: tauri::State<'_, AccountService>,
    account: i64,
) -> Result<Vec<Transaction>, ()> {
    let transactions = accounts.get_transactions(account).await.unwrap();
    Ok(transactions)
}
