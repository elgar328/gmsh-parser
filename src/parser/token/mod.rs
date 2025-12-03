use miette::SourceSpan;
use std::sync::Arc;

mod token_iter;
mod token_line;
mod token_parser;

pub use token_iter::TokenIter;
pub use token_line::TokenLine;

/// Represents a location in the source file
#[derive(Debug, Clone)]
pub struct Span {
    /// Byte offset from the start of the file
    pub offset: usize,
    /// Length in bytes
    pub len: usize,
}

impl Span {
    pub fn new(offset: usize, len: usize) -> Self {
        Self { offset, len }
    }

    /// Convert to miette's SourceSpan for error reporting
    pub fn to_source_span(&self) -> SourceSpan {
        (self.offset, self.len).into()
    }
}

/// A token with position information
#[derive(Debug, Clone)]
pub struct Token {
    /// The string value of the token
    pub value: String,
    /// Location in the source file
    pub span: Span,
    /// Reference to the full source file content (for error reporting)
    pub source: Arc<String>,
}

impl Token {
    pub fn new(value: String, span: Span, source: Arc<String>) -> Self {
        Self {
            value,
            span,
            source,
        }
    }
}
