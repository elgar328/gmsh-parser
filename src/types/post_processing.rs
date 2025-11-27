//! Post-processing data section types
//!
//! Defines types for NodeData, ElementData, and ElementNodeData sections.

/// Post-processing view data associated with nodes
#[derive(Debug, Clone)]
pub struct NodeData {
    /// View name and interpolation scheme name
    pub string_tags: Vec<String>,
    /// Time value and other real parameters
    pub real_tags: Vec<f64>,
    /// Time step, num components, num entities, partition index
    pub integer_tags: Vec<i32>,
    /// Data: (node_tag, values)
    pub data: Vec<(usize, Vec<f64>)>,
}

/// Post-processing view data associated with elements
#[derive(Debug, Clone)]
pub struct ElementData {
    /// View name and interpolation scheme name
    pub string_tags: Vec<String>,
    /// Time value and other real parameters
    pub real_tags: Vec<f64>,
    /// Time step, num components, num entities, partition index
    pub integer_tags: Vec<i32>,
    /// Data: (element_tag, values)
    pub data: Vec<(usize, Vec<f64>)>,
}

/// Post-processing view data associated with element nodes
#[derive(Debug, Clone)]
pub struct ElementNodeData {
    /// View name and interpolation scheme name
    pub string_tags: Vec<String>,
    /// Time value and other real parameters
    pub real_tags: Vec<f64>,
    /// Time step, num components, num entities, partition index
    pub integer_tags: Vec<i32>,
    /// Data: (element_tag, num_nodes_per_element, values)
    pub data: Vec<(usize, usize, Vec<f64>)>,
}
