use database::query::db_test;
use tauri::Manager;
pub mod database;

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

                let salt_path = app
                    .path()
                    .app_local_data_dir()
                    .expect("could not resolve app local data path")
                    .join("salt.txt");
                app.handle()
                    .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![db_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
