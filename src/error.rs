use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Invalid magic number: expected {expected:#x}, got {got:#x}")]
    InvalidMagic { expected: u16, got: u16 },

    #[error("Invalid frame length: {0}")]
    InvalidLength(usize),

    #[error("Incomplete data: need {needed} bytes, got {available}")]
    IncompleteData { needed: usize, available: usize },

    #[error("Checksum mismatch")]
    ChecksumMismatch,

    #[error("Protocol version mismatch: {0}")]
    VersionMismatch(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}

pub type Result<T> = std::result::Result<T, ProtocolError>;
