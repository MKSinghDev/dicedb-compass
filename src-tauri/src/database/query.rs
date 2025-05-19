use dicedb::Client;
use log::info;
use serde_json::json;

use super::DB;

#[tauri::command]
pub async fn db_test() {
    info!("DB Test function invoked!");

    let new_client = Client::new("127.0.0.1", 7379)
        .await
        .expect("Failed to create dicedb client from struct");
    info!("New client created!");

    let res = new_client
        .set("new_key", "new_value".as_bytes().to_vec().as_ref())
        .await;

    info!("Wire Result: {:?}", res);

    let db = DB::new().unwrap();
    db.set("test", json!(555)).unwrap();

    let value = db.get("test").unwrap();
    info!("This is the value: {:?}", value);
}
