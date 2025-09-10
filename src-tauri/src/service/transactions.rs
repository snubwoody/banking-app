use crate::{AppState, db::Account};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum TransactionType {
    Income,
    Expense,
    Transfer,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Default)]
pub struct Transaction {
    pub id: String,
    pub account: Account,
    /// For transfers
    pub to_account: Option<Account>,
    pub amount: f64,
    pub category: Category,
    pub date: NaiveDate,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Default)]
pub struct Category {
    pub id: String,
    pub title: String,
}


pub struct TransactionBuilder{
    transaction_type: TransactionType,
    date: NaiveDate
}

pub struct TransactionService {
    state: AppState,
}

impl TransactionService {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn add_expense(
        &self,
        amount: f64,
        account_id: &str,
        category_id: &str,
        date: NaiveDate,
    ) -> crate::Result<()> {
        let pool = self.state.pool();
        let row = sqlx::query_file!(
            "queries/add_transaction.sql",
            amount,
            account_id,
            category_id,
            date
        )
        .fetch_one(pool)
        .await?;
        dbg!(row);
        Ok(())
    }
}
