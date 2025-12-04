use super::{LineReader, TokenLine};
use crate::error::{ParseError, Result};
use crate::parser::token::TokenIter;
use crate::types::element::{Element, ElementBlock};
use crate::types::{ElementType, Mesh};

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();

    let num_entity_blocks = iter.parse_usize("numEntityBlocks")?;
    let _num_elements = iter.parse_usize("numElements")?;
    let _min_element_tag = iter.parse_usize("minElementTag")?;
    let _max_element_tag = iter.parse_usize("maxElementTag")?;
    iter.expect_no_more()?;

    // Parse each entity block
    for _ in 0..num_entity_blocks {
        let block = parse_element_block(reader)?;
        mesh.element_blocks.push(block);
    }

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("Elements")?;

    Ok(())
}

fn parse_element_block(reader: &mut LineReader) -> Result<ElementBlock> {
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();

    let entity_dim = iter.parse_int("entityDim")?;
    let entity_tag = iter.parse_int("entityTag")?;
    let element_type = iter.parse_element_type("elementType")?;
    let num_elements_in_block = iter.parse_usize("numElementsInBlock")?;
    iter.expect_no_more()?;

    let mut elements = Vec::with_capacity(num_elements_in_block);

    // Get the expected node count for this element type
    let fixed_count = element_type.fixed_node_count();

    for _ in 0..num_elements_in_block {
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();

        let tag = iter.parse_usize("elementTag")?;
        let nodes = parse_element_nodes(&mut iter, &token_line, tag, element_type, fixed_count)?;

        elements.push(Element::new(tag, nodes));
    }

    Ok(ElementBlock::new(
        entity_dim,
        entity_tag,
        element_type,
        elements,
    ))
}

/// Parse nodes for a single element with improved error messages
fn parse_element_nodes(
    iter: &mut TokenIter,
    token_line: &TokenLine,
    tag: usize,
    element_type: ElementType,
    fixed_count: Option<usize>,
) -> Result<Vec<usize>> {
    let mut nodes = Vec::new();

    match fixed_count {
        Some(count) => {
            // Fixed number of nodes
            nodes.reserve(count);
            for i in 0..count {
                match iter.parse_usize("nodeTag") {
                    Ok(node) => nodes.push(node),
                    Err(ParseError::UnexpectedEndOfLine { .. }) => {
                        return Err(token_line.invalid_format(format!(
                            "Element {} ({:?}) requires {} nodes, but line ended after {} nodes",
                            tag, element_type, count, i
                        )));
                    }
                    Err(e) => return Err(e),
                }
            }
            iter.expect_no_more()?;
        }
        None => {
            // Variable number of nodes (Polygon, Polyhedron, etc.)
            while iter.has_next() {
                nodes.push(iter.parse_usize("nodeTag")?);
            }

            // Validate that at least one node is present
            if nodes.is_empty() {
                return Err(token_line.invalid_format(format!(
                    "Element {} ({:?}) has no nodes",
                    tag, element_type
                )));
            }
        }
    }

    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::types::ElementType;

    #[test]
    fn test_parse_elements() {
        let data = r#"1 2 1 2
2 1 3 2
1 1 2 3 4
2 2 5 6 3
$EndElements
"#;

        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);
        let mut mesh = Mesh::dummy();

        let result = parse(&mut reader, &mut mesh);
        assert!(result.is_ok());
        assert_eq!(mesh.element_blocks.len(), 1);

        let block = &mesh.element_blocks[0];
        assert_eq!(block.element_type, ElementType::Quadrangle4);
        assert_eq!(block.entity_dim, 2);
        assert_eq!(block.entity_tag, 1);
        assert_eq!(block.elements.len(), 2);

        assert_eq!(block.elements[0].tag, 1);
        assert_eq!(block.elements[0].nodes, vec![1, 2, 3, 4]);
    }
}
