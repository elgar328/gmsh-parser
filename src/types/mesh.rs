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
            .map(|block| {
                use ElementBlock::*;
                match block {
                    Line2 { elements, .. } => elements.len(),
                    Triangle3 { elements, .. } => elements.len(),
                    Quadrangle4 { elements, .. } => elements.len(),
                    Tetrahedron4 { elements, .. } => elements.len(),
                    Hexahedron8 { elements, .. } => elements.len(),
                    Prism6 { elements, .. } => elements.len(),
                    Pyramid5 { elements, .. } => elements.len(),
                    Line3 { elements, .. } => elements.len(),
                    Triangle6 { elements, .. } => elements.len(),
                    Quadrangle9 { elements, .. } => elements.len(),
                    Tetrahedron10 { elements, .. } => elements.len(),
                    Hexahedron27 { elements, .. } => elements.len(),
                    Prism18 { elements, .. } => elements.len(),
                    Pyramid14 { elements, .. } => elements.len(),
                    Point { elements, .. } => elements.len(),
                    Quadrangle8 { elements, .. } => elements.len(),
                    Hexahedron20 { elements, .. } => elements.len(),
                    Prism15 { elements, .. } => elements.len(),
                    Pyramid13 { elements, .. } => elements.len(),
                    Triangle9 { elements, .. } => elements.len(),
                    Triangle10 { elements, .. } => elements.len(),
                    Triangle12 { elements, .. } => elements.len(),
                    Triangle15 { elements, .. } => elements.len(),
                    Triangle15I { elements, .. } => elements.len(),
                    Triangle21 { elements, .. } => elements.len(),
                    Line4 { elements, .. } => elements.len(),
                    Line5 { elements, .. } => elements.len(),
                    Line6 { elements, .. } => elements.len(),
                    Tetrahedron20 { elements, .. } => elements.len(),
                    Tetrahedron35 { elements, .. } => elements.len(),
                    Tetrahedron56 { elements, .. } => elements.len(),
                    Tetrahedron22 { elements, .. } => elements.len(),
                    Tetrahedron28 { elements, .. } => elements.len(),
                    Polygon { elements, .. } => elements.len(),
                    Polyhedron { elements, .. } => elements.len(),
                    Quadrangle16 { elements, .. } => elements.len(),
                    Quadrangle25 { elements, .. } => elements.len(),
                    Quadrangle36 { elements, .. } => elements.len(),
                    Quadrangle12 { elements, .. } => elements.len(),
                    Quadrangle16I { elements, .. } => elements.len(),
                    Quadrangle20 { elements, .. } => elements.len(),
                    Triangle28 { elements, .. } => elements.len(),
                    Triangle36 { elements, .. } => elements.len(),
                    Triangle45 { elements, .. } => elements.len(),
                    Triangle55 { elements, .. } => elements.len(),
                    Triangle66 { elements, .. } => elements.len(),
                    Quadrangle49 { elements, .. } => elements.len(),
                    Quadrangle64 { elements, .. } => elements.len(),
                    Quadrangle81 { elements, .. } => elements.len(),
                    Quadrangle100 { elements, .. } => elements.len(),
                    Quadrangle121 { elements, .. } => elements.len(),
                    Triangle18 { elements, .. } => elements.len(),
                    Triangle21I { elements, .. } => elements.len(),
                    Triangle24 { elements, .. } => elements.len(),
                    Triangle27 { elements, .. } => elements.len(),
                    Triangle30 { elements, .. } => elements.len(),
                    Quadrangle24 { elements, .. } => elements.len(),
                    Quadrangle28 { elements, .. } => elements.len(),
                    Quadrangle32 { elements, .. } => elements.len(),
                    Quadrangle36I { elements, .. } => elements.len(),
                    Quadrangle40 { elements, .. } => elements.len(),
                    Line7 { elements, .. } => elements.len(),
                    Line8 { elements, .. } => elements.len(),
                    Line9 { elements, .. } => elements.len(),
                    Line10 { elements, .. } => elements.len(),
                    Line11 { elements, .. } => elements.len(),
                    LineB { elements, .. } => elements.len(),
                    TriangleB { elements, .. } => elements.len(),
                    PolygonB { elements, .. } => elements.len(),
                    LineC { elements, .. } => elements.len(),
                    Tetrahedron84 { elements, .. } => elements.len(),
                    Tetrahedron120 { elements, .. } => elements.len(),
                    Tetrahedron165 { elements, .. } => elements.len(),
                    Tetrahedron220 { elements, .. } => elements.len(),
                    Tetrahedron286 { elements, .. } => elements.len(),
                    Tetrahedron34 { elements, .. } => elements.len(),
                    Tetrahedron40 { elements, .. } => elements.len(),
                    Tetrahedron46 { elements, .. } => elements.len(),
                    Tetrahedron52 { elements, .. } => elements.len(),
                    Tetrahedron58 { elements, .. } => elements.len(),
                    Line1 { elements, .. } => elements.len(),
                    Triangle1 { elements, .. } => elements.len(),
                    Quadrangle1 { elements, .. } => elements.len(),
                    Tetrahedron1 { elements, .. } => elements.len(),
                    Hexahedron1 { elements, .. } => elements.len(),
                    Prism1 { elements, .. } => elements.len(),
                    Prism40 { elements, .. } => elements.len(),
                    Prism75 { elements, .. } => elements.len(),
                    Hexahedron64 { elements, .. } => elements.len(),
                    Hexahedron125 { elements, .. } => elements.len(),
                    Hexahedron216 { elements, .. } => elements.len(),
                    Hexahedron343 { elements, .. } => elements.len(),
                    Hexahedron512 { elements, .. } => elements.len(),
                    Hexahedron729 { elements, .. } => elements.len(),
                    Hexahedron1000 { elements, .. } => elements.len(),
                    Hexahedron32 { elements, .. } => elements.len(),
                    Hexahedron44 { elements, .. } => elements.len(),
                    Hexahedron56 { elements, .. } => elements.len(),
                    Hexahedron68 { elements, .. } => elements.len(),
                    Hexahedron80 { elements, .. } => elements.len(),
                    Hexahedron92 { elements, .. } => elements.len(),
                    Hexahedron104 { elements, .. } => elements.len(),
                    Prism126 { elements, .. } => elements.len(),
                    Prism196 { elements, .. } => elements.len(),
                    Prism288 { elements, .. } => elements.len(),
                    Prism405 { elements, .. } => elements.len(),
                    Prism550 { elements, .. } => elements.len(),
                    Prism24 { elements, .. } => elements.len(),
                    Prism33 { elements, .. } => elements.len(),
                    Prism42 { elements, .. } => elements.len(),
                    Prism51 { elements, .. } => elements.len(),
                    Prism60 { elements, .. } => elements.len(),
                    Prism69 { elements, .. } => elements.len(),
                    Prism78 { elements, .. } => elements.len(),
                    Pyramid30 { elements, .. } => elements.len(),
                    Pyramid55 { elements, .. } => elements.len(),
                    Pyramid91 { elements, .. } => elements.len(),
                    Pyramid140 { elements, .. } => elements.len(),
                    Pyramid204 { elements, .. } => elements.len(),
                    Pyramid285 { elements, .. } => elements.len(),
                    Pyramid385 { elements, .. } => elements.len(),
                    Pyramid21 { elements, .. } => elements.len(),
                    Pyramid29 { elements, .. } => elements.len(),
                    Pyramid37 { elements, .. } => elements.len(),
                    Pyramid45 { elements, .. } => elements.len(),
                    Pyramid53 { elements, .. } => elements.len(),
                    Pyramid61 { elements, .. } => elements.len(),
                    Pyramid69 { elements, .. } => elements.len(),
                    Pyramid1 { elements, .. } => elements.len(),
                    PointSub { elements, .. } => elements.len(),
                    LineSub { elements, .. } => elements.len(),
                    TriangleSub { elements, .. } => elements.len(),
                    TetrahedronSub { elements, .. } => elements.len(),
                    Tetrahedron16 { elements, .. } => elements.len(),
                    TriangleMini { elements, .. } => elements.len(),
                    TetrahedronMini { elements, .. } => elements.len(),
                    TriHedron4 { elements, .. } => elements.len(),
                }
            })
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
