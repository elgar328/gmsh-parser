//! # gmsh-parser
//!
//! A Rust library for parsing Gmsh MSH 4.1 format files.
//!
//! This library provides functionality to read and parse Gmsh mesh files in ASCII MSH 4.1 format.
//! It supports parsing of nodes, elements, entities, and physical groups.
//!
//! ## Supported Sections
//! - `$MeshFormat` - File format version and metadata
//! - `$PhysicalNames` - Physical group names
//! - `$Entities` - Geometric entities (points, curves, surfaces, volumes)
//! - `$Nodes` - Mesh nodes organized by entity blocks
//! - `$Elements` - Mesh elements organized by entity blocks
//!
//! ## Example
//!
//! ```no_run
//! use gmsh_parser::parse_msh_file;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mesh = parse_msh_file("example.msh")?;
//!
//!     println!("Version: {}", mesh.format.version);
//!     println!("Total nodes: {}", mesh.num_nodes());
//!     println!("Total elements: {}", mesh.num_elements());
//!
//!     // Iterate by entity
//!     for block in &mesh.element_blocks {
//!         println!("Entity {}-{}: {} elements",
//!             block.entity_dim, block.entity_tag, block.elements.len());
//!     }
//!
//!     // Direct lookup
//!     if let Some(node) = mesh.get_node(1) {
//!         println!("Node 1: ({}, {}, {})", node.x, node.y, node.z);
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod types;
pub mod parser;

// Re-export main types and functions
pub use error::{ParseError, Result};
pub use types::{
    Mesh,
    Node, NodeBlock,
    Element, ElementBlock, ElementType,
    Entities, PointEntity, CurveEntity, SurfaceEntity, VolumeEntity, EntityDimension,
};
pub use parser::{parse_msh_file, parse_msh};
