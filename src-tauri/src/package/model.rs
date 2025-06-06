use crate::package::crypto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionString(String);

impl ConnectionString {
    pub fn new(value: String) -> Self {
        ConnectionString(value)
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

    pub fn get_host_port(&self) -> Option<(&str, u16)> {
        let (host, port_str) = self.0.split_once(':')?;
        let port = port_str.parse::<u16>().ok()?;
        Some((host, port))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub name: String,
    pub conn_string: ConnectionString,
    pub history_depth: usize,
}
