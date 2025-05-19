use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiceDbError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Handshake error: {0}")]
    HandshakeError(String),

    #[error("Wire error: {0}")]
    WireError(#[from] ClientWireError),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Protocol error: {0}")]
    ProtocolError(String),
}

#[derive(Error, Debug)]
pub enum ClientWireError {
    #[error("Connection not established: {0}")]
    NotEstablished(String),

    #[error("Connection terminated: {0}")]
    Terminated(String),

    #[error("Corrupt message: {0}")]
    CorruptMessage(String),

    #[error("Handshake error: {0}")]
    HandshakeError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("Encoding error: {0}")]
    EncodingError(#[from] prost::EncodeError),

    #[error("Decoding error: {0}")]
    DecodingError(#[from] prost::DecodeError),

    #[error("Timeout error: {0}")]
    TimeoutError(String),
}
