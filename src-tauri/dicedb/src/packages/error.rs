use super::wire::WireError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiceDbError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Handshake error: {0}")]
    HandshakeError(String),

    #[error("Wire error: {0}")]
    WireError(#[from] WireError),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}
