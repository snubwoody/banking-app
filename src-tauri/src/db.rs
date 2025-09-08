use crate::Error;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, prelude::FromRow};
use std::str::FromStr;

#[derive(Debug, FromRow, Serialize, Deserialize, Default)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub account_type: AccountType,
    pub starting_balance: i64,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Default)]
pub struct Transaction {
    pub id: i64,
    pub account: Account,
    pub amount: i64,
    pub category: Category,
    pub date: NaiveDate,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Default)]
pub struct Category {
    pub id: i64,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Default, FromRow)]
pub struct AccountType {
    pub id: i64,
    pub title: String,
}

pub struct AccountService {
    pool: SqlitePool,
}

impl AccountService {
    pub async fn new() -> Self {
        let url = "data.db";
        let pool = sqlx::SqlitePool::connect(url).await.unwrap();
        Self { pool }
    }

    pub async fn from_pool(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_account(
        &self,
        name: &str,
        account_type: i64,
        starting_balance: i64,
    ) -> Result<i64, Error> {
        let row = sqlx::query_file!(
            "queries/create_account.sql",
            name,
            account_type,
            starting_balance
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.id)
    }

    pub async fn get_account_types(&self) -> Result<Vec<AccountType>, Error> {
        let account_types = sqlx::query_file_as!(AccountType, "queries/get_account_types.sql")
            .fetch_all(&self.pool)
            .await?;

        Ok(account_types)
    }

    /// Get all the accounts.
    pub async fn get_accounts(&self) -> Result<Vec<Account>, Error> {
        // FIXME
        let rows = sqlx::query_file!("queries/get_accounts.sql")
            .fetch_all(&self.pool)
            .await?;

        let accounts: Vec<Account> = rows
            .into_iter()
            .map(|row| Account {
                id: row.id,
                name: row.name,
                starting_balance: row.starting_balance,
                account_type: AccountType {
                    id: row.account_type_id,
                    title: row.account_type,
                },
            })
            .collect();
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
    pub async fn get_transactions(&self, account_id: i64) -> Result<Vec<Transaction>, Error> {
        let rows = sqlx::query_file!("queries/get_transactions.sql", account_id)
            .fetch_all(&self.pool)
            .await?;

        let transactions: Vec<Transaction> = rows
            .into_iter()
            .map(|row| {
                let category = Category {
                    id: row.category_id,
                    title: row.category_title,
                };

                let account = Account {
                    id: row.account_id,
                    starting_balance: row.account_starting_balance,
                    name: row.account_name,
                    account_type: AccountType {
                        id: row.account_type_id,
                        title: row.account_type,
                    },
                };

                // FIXME
                let date = NaiveDate::from_str(&row.date).unwrap();
                Transaction {
                    id: row.id,
                    date,
                    amount: row.amount,
                    account,
                    category,
                }
            })
            .collect();

        Ok(transactions)
    }

    /// Get all the transactions.
    pub async fn get_all_transactions(&self) -> Result<Vec<Transaction>, Error> {
        let rows = sqlx::query_file!("queries/get_all_transactions.sql")
            .fetch_all(&self.pool)
            .await?;

        let transactions: Vec<Transaction> = rows
            .into_iter()
            .map(|row| {
                let category = Category {
                    id: row.category_id,
                    title: row.category_title,
                };

                let account = Account {
                    id: row.account_id,
                    starting_balance: row.account_starting_balance,
                    name: row.account_name,
                    account_type: AccountType {
                        id: row.account_type_id,
                        title: row.account_type,
                    },
                };

                // FIXME
                let date = NaiveDate::from_str(&row.date).unwrap();
                Transaction {
                    id: row.id,
                    date,
                    amount: row.amount,
                    account,
                    category,
                }
            })
            .collect();

        Ok(transactions)
    }
}

#[tauri::command]
pub async fn get_categories(
    accounts: tauri::State<'_, AccountService>,
) -> Result<Vec<Category>, ()> {
    let categories = accounts.get_categories().await.unwrap();
    Ok(categories)
}

#[tauri::command]
pub async fn create_account(
    accounts: tauri::State<'_, AccountService>,
    name: String,
    account_type: i64,
    starting_balance: i64,
) -> Result<(), ()> {
    accounts
        .create_account(&name, account_type, starting_balance)
        .await
        .unwrap();
    tracing::info!(name, starting_balance, "Created account");
    Ok(())
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
    tracing::info!(
        date=%date,
        amount=amount,
        "New transaction"
    );
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

#[tauri::command]
pub async fn get_all_transactions(
    accounts: tauri::State<'_, AccountService>,
) -> Result<Vec<Transaction>, ()> {
    let transactions = accounts.get_all_transactions().await.unwrap();
    Ok(transactions)
}

#[tauri::command]
pub async fn get_account_types(
    accounts: tauri::State<'_, AccountService>,
) -> Result<Vec<AccountType>, ()> {
    let account_types = accounts.get_account_types().await.unwrap();
    Ok(account_types)
}
