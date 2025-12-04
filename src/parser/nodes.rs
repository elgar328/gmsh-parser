use super::LineReader;
use crate::error::{ParseError, Result};
use crate::parser::token::TokenIter;
use crate::types::node::*;
use crate::types::{EntityDimension, Mesh, NodeBlock};

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();

    let num_entity_blocks = iter.parse_usize("numEntityBlocks")?;

    // Parse metadata and validate later (after parsing all blocks)
    let metadata_iter = iter;

    // Parse each entity block
    for _ in 0..num_entity_blocks {
        let block = parse_node_block(reader)?;
        mesh.node_blocks.push(block);
    }

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("Nodes")?;

    // Validate parsed nodes against metadata
    validate_nodes_metadata(&mesh.node_blocks, metadata_iter)?;

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

/// Validate parsed nodes against metadata from the header
fn validate_nodes_metadata(node_blocks: &[NodeBlock], mut metadata_iter: TokenIter) -> Result<()> {
    // Parse metadata
    let num_nodes_token = metadata_iter.peek_token()?;
    let expected_num_nodes = metadata_iter.parse_usize("numNodes")?;

    let min_node_tag_token = metadata_iter.peek_token()?;
    let expected_min_node_tag = metadata_iter.parse_usize("minNodeTag")?;

    let max_node_tag_token = metadata_iter.peek_token()?;
    let expected_max_node_tag = metadata_iter.parse_usize("maxNodeTag")?;

    metadata_iter.expect_no_more()?;

    // Calculate actual stats
    let mut actual_num_nodes = 0;
    let mut actual_min_tag = usize::MAX;
    let mut actual_max_tag = usize::MIN;

    for block in node_blocks {
        actual_num_nodes += block.num_nodes();
        block.for_each_tag(|tag| {
            actual_min_tag = actual_min_tag.min(tag);
            actual_max_tag = actual_max_tag.max(tag);
        });
    }

    if actual_num_nodes != expected_num_nodes {
        return Err(ParseError::InvalidData {
            message: format!(
                "Node count mismatch: header declares {}, but {} were parsed",
                expected_num_nodes, actual_num_nodes
            ),
            span: num_nodes_token.span.to_source_span(),
            msh_content: num_nodes_token.source.clone(),
        });
    }

    // Handle case with no nodes
    if actual_num_nodes == 0 {
        return Err(ParseError::InvalidData {
            message: "The $Nodes section contains 0 nodes. A valid mesh must have at least one node.".to_string(),
            span: num_nodes_token.span.to_source_span(),
            msh_content: num_nodes_token.source.clone(),
        });
    }

    if actual_min_tag != expected_min_node_tag {
        return Err(ParseError::InvalidData {
            message: format!(
                "Minimum node tag mismatch: header declares {}, but actual minimum is {}",
                expected_min_node_tag, actual_min_tag
            ),
            span: min_node_tag_token.span.to_source_span(),
            msh_content: min_node_tag_token.source.clone(),
        });
    }

    if actual_max_tag != expected_max_node_tag {
        return Err(ParseError::InvalidData {
            message: format!(
                "Maximum node tag mismatch: header declares {}, but actual maximum is {}",
                expected_max_node_tag, actual_max_tag
            ),
            span: max_node_tag_token.span.to_source_span(),
            msh_content: max_node_tag_token.source.clone(),
        });
    }

    Ok(())
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

    #[test]
    fn test_parse_nodes_mismatch_count() {
        let data = r#"1 5 1 3
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
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_nodes_mismatch_min_tag() {
        let data = r#"1 3 10 3
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
        assert!(result.is_err());
    }
}
