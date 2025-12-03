// Core parsing infrastructure
mod reader;
mod token;

// Section-specific parsers
pub mod elements;
pub mod entities;
pub mod ghost_elements;
pub mod interpolation_scheme;
pub mod mesh_format;
pub mod nodes;
pub mod parametrizations;
pub mod partitioned_entities;
pub mod periodic;
pub mod physical_names;
pub mod post_processing;

// Re-exports for public API
pub use reader::{LineReader, SourceFile};
pub use token::{Span, Token, TokenLine};

use std::path::Path;

use crate::error::{ParseError, ParseWarning, Result};
use crate::types::Mesh;

/// Parse a MSH file from a given path
pub fn parse_msh_file<P: AsRef<Path>>(path: P) -> Result<Mesh> {
    let mut line_reader = SourceFile::from_path(&path)?.to_line_reader();
    parse_msh_internal(&mut line_reader)
}

/// Parse MSH data from a string content
pub fn parse_msh(content: impl AsRef<str>) -> Result<Mesh> {
    let mut line_reader = SourceFile::new(content.as_ref().to_string()).to_line_reader();
    parse_msh_internal(&mut line_reader)
}

/// Internal parsing function that works with a LineReader
fn parse_msh_internal(line_reader: &mut LineReader) -> Result<Mesh> {
    // Parse $MeshFormat section first (required)
    let format = mesh_format::parse(line_reader)?;
    let mut mesh = Mesh::new(format);

    // Parse remaining sections
    loop {
        let token_line = match line_reader.read_token_line() {
            Ok(line) => line,
            Err(ParseError::UnexpectedEof) => break,
            Err(e) => return Err(e),
        };

        let first_token = token_line.iter().peek_token()?;

        match first_token.value.as_str() {
            "$MeshFormat" => {
                return Err(ParseError::InvalidData {
                    message: "$MeshFormat section appears more than once".to_string(),
                    span: first_token.span.to_source_span(),
                    msh_content: first_token.source.clone(),
                });
            }
            "$PhysicalNames" => {
                physical_names::parse(line_reader, &mut mesh)?;
            }
            "$Entities" => {
                entities::parse(line_reader, &mut mesh)?;
            }
            "$PartitionedEntities" => {
                partitioned_entities::parse(line_reader, &mut mesh)?;
            }
            "$Nodes" => {
                nodes::parse(line_reader, &mut mesh)?;
            }
            "$Elements" => {
                elements::parse(line_reader, &mut mesh)?;
            }
            "$Periodic" => {
                periodic::parse(line_reader, &mut mesh)?;
            }
            "$GhostElements" => {
                ghost_elements::parse(line_reader, &mut mesh)?;
            }
            "$Parametrizations" => {
                parametrizations::parse(line_reader, &mut mesh)?;
            }
            "$NodeData" => {
                post_processing::parse_node_data(line_reader, &mut mesh)?;
            }
            "$ElementData" => {
                post_processing::parse_element_data(line_reader, &mut mesh)?;
            }
            "$ElementNodeData" => {
                post_processing::parse_element_node_data(line_reader, &mut mesh)?;
            }
            "$InterpolationScheme" => {
                interpolation_scheme::parse(line_reader, &mut mesh)?;
            }
            _ if first_token.value.starts_with('$') && !first_token.value.starts_with("$End") => {
                // Unknown section - skip it and add warning
                let warning = ParseWarning::new(format!("Skipping unknown section: {}", first_token.value));
                mesh.warnings.push(warning);
                skip_section(line_reader, &first_token.value)?;
            }
            _ => {
                // Unexpected content outside of sections - add warning
                let warning = ParseWarning::new(format!(
                    "Unexpected content outside of sections: {}",
                    first_token.value
                ));
                mesh.warnings.push(warning);
            }
        }
    }

    Ok(mesh)
}

/// Skip an unknown section
fn skip_section(reader: &mut LineReader, section_name: &str) -> Result<()> {
    let end_marker = format!("$End{}", &section_name[1..]);

    loop {
        let token_line = reader.read_token_line()?;
        let first_token = token_line.iter().peek_token()?;

        if first_token.value == end_marker {
            return Ok(());
        }
    }
}
