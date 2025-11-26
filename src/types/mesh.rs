use std::collections::HashMap;
use super::{Entities, Node, NodeBlock, Element, ElementBlock};

#[derive(Debug, Clone)]
pub struct MeshFormat {
    pub version: f64,
    pub file_type: i32,
    pub data_size: i32,
}

impl MeshFormat {
    pub fn new(version: f64, file_type: i32, data_size: i32) -> Self {
        Self {
            version,
            file_type,
            data_size,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub format: MeshFormat,
    pub physical_names: HashMap<(i32, i32), String>,  // (dimension, tag) -> name
    pub entities: Entities,
    pub node_blocks: Vec<NodeBlock>,
    pub element_blocks: Vec<ElementBlock>,

    // Flat access structures for O(1) lookup
    pub nodes_by_tag: HashMap<usize, Node>,
    pub elements_by_tag: HashMap<usize, Element>,
}

impl Mesh {
    pub fn new(format: MeshFormat) -> Self {
        Self {
            format,
            physical_names: HashMap::new(),
            entities: Entities::new(),
            node_blocks: Vec::new(),
            element_blocks: Vec::new(),
            nodes_by_tag: HashMap::new(),
            elements_by_tag: HashMap::new(),
        }
    }

    /// Build flat lookup maps from blocks
    pub fn build_indices(&mut self) {
        // Build node lookup map
        self.nodes_by_tag.clear();
        for block in &self.node_blocks {
            for node in &block.nodes {
                self.nodes_by_tag.insert(node.tag, node.clone());
            }
        }

        // Build element lookup map
        self.elements_by_tag.clear();
        for block in &self.element_blocks {
            for element in &block.elements {
                self.elements_by_tag.insert(element.tag, element.clone());
            }
        }
    }

    /// Get a node by its tag
    pub fn get_node(&self, tag: usize) -> Option<&Node> {
        self.nodes_by_tag.get(&tag)
    }

    /// Get an element by its tag
    pub fn get_element(&self, tag: usize) -> Option<&Element> {
        self.elements_by_tag.get(&tag)
    }

    /// Get all nodes in a specific entity
    pub fn nodes_in_entity(&self, dim: i32, tag: i32) -> Vec<&Node> {
        self.node_blocks
            .iter()
            .filter(|block| block.entity_dim == dim && block.entity_tag == tag)
            .flat_map(|block| &block.nodes)
            .collect()
    }

    /// Get all elements in a specific entity
    pub fn elements_in_entity(&self, dim: i32, tag: i32) -> Vec<&Element> {
        self.element_blocks
            .iter()
            .filter(|block| block.entity_dim == dim && block.entity_tag == tag)
            .flat_map(|block| &block.elements)
            .collect()
    }

    /// Iterate over all nodes
    pub fn nodes_iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes_by_tag.values()
    }

    /// Iterate over all elements
    pub fn elements_iter(&self) -> impl Iterator<Item = &Element> {
        self.elements_by_tag.values()
    }

    /// Get physical name for a given dimension and tag
    pub fn physical_name(&self, dim: i32, tag: i32) -> Option<&str> {
        self.physical_names.get(&(dim, tag)).map(|s| s.as_str())
    }

    /// Get total number of nodes
    pub fn num_nodes(&self) -> usize {
        self.nodes_by_tag.len()
    }

    /// Get total number of elements
    pub fn num_elements(&self) -> usize {
        self.elements_by_tag.len()
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new(MeshFormat::new(4.1, 0, 8))
    }
}
