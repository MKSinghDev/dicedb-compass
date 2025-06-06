mod command;
pub mod database;
mod package;
mod util;

use command::connection::{db_test, get_connections};
use database::config_db::ConfigDB;
use package::error::AppError;
use tauri::{path::SafePathBuf, Manager};
use util::{constant::CONFIG_DB_PATH, password::init_keychain};

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
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![db_test, get_connections,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
