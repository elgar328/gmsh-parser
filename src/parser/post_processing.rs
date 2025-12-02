//! Parser for post-processing sections: $NodeData, $ElementData, $ElementNodeData

use crate::error::Result;
use crate::types::{ElementData, ElementNodeData, Mesh, NodeData};

use super::LineReader;

/// Parse $NodeData section
pub fn parse_node_data(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    let mut node_data = NodeData {
        string_tags: Vec::new(),
        real_tags: Vec::new(),
        integer_tags: Vec::new(),
        data: Vec::new(),
    };

    // Read string tags
    let token_line = reader.read_token_line()?;
    let num_string_tags = token_line.tokens[0].parse_usize("numStringTags")?;

    for _ in 0..num_string_tags {
        let token_line = reader.read_token_line()?;
        let tag = token_line.tokens[0].parse_quoted_string_to_line_end()?;
        node_data.string_tags.push(tag);
    }

    // Read real tags
    let token_line = reader.read_token_line()?;
    let num_real_tags = token_line.tokens[0].parse_usize("numRealTags")?;

    for _ in 0..num_real_tags {
        let token_line = reader.read_token_line()?;
        let tag = token_line.tokens[0].parse_float("realTag")?;
        node_data.real_tags.push(tag);
    }

    // Read integer tags
    let token_line = reader.read_token_line()?;
    let num_integer_tags = token_line.tokens[0].parse_usize("numIntegerTags")?;

    for _ in 0..num_integer_tags {
        let token_line = reader.read_token_line()?;
        let tag = token_line.tokens[0].parse_int("integerTag")?;
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
        let token_line = reader.read_token_line()?;

        if token_line.tokens.is_empty() {
            continue;
        }

        let node_tag = token_line.tokens[0].parse_usize("nodeTag")?;

        token_line.expect_min_len(1 + num_components)?;
        let mut values = Vec::with_capacity(num_components);
        for i in 0..num_components {
            let value = token_line.tokens[1 + i].parse_float(&format!("value[{}]", i))?;
            values.push(value);
        }

        node_data.data.push((node_tag, values));
    }

    mesh.node_data.push(node_data);

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("NodeData")?;
    Ok(())
}

/// Parse $ElementData section
pub fn parse_element_data(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    let mut element_data = ElementData {
        string_tags: Vec::new(),
        real_tags: Vec::new(),
        integer_tags: Vec::new(),
        data: Vec::new(),
    };

    // Read string tags
    let token_line = reader.read_token_line()?;
    let num_string_tags = token_line.tokens[0].parse_usize("numStringTags")?;

    for _ in 0..num_string_tags {
        let token_line = reader.read_token_line()?;
        let tag = token_line.tokens[0].parse_quoted_string_to_line_end()?;
        element_data.string_tags.push(tag);
    }

    // Read real tags
    let token_line = reader.read_token_line()?;
    let num_real_tags = token_line.tokens[0].parse_usize("numRealTags")?;

    for _ in 0..num_real_tags {
        let token_line = reader.read_token_line()?;
        let tag = token_line.tokens[0].parse_float("realTag")?;
        element_data.real_tags.push(tag);
    }

    // Read integer tags
    let token_line = reader.read_token_line()?;
    let num_integer_tags = token_line.tokens[0].parse_usize("numIntegerTags")?;

    for _ in 0..num_integer_tags {
        let token_line = reader.read_token_line()?;
        let tag = token_line.tokens[0].parse_int("integerTag")?;
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
        let token_line = reader.read_token_line()?;

        if token_line.tokens.is_empty() {
            continue;
        }

        let element_tag = token_line.tokens[0].parse_usize("elementTag")?;

        token_line.expect_min_len(1 + num_components)?;
        let mut values = Vec::with_capacity(num_components);
        for i in 0..num_components {
            let value = token_line.tokens[1 + i].parse_float(&format!("value[{}]", i))?;
            values.push(value);
        }

        element_data.data.push((element_tag, values));
    }

    mesh.element_data.push(element_data);

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("ElementData")?;
    Ok(())
}

/// Parse $ElementNodeData section
pub fn parse_element_node_data(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    let mut element_node_data = ElementNodeData {
        string_tags: Vec::new(),
        real_tags: Vec::new(),
        integer_tags: Vec::new(),
        data: Vec::new(),
    };

    // Read string tags
    let token_line = reader.read_token_line()?;
    let num_string_tags = token_line.tokens[0].parse_usize("numStringTags")?;

    for _ in 0..num_string_tags {
        let token_line = reader.read_token_line()?;
        let tag = token_line.tokens[0].parse_quoted_string_to_line_end()?;
        element_node_data.string_tags.push(tag);
    }

    // Read real tags
    let token_line = reader.read_token_line()?;
    let num_real_tags = token_line.tokens[0].parse_usize("numRealTags")?;

    for _ in 0..num_real_tags {
        let token_line = reader.read_token_line()?;
        let tag = token_line.tokens[0].parse_float("realTag")?;
        element_node_data.real_tags.push(tag);
    }

    // Read integer tags
    let token_line = reader.read_token_line()?;
    let num_integer_tags = token_line.tokens[0].parse_usize("numIntegerTags")?;

    for _ in 0..num_integer_tags {
        let token_line = reader.read_token_line()?;
        let tag = token_line.tokens[0].parse_int("integerTag")?;
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
        let token_line = reader.read_token_line()?;

        token_line.expect_min_len(2)?;

        let element_tag = token_line.tokens[0].parse_usize("elementTag")?;
        let num_nodes_per_element = token_line.tokens[1].parse_usize("numNodesPerElement")?;

        let total_values = num_components * num_nodes_per_element;
        token_line.expect_min_len(2 + total_values)?;

        let mut values = Vec::with_capacity(total_values);
        for i in 0..total_values {
            let value = token_line.tokens[2 + i].parse_float(&format!("value[{}]", i))?;
            values.push(value);
        }

        element_node_data
            .data
            .push((element_tag, num_nodes_per_element, values));
    }

    mesh.element_node_data.push(element_node_data);

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("ElementNodeData")?;
    Ok(())
}
