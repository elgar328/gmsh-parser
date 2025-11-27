use std::io::{BufReader, Lines};

use crate::error::{ParseError, Result};
use crate::types::{Mesh, NodeBlock};
use crate::types::node::*;
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

    let is_parametric = parametric != 0;

    // First, read all node tags
    let mut node_tags = Vec::with_capacity(num_nodes_in_block);
    for _ in 0..num_nodes_in_block {
        let line = read_line(lines)?;
        let tag: usize = line.parse()?;
        node_tags.push(tag);
    }

    // Then, read all coordinates and create the appropriate NodeBlock
    match (entity_dim, is_parametric) {
        (0, _) => {
            // Point nodes (0D, no parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
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
                nodes.push(Node0D { tag, x, y, z });
            }
            Ok(NodeBlock::Point { entity_tag, nodes })
        }
        (1, false) => {
            // Curve nodes (1D, no parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
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
                nodes.push(Node1D { tag, x, y, z });
            }
            Ok(NodeBlock::Curve { entity_tag, nodes })
        }
        (1, true) => {
            // Curve nodes (1D, with parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags {
                let line = read_line(lines)?;
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 4 {
                    return Err(ParseError::InvalidData(
                        "Nodes/Block".to_string(),
                        format!("Expected at least 4 values (x y z u), got {}", parts.len()),
                    ));
                }
                let x: f64 = parts[0].parse()?;
                let y: f64 = parts[1].parse()?;
                let z: f64 = parts[2].parse()?;
                let u: f64 = parts[3].parse()?;
                nodes.push(Node1DParametric { tag, x, y, z, u });
            }
            Ok(NodeBlock::CurveParametric { entity_tag, nodes })
        }
        (2, false) => {
            // Surface nodes (2D, no parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
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
                nodes.push(Node2D { tag, x, y, z });
            }
            Ok(NodeBlock::Surface { entity_tag, nodes })
        }
        (2, true) => {
            // Surface nodes (2D, with parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags {
                let line = read_line(lines)?;
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 5 {
                    return Err(ParseError::InvalidData(
                        "Nodes/Block".to_string(),
                        format!("Expected at least 5 values (x y z u v), got {}", parts.len()),
                    ));
                }
                let x: f64 = parts[0].parse()?;
                let y: f64 = parts[1].parse()?;
                let z: f64 = parts[2].parse()?;
                let u: f64 = parts[3].parse()?;
                let v: f64 = parts[4].parse()?;
                nodes.push(Node2DParametric { tag, x, y, z, u, v });
            }
            Ok(NodeBlock::SurfaceParametric { entity_tag, nodes })
        }
        (3, false) => {
            // Volume nodes (3D, no parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
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
                nodes.push(Node3D { tag, x, y, z });
            }
            Ok(NodeBlock::Volume { entity_tag, nodes })
        }
        (3, true) => {
            // Volume nodes (3D, with parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags {
                let line = read_line(lines)?;
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 6 {
                    return Err(ParseError::InvalidData(
                        "Nodes/Block".to_string(),
                        format!("Expected at least 6 values (x y z u v w), got {}", parts.len()),
                    ));
                }
                let x: f64 = parts[0].parse()?;
                let y: f64 = parts[1].parse()?;
                let z: f64 = parts[2].parse()?;
                let u: f64 = parts[3].parse()?;
                let v: f64 = parts[4].parse()?;
                let w: f64 = parts[5].parse()?;
                nodes.push(Node3DParametric { tag, x, y, z, u, v, w });
            }
            Ok(NodeBlock::VolumeParametric { entity_tag, nodes })
        }
        _ => Err(ParseError::InvalidData(
            "Nodes/Block".to_string(),
            format!("Invalid entity dimension: {}", entity_dim),
        )),
    }
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

        match &mesh.node_blocks[0] {
            NodeBlock::Surface { entity_tag, nodes } => {
                assert_eq!(*entity_tag, 1);
                assert_eq!(nodes.len(), 3);
                assert_eq!(nodes[0].tag, 1);
                assert_eq!(nodes[0].x, 0.0);
                assert_eq!(nodes[0].y, 0.0);
                assert_eq!(nodes[0].z, 0.0);
            }
            _ => panic!("Expected Surface node block"),
        }
    }
}
