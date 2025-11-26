# gmsh-parser

A Rust library for parsing Gmsh MSH 4.1 format files.

## Features

- **MSH 4.1 ASCII format support**: Parse Gmsh mesh files in ASCII format
- **Entity-based organization**: Preserves the entity-based structure of MSH 4.1
- **Efficient data access**: Both entity-based iteration and O(1) tag-based lookup
- **Comprehensive parsing**: Supports MeshFormat, PhysicalNames, Entities, Nodes, and Elements sections
- **Type-safe element types**: All 33 standard Gmsh element types (1-31, 92-93) supported

## Supported Sections

- ✅ `$MeshFormat` - File format version and metadata
- ✅ `$PhysicalNames` - Physical group names
- ✅ `$Entities` - Geometric entities (points, curves, surfaces, volumes)
- ✅ `$Nodes` - Mesh nodes organized by entity blocks
- ✅ `$Elements` - Mesh elements organized by entity blocks
- ⏳ `$Periodic` - Periodicity relations (future)
- ⏳ `$NodeData` - Node-based post-processing data (future)
- ⏳ `$ElementData` - Element-based post-processing data (future)
- ⏳ `$ElementNodeData` - Element-node-based post-processing data (future)

## Not Supported

- Binary MSH files (only ASCII)
- Legacy MSH formats (1.0, 2.x, 4.0)
- Writing/generating MSH files
- Partitioned entities, ghost elements, parametrizations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
gmsh-parser = "0.1.0"
```

## Usage

```rust
use gmsh_parser::parse_msh_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse a MSH file
    let mesh = parse_msh_file("mesh.msh")?;

    // Access format information
    println!("MSH version: {}", mesh.format.version);

    // Access physical names
    if let Some(name) = mesh.physical_name(3, 1) {
        println!("Volume 1: {}", name);
    }

    // Count nodes and elements
    println!("Total nodes: {}", mesh.num_nodes());
    println!("Total elements: {}", mesh.num_elements());

    // Direct lookup by tag (O(1))
    if let Some(node) = mesh.get_node(1) {
        println!("Node 1: ({}, {}, {})", node.x, node.y, node.z);
    }

    // Iterate over all nodes
    for node in mesh.nodes_iter() {
        println!("Node {}: ({}, {}, {})", node.tag, node.x, node.y, node.z);
    }

    // Entity-based queries
    for block in &mesh.element_blocks {
        println!("Entity {}-{}: {} elements of type {:?}",
            block.entity_dim,
            block.entity_tag,
            block.elements.len(),
            block.element_type
        );
    }

    // Get all elements in a specific entity
    let elements = mesh.elements_in_entity(3, 1);
    println!("Elements in volume 1: {}", elements.len());

    Ok(())
}
```

## Data Structure

The parser provides a hybrid structure:

### Entity-based blocks
- Preserves file structure and entity relationships
- Efficient for entity-based iteration
- Useful for understanding mesh topology

```rust
// Iterate by entity blocks
for block in &mesh.node_blocks {
    println!("Entity {}-{}: {} nodes",
        block.entity_dim, block.entity_tag, block.nodes.len());
}
```

### Flat lookup maps
- O(1) access to nodes and elements by tag
- Convenient for direct queries

```rust
// Direct lookup
let node = mesh.get_node(42)?;
let element = mesh.get_element(100)?;
```

## Element Types

Supports all standard Gmsh element types (1-31, 92-93):

- **Linear**: Point, Line2, Triangle3, Quadrangle4, Tetrahedron4, Hexahedron8, Prism6, Pyramid5
- **Second order**: Line3, Triangle6, Quadrangle8/9, Tetrahedron10, Hexahedron20/27, Prism15/18, Pyramid13/14
- **Higher order**: Lines (Line4-6), Triangles (Triangle9-21), Tetrahedra (Tetrahedron20-56), Hexahedra (Hexahedron64-125)

## Examples

Run the included example:

```bash
cargo run --example parse_box
```

This will parse the `tests/data/box.msh` file and display:
- Format information
- Physical groups
- Entity counts
- Node and element statistics
- Element type distribution

## Testing

Run unit tests:

```bash
cargo test
```

Run integration tests with real MSH files:

```bash
cargo test --test integration_test
```

## Performance

- **Memory**: Nodes and elements stored in both blocks and lookup maps (some duplication)
- **Speed**: O(1) lookup by tag, O(n) iteration
- **Large files**: Tested with files up to 1GB

## Reference

Based on the official Gmsh MSH 4.1 file format specification.

Python reference implementation: [ahojukka5/gmshparser](https://github.com/ahojukka5/gmshparser)

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Roadmap

### Near-term
- [ ] `$Periodic` section parsing
- [ ] Post-processing data sections (`$NodeData`, `$ElementData`, `$ElementNodeData`)
- [ ] Binary MSH 4.1 support

### Long-term
- [ ] MSH file writing
- [ ] Mesh manipulation utilities
- [ ] Python bindings (PyO3)
