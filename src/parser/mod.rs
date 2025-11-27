pub mod mesh_format;
pub mod physical_names;
pub mod entities;
pub mod nodes;
pub mod elements;
pub mod periodic;
pub mod ghost_elements;
pub mod partitioned_entities;
pub mod parametrizations;
pub mod post_processing;
pub mod interpolation_scheme;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

use crate::error::{ParseError, Result};
use crate::types::Mesh;

pub type LineIterator<R> = Lines<BufReader<R>>;

/// Parse a MSH file from a given path
pub fn parse_msh_file<P: AsRef<Path>>(path: P) -> Result<Mesh> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    parse_msh(reader)
}

/// Parse MSH data from any reader
pub fn parse_msh<R: std::io::Read>(reader: R) -> Result<Mesh> {
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();

    let mut mesh = Mesh::default();
    let mut mesh_format_parsed = false;

    while let Some(line) = lines.next() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        match trimmed {
            "$MeshFormat" => {
                mesh_format::parse(&mut lines, &mut mesh)?;
                mesh_format_parsed = true;
            }
            "$PhysicalNames" => {
                physical_names::parse(&mut lines, &mut mesh)?;
            }
            "$Entities" => {
                entities::parse(&mut lines, &mut mesh)?;
            }
            "$PartitionedEntities" => {
                partitioned_entities::parse(&mut lines, &mut mesh)?;
            }
            "$Nodes" => {
                nodes::parse(&mut lines, &mut mesh)?;
            }
            "$Elements" => {
                elements::parse(&mut lines, &mut mesh)?;
            }
            "$Periodic" => {
                periodic::parse(&mut lines, &mut mesh)?;
            }
            "$GhostElements" => {
                ghost_elements::parse(&mut lines, &mut mesh)?;
            }
            "$Parametrizations" => {
                parametrizations::parse(&mut lines, &mut mesh)?;
            }
            "$NodeData" => {
                post_processing::parse_node_data(&mut lines, &mut mesh)?;
            }
            "$ElementData" => {
                post_processing::parse_element_data(&mut lines, &mut mesh)?;
            }
            "$ElementNodeData" => {
                post_processing::parse_element_node_data(&mut lines, &mut mesh)?;
            }
            "$InterpolationScheme" => {
                interpolation_scheme::parse(&mut lines, &mut mesh)?;
            }
            _ if trimmed.starts_with('$') && !trimmed.starts_with("$End") => {
                // Unknown section - skip it
                skip_section(&mut lines, trimmed)?;
            }
            _ => {
                // Ignore other lines (might be part of skipped sections)
            }
        }
    }

    if !mesh_format_parsed {
        return Err(ParseError::MissingSection("MeshFormat".to_string()));
    }

    Ok(mesh)
}

/// Skip an unknown section
fn skip_section<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    section_name: &str,
) -> Result<()> {
    let end_marker = format!("$End{}", &section_name[1..]);

    while let Some(line) = lines.next() {
        let line = line?;
        if line.trim() == end_marker {
            return Ok(());
        }
    }

    Err(ParseError::UnexpectedEof)
}

/// Helper function to read the next non-empty line
pub fn read_line<R: std::io::Read>(lines: &mut Lines<BufReader<R>>) -> Result<String> {
    loop {
        match lines.next() {
            Some(Ok(line)) => {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    return Ok(trimmed.to_string());
                }
            }
            Some(Err(e)) => return Err(e.into()),
            None => return Err(ParseError::UnexpectedEof),
        }
    }
}

/// Helper function to expect a specific end marker
pub fn expect_end_marker<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    section_name: &str,
) -> Result<()> {
    let line = read_line(lines)?;
    let expected = format!("$End{}", section_name);

    if line == expected {
        Ok(())
    } else {
        Err(ParseError::ExpectedEndOfSection(
            section_name.to_string(),
            line,
        ))
    }
}
