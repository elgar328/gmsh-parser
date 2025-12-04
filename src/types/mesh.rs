//! Mesh structure - pure parsing result

use super::{
    ElementBlock, ElementData, ElementNodeData, Entities, GhostElement, InterpolationScheme,
    MeshFormat, NodeBlock, NodeData, Parametrizations, PartitionedEntities, PeriodicLink,
    PhysicalName,
};
use crate::error::ParseWarning;

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
    pub warnings: Vec<ParseWarning>,
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
            warnings: Vec::new(),
        }
    }

    /// Print a summary of the mesh contents
    pub fn print_summary(&self) {
        println!("=== Mesh Summary ===\n");

        // Format information
        println!("Format:");
        println!("  Version: {}", self.format.version);
        println!("  File Type: {}", self.format.file_type);
        println!("  Data Size: {}", self.format.data_size);

        // Physical names
        println!("\nPhysical Groups: {}", self.physical_names.len());
        if !self.physical_names.is_empty() {
            for pn in &self.physical_names {
                println!(
                    "  [dim={}, tag={}]: \"{}\"",
                    pn.dimension as i32, pn.tag, pn.name
                );
            }
        }

        // Entities
        if let Some(entities) = &self.entities {
            println!("\nEntities:");
            println!("  Points: {}", entities.points.len());
            println!("  Curves: {}", entities.curves.len());
            println!("  Surfaces: {}", entities.surfaces.len());
            println!("  Volumes: {}", entities.volumes.len());
        }

        // Nodes
        let total_nodes: usize = self
            .node_blocks
            .iter()
            .map(|block| match block {
                NodeBlock::Point { nodes, .. } => nodes.len(),
                NodeBlock::Curve { nodes, .. } => nodes.len(),
                NodeBlock::CurveParametric { nodes, .. } => nodes.len(),
                NodeBlock::Surface { nodes, .. } => nodes.len(),
                NodeBlock::SurfaceParametric { nodes, .. } => nodes.len(),
                NodeBlock::Volume { nodes, .. } => nodes.len(),
                NodeBlock::VolumeParametric { nodes, .. } => nodes.len(),
            })
            .sum();
        println!("\nNodes:");
        println!("  Node blocks: {}", self.node_blocks.len());
        println!("  Total nodes: {}", total_nodes);

        // Elements
        let total_elements: usize = self
            .element_blocks
            .iter()
            .map(|block| block.elements.len())
            .sum();
        println!("\nElements:");
        println!("  Element blocks: {}", self.element_blocks.len());
        println!("  Total elements: {}", total_elements);

        // Other data
        if !self.periodic_links.is_empty() {
            println!("\nPeriodic Links: {}", self.periodic_links.len());
        }
        if !self.ghost_elements.is_empty() {
            println!("\nGhost Elements: {}", self.ghost_elements.len());
        }
        if !self.node_data.is_empty() {
            println!("\nNode Data: {}", self.node_data.len());
        }
        if !self.element_data.is_empty() {
            println!("\nElement Data: {}", self.element_data.len());
        }
        if !self.element_node_data.is_empty() {
            println!("\nElement Node Data: {}", self.element_node_data.len());
        }
        if !self.interpolation_schemes.is_empty() {
            println!(
                "\nInterpolation Schemes: {}",
                self.interpolation_schemes.len()
            );
        }

        // Warnings
        if !self.warnings.is_empty() {
            println!("\nWarnings: {}", self.warnings.len());
            for warning in &self.warnings {
                println!("  - {}", warning.message);
            }
        }
    }

    /// Create a dummy Mesh for testing purposes
    #[cfg(test)]
    pub fn dummy() -> Self {
        use crate::parser::{SourceFile, Span, Token};

        // Simulating: "$MeshFormat\n4.1 0 8\n$EndMeshFormat\n"
        // "4.1" starts at offset 12 (after "$MeshFormat\n")
        let source_content = "$MeshFormat\n4.1 0 8\n$EndMeshFormat\n";
        let source_file = SourceFile::new(source_content.into());
        let token = Token::new(
            "4.1".to_string(),
            Span::new(12, 3), // offset=12, len=3
            source_file.content,
        );
        let version = super::Version::new(4, 1, token);
        let format = MeshFormat::new(version, super::FileType::Ascii, 8);
        Self::new(format)
    }
}