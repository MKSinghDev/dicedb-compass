pub mod database;
mod packages;
mod queries;
mod store;
mod utils;

use database::config_db::ConfigDB;
use packages::error::AppError;
use queries::connection::{db_test, get_connections_name};
use store::stronghold::{
    auto_init_stronghold, get_keys, get_secret, has_stored_password, init_stronghold,
    is_stronghold_initialized, is_stronghold_ready, lock_stronghold, remove_secret,
    reset_stronghold, save_secret, unlock_stronghold,
};
use tauri::{path::SafePathBuf, Manager};
use utils::{constants::CONFIG_DB_PATH, password::init_keychain};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let config_db = tauri::async_runtime::block_on(async {
                init_keychain().await;
                match SafePathBuf::new(CONFIG_DB_PATH.into()) {
                    Ok(config_path) => {
                        // Try to open existing config DB first, then create new if it fails
                        match ConfigDB::open(&config_path).await {
                            Ok(db) => Ok(db),
                            Err(_) => {
                                // If opening fails, create a new one
                                ConfigDB::new(&config_path).await
                            }
                        }
                    }
                    Err(_) => Err(AppError::InvalidFormat),
                }
            })?;

            app.manage(config_db);

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match auto_init_stronghold(app_handle).await {
                    Ok(_) => println!("Stronghold auto-initialization completed"),
                    Err(e) => println!("Stronghold auto-initialization failed: {}", e),
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            db_test,
            get_connections_name,
            has_stored_password,
            init_stronghold,
            is_stronghold_initialized,
            is_stronghold_ready,
            lock_stronghold,
            save_secret,
            get_keys,
            get_secret,
            remove_secret,
            reset_stronghold,
            unlock_stronghold
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
