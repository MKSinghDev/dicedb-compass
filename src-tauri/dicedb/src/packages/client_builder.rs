use super::error::DiceDbError;
use crate::Client;

pub struct ClientBuilder {
    pub id: Option<String>,
    pub host: String,
    pub port: u16,
}

impl ClientBuilder {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            id: None,
            host: host.to_string(),
            port,
        }
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn build(self) -> Result<Client, DiceDbError> {
        Client::new_with_options(self.host, self.port, self.id)
    }
}
