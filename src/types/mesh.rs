//! Mesh structure - pure parsing result

use super::{
    Entities, NodeBlock, ElementBlock, PhysicalName,
    PeriodicLink, GhostElement, PartitionedEntities,
    Parametrizations, NodeData, ElementData, ElementNodeData,
    InterpolationScheme,
};

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
    pub physical_names: Vec<PhysicalName>,
    pub entities: Option<Entities>,
    pub partitioned_entities: Option<PartitionedEntities>,
    pub node_blocks: Vec<NodeBlock>,
    pub element_blocks: Vec<ElementBlock>,
    pub periodic_links: Vec<PeriodicLink>,
    pub ghost_elements: Vec<GhostElement>,
    pub parametrizations: Option<Parametrizations>,
    pub node_data: Vec<NodeData>,
    pub element_data: Vec<ElementData>,
    pub element_node_data: Vec<ElementNodeData>,
    pub interpolation_schemes: Vec<InterpolationScheme>,
}

impl Mesh {
    pub fn new(format: MeshFormat) -> Self {
        Self {
            format,
            physical_names: Vec::new(),
            entities: None,
            partitioned_entities: None,
            node_blocks: Vec::new(),
            element_blocks: Vec::new(),
            periodic_links: Vec::new(),
            ghost_elements: Vec::new(),
            parametrizations: None,
            node_data: Vec::new(),
            element_data: Vec::new(),
            element_node_data: Vec::new(),
            interpolation_schemes: Vec::new(),
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new(MeshFormat::new(4.1, 0, 8))
    }
}
