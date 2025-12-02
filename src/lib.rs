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
//! - `$PartitionedEntities` - Partitioned entity information
//! - `$Nodes` - Mesh nodes organized by entity blocks
//! - `$Elements` - Mesh elements organized by entity blocks
//! - `$Periodic` - Periodic boundary conditions
//! - `$GhostElements` - Ghost element information for parallel meshes
//! - `$Parametrizations` - Parametric curve and surface definitions
//! - `$NodeData`, `$ElementData`, `$ElementNodeData` - Post-processing data
//! - `$InterpolationScheme` - Custom interpolation schemes
//!
//! ## Unknown Sections
//!
//! When the parser encounters unknown sections (not listed above), it will:
//! - Skip the section content
//! - Add a warning to `mesh.warnings`
//! - Continue parsing the rest of the file
//!
//! This allows the parser to handle MSH files that contain sections not yet supported
//! or custom sections added by specific Gmsh versions.
//!
//! ## Example
//!
//! ### Quick Summary
//!
//! ```no_run
//! use gmsh_parser::parse_msh_file;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mesh = parse_msh_file("example.msh")?;
//!
//!     // Print a comprehensive summary of the mesh
//!     mesh.print_summary();
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Detailed Access
//!
//! ```no_run
//! use gmsh_parser::parse_msh_file;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mesh = parse_msh_file("example.msh")?;
//!
//!     // Access format information
//!     println!("Version: {}", mesh.format.version);
//!     println!("File type: {}", mesh.format.file_type);
//!
//!     // Access physical names
//!     println!("Physical groups: {}", mesh.physical_names.len());
//!
//!     // Access entities
//!     if let Some(entities) = &mesh.entities {
//!         println!("Volumes: {}", entities.volumes.len());
//!     }
//!
//!     // Access nodes and elements
//!     println!("Node blocks: {}", mesh.node_blocks.len());
//!     println!("Element blocks: {}", mesh.element_blocks.len());
//!
//!     // Check for warnings
//!     if !mesh.warnings.is_empty() {
//!         println!("\nWarnings during parsing:");
//!         for warning in &mesh.warnings {
//!             println!("  - {}", warning.message);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod parser;
pub mod types;

// Re-export main types and functions
pub use error::{ParseError, ParseWarning, Result};
pub use parser::{parse_msh, parse_msh_file};
pub use types::{
    CurveEntity, ElementBlock, ElementTopology, ElementType, Entities, EntityDimension, FileType,
    Mesh, MeshFormat, NodeBlock, PhysicalName, PointEntity, SurfaceEntity, Version, VolumeEntity,
};
