use std::io::{BufReader, Lines};

use crate::error::{ParseError, Result};
use crate::types::{Mesh, mesh::MeshFormat};
use super::{read_line, expect_end_marker};

pub fn parse<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    let line = read_line(lines)?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 3 {
        return Err(ParseError::InvalidFormat(
            format!("Expected 3 values in MeshFormat, got {}", parts.len())
        ));
    }

    let version: f64 = parts[0].parse()?;
    let file_type: i32 = parts[1].parse()?;
    let data_size: i32 = parts[2].parse()?;

    // Validate version
    if (version - 4.1).abs() > 0.01 {
        return Err(ParseError::UnsupportedVersion(version));
    }

    // Validate file type (0 = ASCII, 1 = binary)
    if file_type != 0 {
        return Err(ParseError::UnsupportedFileType(file_type));
    }

    mesh.format = MeshFormat::new(version, file_type, data_size);

    expect_end_marker(lines, "MeshFormat")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, Cursor};

    #[test]
    fn test_parse_mesh_format() {
        let data = "4.1 0 8\n$EndMeshFormat\n";
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        let mut lines = reader.lines();
        let mut mesh = Mesh::default();

        let result = parse(&mut lines, &mut mesh);
        assert!(result.is_ok());
        assert_eq!(mesh.format.version, 4.1);
        assert_eq!(mesh.format.file_type, 0);
        assert_eq!(mesh.format.data_size, 8);
    }

    #[test]
    fn test_unsupported_version() {
        let data = "2.2 0 8\n$EndMeshFormat\n";
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        let mut lines = reader.lines();
        let mut mesh = Mesh::default();

        let result = parse(&mut lines, &mut mesh);
        assert!(matches!(result, Err(ParseError::UnsupportedVersion(_))));
    }

    #[test]
    fn test_binary_not_supported() {
        let data = "4.1 1 8\n$EndMeshFormat\n";
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        let mut lines = reader.lines();
        let mut mesh = Mesh::default();

        let result = parse(&mut lines, &mut mesh);
        assert!(matches!(result, Err(ParseError::UnsupportedFileType(_))));
    }
}
