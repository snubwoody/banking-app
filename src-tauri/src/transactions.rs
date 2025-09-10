use crate::{db::Account, Error};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, prelude::FromRow};

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

pub struct TransactionService{
    
}