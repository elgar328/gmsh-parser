//! Parser for $GhostElements section

use std::io::BufReader;
use std::io::Lines;

use crate::error::{ParseError, Result};
use crate::types::{Mesh, GhostElement};

use super::{read_line, expect_end_marker};

pub fn parse<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    // Read number of ghost elements
    let header = read_line(lines)?;
    let num_ghost_elements: usize = header
        .parse()
        .map_err(|_| ParseError::InvalidFormat(format!("Invalid numGhostElements: {}", header)))?;

    for _ in 0..num_ghost_elements {
        // Read: elementTag partitionTag numGhostPartitions ghostPartitionTag ...
        let line = read_line(lines)?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(ParseError::InvalidFormat(format!(
                "Invalid ghost element line: {}",
                line
            )));
        }

        let element_tag: usize = parts[0].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid element_tag: {}", parts[0]))
        })?;
        let partition_tag: i32 = parts[1].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid partition_tag: {}", parts[1]))
        })?;
        let num_ghost_partitions: usize = parts[2].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid numGhostPartitions: {}", parts[2]))
        })?;

        if parts.len() < 3 + num_ghost_partitions {
            return Err(ParseError::InvalidFormat(format!(
                "Not enough ghost partition tags in line: {}",
                line
            )));
        }

        let mut ghost_partition_tags = Vec::with_capacity(num_ghost_partitions);
        for i in 0..num_ghost_partitions {
            let tag: i32 = parts[3 + i].parse().map_err(|_| {
                ParseError::InvalidFormat(format!("Invalid ghost partition tag: {}", parts[3 + i]))
            })?;
            ghost_partition_tags.push(tag);
        }

        mesh.ghost_elements.push(GhostElement {
            element_tag,
            partition_tag,
            ghost_partition_tags,
        });
    }

    expect_end_marker(lines, "GhostElements")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, Cursor};

    #[test]
    fn test_parse_ghost_elements() {
        let data = r#"2
1 0 2 1 2
5 1 1 0
$EndGhostElements
"#;
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        let mut lines = reader.lines();
        let mut mesh = Mesh::default();

        parse(&mut lines, &mut mesh).unwrap();

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
