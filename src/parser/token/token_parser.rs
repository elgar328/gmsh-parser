use super::TokenIter;
use crate::error::{ParseError, Result};

/// Parsing methods for TokenIter
impl<'a> TokenIter<'a> {
    /// Parse the next token as an integer and advance
    pub fn parse_int(&mut self, field: &str) -> Result<i32> {
        let token = self.next_token()?;
        token
            .value
            .parse()
            .map_err(|parse_error| ParseError::ParseIntError {
                field: field.to_string(),
                value: token.value.clone(),
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
                cause: parse_error,
            })
    }

    /// Parse the next token as a usize and advance
    pub fn parse_usize(&mut self, field: &str) -> Result<usize> {
        let token = self.next_token()?;
        token
            .value
            .parse()
            .map_err(|parse_error| ParseError::ParseIntError {
                field: field.to_string(),
                value: token.value.clone(),
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
                cause: parse_error,
            })
    }

    /// Parse the next token as a float and advance
    pub fn parse_float(&mut self, field: &str) -> Result<f64> {
        let token = self.next_token()?;
        token
            .value
            .parse()
            .map_err(|parse_error| ParseError::ParseFloatError {
                field: field.to_string(),
                value: token.value.clone(),
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
                cause: parse_error,
            })
    }

    /// Parse the next token as a boolean (0 or 1) and advance
    pub fn parse_bool(&mut self, field: &str) -> Result<bool> {
        let token = self.next_token()?;
        match token.value.as_str() {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(ParseError::InvalidData {
                message: format!("{} must be 0 or 1, found '{}'", field, token.value),
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
            }),
        }
    }

    /// Parse the next token as a Version and advance
    pub fn parse_version(&mut self) -> Result<crate::types::Version> {
        let token = self.next_token()?;
        let parts: Vec<&str> = token.value.split('.').collect();

        let make_error = || ParseError::InvalidVersionFormat {
            version: token.value.clone(),
            span: token.span.to_source_span(),
            msh_content: token.source.clone(),
        };

        if parts.len() != 2 {
            return Err(make_error());
        }

        let major = parts[0].parse::<u32>().map_err(|_| make_error())?;
        let minor = parts[1].parse::<u32>().map_err(|_| make_error())?;

        Ok(crate::types::Version::new(major, minor, token.clone()))
    }

    /// Parse the next token as an EntityDimension and advance
    pub fn parse_entity_dimension(&mut self, field: &str) -> Result<crate::types::EntityDimension> {
        let token = self.next_token()?;
        let value = token
            .value
            .parse()
            .map_err(|parse_error| ParseError::ParseIntError {
                field: field.to_string(),
                value: token.value.clone(),
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
                cause: parse_error,
            })?;
        crate::types::EntityDimension::from_i32(value).ok_or_else(|| {
            ParseError::InvalidEntityDimension {
                dimension: value,
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
            }
        })
    }

    /// Parse the next token as an ElementType and advance
    pub fn parse_element_type(&mut self, field: &str) -> Result<crate::types::ElementType> {
        let token = self.next_token()?;
        let id: i32 = token
            .value
            .parse()
            .map_err(|parse_error| ParseError::ParseIntError {
                field: field.to_string(),
                value: token.value.clone(),
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
                cause: parse_error,
            })?;
        crate::types::ElementType::from_i32(id).ok_or_else(|| ParseError::InvalidElementType {
            element_type: id,
            span: token.span.to_source_span(),
            msh_content: token.source.clone(),
        })
    }

    /// Parse the next token as an ElementTopology and advance
    pub fn parse_element_topology(&mut self, field: &str) -> Result<crate::types::ElementTopology> {
        let token = self.next_token()?;
        let id: i32 = token
            .value
            .parse()
            .map_err(|parse_error| ParseError::ParseIntError {
                field: field.to_string(),
                value: token.value.clone(),
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
                cause: parse_error,
            })?;
        crate::types::ElementTopology::from_i32(id).ok_or_else(|| {
            ParseError::InvalidElementTopology {
                element_topology: id,
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
            }
        })
    }

    /// Parse the next token as a FileType and advance
    pub fn parse_file_type(&mut self, field: &str) -> Result<crate::types::FileType> {
        let token = self.next_token()?;
        let value: i32 = token
            .value
            .parse()
            .map_err(|parse_error| ParseError::ParseIntError {
                field: field.to_string(),
                value: token.value.clone(),
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
                cause: parse_error,
            })?;

        match value {
            0 => Ok(crate::types::FileType::Ascii),
            1 => Ok(crate::types::FileType::Binary),
            _ => Err(ParseError::InvalidFileType {
                file_type: value,
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
            }),
        }
    }

    /// Parse a quoted string from the current position to the end of the line
    ///
    /// Collects all remaining tokens on this line and expects them to form a quoted string.
    /// This is much cleaner than the old approach of manually searching for newlines in the source.
    ///
    /// # Returns
    /// The string content without the surrounding quotes
    ///
    /// # Errors
    /// Returns an error if:
    /// - The content doesn't start with a quote
    /// - The content doesn't end with a quote
    pub fn parse_quoted_string_to_line_end(&mut self) -> Result<String> {
        let start_token = self.peek_token()?;
        let start_span = start_token.span.clone();
        let source = start_token.source.clone();

        // Collect all remaining tokens on this line using standard iterator methods
        let parts: Vec<&str> = self.map(|token| token.value.as_str()).collect();
        let combined = parts.join(" ");
        let trimmed = combined.trim();

        // Check if it starts with a quote
        if !trimmed.starts_with('"') {
            return Err(ParseError::InvalidData {
                message: format!("Expected quoted string, but found: {}", trimmed),
                span: start_span.to_source_span(),
                msh_content: source,
            });
        }

        // Check if it ends with a quote
        if !trimmed.ends_with('"') || trimmed.len() < 2 {
            return Err(ParseError::InvalidData {
                message: "Quoted string must end with a closing quote".to_string(),
                span: start_span.to_source_span(),
                msh_content: source,
            });
        }

        // Extract the content between quotes
        let string_content = &trimmed[1..trimmed.len() - 1];
        Ok(string_content.to_string())
    }

    /// Parse multiple floats starting from the current position
    pub fn parse_floats(&mut self, count: usize, field_prefix: &str) -> Result<Vec<f64>> {
        (0..count)
            .map(|i| self.parse_float(&format!("{}[{}]", field_prefix, i)))
            .collect()
    }

    /// Parse multiple integers starting from the current position
    pub fn parse_ints(&mut self, count: usize, field_prefix: &str) -> Result<Vec<i32>> {
        (0..count)
            .map(|i| self.parse_int(&format!("{}[{}]", field_prefix, i)))
            .collect()
    }
}
