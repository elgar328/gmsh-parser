pub mod type_enum;
pub use type_enum::ElementType;

/// Element structure definition
///
/// Simplified to a single generic structure used for all element types.
/// The node count is validated at runtime during parsing.
#[derive(Debug, Clone)]
pub struct Element {
    pub tag: usize,
    pub nodes: Vec<usize>,
}

impl Element {
    pub fn new(tag: usize, nodes: Vec<usize>) -> Self {
        Self { tag, nodes }
    }
}

/// ElementBlock definition
///
/// Represents a block of elements sharing the same type, dimension, and entity tag.
#[derive(Debug, Clone)]
pub struct ElementBlock {
    pub entity_dim: i32,
    pub entity_tag: i32,
    pub element_type: ElementType,
    pub elements: Vec<Element>,
}

impl ElementBlock {
    pub fn new(
        entity_dim: i32,
        entity_tag: i32,
        element_type: ElementType,
        elements: Vec<Element>,
    ) -> Self {
        Self {
            entity_dim,
            entity_tag,
            element_type,
            elements,
        }
    }
}
