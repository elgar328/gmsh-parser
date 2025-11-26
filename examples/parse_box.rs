use gmsh_parser::parse_msh_file;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mesh = parse_msh_file("tests/data/box.msh")?;

    println!("=== Gmsh MSH Parser Example ===\n");

    // Format information
    println!("Format Information:");
    println!("  Version: {}", mesh.format.version);
    println!("  File Type: {} (0=ASCII, 1=binary)", mesh.format.file_type);
    println!("  Data Size: {}", mesh.format.data_size);

    // Physical names
    println!("\nPhysical Groups:");
    for ((dim, tag), name) in &mesh.physical_names {
        println!("  [dim={}, tag={}]: \"{}\"", dim, tag, name);
    }

    // Entities
    println!("\nEntities:");
    println!("  Points: {}", mesh.entities.points.len());
    println!("  Curves: {}", mesh.entities.curves.len());
    println!("  Surfaces: {}", mesh.entities.surfaces.len());
    println!("  Volumes: {}", mesh.entities.volumes.len());

    // Nodes
    println!("\nNodes:");
    println!("  Total nodes: {}", mesh.num_nodes());
    println!("  Node blocks: {}", mesh.node_blocks.len());

    // Show first few nodes
    println!("\n  First 5 nodes:");
    for node in mesh.nodes_iter().take(5) {
        println!("    Node {}: ({:.3}, {:.3}, {:.3})",
            node.tag, node.x, node.y, node.z);
    }

    // Elements
    println!("\nElements:");
    println!("  Total elements: {}", mesh.num_elements());
    println!("  Element blocks: {}", mesh.element_blocks.len());

    // Count element types
    let mut type_counts: HashMap<_, usize> = HashMap::new();
    for block in &mesh.element_blocks {
        *type_counts.entry(block.element_type).or_insert(0) += block.elements.len();
    }

    println!("\n  Element type distribution:");
    for (elem_type, count) in &type_counts {
        println!("    {:?}: {}", elem_type, count);
    }

    // Entity blocks summary
    println!("\nEntity Blocks:");
    println!("  Node blocks by dimension:");
    let mut node_blocks_by_dim: HashMap<i32, usize> = HashMap::new();
    for block in &mesh.node_blocks {
        *node_blocks_by_dim.entry(block.entity_dim).or_insert(0) += 1;
    }
    for (dim, count) in &node_blocks_by_dim {
        println!("    Dimension {}: {} blocks", dim, count);
    }

    println!("\n  Element blocks by dimension:");
    let mut elem_blocks_by_dim: HashMap<i32, usize> = HashMap::new();
    for block in &mesh.element_blocks {
        *elem_blocks_by_dim.entry(block.entity_dim).or_insert(0) += 1;
    }
    for (dim, count) in &elem_blocks_by_dim {
        println!("    Dimension {}: {} blocks", dim, count);
    }

    // Test entity-based queries
    println!("\nEntity-based Query Example:");
    if let Some(volume) = mesh.entities.volumes.values().next() {
        let nodes = mesh.nodes_in_entity(3, volume.tag);
        let elements = mesh.elements_in_entity(3, volume.tag);
        println!("  Volume {} (tag={}):", volume.tag, volume.tag);
        println!("    Nodes: {}", nodes.len());
        println!("    Elements: {}", elements.len());
        if !volume.physical_tags.is_empty() {
            print!("    Physical tags:");
            for tag in &volume.physical_tags {
                if let Some(name) = mesh.physical_name(3, *tag) {
                    print!(" {} (\"{}\")", tag, name);
                } else {
                    print!(" {}", tag);
                }
            }
            println!();
        }
    }

    Ok(())
}
