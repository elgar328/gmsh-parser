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
        }
    }

    fn next_line(&mut self) -> Result<String> {
        let line = self
            .lines
            .next()
            .ok_or(ParseError::UnexpectedEof)?
            .expect("I/O error cannot occur when reading from Cursor");
        self.current_offset += line.len() + 1;
        Ok(line)
    }

    /// Read the next non-empty line and tokenize it
    pub fn read_token_line(&mut self) -> Result<TokenLine> {
        loop {
            let line_start_offset = self.current_offset;
            let line = self.next_line()?;

            if line.trim().is_empty() {
                continue;
            }

            // Tokenize the line
            let mut tokens = Vec::new();
            let mut current_pos = 0;

            for word in line.split_whitespace() {
                // Find the position of this word in the original line
                let word_start = line[current_pos..].find(word).unwrap() + current_pos;
                let byte_offset = line_start_offset + word_start;

                let token = Token::new(
                    word.to_string(),
                    Span::new(byte_offset, word.len()),
                    Arc::clone(&self.source),
                );

                tokens.push(token);
                current_pos = word_start + word.len();
            }

            return Ok(TokenLine::new(tokens));
        }
    }
}
