//! Parser for post-processing sections: $NodeData, $ElementData, $ElementNodeData

use std::io::BufReader;
use std::io::Lines;

use crate::error::{ParseError, Result};
use crate::types::{Mesh, NodeData, ElementData, ElementNodeData};

use super::{read_line, expect_end_marker};

/// Parse $NodeData section
pub fn parse_node_data<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    let mut node_data = NodeData {
        string_tags: Vec::new(),
        real_tags: Vec::new(),
        integer_tags: Vec::new(),
        data: Vec::new(),
    };

    // Read string tags
    let num_string_tags: usize = read_line(lines)?
        .parse()
        .map_err(|_| ParseError::InvalidFormat("Invalid numStringTags".to_string()))?;

    for _ in 0..num_string_tags {
        let tag = read_line(lines)?;
        // Remove quotes if present
        let tag = tag.trim_matches('"').to_string();
        node_data.string_tags.push(tag);
    }

    // Read real tags
    let num_real_tags: usize = read_line(lines)?
        .parse()
        .map_err(|_| ParseError::InvalidFormat("Invalid numRealTags".to_string()))?;

    for _ in 0..num_real_tags {
        let tag: f64 = read_line(lines)?
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Invalid real tag".to_string()))?;
        node_data.real_tags.push(tag);
    }

    // Read integer tags
    let num_integer_tags: usize = read_line(lines)?
        .parse()
        .map_err(|_| ParseError::InvalidFormat("Invalid numIntegerTags".to_string()))?;

    for _ in 0..num_integer_tags {
        let tag: i32 = read_line(lines)?
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Invalid integer tag".to_string()))?;
        node_data.integer_tags.push(tag);
    }

    // Get number of components and entities from integer tags
    let num_components = if node_data.integer_tags.len() >= 2 {
        node_data.integer_tags[1] as usize
    } else {
        1
    };
    let num_entities = if node_data.integer_tags.len() >= 3 {
        node_data.integer_tags[2] as usize
    } else {
        0
    };

    // Read data
    for _ in 0..num_entities {
        let data_line = read_line(lines)?;
        let parts: Vec<&str> = data_line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let node_tag: usize = parts[0]
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Invalid node tag".to_string()))?;

        let mut values = Vec::with_capacity(num_components);
        for i in 0..num_components {
            if 1 + i >= parts.len() {
                return Err(ParseError::InvalidFormat("Not enough values".to_string()));
            }
            let value: f64 = parts[1 + i]
                .parse()
                .map_err(|_| ParseError::InvalidFormat("Invalid value".to_string()))?;
            values.push(value);
        }

        node_data.data.push((node_tag, values));
    }

    mesh.node_data.push(node_data);
    expect_end_marker(lines, "NodeData")?;
    Ok(())
}

/// Parse $ElementData section
pub fn parse_element_data<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    let mut element_data = ElementData {
        string_tags: Vec::new(),
        real_tags: Vec::new(),
        integer_tags: Vec::new(),
        data: Vec::new(),
    };

    // Read string tags
    let num_string_tags: usize = read_line(lines)?
        .parse()
        .map_err(|_| ParseError::InvalidFormat("Invalid numStringTags".to_string()))?;

    for _ in 0..num_string_tags {
        let tag = read_line(lines)?;
        let tag = tag.trim_matches('"').to_string();
        element_data.string_tags.push(tag);
    }

    // Read real tags
    let num_real_tags: usize = read_line(lines)?
        .parse()
        .map_err(|_| ParseError::InvalidFormat("Invalid numRealTags".to_string()))?;

    for _ in 0..num_real_tags {
        let tag: f64 = read_line(lines)?
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Invalid real tag".to_string()))?;
        element_data.real_tags.push(tag);
    }

    // Read integer tags
    let num_integer_tags: usize = read_line(lines)?
        .parse()
        .map_err(|_| ParseError::InvalidFormat("Invalid numIntegerTags".to_string()))?;

    for _ in 0..num_integer_tags {
        let tag: i32 = read_line(lines)?
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Invalid integer tag".to_string()))?;
        element_data.integer_tags.push(tag);
    }

    // Get number of components and entities from integer tags
    let num_components = if element_data.integer_tags.len() >= 2 {
        element_data.integer_tags[1] as usize
    } else {
        1
    };
    let num_entities = if element_data.integer_tags.len() >= 3 {
        element_data.integer_tags[2] as usize
    } else {
        0
    };

    // Read data
    for _ in 0..num_entities {
        let data_line = read_line(lines)?;
        let parts: Vec<&str> = data_line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let element_tag: usize = parts[0]
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Invalid element tag".to_string()))?;

        let mut values = Vec::with_capacity(num_components);
        for i in 0..num_components {
            if 1 + i >= parts.len() {
                return Err(ParseError::InvalidFormat("Not enough values".to_string()));
            }
            let value: f64 = parts[1 + i]
                .parse()
                .map_err(|_| ParseError::InvalidFormat("Invalid value".to_string()))?;
            values.push(value);
        }

        element_data.data.push((element_tag, values));
    }

    mesh.element_data.push(element_data);
    expect_end_marker(lines, "ElementData")?;
    Ok(())
}

/// Parse $ElementNodeData section
pub fn parse_element_node_data<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    let mut element_node_data = ElementNodeData {
        string_tags: Vec::new(),
        real_tags: Vec::new(),
        integer_tags: Vec::new(),
        data: Vec::new(),
    };

    // Read string tags
    let num_string_tags: usize = read_line(lines)?
        .parse()
        .map_err(|_| ParseError::InvalidFormat("Invalid numStringTags".to_string()))?;

    for _ in 0..num_string_tags {
        let tag = read_line(lines)?;
        let tag = tag.trim_matches('"').to_string();
        element_node_data.string_tags.push(tag);
    }

    // Read real tags
    let num_real_tags: usize = read_line(lines)?
        .parse()
        .map_err(|_| ParseError::InvalidFormat("Invalid numRealTags".to_string()))?;

    for _ in 0..num_real_tags {
        let tag: f64 = read_line(lines)?
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Invalid real tag".to_string()))?;
        element_node_data.real_tags.push(tag);
    }

    // Read integer tags
    let num_integer_tags: usize = read_line(lines)?
        .parse()
        .map_err(|_| ParseError::InvalidFormat("Invalid numIntegerTags".to_string()))?;

    for _ in 0..num_integer_tags {
        let tag: i32 = read_line(lines)?
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Invalid integer tag".to_string()))?;
        element_node_data.integer_tags.push(tag);
    }

    // Get number of components and entities from integer tags
    let num_components = if element_node_data.integer_tags.len() >= 2 {
        element_node_data.integer_tags[1] as usize
    } else {
        1
    };
    let num_entities = if element_node_data.integer_tags.len() >= 3 {
        element_node_data.integer_tags[2] as usize
    } else {
        0
    };

    // Read data
    for _ in 0..num_entities {
        let data_line = read_line(lines)?;
        let parts: Vec<&str> = data_line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let element_tag: usize = parts[0]
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Invalid element tag".to_string()))?;
        let num_nodes_per_element: usize = parts[1]
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Invalid numNodesPerElement".to_string()))?;

        let total_values = num_components * num_nodes_per_element;
        let mut values = Vec::with_capacity(total_values);
        for i in 0..total_values {
            if 2 + i >= parts.len() {
                return Err(ParseError::InvalidFormat("Not enough values".to_string()));
            }
            let value: f64 = parts[2 + i]
                .parse()
                .map_err(|_| ParseError::InvalidFormat("Invalid value".to_string()))?;
            values.push(value);
        }

        element_node_data
            .data
            .push((element_tag, num_nodes_per_element, values));
    }

    mesh.element_node_data.push(element_node_data);
    expect_end_marker(lines, "ElementNodeData")?;
    Ok(())
}
