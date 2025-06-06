#[derive(thiserror::Error, Debug)]
pub enum AppError {
    // Database errors - these are likely the large ones
    #[error("Database error: {0}")]
    RedbError(#[from] Box<redb::Error>),
    #[error("Database error: {0}")]
    DatabaseError(#[from] Box<redb::DatabaseError>),
    #[error("Transaction error: {0}")]
    TransactionError(#[from] Box<redb::TransactionError>),
    #[error("Table error: {0}")]
    TableError(#[from] Box<redb::TableError>),
    #[error("Storage error: {0}")]
    StorageError(#[from] Box<redb::StorageError>),
    #[error("Commit error: {0}")]
    CommitError(#[from] Box<redb::CommitError>),

    // These are probably smaller
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Value not found for key: {0}")]
    NotFound(String),
    #[error("Encryption/decryption error: {0}")]
    ChaCha20Poly1305Error(String),
    #[error("Base64 decoding error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Invalid format: expected nonce:ciphertext")]
    InvalidFormat,

    // Keyring errors
    #[error("Keyring Error: Failed to create keyring entry")]
    KeyringEntryError,
    #[error("Keyring Error: Failed to retrieve password from keychain")]
    KeyringRetrievalError,

    // Conversion
    #[error("Conversion Error: Password to byte array conversion failed")]
    ByteArrayConversionError,
}

// You'll need to update the From implementations for the boxed variants
impl From<redb::Error> for AppError {
    fn from(err: redb::Error) -> Self {
        AppError::RedbError(Box::new(err))
    }
}

impl From<redb::DatabaseError> for AppError {
    fn from(err: redb::DatabaseError) -> Self {
        AppError::DatabaseError(Box::new(err))
    }
}

impl From<redb::TransactionError> for AppError {
    fn from(err: redb::TransactionError) -> Self {
        AppError::TransactionError(Box::new(err))
    }
}

impl From<redb::TableError> for AppError {
    fn from(err: redb::TableError) -> Self {
        AppError::TableError(Box::new(err))
    }
}

impl From<redb::StorageError> for AppError {
    fn from(err: redb::StorageError) -> Self {
        AppError::StorageError(Box::new(err))
    }
}

impl From<redb::CommitError> for AppError {
    fn from(err: redb::CommitError) -> Self {
        AppError::CommitError(Box::new(err))
    }
}

impl From<chacha20poly1305::Error> for AppError {
    fn from(err: chacha20poly1305::Error) -> Self {
        AppError::ChaCha20Poly1305Error(err.to_string())
    }
}
