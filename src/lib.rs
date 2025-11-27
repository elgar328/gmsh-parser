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
//!     println!("Node blocks: {}", mesh.node_blocks.len());
//!     println!("Element blocks: {}", mesh.element_blocks.len());
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
    Mesh, MeshFormat,
    NodeBlock,
    ElementBlock, ElementType,
    Entities, PointEntity, CurveEntity, SurfaceEntity, VolumeEntity, EntityDimension,
    PhysicalName,
};
pub use parser::{parse_msh_file, parse_msh};
