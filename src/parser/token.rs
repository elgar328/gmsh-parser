use crate::error::{ParseError, Result};
use miette::SourceSpan;
use std::sync::Arc;

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

    /// Parse this token as an integer
    pub fn parse_int(&self, field: &str) -> Result<i32> {
        self.value
            .parse()
            .map_err(|parse_error| ParseError::ParseIntError {
                field: field.to_string(),
                value: self.value.clone(),
                span: self.span.to_source_span(),
                msh_content: self.source.clone(),
                cause: parse_error,
            })
    }

    /// Parse this token as a usize
    pub fn parse_usize(&self, field: &str) -> Result<usize> {
        self.value
            .parse()
            .map_err(|parse_error| ParseError::ParseIntError {
                field: field.to_string(),
                value: self.value.clone(),
                span: self.span.to_source_span(),
                msh_content: self.source.clone(),
                cause: parse_error,
            })
    }

    /// Parse this token as a float
    pub fn parse_float(&self, field: &str) -> Result<f64> {
        self.value
            .parse()
            .map_err(|parse_error| ParseError::ParseFloatError {
                field: field.to_string(),
                value: self.value.clone(),
                span: self.span.to_source_span(),
                msh_content: self.source.clone(),
                cause: parse_error,
            })
    }

    /// Parse this token as a Version
    pub fn parse_version(&self) -> Result<crate::types::Version> {
        crate::types::Version::from_str(&self.value).ok_or_else(|| {
            ParseError::InvalidVersionFormat {
                version: self.value.clone(),
                span: self.span.to_source_span(),
                msh_content: self.source.clone(),
            }
        })
    }

    /// Parse this token as an EntityDimension
    pub fn parse_entity_dimension(&self, field: &str) -> Result<crate::types::EntityDimension> {
        let value = self.parse_int(field)?;
        crate::types::EntityDimension::from_i32(value)
            .ok_or_else(|| self.invalid_entity_dimension(value))
    }

    /// Create an InvalidEntityDimension error with this token's location
    pub fn invalid_entity_dimension(&self, dimension: i32) -> ParseError {
        ParseError::InvalidEntityDimension {
            dimension,
            span: self.span.to_source_span(),
            msh_content: self.source.clone(),
        }
    }

    /// Parse this token as an ElementType
    pub fn parse_element_type(&self, field: &str) -> Result<crate::types::ElementType> {
        let id = self.parse_int(field)?;
        crate::types::ElementType::from_i32(id).ok_or_else(|| self.invalid_element_type(id))
    }

    /// Create an InvalidElementType error with this token's location
    pub fn invalid_element_type(&self, element_type: i32) -> ParseError {
        ParseError::InvalidElementType {
            element_type,
            span: self.span.to_source_span(),
            msh_content: self.source.clone(),
        }
    }

    /// Parse this token as an ElementTopology
    pub fn parse_element_topology(&self, field: &str) -> Result<crate::types::ElementTopology> {
        let id = self.parse_int(field)?;
        crate::types::ElementTopology::from_i32(id).ok_or_else(|| self.invalid_element_topology(id))
    }

    /// Create an InvalidElementTopology error with this token's location
    pub fn invalid_element_topology(&self, element_topology: i32) -> ParseError {
        ParseError::InvalidElementTopology {
            element_topology,
            span: self.span.to_source_span(),
            msh_content: self.source.clone(),
        }
    }

    /// Parse this token as a FileType
    pub fn parse_file_type(&self, field: &str) -> Result<crate::types::FileType> {
        let value = self.parse_int(field)?;
        crate::types::FileType::from_i32(value).ok_or_else(|| self.invalid_file_type(value))
    }

    /// Create an InvalidFileType error with this token's location
    pub fn invalid_file_type(&self, file_type: i32) -> ParseError {
        ParseError::InvalidFileType {
            file_type,
            span: self.span.to_source_span(),
            msh_content: self.source.clone(),
        }
    }

    /// Create an InvalidFormat error with this token's location
    pub fn invalid_format(&self, message: impl Into<String>) -> ParseError {
        ParseError::InvalidFormat {
            message: message.into(),
            span: self.span.to_source_span(),
            msh_content: self.source.clone(),
        }
    }

    /// Create an InvalidData error with this token's location
    pub fn invalid_data(&self, message: impl Into<String>) -> ParseError {
        ParseError::InvalidData {
            message: message.into(),
            span: self.span.to_source_span(),
            msh_content: self.source.clone(),
        }
    }

    /// Create an InvalidSection error with this token's location
    pub fn invalid_section(&self, message: impl Into<String>) -> ParseError {
        ParseError::InvalidSection {
            message: message.into(),
            span: self.span.to_source_span(),
            msh_content: self.source.clone(),
        }
    }

    /// Parse a quoted string from this token's position to the end of the line
    ///
    /// This extracts the content from this token's start position to the end of the line,
    /// and expects it to be a quoted string in the format: "content"
    /// The opening quote must be the first non-whitespace character, and the closing
    /// quote must be the last non-whitespace character before the line end.
    ///
    /// # Returns
    /// The string content without the surrounding quotes
    ///
    /// # Errors
    /// Returns an error if:
    /// - The content doesn't start with a quote
    /// - The content doesn't end with a quote
    pub fn parse_quoted_string_to_line_end(&self) -> Result<String> {
        // Start from this token's position
        let start_pos = self.span.offset;

        // Find the end of the line (newline or end of file)
        let line_end = self.source[start_pos..]
            .find('\n')
            .map(|pos| start_pos + pos)
            .unwrap_or(self.source.len());

        // Extract the content from this token to the line end
        let content = &self.source[start_pos..line_end];

        // Trim whitespace
        let trimmed = content.trim();

        // Check if it starts with a quote
        if !trimmed.starts_with('"') {
            return Err(ParseError::InvalidData {
                message: format!("Expected quoted string, but found: {}", trimmed),
                span: (start_pos, content.len().max(1)).into(),
                msh_content: self.source.clone(),
            });
        }

        // Check if it ends with a quote
        if !trimmed.ends_with('"') || trimmed.len() < 2 {
            return Err(ParseError::InvalidData {
                message: "Quoted string must end with a closing quote".to_string(),
                span: (start_pos, content.len().max(1)).into(),
                msh_content: self.source.clone(),
            });
        }

        // Extract the content between quotes
        let string_content = &trimmed[1..trimmed.len() - 1];

        Ok(string_content.to_string())
    }
}

/// A line of tokens with metadata
#[derive(Debug)]
pub struct TokenLine {
    /// The tokens in this line
    pub tokens: Vec<Token>,
    /// The line number (1-indexed)
    pub line_number: usize,
    /// The full line content
    pub content: String,
}

impl TokenLine {
    pub fn new(tokens: Vec<Token>, line_number: usize, content: String) -> Self {
        debug_assert!(!tokens.is_empty(), "TokenLine must have at least one token");
        Self {
            tokens,
            line_number,
            content,
        }
    }

    /// Get a token by index, returns None if out of bounds
    pub fn get(&self, index: usize) -> Option<&Token> {
        self.tokens.get(index)
    }

    /// Get the number of tokens
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Create an InvalidFormat error for this entire line
    pub fn invalid_format(&self, message: impl Into<String>) -> ParseError {
        ParseError::InvalidFormat {
            message: message.into(),
            span: self.line_span(),
            msh_content: self.tokens[0].source.clone(),
        }
    }

    /// Expect exactly `expected` number of tokens
    pub fn expect_len(&self, expected: usize) -> Result<()> {
        if self.len() != expected {
            Err(self.invalid_format(format!("Expected {} tokens, got {}", expected, self.len())))
        } else {
            Ok(())
        }
    }

    /// Expect at least `min` number of tokens
    pub fn expect_min_len(&self, min: usize) -> Result<()> {
        if self.len() < min {
            Err(self.invalid_format(format!(
                "Expected at least {} tokens, got {}",
                min,
                self.len()
            )))
        } else {
            Ok(())
        }
    }

    /// Get the span for the entire line
    pub fn line_span(&self) -> SourceSpan {
        if self.tokens.is_empty() {
            // Should not happen, but provide a fallback
            (0, 0).into()
        } else if self.tokens.len() == 1 {
            self.tokens[0].span.to_source_span()
        } else {
            let first = &self.tokens[0].span;
            let last = &self.tokens[self.tokens.len() - 1].span;
            let start = first.offset;
            let end = last.offset + last.len;
            (start, end - start).into()
        }
    }

    /// Expect a specific section end marker (e.g., "$EndMeshFormat")
    pub fn expect_end_marker(&self, section_name: &str) -> Result<()> {
        let expected = format!("$End{}", section_name);
        let token = &self.tokens[0];

        if token.value == expected {
            Ok(())
        } else {
            Err(ParseError::ExpectedEndOfSection {
                expected,
                found: token.value.clone(),
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
            })
        }
    }
}
