use super::LineReader;
use crate::error::Result;
use crate::types::node::*;
use crate::types::{EntityDimension, Mesh, NodeBlock};

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    let token_line = reader.read_token_line()?;

    token_line.expect_len(4)?;

    let num_entity_blocks = token_line.tokens[0].parse_usize("numEntityBlocks")?;
    let _num_nodes = token_line.tokens[1].parse_usize("numNodes")?;
    let _min_node_tag = token_line.tokens[2].parse_usize("minNodeTag")?;
    let _max_node_tag = token_line.tokens[3].parse_usize("maxNodeTag")?;

    // Parse each entity block
    for _ in 0..num_entity_blocks {
        let block = parse_node_block(reader)?;
        mesh.node_blocks.push(block);
    }

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("Nodes")?;

    Ok(())
}

fn parse_node_block(reader: &mut LineReader) -> Result<NodeBlock> {
    let token_line = reader.read_token_line()?;

    token_line.expect_len(4)?;

    let entity_dim = token_line.tokens[0].parse_entity_dimension("entityDim")?;
    let entity_tag = token_line.tokens[1].parse_int("entityTag")?;
    let parametric_value = token_line.tokens[2].parse_int("parametric")?;
    let num_nodes_in_block = token_line.tokens[3].parse_usize("numNodesInBlock")?;

    // Validate parametric value (must be 0 or 1)
    let is_parametric = match parametric_value {
        0 => false,
        1 => true,
        _ => {
            return Err(token_line.tokens[2].invalid_data(format!(
                "parametric must be 0 or 1, found {}",
                parametric_value
            )))
        }
    };

    // First, read all node tags
    let mut node_tags = Vec::with_capacity(num_nodes_in_block);
    for _ in 0..num_nodes_in_block {
        let token_line = reader.read_token_line()?;
        let tag = token_line.tokens[0].parse_usize("nodeTag")?;
        node_tags.push(tag);
    }

    // Then, read all coordinates and create the appropriate NodeBlock
    match (entity_dim, is_parametric) {
        (EntityDimension::Point, _) => {
            // Point nodes (0D, no parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                token_line.expect_len(3)?;
                let x = token_line.tokens[0].parse_float("x")?;
                let y = token_line.tokens[1].parse_float("y")?;
                let z = token_line.tokens[2].parse_float("z")?;
                nodes.push(Node0D { tag, x, y, z });
            }
            Ok(NodeBlock::Point { entity_tag, nodes })
        }
        (EntityDimension::Curve, false) => {
            // Curve nodes (1D, no parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                token_line.expect_len(3)?;
                let x = token_line.tokens[0].parse_float("x")?;
                let y = token_line.tokens[1].parse_float("y")?;
                let z = token_line.tokens[2].parse_float("z")?;
                nodes.push(Node1D { tag, x, y, z });
            }
            Ok(NodeBlock::Curve { entity_tag, nodes })
        }
        (EntityDimension::Curve, true) => {
            // Curve nodes (1D, with parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                token_line.expect_len(4)?;
                let x = token_line.tokens[0].parse_float("x")?;
                let y = token_line.tokens[1].parse_float("y")?;
                let z = token_line.tokens[2].parse_float("z")?;
                let u = token_line.tokens[3].parse_float("u")?;
                nodes.push(Node1DParametric { tag, x, y, z, u });
            }
            Ok(NodeBlock::CurveParametric { entity_tag, nodes })
        }
        (EntityDimension::Surface, false) => {
            // Surface nodes (2D, no parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                token_line.expect_len(3)?;
                let x = token_line.tokens[0].parse_float("x")?;
                let y = token_line.tokens[1].parse_float("y")?;
                let z = token_line.tokens[2].parse_float("z")?;
                nodes.push(Node2D { tag, x, y, z });
            }
            Ok(NodeBlock::Surface { entity_tag, nodes })
        }
        (EntityDimension::Surface, true) => {
            // Surface nodes (2D, with parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                token_line.expect_len(5)?;
                let x = token_line.tokens[0].parse_float("x")?;
                let y = token_line.tokens[1].parse_float("y")?;
                let z = token_line.tokens[2].parse_float("z")?;
                let u = token_line.tokens[3].parse_float("u")?;
                let v = token_line.tokens[4].parse_float("v")?;
                nodes.push(Node2DParametric { tag, x, y, z, u, v });
            }
            Ok(NodeBlock::SurfaceParametric { entity_tag, nodes })
        }
        (EntityDimension::Volume, false) => {
            // Volume nodes (3D, no parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                token_line.expect_len(3)?;
                let x = token_line.tokens[0].parse_float("x")?;
                let y = token_line.tokens[1].parse_float("y")?;
                let z = token_line.tokens[2].parse_float("z")?;
                nodes.push(Node3D { tag, x, y, z });
            }
            Ok(NodeBlock::Volume { entity_tag, nodes })
        }
        (EntityDimension::Volume, true) => {
            // Volume nodes (3D, with parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                token_line.expect_len(6)?;
                let x = token_line.tokens[0].parse_float("x")?;
                let y = token_line.tokens[1].parse_float("y")?;
                let z = token_line.tokens[2].parse_float("z")?;
                let u = token_line.tokens[3].parse_float("u")?;
                let v = token_line.tokens[4].parse_float("v")?;
                let w = token_line.tokens[5].parse_float("w")?;
                nodes.push(Node3DParametric {
                    tag,
                    x,
                    y,
                    z,
                    u,
                    v,
                    w,
                });
            }
            Ok(NodeBlock::VolumeParametric { entity_tag, nodes })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

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

        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);
        let mut mesh = Mesh::default();

        let result = parse(&mut reader, &mut mesh);
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
