#[tauri::command]
pub async fn get_keys(connection: String) -> Result<Vec<String>, String> {
    Ok(vec![
        "test".to_string(),
        "test2".to_string(),
        "test3".to_string(),
        "test4".to_string(),
        "test5".to_string(),
    ])
}
