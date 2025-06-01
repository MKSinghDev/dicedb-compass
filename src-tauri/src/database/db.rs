use redb::{Database, ReadableTable, TableDefinition};
use serde_json::Value;
use std::path::Path;

use crate::packages::error::AppError;

const TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("data");

pub struct DB {
    db: Database,
}

impl DB {
    /// Create a new database at the default path
    pub fn new() -> Result<Self, AppError> {
        let db = Database::create("data.redb")?;
        Ok(Self { db })
    }

    /// Open an existing database
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, AppError> {
        let db = Database::open(path)?;
        Ok(Self { db })
    }

    /// Store a JSON value with the given key
    pub fn set(&self, key: &str, value: Value) -> Result<(), AppError> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE)?;
            let serialized = serde_json::to_vec(&value)?;
            table.insert(key, serialized.as_slice())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    /// Retrieve a JSON value by key
    pub fn get(&self, key: &str) -> Result<Value, AppError> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;

        match table.get(key)? {
            Some(bytes) => {
                let value: Value = serde_json::from_slice(bytes.value())?;
                Ok(value)
            }
            None => Err(AppError::NotFound(key.to_string())),
        }
    }

    /// Check if a key exists
    pub fn contains_key(&self, key: &str) -> Result<bool, AppError> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;

        Ok(table.get(key)?.is_some())
    }

    /// Get a value with a default if not found
    pub fn get_or_default(&self, key: &str, default: Value) -> Result<Value, AppError> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;

        match table.get(key)? {
            Some(bytes) => {
                let value: Value = serde_json::from_slice(bytes.value())?;
                Ok(value)
            }
            None => Ok(default),
        }
    }

    /// Delete a key-value pair
    pub fn delete(&self, key: &str) -> Result<bool, AppError> {
        let write_txn = self.db.begin_write()?;
        let exists;
        {
            let mut table = write_txn.open_table(TABLE)?;
            exists = table.remove(key)?.is_some();
        }
        write_txn.commit()?;
        Ok(exists)
    }

    /// List all keys in the database
    pub fn keys(&self) -> Result<Vec<String>, AppError> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;

        let mut keys = Vec::new();
        let iter = table.iter()?;
        for key_value in iter {
            let (key, _) = key_value?;
            keys.push(key.value().to_string());
        }

        Ok(keys)
    }

    /// Update a value with a transformation function
    pub fn update<F>(&self, key: &str, update_fn: F) -> Result<Value, AppError>
    where
        F: FnOnce(Option<Value>) -> Value,
    {
        // First get the current value
        let current_value = match self.get(key) {
            Ok(value) => Some(value),
            Err(AppError::NotFound(_)) => None,
            Err(e) => return Err(e),
        };

        // Apply the transformation
        let new_value = update_fn(current_value);

        // Store the new value
        self.set(key, new_value.clone())?;

        Ok(new_value)
    }

    /// Get the total number of entries
    pub fn len(&self) -> Result<usize, AppError> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;

        let mut count = 0;
        let iter = table.iter()?;
        for result in iter {
            result?; // Propagate any errors
            count += 1;
        }

        Ok(count)
    }

    /// Check if the database is empty
    pub fn is_empty(&self) -> Result<bool, AppError> {
        Ok(self.len()? == 0)
    }

    /// Clear all entries in the database
    pub fn clear(&self) -> Result<(), AppError> {
        let write_txn = self.db.begin_write()?;
        {
            // In ReDB, the easiest way to clear is to drop and recreate the table
            write_txn.delete_table(TABLE)?;
            write_txn.open_table(TABLE)?;
        }
        write_txn.commit()?;
        Ok(())
    }
}
