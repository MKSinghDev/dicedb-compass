use dicedb::{connect_to_dicedb, set_key};
use serde_json::json;

use super::DB;

#[tauri::command]
pub async fn db_test() {
    println!("DB Test function invoked!");

    let mut client = connect_to_dicedb("127.0.0.1:7379").await.unwrap();
    println!("Connected to dicedb server! 💡💡");
    set_key(
        &mut client,
        String::from("test1"),
        "My first value".as_bytes().to_vec(),
    )
    .await
    .unwrap();

    println!("Set key successfully!");

    let db = DB::new().unwrap();
    db.set("test", json!(555)).unwrap();

    let value = db.get("test").unwrap();
    println!("This is the value: {:?}", value);
}

// #[tauri::command]
// pub fn db_test() {
//     println!("DB Test function invoked!");
//     let client = Client::new("127.0.0.1", 7379).unwrap();
//     let result = client.fire_string("GET test");
//
//     // Check the status of the wire::Result
//     if result.status == wire::Status::Ok {
//         // Extract the actual data from the result
//         if let Some(data) = result.data {
//             // Process the data based on your wire protocol
//             // For example, if data is a Vec<u8> that contains UTF-8 text:
//             match String::from_utf8(data) {
//                 Ok(value_str) => {
//                     println!("Value: {}", value_str);
//                     // Use value_str here
//                 }
//                 Err(e) => {
//                     println!("Error decoding data: {:?}", e);
//                 }
//             }
//         } else {
//             println!("Command successful but no data returned");
//         }
//     } else {
//         println!("Command failed: {}", result.message);
//     }
//
//     let db = DB::new().unwrap();
//     db.set("test", json!(555)).unwrap();
//
//     let value = db.get("test").unwrap();
//     println!("This is the value: {:?}", value);
// }
