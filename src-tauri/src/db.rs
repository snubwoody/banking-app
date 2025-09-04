use sqlx::{prelude::FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Clone)]
pub struct State {
    pool: SqlitePool,
}

impl State {
    pub async fn new() -> Self {
        let url = "../data.db";
        let pool = sqlx::SqlitePool::connect(&url).await.unwrap();
        Self { pool }
    }

    pub fn pool(&self) -> &sqlx::SqlitePool {
        &self.pool
    }
}

#[derive(Debug, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
}

pub async fn create_account(pool: &SqlitePool, name: &str) -> Account {
    let id = Uuid::new_v4();

    tracing::info!("Created new account");
    
    sqlx::query_as("INSERT INTO accounts(id,name) VALUES($1,$2) RETURNING *")
        .bind(id)
        .bind(name)
        .fetch_one(pool)
        .await
        .unwrap()
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
        let state = State::new().await;
        let account = create_account(state.pool(), "Credit card").await;
        let row: Account = sqlx::query_as("SELECT * FROM accounts WHERE id=$1")
            .bind(account.id)
            .fetch_one(state.pool())
            .await
            .unwrap();

        assert_eq!(account.id, row.id);
        assert_eq!(account.name, row.name);
    }
}
