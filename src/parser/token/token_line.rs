use super::{Token, TokenIter};
use crate::error::{ParseError, Result};
use miette::SourceSpan;

/// A line of tokens with metadata
#[derive(Debug)]
pub struct TokenLine {
    /// The tokens in this line (private - use iter() for access)
    tokens: Vec<Token>,
}

impl TokenLine {
    pub fn new(tokens: Vec<Token>) -> Self {
        assert!(!tokens.is_empty(), "TokenLine must have at least one token");
        Self { tokens }
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

    /// Get the span for the entire line
    pub fn line_span(&self) -> SourceSpan {
        let first = self
            .tokens
            .first()
            .expect("TokenLine should never be empty (guaranteed by TokenLine::new)");
        let last = self
            .tokens
            .last()
            .expect("TokenLine should never be empty (guaranteed by TokenLine::new)");

        let start = first.span.offset;
        let end = last.span.offset + last.span.len;
        (start, end - start).into()
    }

    /// Expect a specific section start marker (e.g., "$MeshFormat")
    pub fn expect_section_start(&self, section_name: &str) -> Result<()> {
        let mut iter = self.iter();
        let expected = format!("${}", section_name);

        let token = iter.next_token()?;
        if token.value != expected {
            return Err(ParseError::InvalidData {
                message: format!("Expected '{}', found '{}'", expected, token.value),
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
            });
        }

        // Verify no extra data after the section start marker
        iter.expect_no_more()?;
        Ok(())
    }

    /// Expect a specific section end marker (e.g., "$EndMeshFormat")
    pub fn expect_end_marker(&self, section_name: &str) -> Result<()> {
        let mut iter = self.iter();
        let expected = format!("$End{}", section_name);

        let token = iter.next_token()?;
        if token.value != expected {
            return Err(ParseError::ExpectedEndOfSection {
                expected,
                found: token.value.clone(),
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
            });
        }

        // Verify no extra data after the end marker
        iter.expect_no_more()?;
        Ok(())
    }

    /// Create an iterator over the tokens in this line
    pub fn iter(&self) -> TokenIter<'_> {
        TokenIter::new(&self.tokens)
    }
}
