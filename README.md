# gmsh-parser

A Rust library for parsing Gmsh MSH 4.1 format files.

## Features

- **MSH 4.1 ASCII format support**: Parse Gmsh mesh files in ASCII format
- **Entity-based organization**: Preserves the entity-based structure of MSH 4.1
- **Comprehensive section support**: All standard MSH 4.1 sections including post-processing data
- **Type-safe element types**: Complete support for all 140 Gmsh element types
- **Better error reporting**: Uses miette for detailed, user-friendly error messages

## Supported Sections

- ✅ `$MeshFormat` - File format version and metadata
- ✅ `$PhysicalNames` - Physical group names
- ✅ `$Entities` - Geometric entities (points, curves, surfaces, volumes)
- ✅ `$PartitionedEntities` - Partitioned entity information
- ✅ `$Nodes` - Mesh nodes organized by entity blocks
- ✅ `$Elements` - Mesh elements organized by entity blocks
- ✅ `$Periodic` - Periodic boundary conditions
- ✅ `$GhostElements` - Ghost element information for parallel meshes
- ✅ `$Parametrizations` - Parametric curve and surface definitions
- ✅ `$NodeData` - Node-based post-processing data
- ✅ `$ElementData` - Element-based post-processing data
- ✅ `$ElementNodeData` - Element-node-based post-processing data
- ✅ `$InterpolationScheme` - Custom interpolation schemes

## Not Supported

- Binary MSH files (only ASCII)
- Legacy MSH formats (1.0, 2.x, 4.0)
- Writing/generating MSH files

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gmsh-parser = { git = "https://github.com/elgar328/gmsh-parser" }
```

## Usage

### Quick Summary

The easiest way to get an overview of a mesh file:

```rust
use gmsh_parser::parse_msh_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mesh = parse_msh_file("mesh.msh")?;

    // Print a comprehensive summary
    mesh.print_summary();

    Ok(())
}
```

### Detailed Access

For programmatic access to mesh data:

```rust
use gmsh_parser::{parse_msh_file, NodeBlock, ElementBlock};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a MSH file
    let mesh = parse_msh_file("mesh.msh")?;

    // Access format information
    println!("MSH version: {}", mesh.format.version);
    println!("File type: {}", mesh.format.file_type);

    // Access physical names
    for pn in &mesh.physical_names {
        println!("Physical group [dim={}, tag={}]: \"{}\"",
            pn.dimension as i32, pn.tag, pn.name);
    }

    // Access entities
    if let Some(entities) = &mesh.entities {
        println!("Points: {}", entities.points.len());
        println!("Curves: {}", entities.curves.len());
        println!("Surfaces: {}", entities.surfaces.len());
        println!("Volumes: {}", entities.volumes.len());
    }

    // Iterate over node blocks
    for block in &mesh.node_blocks {
        let node_count = match block {
            NodeBlock::Point { nodes, .. } => nodes.len(),
            NodeBlock::Curve { nodes, .. } => nodes.len(),
            NodeBlock::Surface { nodes, .. } => nodes.len(),
            NodeBlock::Volume { nodes, .. } => nodes.len(),
            _ => 0,
        };
        println!("Node block: {} nodes", node_count);
    }

    // Iterate over element blocks
    for block in &mesh.element_blocks {
        use ElementBlock::*;
        let elem_count = match block {
            Triangle3 { elements, .. } => elements.len(),
            Tetrahedron4 { elements, .. } => elements.len(),
            _ => 0,
        };
        println!("Element block: {} elements", elem_count);
    }

    // Check for warnings
    if !mesh.warnings.is_empty() {
        println!("\nWarnings:");
        for warning in &mesh.warnings {
            println!("  - {}", warning.message);
        }
    }

    Ok(())
}
```

## Data Structure

The parser provides entity-based organization:

### Entity-based blocks
- Preserves file structure and entity relationships
- Efficient for entity-based iteration
- Useful for understanding mesh topology

```rust
// Iterate by entity blocks
for block in &mesh.node_blocks {
    match block {
        NodeBlock::Volume { entity_tag, nodes } => {
            println!("Volume entity {}: {} nodes", entity_tag, nodes.len());
        }
        _ => {}
    }
}
```

## Element Types

Complete support for all 140 Gmsh element types (1-140):

- **Linear**: Point, Line2, Triangle3, Quadrangle4, Tetrahedron4, Hexahedron8, Prism6, Pyramid5
- **Second order**: Line3, Triangle6, Quadrangle8/9, Tetrahedron10, Hexahedron20/27, Prism15/18, Pyramid13/14
- **Higher order** (up to 10th order):
  - Lines: Line4 through Line11
  - Triangles: Triangle9 through Triangle66
  - Quadrangles: Quadrangle12 through Quadrangle121
  - Tetrahedra: Tetrahedron16 through Tetrahedron286
  - Hexahedra: Hexahedron32 through Hexahedron1000
  - Prisms: Prism24 through Prism550
  - Pyramids: Pyramid21 through Pyramid385
- **Variable size**: Polygon, Polyhedron, LineB, TriangleB, PolygonB, LineC
- **Special**: 1-node elements, Sub elements, Mini elements

## Examples

Run the included example:

```bash
cargo run --example parse_box
```

This will parse the `tests/data/custom/box.msh` file and display a comprehensive summary including:
- Format information (version, file type, data size)
- Physical groups with their names
- Entity counts (points, curves, surfaces, volumes)
- Node and element statistics
- Optional sections (periodic links, ghost elements, post-processing data)
- Any warnings encountered during parsing

## Testing

Run unit tests:

```bash
cargo test
```

Run integration tests with real MSH files:

```bash
cargo test --test integration_test
```
