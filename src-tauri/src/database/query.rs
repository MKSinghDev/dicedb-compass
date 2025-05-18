use serde_json::json;

use super::DB;

#[tauri::command]
pub fn db_test() {
    println!("DB Test function invoked!");
    let db = DB::new().unwrap();
    db.set("test", json!(555)).unwrap();

    let value = db.get("test").unwrap();
    println!("This is the value: {:?}", value);
}
