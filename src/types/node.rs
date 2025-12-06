use crate::types::EntityDimension;

/// Unified NodeBlock structure.
/// Corresponds to each entity block in the $Nodes section.
#[derive(Debug, Clone)]
pub struct NodeBlock {
    pub entity_dim: EntityDimension,
    pub entity_tag: i32,
    pub parametric: bool, // Parametric flag parsed from the block header (0 or 1)
    pub nodes: Vec<Node>,
}

impl NodeBlock {
    /// Returns the number of nodes in this block.
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the dimension of the entity this block belongs to as i32.
    /// This is for compatibility with existing code that expects an i32.
    pub fn entity_dim(&self) -> i32 {
        self.entity_dim as i32
    }

    /// Returns the tag of the entity this block belongs to.
    pub fn entity_tag(&self) -> i32 {
        self.entity_tag
    }
}

/// Unified Node structure.
/// Uses the same type regardless of dimension or parametric status.
#[derive(Debug, Clone)]
pub struct Node {
    pub tag: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// Optional parametric coordinates.
    pub parametric_coords: Option<Vec<f64>>,
}
