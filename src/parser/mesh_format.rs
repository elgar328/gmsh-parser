use super::LineReader;
use crate::error::{ParseError, Result};
use crate::types::{FileType, Mesh, MeshFormat, Version};

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    let token_line = reader.read_token_line()?;

    token_line.expect_len(3)?;

    let version_token = &token_line.tokens[0];
    let version = version_token.parse_version()?;

    let file_type_token = &token_line.tokens[1];
    let file_type = file_type_token.parse_file_type("file_type")?;
    let data_size = token_line.tokens[2].parse_int("data_size")?;

    // Validate version (only support MSH 4.1 and 4.1.0)
    if version != Version::new(4, 1, None) && version != Version::new(4, 1, Some(0)) {
        return Err(ParseError::UnsupportedVersion {
            version: format!("{}", version),
            span: version_token.span.to_source_span(),
            msh_content: version_token.source.clone(),
        });
    }

    // Only support ASCII for now
    if file_type != FileType::Ascii {
        return Err(ParseError::UnsupportedFileType {
            file_type,
            span: file_type_token.span.to_source_span(),
            msh_content: file_type_token.source.clone(),
        });
    }

    mesh.format = MeshFormat::new(version, file_type, data_size);

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("MeshFormat")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn test_parse_mesh_format() {
        let data = "4.1 0 8\n$EndMeshFormat\n";
        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);
        let mut mesh = Mesh::default();

        let result = parse(&mut reader, &mut mesh);
        assert!(result.is_ok());
        assert_eq!(mesh.format.version, Version::new(4, 1, None));
        assert_eq!(mesh.format.file_type, FileType::Ascii);
        assert_eq!(mesh.format.data_size, 8);
    }

    #[test]
    fn test_unsupported_version() {
        let data = "2.2 0 8\n$EndMeshFormat\n";
        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);
        let mut mesh = Mesh::default();

        let result = parse(&mut reader, &mut mesh);
        assert!(matches!(result, Err(ParseError::UnsupportedVersion { .. })));
    }

    #[test]
    fn test_binary_not_supported() {
        let data = "4.1 1 8\n$EndMeshFormat\n";
        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);
        let mut mesh = Mesh::default();

        let result = parse(&mut reader, &mut mesh);
        assert!(matches!(
            result,
            Err(ParseError::UnsupportedFileType { .. })
        ));
    }

    #[test]
    fn test_parse_mesh_format_4_1_0() {
        let data = "4.1.0 0 8\n$EndMeshFormat\n";
        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);
        let mut mesh = Mesh::default();

        let result = parse(&mut reader, &mut mesh);
        assert!(result.is_ok());
        assert_eq!(mesh.format.version, Version::new(4, 1, Some(0)));
        assert_eq!(mesh.format.file_type, FileType::Ascii);
        assert_eq!(mesh.format.data_size, 8);
    }

    #[test]
    fn test_unsupported_patch_version() {
        let data = "4.1.1 0 8\n$EndMeshFormat\n";
        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);
        let mut mesh = Mesh::default();

        let result = parse(&mut reader, &mut mesh);
        assert!(matches!(result, Err(ParseError::UnsupportedVersion { .. })));
    }
}
