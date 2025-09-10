use crate::{service::{Account, AccountType}, AppState};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, FromRow, Serialize, Deserialize, Default)]
pub struct Transaction {
    pub id: String,
    pub account: Account,
    pub amount: f64,
    pub category: Category,
    pub date: NaiveDate,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Default)]
pub struct Category {
    pub id: String,
    pub title: String,
}

pub struct TransactionService {
    state: AppState,
}

impl TransactionService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn get_transaction(&self, id: &str) -> crate::Result<Transaction>{
        let pool = self.state.pool();
        let row = sqlx::query_file!("queries/get_transaction.sql",id)
            .fetch_one(pool)
            .await?;

        let account_type = AccountType{
            id: row.account_type_id,
            title: row.account_type_title
        };

        // TODO: maybe return error
        let date = NaiveDate::parse_from_str(&row.date, "%Y-%m-%d")
            .unwrap_or_default();

        let transaction = Transaction{
            id: row.id,
            date,
            amount: row.amount,
            account: Account { 
                id: row.account_id, 
                name: row.account_name, 
                account_type, 
                starting_balance: row.account_starting_balance 
            },
            category: Category { 
                id: row.category_id, 
                title: row.category_title 
            }
        };
        Ok(transaction)
    }

    /// Add an expense to the database
    pub async fn add_expense(
        &self,
        amount: f64,
        account_id: &str,
        category_id: &str,
        date: NaiveDate,
    ) -> crate::Result<String> {
        let pool = self.state.pool();
        let amount = - amount;
        let row = sqlx::query_file!(
            "queries/add_new_transaction.sql",
            amount,
            account_id,
            category_id,
            date
        )
        .fetch_one(pool)
        .await?;

        Ok(row.id)
    }

    /// Add an income to the database
    pub async fn add_income(
        &self,
        amount: f64,
        account_id: &str,
        category_id: &str,
        date: NaiveDate,
    ) -> crate::Result<String> {
        let pool = self.state.pool();
        let row = sqlx::query_file!(
            "queries/add_new_transaction.sql",
            amount,
            account_id,
            category_id,
            date
        )
        .fetch_one(pool)
        .await?;

        Ok(row.id)
    }
}
