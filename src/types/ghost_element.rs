//! Ghost elements section types
//!
//! Defines ghost elements for parallel processing.

/// Ghost element information
#[derive(Debug, Clone)]
pub struct GhostElement {
    /// Element tag
    pub element_tag: usize,
    /// Partition tag
    pub partition_tag: i32,
    /// Ghost partition tags
    pub ghost_partition_tags: Vec<i32>,
}
