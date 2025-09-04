use std::env;
use uuid::Uuid;

pub async fn create_account(){
    let url = "../data.db";
    let pool = sqlx::SqlitePool::connect(&url).await.unwrap();
    let id = Uuid::new_v4();
    
    sqlx::query("INSERT INTO accounts(id,name) VALUES($1,$2)")
        .bind(id)
        .bind("Savings")
        .execute(&pool)
        .await
        .unwrap();
}

#[cfg(test)]
mod test{
    use super::*;

    #[tokio::test]
    async fn create_new_account(){
        create_account().await;
    }
}