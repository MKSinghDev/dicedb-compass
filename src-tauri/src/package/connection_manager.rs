use crate::database::connection_db::ConnectionDB;

use super::model::ConnectionConfig;
use dicedb::Client;
use std::{collections::HashMap, sync::Arc};
use tauri::path::SafePathBuf;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Connection(Client, ConnectionDB);

#[derive(Debug)]
pub struct ConnectionManager(HashMap<String, Connection>);

impl ConnectionManager {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub async fn add_connection(&mut self, config: ConnectionConfig) -> Result<bool, String> {
        if let Some((host, port)) = config.conn_string.get_host_port() {
            let client = Client::new(host, port)
                .await
                .map_err(|e| format!("Failed to create client: {}", e))?;

            let conn_db_path = SafePathBuf::new(format!("{}.redb", config.name).into())?;
            let conn_db = match ConnectionDB::open(&conn_db_path) {
                Ok(db) => db,
                Err(_) => {
                    ConnectionDB::new(&conn_db_path).expect("Failed to create connection db.")
                }
            };
            self.0.insert(config.name, Connection(client, conn_db));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn close_connection(&mut self, connection_name: &str) -> bool {
        if let Some(mut val) = self.0.remove(connection_name) {
            val.0.close().await;
            true
        } else {
            false
        }
    }

    pub fn get_connection(&self, connection_name: &str) -> Option<&Client> {
        match self.0.get(connection_name) {
            Some(val) => Some(&val.0),
            None => None,
        }
    }

    pub fn list_connections(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }

    pub fn has_connections(&self) -> bool {
        !self.0.is_empty()
    }
}

pub type ConnectionManagerState = Arc<RwLock<ConnectionManager>>;

pub fn create_connections_state() -> ConnectionManagerState {
    Arc::new(RwLock::new(ConnectionManager::new()))
}
