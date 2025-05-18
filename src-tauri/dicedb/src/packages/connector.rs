use super::wire::{ErrKind, WireError};

pub struct TcpConnection {
    pub host: String,
    pub port: u16,
    pub connected: bool,
}

impl TcpConnection {
    pub fn new(host: &str, port: u16) -> Result<Self, WireError> {
        // In a real implementation, this would establish a TCP connection
        // For now we'll simulate it

        // Simulating connection failure possibility
        if host.is_empty() {
            return Err(WireError {
                kind: ErrKind::NotEstablished,
                cause: "Invalid host".to_string(),
            });
        }

        Ok(Self {
            host: host.to_string(),
            port,
            connected: true,
        })
    }

    pub fn close(&mut self) {
        self.connected = false;
    }
}
