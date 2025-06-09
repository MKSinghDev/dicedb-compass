use crate::package::string_ext::StringExt;

#[tauri::command]
pub async fn get_keys(connection: String) -> Result<Vec<String>, String> {
    Ok(vec![
        "test 1".slugify(),
        "test 2".slugify(),
        "test 3".slugify(),
        "test 4".slugify(),
        "test 5".slugify(),
    ])
}
