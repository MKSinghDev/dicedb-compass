use crate::{database::config_db::ConfigDB, package::model::ConnectionConfig};
use dicedb::Client;

#[tauri::command]
pub async fn save_connection(
    config_db: tauri::State<'_, ConfigDB>,
    config: ConnectionConfig,
) -> Result<bool, String> {
    config_db
        .add_connection(config)
        .map_err(|e| format!("Failed to save connection {}", e))
}

#[tauri::command]
pub async fn get_connections(
    config_db: tauri::State<'_, ConfigDB>,
) -> Result<Option<Vec<ConnectionConfig>>, String> {
    match config_db.list_connections() {
        Ok(conns) => Ok(Some(conns)),
        Err(_) => Ok(None),
    }
}

#[tauri::command]
pub async fn connect(config: ConnectionConfig) -> Result<bool, String> {
    if let Some((host, port)) = config.conn_string.get_host_port() {
        Client::new(host, port)
            .await
            .expect("Failed to establish connection");
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn save_and_connect(
    config_db: tauri::State<'_, ConfigDB>,
    config: ConnectionConfig,
) -> Result<bool, String> {
    if let Some((host, port)) = config.conn_string.get_host_port() {
        Client::new(host, port)
            .await
            .expect("Failed to establish connection");
        save_connection(config_db, config).await
    } else {
        Ok(false)
    }
}
