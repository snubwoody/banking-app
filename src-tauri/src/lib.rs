pub mod db;
pub mod transactions;
pub mod error;
pub use db::AccountService;
pub use error::Error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let account_service = AccountService::new().await.unwrap();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(account_service)
        .invoke_handler(tauri::generate_handler![
            db::create_account,
            db::fetch_accounts,
            db::delete_account,
            db::get_transactions,
            db::add_transaction,
            db::get_categories,
            db::add_category,
            db::get_all_transactions,
            db::get_account_types,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
