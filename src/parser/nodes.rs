use super::LineReader;
use crate::error::Result;
use crate::types::node::*;
use crate::types::{EntityDimension, Mesh, NodeBlock};

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();

    let num_entity_blocks = iter.parse_usize("numEntityBlocks")?;
    let _num_nodes = iter.parse_usize("numNodes")?;
    let _min_node_tag = iter.parse_usize("minNodeTag")?;
    let _max_node_tag = iter.parse_usize("maxNodeTag")?;

    iter.expect_no_more()?;

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
    let mut iter = token_line.iter();

    let entity_dim = iter.parse_entity_dimension("entityDim")?;
    let entity_tag = iter.parse_int("entityTag")?;
    let is_parametric = iter.parse_bool("parametric")?;
    let num_nodes_in_block = iter.parse_usize("numNodesInBlock")?;

    iter.expect_no_more()?;

    // First, read all node tags
    let mut node_tags = Vec::with_capacity(num_nodes_in_block);
    for _ in 0..num_nodes_in_block {
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();
        let tag = iter.parse_usize("nodeTag")?;
        iter.expect_no_more()?;
        node_tags.push(tag);
    }

    // Then, read all coordinates and create the appropriate NodeBlock
    match (entity_dim, is_parametric) {
        (EntityDimension::Point, _) => {
            // Point nodes (0D, no parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                let mut iter = token_line.iter();
                let x = iter.parse_float("x")?;
                let y = iter.parse_float("y")?;
                let z = iter.parse_float("z")?;
                iter.expect_no_more()?;
                nodes.push(Node0D { tag, x, y, z });
            }
            Ok(NodeBlock::Point { entity_tag, nodes })
        }
        (EntityDimension::Curve, false) => {
            // Curve nodes (1D, no parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                let mut iter = token_line.iter();
                let x = iter.parse_float("x")?;
                let y = iter.parse_float("y")?;
                let z = iter.parse_float("z")?;
                iter.expect_no_more()?;
                nodes.push(Node1D { tag, x, y, z });
            }
            Ok(NodeBlock::Curve { entity_tag, nodes })
        }
        (EntityDimension::Curve, true) => {
            // Curve nodes (1D, with parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                let mut iter = token_line.iter();
                let x = iter.parse_float("x")?;
                let y = iter.parse_float("y")?;
                let z = iter.parse_float("z")?;
                let u = iter.parse_float("u")?;
                iter.expect_no_more()?;
                nodes.push(Node1DParametric { tag, x, y, z, u });
            }
            Ok(NodeBlock::CurveParametric { entity_tag, nodes })
        }
        (EntityDimension::Surface, false) => {
            // Surface nodes (2D, no parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                let mut iter = token_line.iter();
                let x = iter.parse_float("x")?;
                let y = iter.parse_float("y")?;
                let z = iter.parse_float("z")?;
                iter.expect_no_more()?;
                nodes.push(Node2D { tag, x, y, z });
            }
            Ok(NodeBlock::Surface { entity_tag, nodes })
        }
        (EntityDimension::Surface, true) => {
            // Surface nodes (2D, with parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                let mut iter = token_line.iter();
                let x = iter.parse_float("x")?;
                let y = iter.parse_float("y")?;
                let z = iter.parse_float("z")?;
                let u = iter.parse_float("u")?;
                let v = iter.parse_float("v")?;
                iter.expect_no_more()?;
                nodes.push(Node2DParametric { tag, x, y, z, u, v });
            }
            Ok(NodeBlock::SurfaceParametric { entity_tag, nodes })
        }
        (EntityDimension::Volume, false) => {
            // Volume nodes (3D, no parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                let mut iter = token_line.iter();
                let x = iter.parse_float("x")?;
                let y = iter.parse_float("y")?;
                let z = iter.parse_float("z")?;
                iter.expect_no_more()?;
                nodes.push(Node3D { tag, x, y, z });
            }
            Ok(NodeBlock::Volume { entity_tag, nodes })
        }
        (EntityDimension::Volume, true) => {
            // Volume nodes (3D, with parametric coordinates)
            let mut nodes = Vec::with_capacity(num_nodes_in_block);
            for tag in node_tags.into_iter() {
                let token_line = reader.read_token_line()?;
                let mut iter = token_line.iter();
                let x = iter.parse_float("x")?;
                let y = iter.parse_float("y")?;
                let z = iter.parse_float("z")?;
                let u = iter.parse_float("u")?;
                let v = iter.parse_float("v")?;
                let w = iter.parse_float("w")?;
                iter.expect_no_more()?;
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
        let mut mesh = Mesh::dummy();

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
