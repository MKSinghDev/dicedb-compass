use crate::{
    package::{error::AppError, model::ConnectionConfig},
    util::password::get_password_from_keychain,
};
use redb::{Database, ReadableTable, TableDefinition};
use serde_json;
use tauri::path::SafePathBuf;

const CONFIG_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("connections");

pub struct ConfigDB {
    db: Database,
    key: [u8; 32],
}

impl ConfigDB {
    pub async fn new(db_path: &SafePathBuf) -> Result<Self, AppError> {
        let db = Database::create(db_path)?;
        let key = get_password_from_keychain()
            .await?
            .as_bytes()
            .try_into()
            .map_err(|_| AppError::ByteArrayConversionError)?;
        Ok(ConfigDB { db, key })
    }

    pub async fn open(db_path: &SafePathBuf) -> Result<Self, AppError> {
        let db = Database::open(db_path)?;
        let key = get_password_from_keychain()
            .await?
            .as_bytes()
            .try_into()
            .map_err(|_| AppError::ByteArrayConversionError)?;
        Ok(ConfigDB { db, key })
    }

    pub fn add_connection(&self, config: ConnectionConfig) -> Result<(), AppError> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(CONFIG_TABLE)?;
            let mut copy = config.clone();
            copy.conn_string.encrypt(&self.key);
            let serialized = serde_json::to_vec(&copy)?;
            table.insert(config.name.as_str(), serialized.as_slice())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn get_connection(&self, name: &str) -> Result<ConnectionConfig, AppError> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(CONFIG_TABLE)?;
        if let Some(bytes) = table.get(name)? {
            let mut config: ConnectionConfig = serde_json::from_slice(bytes.value())?;
            config.conn_string.decrypt(&self.key);
            Ok(config)
        } else {
            Err(AppError::NotFound(name.to_string()))
        }
    }

    pub fn list_connections(&self) -> Result<Vec<ConnectionConfig>, AppError> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(CONFIG_TABLE)?;
        table
            .iter()?
            .map(|entry| {
                let (_, bytes) = entry?;
                let mut config: ConnectionConfig = serde_json::from_slice(bytes.value())?;
                config.conn_string.decrypt(&self.key);
                Ok(config)
            })
            .collect()
    }

    pub fn remove_connection(&self, name: &str) -> Result<bool, AppError> {
        let txn = self.db.begin_write()?;
        let mut table = txn.open_table(CONFIG_TABLE)?;
        let result = table.remove(name)?.is_some();
        drop(table);
        txn.commit()?;
        Ok(result)
    }
}
