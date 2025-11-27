//! Periodic section types
//!
//! Defines periodicity relations between entities and their corresponding nodes.

/// Periodic link between two entities
#[derive(Debug, Clone)]
pub struct PeriodicLink {
    /// Dimension of the entity (0, 1, 2, or 3)
    pub entity_dim: i32,
    /// Tag of the slave entity
    pub entity_tag: i32,
    /// Tag of the master entity
    pub entity_tag_master: i32,
    /// Affine transformation matrix (can be empty if not provided)
    pub affine_transform: Vec<f64>,
    /// Node correspondences: (slave_node_tag, master_node_tag)
    pub node_correspondences: Vec<(usize, usize)>,
}
