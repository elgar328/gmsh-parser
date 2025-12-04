//! Mesh structure - pure parsing result

use super::{
    ElementBlock, ElementData, ElementNodeData, Entities, GhostElement, InterpolationScheme,
    MeshFormat, NodeBlock, NodeData, Parametrizations, PartitionedEntities, PeriodicLink,
    PhysicalName,
};
use crate::error::{ParseError, ParseWarning};
use std::collections::HashSet;

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

    /// Validate mesh consistency
    ///
    /// Checks for:
    /// - Duplicate node tags
    /// - Duplicate element tags
    /// - Duplicate entity tags (if entities section is present)
    /// - Elements referencing missing nodes
    /// - Nodes referencing missing entities (if entities section is present)
    /// - Elements referencing missing entities (if entities section is present)
    pub fn validate(&self) -> crate::error::Result<()> {
        let entity_tags = self.validate_and_collect_entity_tags()?;
        let node_tags = self.validate_nodes(&entity_tags)?;
        self.validate_elements(&entity_tags, &node_tags)?;
        Ok(())
    }

    /// Validate and collect entity tags (dim, tag)
    fn validate_and_collect_entity_tags(
        &self,
    ) -> crate::error::Result<HashSet<(i32, i32)>> {
        // Collect entity tags: (dim, tag)
        let mut entity_tags = HashSet::new();

        if let Some(entities) = &self.entities {
            for p in &entities.points {
                if !entity_tags.insert((0, p.tag)) {
                    return Err(ParseError::MeshValidationError(format!(
                        "Duplicate point entity tag: {}",
                        p.tag
                    )));
                }
            }
            for c in &entities.curves {
                if !entity_tags.insert((1, c.tag)) {
                    return Err(ParseError::MeshValidationError(format!(
                        "Duplicate curve entity tag: {}",
                        c.tag
                    )));
                }
            }
            for s in &entities.surfaces {
                if !entity_tags.insert((2, s.tag)) {
                    return Err(ParseError::MeshValidationError(format!(
                        "Duplicate surface entity tag: {}",
                        s.tag
                    )));
                }
            }
            for v in &entities.volumes {
                if !entity_tags.insert((3, v.tag)) {
                    return Err(ParseError::MeshValidationError(format!(
                        "Duplicate volume entity tag: {}",
                        v.tag
                    )));
                }
            }
        }

        // Also include entities defined in the partitioned entities section.
        if let Some(partitioned) = &self.partitioned_entities {
            for p in &partitioned.points {
                entity_tags.insert((0, p.tag));
            }
            for c in &partitioned.curves {
                entity_tags.insert((1, c.tag));
            }
            for s in &partitioned.surfaces {
                entity_tags.insert((2, s.tag));
            }
            for v in &partitioned.volumes {
                entity_tags.insert((3, v.tag));
            }
        }

        Ok(entity_tags)
    }

    /// Validate nodes (duplicate tags, entity references)
    fn validate_nodes(
        &self,
        entity_tags: &HashSet<(i32, i32)>,
    ) -> crate::error::Result<HashSet<usize>> {
        let mut node_tags = HashSet::new();
        let mut duplicate_node_tag = None;
        let has_entity_info = self.entities.is_some() || self.partitioned_entities.is_some();

        for block in &self.node_blocks {
            // Check entity reference
            // Only check if we have any entity info at all (either regular or partitioned)
            if has_entity_info && !entity_tags.contains(&(block.entity_dim(), block.entity_tag())) {
                return Err(ParseError::MeshValidationError(format!(
                    "Node block references missing entity: dim={}, tag={}",
                    block.entity_dim(),
                    block.entity_tag()
                )));
            }

            block.for_each_tag(|tag| {
                if duplicate_node_tag.is_none() && !node_tags.insert(tag) {
                    duplicate_node_tag = Some(tag);
                }
            });

            if let Some(tag) = duplicate_node_tag {
                return Err(ParseError::MeshValidationError(format!(
                    "Duplicate node tag: {}",
                    tag
                )));
            }
        }

        Ok(node_tags)
    }

    /// Validate elements (duplicate tags, entity references, node references)
    fn validate_elements(
        &self,
        entity_tags: &HashSet<(i32, i32)>,
        node_tags: &HashSet<usize>,
    ) -> crate::error::Result<()> {
        let mut element_tags = HashSet::new();
        let has_entity_info = self.entities.is_some() || self.partitioned_entities.is_some();

        for block in &self.element_blocks {
            // Check entity reference
            if has_entity_info && !entity_tags.contains(&(block.entity_dim, block.entity_tag)) {
                return Err(ParseError::MeshValidationError(format!(
                    "Element block references missing entity: dim={}, tag={}",
                    block.entity_dim,
                    block.entity_tag
                )));
            }

            for element in &block.elements {
                if !element_tags.insert(element.tag) {
                    return Err(ParseError::MeshValidationError(format!(
                        "Duplicate element tag: {}",
                        element.tag
                    )));
                }

                // Check node references
                for node_tag in &element.nodes {
                    if !node_tags.contains(node_tag) {
                        return Err(ParseError::MeshValidationError(format!(
                            "Element {} references missing node {}",
                            element.tag, node_tag
                        )));
                    }
                }
            }
        }

        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::element::Element;
    use crate::types::{ElementBlock, ElementType, Node0D, NodeBlock, PointEntity};

    #[test]
    fn test_validate_duplicate_node_tag() {
        let mut mesh = Mesh::dummy();
        mesh.node_blocks.push(NodeBlock::Point {
            entity_tag: 1,
            nodes: vec![
                Node0D {
                    tag: 1,
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Node0D {
                    tag: 1, // Duplicate tag
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            ],
        });

        let result = mesh.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate node tag: 1"));
    }

    #[test]
    fn test_validate_duplicate_element_tag() {
        let mut mesh = Mesh::dummy();
        // Add valid nodes
        mesh.node_blocks.push(NodeBlock::Point {
            entity_tag: 1,
            nodes: vec![Node0D {
                tag: 1,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }],
        });

        // Add duplicate elements
        mesh.element_blocks.push(ElementBlock {
            entity_dim: 0,
            entity_tag: 1,
            element_type: ElementType::Point,
            elements: vec![
                Element {
                    tag: 1,
                    nodes: vec![1],
                },
                Element {
                    tag: 1, // Duplicate tag
                    nodes: vec![1],
                },
            ],
        });

        let result = mesh.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate element tag: 1"));
    }

    #[test]
    fn test_validate_missing_node_reference() {
        let mut mesh = Mesh::dummy();
        // Add valid nodes
        mesh.node_blocks.push(NodeBlock::Point {
            entity_tag: 1,
            nodes: vec![Node0D {
                tag: 1,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }],
        });

        // Add element referencing missing node
        mesh.element_blocks.push(ElementBlock {
            entity_dim: 0,
            entity_tag: 1,
            element_type: ElementType::Point,
            elements: vec![Element {
                tag: 1,
                nodes: vec![2], // Missing node 2
            }],
        });

        let result = mesh.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Element 1 references missing node 2"));
    }

    #[test]
    fn test_validate_missing_entity_reference() {
        let mut mesh = Mesh::dummy();
        // Define entities
        let mut entities = Entities::new();
        entities.points.push(PointEntity {
            tag: 1,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            physical_tags: vec![],
        });
        mesh.entities = Some(entities);

        // Add node block referencing missing entity
        mesh.node_blocks.push(NodeBlock::Point {
            entity_tag: 2, // Missing entity 2
            nodes: vec![Node0D {
                tag: 1,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }],
        });

        let result = mesh.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Node block references missing entity: dim=0, tag=2"));
    }

    #[test]
    fn test_validate_valid_mesh() {
        let mut mesh = Mesh::dummy();
        // Define entities
        let mut entities = Entities::new();
        entities.points.push(PointEntity {
            tag: 1,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            physical_tags: vec![],
        });
        mesh.entities = Some(entities);

        // Add valid nodes
        mesh.node_blocks.push(NodeBlock::Point {
            entity_tag: 1,
            nodes: vec![Node0D {
                tag: 1,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }],
        });

        // Add valid elements
        mesh.element_blocks.push(ElementBlock {
            entity_dim: 0,
            entity_tag: 1,
            element_type: ElementType::Point,
            elements: vec![Element {
                tag: 1,
                nodes: vec![1],
            }],
        });

        let result = mesh.validate();
        assert!(result.is_ok());
    }
}