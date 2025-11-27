use std::io::{BufReader, Lines};

use crate::error::{ParseError, Result};
use crate::types::{Mesh, PhysicalName};
use super::{read_line, expect_end_marker};

pub fn parse<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    let line = read_line(lines)?;
    let num_physical_names: usize = line.parse()?;

    for _ in 0..num_physical_names {
        let line = read_line(lines)?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 3 {
            return Err(ParseError::InvalidData(
                "PhysicalNames".to_string(),
                format!("Expected at least 3 parts, got {}", parts.len()),
            ));
        }

        let dimension: i32 = parts[0].parse()?;
        let tag: i32 = parts[1].parse()?;

        // The name is quoted, need to extract it
        let name_start = line.find('"').ok_or_else(|| {
            ParseError::InvalidData(
                "PhysicalNames".to_string(),
                "Name must be quoted".to_string(),
            )
        })?;

        let name_part = &line[name_start..];
        let name = parse_quoted_string(name_part)?;

        mesh.physical_names.push(PhysicalName::new(dimension, tag, name));
    }

    expect_end_marker(lines, "PhysicalNames")?;

    Ok(())
}

fn parse_quoted_string(s: &str) -> Result<String> {
    if !s.starts_with('"') {
        return Err(ParseError::InvalidData(
            "PhysicalNames".to_string(),
            "String must start with quote".to_string(),
        ));
    }

    let end_quote = s[1..].find('"').ok_or_else(|| {
        ParseError::InvalidData(
            "PhysicalNames".to_string(),
            "String must end with quote".to_string(),
        )
    })?;

    Ok(s[1..=end_quote].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, Cursor};

    #[test]
    fn test_parse_physical_names() {
        let data = r#"2
2 1 "Surface"
3 2 "Volume"
$EndPhysicalNames
"#;
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        let mut lines = reader.lines();
        let mut mesh = Mesh::default();

        let result = parse(&mut lines, &mut mesh);
        assert!(result.is_ok());
        assert_eq!(mesh.physical_names.len(), 2);

        assert_eq!(mesh.physical_names[0].dimension, 2);
        assert_eq!(mesh.physical_names[0].tag, 1);
        assert_eq!(mesh.physical_names[0].name, "Surface");

        assert_eq!(mesh.physical_names[1].dimension, 3);
        assert_eq!(mesh.physical_names[1].tag, 2);
        assert_eq!(mesh.physical_names[1].name, "Volume");
    }

    #[test]
    fn test_parse_quoted_string() {
        assert_eq!(parse_quoted_string(r#""test""#).unwrap(), "test");
        assert_eq!(parse_quoted_string(r#""with space""#).unwrap(), "with space");
    }
}
