use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Invalid magic number: expected {expected:#x}, got {got:#x}")]
    InvalidMagic { expected: u16, got: u16 },
}

pub type Result<T> = std::result::Result<T, ProtocolError>;
