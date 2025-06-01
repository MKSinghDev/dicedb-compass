pub mod database;
mod packages;
mod store;
mod utils;

use database::query::db_test;
use store::stronghold::{
    auto_init_stronghold, get_keys, get_secret, has_stored_password, init_stronghold,
    is_stronghold_initialized, is_stronghold_ready, lock_stronghold, remove_secret,
    reset_stronghold, save_secret, unlock_stronghold, StrongholdState,
};
use tauri::Manager;

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
            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            app.handle()
                .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

            app.manage(StrongholdState::new());

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
