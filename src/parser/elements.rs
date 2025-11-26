use std::io::{BufReader, Lines};

use crate::error::{ParseError, Result};
use crate::types::{Mesh, Element, ElementBlock, ElementType};
use super::{read_line, expect_end_marker};

pub fn parse<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    let line = read_line(lines)?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 4 {
        return Err(ParseError::InvalidData(
            "Elements".to_string(),
            format!("Expected 4 values in header, got {}", parts.len()),
        ));
    }

    let num_entity_blocks: usize = parts[0].parse()?;
    let _num_elements: usize = parts[1].parse()?;
    let _min_element_tag: usize = parts[2].parse()?;
    let _max_element_tag: usize = parts[3].parse()?;

    // Parse each entity block
    for _ in 0..num_entity_blocks {
        let block = parse_element_block(lines)?;
        mesh.element_blocks.push(block);
    }

    expect_end_marker(lines, "Elements")?;

    Ok(())
}

fn parse_element_block<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
) -> Result<ElementBlock> {
    let line = read_line(lines)?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 4 {
        return Err(ParseError::InvalidData(
            "Elements/Block".to_string(),
            format!("Expected 4 values in block header, got {}", parts.len()),
        ));
    }

    let entity_dim: i32 = parts[0].parse()?;
    let entity_tag: i32 = parts[1].parse()?;
    let element_type_id: i32 = parts[2].parse()?;
    let num_elements_in_block: usize = parts[3].parse()?;

    let element_type = ElementType::from_i32(element_type_id)?;
    let num_nodes_per_element = element_type.num_nodes();

    let mut block = ElementBlock::new(entity_dim, entity_tag, element_type);

    // Parse each element
    for _ in 0..num_elements_in_block {
        let line = read_line(lines)?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 1 + num_nodes_per_element {
            return Err(ParseError::InvalidData(
                "Elements/Block".to_string(),
                format!(
                    "Expected {} values (1 tag + {} nodes), got {}",
                    1 + num_nodes_per_element,
                    num_nodes_per_element,
                    parts.len()
                ),
            ));
        }

        let element_tag: usize = parts[0].parse()?;

        let node_tags: Result<Vec<usize>> = parts[1..1 + num_nodes_per_element]
            .iter()
            .map(|s| s.parse().map_err(Into::into))
            .collect();

        let element = Element::new(element_tag, element_type, node_tags?);
        block.elements.push(element);
    }

    Ok(block)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, Cursor};

    #[test]
    fn test_parse_elements() {
        let data = r#"1 2 1 2
2 1 3 2
1 1 2 3 4
2 2 5 6 3
$EndElements
"#;
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        let mut lines = reader.lines();
        let mut mesh = Mesh::default();

        let result = parse(&mut lines, &mut mesh);
        assert!(result.is_ok());
        assert_eq!(mesh.element_blocks.len(), 1);

        let block = &mesh.element_blocks[0];
        assert_eq!(block.entity_dim, 2);
        assert_eq!(block.entity_tag, 1);
        assert_eq!(block.element_type, ElementType::Quadrangle4);
        assert_eq!(block.elements.len(), 2);

        let element = &block.elements[0];
        assert_eq!(element.tag, 1);
        assert_eq!(element.node_tags, vec![1, 2, 3, 4]);
    }
}
