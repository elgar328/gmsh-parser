use super::token::{Span, Token, TokenLine};
use crate::error::{ParseError, Result};
use std::io::{BufRead, BufReader, Cursor};
use std::path::Path;
use std::sync::Arc;

/// Represents a source file with its content
#[derive(Debug, Clone)]
pub struct SourceFile {
    /// Full file content (shared across all tokens)
    pub content: Arc<String>,
}

impl SourceFile {
    pub fn new(content: String) -> Self {
        Self {
            content: Arc::new(content),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(&path)?;
        Ok(Self::new(content))
    }

    /// Create a LineReader from this SourceFile
    pub fn to_line_reader(self) -> LineReader {
        LineReader::new(self)
    }
}

/// Line reader that tracks positions and generates tokens
pub struct LineReader {
    lines: std::io::Lines<BufReader<Cursor<Vec<u8>>>>,
    source: Arc<String>,
    current_offset: usize,
    line_number: usize,
}

impl LineReader {
    pub fn new(source: SourceFile) -> Self {
        let bytes = source.content.as_bytes().to_vec();
        let cursor = Cursor::new(bytes);
        let reader = BufReader::new(cursor);

        Self {
            lines: reader.lines(),
            source: source.content,
            current_offset: 0,
            line_number: 0,
        }
    }

    pub fn current_line(&self) -> usize {
        self.line_number
    }

    pub fn next_line(&mut self) -> Option<String> {
        self.lines.next().map(|result| {
            let line = result.expect("I/O error cannot occur when reading from Cursor");
            self.line_number += 1;
            // Update offset: add line length + newline character
            self.current_offset += line.len() + 1;
            line
        })
    }

    /// Read the next non-empty line and tokenize it
    pub fn read_token_line(&mut self) -> Result<TokenLine> {
        loop {
            match self.next_line() {
                Some(line) => {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        let tokens = self.tokenize_line(&line, trimmed);
                        return Ok(TokenLine::new(tokens, self.line_number, line));
                    }
                }
                None => return Err(ParseError::UnexpectedEof),
            }
        }
    }

    fn tokenize_line(&self, full_line: &str, trimmed: &str) -> Vec<Token> {
        let line_start_offset = self.current_offset - full_line.len() - 1;

        let mut tokens = Vec::new();
        let mut current_pos = 0;

        for word in trimmed.split_whitespace() {
            // Find the position of this word in the original line
            let word_start = full_line[current_pos..].find(word).unwrap() + current_pos;
            let byte_offset = line_start_offset + word_start;

            let token = Token::new(
                word.to_string(),
                Span::new(byte_offset, word.len()),
                Arc::clone(&self.source),
            );

            tokens.push(token);
            current_pos = word_start + word.len();
        }

        tokens
    }
}
