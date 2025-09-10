use crate::service::{Account,Transaction,Category,AccountService,AccountType};
use chrono::NaiveDate;

#[tauri::command]
pub async fn get_categories(
    accounts: tauri::State<'_, AccountService>,
) -> Result<Vec<Category>, ()> {
    let categories = accounts.get_categories().await.unwrap();
    Ok(categories)
}

#[tauri::command]
pub async fn add_category(
    accounts: tauri::State<'_, AccountService>,
    title: &str,
) -> Result<(), ()> {
    accounts.add_category(title).await.unwrap();
    Ok(())
}

#[tauri::command]
pub async fn create_account(
    accounts: tauri::State<'_, AccountService>,
    name: String,
    account_type: &str,
    starting_balance: f64,
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
pub async fn delete_account(
    accounts: tauri::State<'_, AccountService>,
    id: &str,
) -> Result<(), ()> {
    accounts.delete_account(id).await.unwrap();
    tracing::info!("Deleted account: {id}");
    Ok(())
}

#[tauri::command]
pub async fn add_transaction(
    accounts: tauri::State<'_, AccountService>,
    amount: i32,
    account: &str,
    category: &str,
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
    account: &str,
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
