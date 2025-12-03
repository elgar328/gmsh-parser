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
    let mut iter = token_line.iter();
    let num_string_tags = iter.parse_usize("numStringTags")?;
    iter.expect_no_more()?;

    for _ in 0..num_string_tags {
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();
        let tag = iter.parse_quoted_string_to_line_end()?;
        node_data.string_tags.push(tag);
    }

    // Read real tags
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();
    let num_real_tags = iter.parse_usize("numRealTags")?;
    iter.expect_no_more()?;

    for _ in 0..num_real_tags {
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();
        let tag = iter.parse_float("realTag")?;
        iter.expect_no_more()?;
        node_data.real_tags.push(tag);
    }

    // Read integer tags
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();
    let num_integer_tags = iter.parse_usize("numIntegerTags")?;
    iter.expect_no_more()?;

    for _ in 0..num_integer_tags {
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();
        let tag = iter.parse_int("integerTag")?;
        iter.expect_no_more()?;
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
        let mut iter = token_line.iter();

        let node_tag = iter.parse_usize("nodeTag")?;
        let values = iter.parse_floats(num_components, "value")?;
        iter.expect_no_more()?;

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
    let mut iter = token_line.iter();
    let num_string_tags = iter.parse_usize("numStringTags")?;
    iter.expect_no_more()?;

    for _ in 0..num_string_tags {
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();
        let tag = iter.parse_quoted_string_to_line_end()?;
        element_data.string_tags.push(tag);
    }

    // Read real tags
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();
    let num_real_tags = iter.parse_usize("numRealTags")?;
    iter.expect_no_more()?;

    for _ in 0..num_real_tags {
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();
        let tag = iter.parse_float("realTag")?;
        iter.expect_no_more()?;
        element_data.real_tags.push(tag);
    }

    // Read integer tags
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();
    let num_integer_tags = iter.parse_usize("numIntegerTags")?;
    iter.expect_no_more()?;

    for _ in 0..num_integer_tags {
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();
        let tag = iter.parse_int("integerTag")?;
        iter.expect_no_more()?;
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
        let mut iter = token_line.iter();

        let element_tag = iter.parse_usize("elementTag")?;
        let values = iter.parse_floats(num_components, "value")?;
        iter.expect_no_more()?;

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
    let mut iter = token_line.iter();
    let num_string_tags = iter.parse_usize("numStringTags")?;
    iter.expect_no_more()?;

    for _ in 0..num_string_tags {
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();
        let tag = iter.parse_quoted_string_to_line_end()?;
        element_node_data.string_tags.push(tag);
    }

    // Read real tags
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();
    let num_real_tags = iter.parse_usize("numRealTags")?;
    iter.expect_no_more()?;

    for _ in 0..num_real_tags {
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();
        let tag = iter.parse_float("realTag")?;
        iter.expect_no_more()?;
        element_node_data.real_tags.push(tag);
    }

    // Read integer tags
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();
    let num_integer_tags = iter.parse_usize("numIntegerTags")?;
    iter.expect_no_more()?;

    for _ in 0..num_integer_tags {
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();
        let tag = iter.parse_int("integerTag")?;
        iter.expect_no_more()?;
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
        let mut iter = token_line.iter();

        let element_tag = iter.parse_usize("elementTag")?;
        let num_nodes_per_element = iter.parse_usize("numNodesPerElement")?;

        let total_values = num_components * num_nodes_per_element;
        let values = iter.parse_floats(total_values, "value")?;
        iter.expect_no_more()?;

        element_node_data
            .data
            .push((element_tag, num_nodes_per_element, values));
    }

    mesh.element_node_data.push(element_node_data);

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("ElementNodeData")?;
    Ok(())
}
