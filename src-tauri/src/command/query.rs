use crate::package::connection_manager::ConnectionManagerState;
use dicedb::wire::res::result::Response::{DelRes, GetRes, KeysRes, SetRes};

#[tauri::command]
pub async fn get_keys(
    connection_manager: tauri::State<'_, ConnectionManagerState>,
    connection: String,
) -> Result<Vec<String>, String> {
    let manager = connection_manager.read().await;
    let conn = manager
        .get_connection(&connection)
        .expect("Connection not found");

    let response = conn
        .fire_string("KEYS *")
        .await
        .response
        .expect("No keys response");

    if let KeysRes(keys_res) = response {
        Ok(keys_res.keys)
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
pub async fn get_key(
    connection_manager: tauri::State<'_, ConnectionManagerState>,
    connection: String,
    key: String,
) -> Result<Option<String>, String> {
    let manager = connection_manager.read().await;
    let conn = manager
        .get_connection(&connection)
        .expect("Connection not found");

    let response = conn.get(&key).await.response.expect("No keys response");

    if let GetRes(keys_res) = response {
        Ok(Some(keys_res.value))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn search_key(
    connection_manager: tauri::State<'_, ConnectionManagerState>,
    connection: &str,
    key: &str,
) -> Result<Vec<String>, String> {
    let manager = connection_manager.read().await;
    let conn = manager
        .get_connection(connection)
        .expect("Connection not found");

    let response = conn
        .fire_string(&format!("KEYS {}", key))
        .await
        .response
        .expect("Failed to execute command");
    if let KeysRes(keys_res) = response {
        Ok(keys_res.keys)
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
pub async fn add_key(
    connection_manager: tauri::State<'_, ConnectionManagerState>,
    connection: String,
    name: &str,
    value: &str,
) -> Result<bool, String> {
    let manager = connection_manager.write().await;
    let conn = manager
        .get_connection(&connection)
        .expect("Connection not found");

    let val = value.as_bytes();
    let response = conn
        .set(name, val)
        .await
        .response
        .expect("Failed to add key:value");
    if let SetRes(_) = response {
        Ok(true)
    } else {
        Err("Something went worng while adding value".to_owned())
    }
}

#[tauri::command]
pub async fn remove_key(
    connection_manager: tauri::State<'_, ConnectionManagerState>,
    connection: String,
    key: String,
) -> Result<bool, String> {
    let manager = connection_manager.write().await;
    let conn = manager
        .get_connection(&connection)
        .expect("Connection not found");

    let response = conn.del(&key).await.response.expect("No key found");
    if let DelRes(_) = response {
        Ok(true)
    } else {
        Ok(false)
    }
}
