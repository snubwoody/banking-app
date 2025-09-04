use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pool: SqlitePool,
}

impl AppState {
    pub async fn new() -> Self {
        let url = "../data.db";
        let pool = sqlx::SqlitePool::connect(&url).await.unwrap();
        Self { pool }
    }

    pub fn pool(&self) -> &sqlx::SqlitePool {
        &self.pool
    }
}

#[derive(Debug, FromRow,Serialize,Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
}

#[tauri::command]
pub async fn create_account(state: tauri::State<'_,AppState>, name: String) -> Result<Account,()> {
    let id = Uuid::new_v4();
    let pool = state.pool();
    tracing::info!("Created new account");
    
    let account = sqlx::query_as("INSERT INTO accounts(id,name) VALUES($1,$2) RETURNING *")
        .bind(id)
        .bind(name)
        .fetch_one(pool)
        .await
        .unwrap();

    Ok(account)
}

pub async fn fetch_accoutns(pool: &SqlitePool) -> Vec<Account> {
    sqlx::query_as("SELECT * FROM accounts")
        .fetch_all(pool)
        .await
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn create_new_account() {
        // FIXME
        // let state = AppState::new().await;
        // let account = create_account(state.pool(), "Credit card").await;
        // let row: Account = sqlx::query_as("SELECT * FROM accounts WHERE id=$1")
        //     .bind(account.id)
        //     .fetch_one(state.pool())
        //     .await
        //     .unwrap();

        // assert_eq!(account.id, row.id);
        // assert_eq!(account.name, row.name);
    }
}
