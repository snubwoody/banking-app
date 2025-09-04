use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, FromRow,Serialize,Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
}



pub struct AccountService{
    pool: SqlitePool
}

impl AccountService{
    pub async fn new() -> Self{
        let url = "../data.db";
        let pool = sqlx::SqlitePool::connect(&url).await.unwrap();
        Self { pool }
    }

    pub async fn create_account(&self, name: &str) -> Account{
        let id = Uuid::new_v4();
        
        let account = sqlx::query_as("INSERT INTO accounts(id,name) VALUES($1,$2) RETURNING *")
            .bind(id)
            .bind(name)
            .fetch_one(&self.pool)
            .await
            .unwrap();
    
        tracing::info!("Created new account: {id}");

        account
    }
}

#[tauri::command]
pub async fn create_account(accounts: tauri::State<'_,AccountService>, name: String) -> Result<Account,()> {
    let account = accounts.create_account(&name).await;
    Ok(account)
}

pub async fn fetch_accoutns(pool: &SqlitePool) -> Vec<Account> {
    sqlx::query_as("SELECT * FROM accounts")
        .fetch_all(pool)
        .await
        .unwrap()
}