use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid MSH format: {0}")]
    InvalidFormat(String),

    #[error("Unsupported MSH version: {0} (only 4.1 is supported)")]
    UnsupportedVersion(f64),

    #[error("Unsupported file type: {0} (only ASCII mode is supported)")]
    UnsupportedFileType(i32),

    #[error("Invalid section: {0}")]
    InvalidSection(String),

    #[error("Invalid entity dimension: {0}")]
    InvalidEntityDimension(i32),

    #[error("Invalid element type: {0}")]
    InvalidElementType(i32),

    #[error("Missing required section: {0}")]
    MissingSection(String),

    #[error("Invalid data in section {0}: {1}")]
    InvalidData(String, String),

    #[error("Duplicate tag in {0}: {1}")]
    DuplicateTag(String, usize),

    #[error("Failed to parse integer: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Failed to parse float: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error("Unexpected end of file")]
    UnexpectedEof,

    #[error("Expected end of section {0}, but got: {1}")]
    ExpectedEndOfSection(String, String),
}

pub type Result<T> = std::result::Result<T, ParseError>;
