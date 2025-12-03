use super::LineReader;
use crate::error::{ParseError, Result};
use crate::types::{FileType, MeshFormat};

/// Parse $MeshFormat section and return MeshFormat
/// This function expects the reader to be positioned at the line containing "$MeshFormat"
pub fn parse(reader: &mut LineReader) -> Result<MeshFormat> {
    // Verify $MeshFormat header
    let token_line = reader.read_token_line()?;
    token_line.expect_section_start("MeshFormat")?;

    // Parse format line: version file_type data_size
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();

    let version = iter.parse_version()?;
    let file_type = iter.parse_file_type("file_type")?;
    let data_size = iter.parse_int("data_size")?;

    iter.expect_no_more()?;

    // Validate version (only support MSH 4.1)
    if !version.is_supported() {
        return Err(ParseError::UnsupportedVersion {
            version: format!("{}", version),
            span: version.token.span.to_source_span(),
            msh_content: version.token.source.clone(),
        });
    }

    // Only support ASCII for now
    if file_type != FileType::Ascii {
        return Err(ParseError::UnsupportedFileType { file_type });
    }

    // Verify $EndMeshFormat
    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("MeshFormat")?;

    Ok(MeshFormat::new(version, file_type, data_size))
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn test_parse_mesh_format() {
        let data = "$MeshFormat\n4.1 0 8\n$EndMeshFormat\n";
        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);

        let result = parse(&mut reader);
        assert!(result.is_ok());
        let format = result.unwrap();
        assert_eq!(format.version.major, 4);
        assert_eq!(format.version.minor, 1);
        assert_eq!(format.file_type, FileType::Ascii);
        assert_eq!(format.data_size, 8);
    }

    #[test]
    fn test_unsupported_version() {
        let data = "$MeshFormat\n2.2 0 8\n$EndMeshFormat\n";
        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);

        let result = parse(&mut reader);
        assert!(matches!(result, Err(ParseError::UnsupportedVersion { .. })));
    }

    #[test]
    fn test_binary_not_supported() {
        let data = "$MeshFormat\n4.1 1 8\n$EndMeshFormat\n";
        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);

        let result = parse(&mut reader);
        assert!(matches!(
            result,
            Err(ParseError::UnsupportedFileType { .. })
        ));
    }
}
