# gmsh-parser

A simple Rust parser for Gmsh MSH 4.1 ASCII format files.

## ⚠️ Project Status

This is a **personal project** created for specific use cases. It is **not production-ready** and has not been extensively tested. Use at your own risk.

## What This Library Does

- **Parses MSH 4.1 ASCII format only** - No binary support, no legacy formats (1.0, 2.x, 4.0)
- **Preserves file structure** - Maintains the block-based organization of MSH files
- **Read-only parser** - Does not generate or write MSH files
- **Comprehensive section support** - Parses all standard MSH 4.1 sections including post-processing data
- **Clear error messages** - Uses [miette](https://github.com/zkat/miette) for detailed, user-friendly parsing errors with source code snippets

## Supported Sections

All standard MSH 4.1 sections are supported:

- `$MeshFormat`, `$PhysicalNames`, `$Entities`, `$PartitionedEntities`
- `$Nodes`, `$Elements`, `$Periodic`, `$GhostElements`
- `$Parametrizations`, `$NodeData`, `$ElementData`, `$ElementNodeData`
- `$InterpolationScheme`

## Installation

```toml
[dependencies]
gmsh-parser = { git = "https://github.com/elgar328/gmsh-parser" }
```

## Quick Example

```rust
use gmsh_parser::parse_msh_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mesh = parse_msh_file("mesh.msh")?;

    // Print summary
    mesh.print_summary();

    // Access nodes
    for block in &mesh.node_blocks {
        println!("Entity {}: {} nodes", block.entity_tag(), block.num_nodes());
        for node in &block.nodes {
            println!("  Node {}: ({}, {}, {})", node.tag, node.x, node.y, node.z);
        }
    }

    // Access elements
    for block in &mesh.element_blocks {
        println!("Block: {} elements", block.elements.len());
        for elem in &block.elements {
            println!("  Element {}: {:?}", elem.tag, elem.nodes);
        }
    }

    Ok(())
}
```

## Error Reporting

Parse errors provide detailed context using miette:

```
📄 box.msh
  × Invalid data
    ╭─[41:10]
 40 │ $Nodes
 41 │ 27 335 1 336
    ·          ─┬─
    ·           ╰── Maximum node tag mismatch: header declares 336, but actual maximum is 335
 42 │ 0 1 0 1
    ╰────
```

## Running Examples

```bash
cargo run --example parse_box
```

## License

MIT
