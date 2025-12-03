use super::Token;
use crate::error::{ParseError, Result};

/// Iterator over tokens in a TokenLine with parsing methods
pub struct TokenIter<'a> {
    pub(super) tokens: &'a [Token],
    pub(super) index: usize,
}

impl<'a> TokenIter<'a> {
    pub(super) fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, index: 0 }
    }

    /// Peek at the current token without advancing
    pub fn peek_token(&self) -> Result<&'a Token> {
        self.tokens.get(self.index).ok_or_else(|| {
            // Point to the end of the line (after the last token)
            // TokenLine guarantees at least one token, so unwrap is safe
            let last_token = self
                .tokens
                .last()
                .expect("TokenLine should never be empty (guaranteed by TokenLine::new)");
            let end_offset = last_token.span.offset + last_token.span.len;

            ParseError::InvalidFormat {
                message: "Line ended unexpectedly, expected more data".to_string(),
                span: (end_offset, 1).into(),
                msh_content: last_token.source.clone(),
            }
        })
    }

    /// Internal helper: get the next token and advance the index
    pub(super) fn next_token(&mut self) -> Result<&'a Token> {
        let token = self.peek_token()?;
        self.index += 1;
        Ok(token)
    }

    /// Get the number of remaining tokens
    pub fn remaining(&self) -> usize {
        self.tokens.len().saturating_sub(self.index)
    }

    /// Check if there are more tokens
    pub fn has_next(&self) -> bool {
        self.index < self.tokens.len()
    }

    /// Verify that there are no more tokens remaining
    ///
    /// This is useful for validating that a line has exactly the expected number of tokens
    /// after parsing all required values.
    ///
    /// # Errors
    /// Returns an error if there are remaining tokens
    pub fn expect_no_more(&mut self) -> Result<()> {
        if let Some(token) = self.next() {
            return Err(ParseError::InvalidFormat {
                message: "Unexpected extra data at end of line".to_string(),
                span: token.span.to_source_span(),
                msh_content: token.source.clone(),
            });
        }
        Ok(())
    }
}

/// Implement standard Iterator trait for TokenIter
/// This allows using standard iterator methods like map, filter, collect, etc.
impl<'a> Iterator for TokenIter<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tokens.len() {
            let token = &self.tokens[self.index];
            self.index += 1;
            Some(token)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining();
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for TokenIter<'a> {
    fn len(&self) -> usize {
        self.remaining()
    }
}
