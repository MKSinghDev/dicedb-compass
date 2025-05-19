use super::error::DiceDbError;
use crate::{
    Client,
    constant::{DEFAULT_BACKOFF_DURATION_SECS, DEFAULT_MAX_RETRIES},
};
use std::time::Duration;

pub struct ClientBuilder {
    id: Option<String>,
    host: String,
    port: u16,
    max_retries: u32,
    backoff_duration: Duration,
}

impl ClientBuilder {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            id: None,
            host: host.to_string(),
            port,
            max_retries: DEFAULT_MAX_RETRIES,
            backoff_duration: Duration::from_secs(DEFAULT_BACKOFF_DURATION_SECS),
        }
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_backoff_duration(mut self, duration: Duration) -> Self {
        self.backoff_duration = duration;
        self
    }

    pub async fn build(self) -> Result<Client, DiceDbError> {
        Client::new_with_options(
            &self.host,
            self.port,
            self.id.as_deref(),
            self.max_retries,
            self.backoff_duration,
        )
        .await
    }
}
