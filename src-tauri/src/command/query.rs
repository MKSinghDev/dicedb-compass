use crate::package::connection_manager::ConnectionManagerState;
use dicedb::wire::res::result::Response::{GetRes, KeysRes};

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
