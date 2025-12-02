use super::LineReader;
use crate::error::Result;
use crate::types::{Mesh, PhysicalName};

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    let token_line = reader.read_token_line()?;
    let num_physical_names = token_line.tokens[0].parse_usize("numPhysicalNames")?;

    for _ in 0..num_physical_names {
        let token_line = reader.read_token_line()?;

        token_line.expect_min_len(3)?;

        let dimension = token_line.tokens[0].parse_entity_dimension("PhysicalNames")?;
        let tag = token_line.tokens[1].parse_int("tag")?;
        let name = token_line.tokens[2].parse_quoted_string_to_line_end()?;

        mesh.physical_names
            .push(PhysicalName::new(dimension, tag, name));
    }

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("PhysicalNames")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::super::*;
    use super::super::*;
    use super::*;

    #[test]
    fn test_parse_physical_names() {
        let data = r#"2
2 1 "Surface"
3 2 "Volume"
$EndPhysicalNames
"#;

        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);
        let mut mesh = Mesh::default();

        let result = parse(&mut reader, &mut mesh);
        assert!(result.is_ok());
        assert_eq!(mesh.physical_names.len(), 2);

        assert_eq!(mesh.physical_names[0].dimension, EntityDimension::Surface);
        assert_eq!(mesh.physical_names[0].tag, 1);
        assert_eq!(mesh.physical_names[0].name, "Surface");

        assert_eq!(mesh.physical_names[1].dimension, EntityDimension::Volume);
        assert_eq!(mesh.physical_names[1].tag, 2);
        assert_eq!(mesh.physical_names[1].name, "Volume");
    }
}
