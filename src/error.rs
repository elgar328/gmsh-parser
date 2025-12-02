use crate::types::FileType;
use miette::{Diagnostic, SourceSpan};
use std::sync::Arc;
use thiserror::Error;

/// Warning generated during parsing (non-fatal issues)
#[derive(Debug, Clone)]
pub struct ParseWarning {
    /// Description of the warning
    pub message: String,
}

impl ParseWarning {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Debug, Error, Diagnostic)]
pub enum ParseError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid MSH format")]
    InvalidFormat {
        message: String,

        #[label("{message}")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,
    },

    #[error("Invalid version format")]
    InvalidVersionFormat {
        version: String,

        #[label("invalid version format: {version}")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,
    },

    #[error("Unsupported MSH version (only 4.1 and 4.1.0 are supported)")]
    UnsupportedVersion {
        version: String,

        #[label("unsupported version: {version}")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,
    },

    #[error("Invalid file type")]
    InvalidFileType {
        file_type: i32,

        #[label("invalid file type: {file_type}")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,
    },

    #[error("Unsupported file type (only ASCII mode is supported)")]
    UnsupportedFileType {
        file_type: FileType,

        #[label("unsupported file type: {file_type:?}")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,
    },

    #[error("Invalid section")]
    InvalidSection {
        message: String,

        #[label("{message}")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,
    },

    #[error("Invalid entity dimension")]
    InvalidEntityDimension {
        dimension: i32,

        #[label("invalid entity dimension: {dimension}")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,
    },

    #[error("Invalid element type")]
    InvalidElementType {
        element_type: i32,

        #[label("invalid element type: {element_type}")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,
    },

    #[error("Invalid element topology")]
    InvalidElementTopology {
        element_topology: i32,

        #[label("invalid element topology: {element_topology}")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,
    },

    #[error("Missing required section: {0}")]
    MissingSection(String),

    #[error("Invalid data")]
    InvalidData {
        message: String,

        #[label("{message}")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,
    },

    #[error("Duplicate tag: {tag}")]
    DuplicateTag {
        tag: usize,

        #[label("duplicate tag: {tag}")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,
    },

    #[error("Parse error")]
    ParseIntError {
        field: String,
        value: String,

        #[label("expected integer for '{field}', found '{value}'")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,

        #[source]
        cause: std::num::ParseIntError,
    },

    #[error("Parse error")]
    ParseFloatError {
        field: String,
        value: String,

        #[label("expected float for '{field}', found '{value}'")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,

        #[source]
        cause: std::num::ParseFloatError,
    },

    #[error("Unexpected end of file")]
    UnexpectedEof,

    #[error("Expected end of section marker")]
    ExpectedEndOfSection {
        expected: String,
        found: String,

        #[label("expected {expected}, but found: {found}")]
        span: SourceSpan,

        #[source_code]
        msh_content: Arc<String>,
    },
}

pub type Result<T> = std::result::Result<T, ParseError>;
