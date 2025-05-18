use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Ok,
    Err,
}

#[derive(Debug, Clone)]
pub struct Result {
    pub status: Status,
    pub message: String,
    pub data: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub cmd: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrKind {
    NotEstablished,
    Terminated,
    CorruptMessage,
    Unknown,
}

#[derive(Debug)]
pub struct WireError {
    pub kind: ErrKind,
    pub cause: String,
}

impl fmt::Display for WireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.cause)
    }
}

impl std::error::Error for WireError {}
