use crate::package::crypto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionString(String);

impl ConnectionString {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn encrypt(&mut self, key: &[u8; 32]) {
        self.0 = crypto::encrypt(&self.0, key).expect("Encryption failed");
    }

    pub fn decrypt(&mut self, key: &[u8; 32]) {
        self.0 = crypto::decrypt(&self.0, key).expect("Decryption failed");
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub name: String,
    pub conn_string: ConnectionString,
    pub history_depth: usize,
}
