use chrono::Utc;
use redb::{Database, ReadableTable, ReadableTableMetadata, TableDefinition};
use serde_json::Value;
use std::path::PathBuf;

use crate::packages::error::AppError;

const HISTORY_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("history");

pub struct ConnectionDB {
    db: Database,
}

impl ConnectionDB {
    pub fn open(db_path: PathBuf) -> Result<Self, AppError> {
        let db = Database::open(db_path)?;
        Ok(Self { db })
    }

    pub fn add_history(&self, value: Value) -> Result<(), AppError> {
        let ts = Utc::now().to_rfc3339();
        let data = serde_json::to_vec(&value)?;
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_table(HISTORY_TABLE)?;
            table.insert(ts.as_str(), data.as_slice())?;
        }
        txn.commit()?;
        Ok(())
    }

    pub fn get_last_n(&self, n: usize) -> Result<Vec<Value>, AppError> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(HISTORY_TABLE)?;
        let mut entries = table.iter()?.collect::<Result<Vec<_>, _>>()?;
        entries.reverse();
        Ok(entries
            .into_iter()
            .take(n)
            .map(|(_, val)| serde_json::from_slice(val.value()))
            .collect::<Result<Vec<_>, _>>()?)
    }

    pub fn clear(&self) -> Result<(), AppError> {
        let txn = self.db.begin_write()?;
        {
            txn.delete_table(HISTORY_TABLE)?;
            txn.open_table(HISTORY_TABLE)?;
        }
        txn.commit()?;
        Ok(())
    }

    pub fn len(&self) -> Result<usize, AppError> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(HISTORY_TABLE)?;
        Ok(table.iter()?.count())
    }

    pub fn is_empty(&self) -> Result<bool, AppError> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(HISTORY_TABLE)?;
        Ok(table.is_empty()?)
    }
}
