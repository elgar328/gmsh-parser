use std::io::{BufReader, Lines};

use crate::error::{ParseError, Result};
use crate::types::{Mesh, Node, NodeBlock};
use super::{read_line, expect_end_marker};

pub fn parse<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    let line = read_line(lines)?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 4 {
        return Err(ParseError::InvalidData(
            "Nodes".to_string(),
            format!("Expected 4 values in header, got {}", parts.len()),
        ));
    }

    let num_entity_blocks: usize = parts[0].parse()?;
    let _num_nodes: usize = parts[1].parse()?;
    let _min_node_tag: usize = parts[2].parse()?;
    let _max_node_tag: usize = parts[3].parse()?;

    // Parse each entity block
    for _ in 0..num_entity_blocks {
        let block = parse_node_block(lines)?;
        mesh.node_blocks.push(block);
    }

    expect_end_marker(lines, "Nodes")?;

    Ok(())
}

fn parse_node_block<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
) -> Result<NodeBlock> {
    let line = read_line(lines)?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 4 {
        return Err(ParseError::InvalidData(
            "Nodes/Block".to_string(),
            format!("Expected 4 values in block header, got {}", parts.len()),
        ));
    }

    let entity_dim: i32 = parts[0].parse()?;
    let entity_tag: i32 = parts[1].parse()?;
    let parametric: i32 = parts[2].parse()?;
    let num_nodes_in_block: usize = parts[3].parse()?;

    let parametric = parametric != 0;

    let mut block = NodeBlock::new(entity_dim, entity_tag, parametric);

    // First, read all node tags
    let mut node_tags = Vec::with_capacity(num_nodes_in_block);
    for _ in 0..num_nodes_in_block {
        let line = read_line(lines)?;
        let tag: usize = line.parse()?;
        node_tags.push(tag);
    }

    // Then, read all coordinates
    for tag in node_tags {
        let line = read_line(lines)?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 3 {
            return Err(ParseError::InvalidData(
                "Nodes/Block".to_string(),
                format!("Expected at least 3 coordinates, got {}", parts.len()),
            ));
        }

        let x: f64 = parts[0].parse()?;
        let y: f64 = parts[1].parse()?;
        let z: f64 = parts[2].parse()?;

        let mut node = Node::new(tag, x, y, z);

        // Parse parametric coordinates if present
        if parametric {
            let u = if entity_dim >= 1 && parts.len() > 3 {
                Some(parts[3].parse()?)
            } else {
                None
            };

            let v = if entity_dim >= 2 && parts.len() > 4 {
                Some(parts[4].parse()?)
            } else {
                None
            };

            let w = if entity_dim == 3 && parts.len() > 5 {
                Some(parts[5].parse()?)
            } else {
                None
            };

            node = node.with_parametric(u, v, w);
        }

        block.nodes.push(node);
    }

    Ok(block)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, Cursor};

    #[test]
    fn test_parse_nodes() {
        let data = r#"1 3 1 3
2 1 0 3
1
2
3
0.0 0.0 0.0
1.0 0.0 0.0
1.0 1.0 0.0
$EndNodes
"#;
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        let mut lines = reader.lines();
        let mut mesh = Mesh::default();

        let result = parse(&mut lines, &mut mesh);
        assert!(result.is_ok());
        assert_eq!(mesh.node_blocks.len(), 1);

        let block = &mesh.node_blocks[0];
        assert_eq!(block.entity_dim, 2);
        assert_eq!(block.entity_tag, 1);
        assert!(!block.parametric);
        assert_eq!(block.nodes.len(), 3);

        let node = &block.nodes[0];
        assert_eq!(node.tag, 1);
        assert_eq!(node.x, 0.0);
        assert_eq!(node.y, 0.0);
        assert_eq!(node.z, 0.0);
    }
}
