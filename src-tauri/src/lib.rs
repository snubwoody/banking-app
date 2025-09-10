pub mod db;
pub mod error;
pub mod service;
mod state;
mod cmd;
pub use service::AccountService;
pub use error::{Error, Result};
pub use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    // FIXME don't unwrap
    let account_service = AccountService::new().await.unwrap();
    let state = AppState::init().await.unwrap();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(account_service)
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            cmd::create_account,
            cmd::fetch_accounts,
            cmd::delete_account,
            cmd::get_transactions,
            cmd::add_transaction,
            cmd::get_categories,
            cmd::add_category,
            cmd::get_all_transactions,
            cmd::get_account_types,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
