use crate::{
    database::{config_db::ConfigDB, db::DB},
    packages::models::ConnectionConfig,
};
use dicedb::{wire::res::result::Response::KeysRes, Client};
use log::{error, info};
use serde_json::json;

#[tauri::command]
pub async fn db_test(
    config_db: tauri::State<'_, ConfigDB>,
    config: ConnectionConfig,
) -> Result<(), String> {
    info!("DB Test function invoked!");

    let conn = config_db.get_connection("First DB");
    match conn {
        Ok(conn) => info!("This is fetched connection config {:?}", conn),
        Err(err) => error!("Caught error while fetching connection config {:?}", err),
    };

    let _ = config_db.add_connection(config);

    let conn = config_db.list_connections();
    match conn {
        Ok(conn) => info!("This is 2nd fetched connection config {:?}", conn),
        Err(err) => error!(
            "Caught error while 2nd fetching connection config {:?}",
            err
        ),
    };

    let new_client = Client::new("127.0.0.1", 7379)
        .await
        .expect("Failed to create dicedb client from struct");

    info!("New client created!");

    let res = new_client
        .set("new_key", "new_value".as_bytes().to_vec().as_ref())
        .await;

    info!("Wire Result: {:?}", res);

    let client = Client::builder("localhost", 7379)
        .build()
        .await
        .expect("Failed to create dicedb client from builder");

    let keys = client.fire_string("KEYS *").await;
    let response = keys.response.unwrap();

    info!("Response: {:?}", response);

    if let KeysRes(keys_res) = response {
        let keys_vec = keys_res.keys;
        info!("Keys: {:?}", keys_vec);
    }

    let db = DB::new().unwrap();
    db.set("test", json!(555)).unwrap();

    let value = db.get("test").unwrap();
    info!("This is the value: {:?}", value);

    Ok(())
}

#[tauri::command]
pub async fn get_connections_name(
    config_db: tauri::State<'_, ConfigDB>,
) -> Result<Vec<String>, String> {
    config_db.list_connections().map_err(|e| {
        error!("Failed to get the connection {:?}", e);
        format!("Failed to get the connections: {:?}", e)
    })
}
