//! Parser for $GhostElements section

use crate::error::Result;
use crate::types::{GhostElement, Mesh};

use super::LineReader;

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    // Read number of ghost elements
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();

    let num_ghost_elements = iter.parse_usize("numGhostElements")?;

    for _ in 0..num_ghost_elements {
        // Read: elementTag partitionTag numGhostPartitions ghostPartitionTag ...
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();

        let element_tag = iter.parse_usize("elementTag")?;
        let partition_tag = iter.parse_int("partitionTag")?;

        let num_ghost_partitions = iter.parse_usize("numGhostPartitions")?;
        let ghost_partition_tags = iter.parse_ints(num_ghost_partitions, "ghostPartitionTag")?;

        iter.expect_no_more()?;

        mesh.ghost_elements.push(GhostElement {
            element_tag,
            partition_tag,
            ghost_partition_tags,
        });
    }

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("GhostElements")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn test_parse_ghost_elements() {
        let data = r#"2
1 0 2 1 2
5 1 1 0
$EndGhostElements
"#;

        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);
        let mut mesh = Mesh::dummy();

        parse(&mut reader, &mut mesh).unwrap();

        assert_eq!(mesh.ghost_elements.len(), 2);

        let elem1 = &mesh.ghost_elements[0];
        assert_eq!(elem1.element_tag, 1);
        assert_eq!(elem1.partition_tag, 0);
        assert_eq!(elem1.ghost_partition_tags, vec![1, 2]);

        let elem2 = &mesh.ghost_elements[1];
        assert_eq!(elem2.element_tag, 5);
        assert_eq!(elem2.partition_tag, 1);
        assert_eq!(elem2.ghost_partition_tags, vec![0]);
    }
}
