use gmsh_parser::parse_msh_file;
use gmsh_parser::types::{ElementBlock, NodeBlock};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Parse the mesh file
    let mesh = parse_msh_file("tests/data/valid/box.msh")?;
    println!(
        "Successfully parsed MSH file (Version {})
",
        mesh.format.version
    );

    // 2. Print a high-level summary using the built-in method
    mesh.print_summary();

    println!("\n=== Data Access Examples ===\n");

    // 3. Access Physical Groups (Names)
    if !mesh.physical_names.is_empty() {
        println!("Physical Groups:");
        for pn in &mesh.physical_names {
            println!(
                "  - \"{}\" (Dim: {}, Tag: {})",
                pn.name, pn.dimension, pn.tag
            );
        }
    }

    // 4. Access Geometric Entities
    // The 'entities' section defines the points, curves, surfaces, and volumes.
    if let Some(entities) = &mesh.entities {
        println!("\nGeometric Entities:");
        println!(
            "  Points: {}, Curves: {}, Surfaces: {}, Volumes: {}",
            entities.points.len(),
            entities.curves.len(),
            entities.surfaces.len(),
            entities.volumes.len()
        );

        // Example: Check the bounds of the first curve
        if let Some(curve) = entities.curves.first() {
            println!(
                "  First Curve (Tag {}): x[{:.2}, {:.2}], y[{:.2}, {:.2}], z[{:.2}, {:.2}]",
                curve.tag,
                curve.min_x,
                curve.max_x,
                curve.min_y,
                curve.max_y,
                curve.min_z,
                curve.max_z
            );
        }
    }

    // 5. Access Nodes
    // Directly access a specific node (e.g., the 3rd node in the 10th node block)
    println!("\nNode Access Example (Specific Node):");
    if let Some(block) = mesh.node_blocks.get(9) {
        // Get the 10th block (index 9)
        match block {
            NodeBlock::Point { nodes, .. } => {
                if let Some(node) = nodes.get(2) { // Get the 3rd node (index 2)
                    println!("  Accessed 3rd node in 10th Point block: Tag {}, Coord ({:.3}, {:.3}, {:.3})", node.tag, node.x, node.y, node.z);
                } else {
                    println!("  2nd Point block does not have a 3rd node.");
                }
            },
            NodeBlock::Curve { nodes, .. } => {
                if let Some(node) = nodes.get(2) {
                    println!("  Accessed 3rd node in 10th Curve block: Tag {}, Coord ({:.3}, {:.3}, {:.3})", node.tag, node.x, node.y, node.z);
                } else {
                    println!("  2nd Curve block does not have a 3rd node.");
                }
            },
            NodeBlock::Surface { nodes, .. } => {
                if let Some(node) = nodes.get(2) {
                    println!("  Accessed 3rd node in 10th Surface block: Tag {}, Coord ({:.3}, {:.3}, {:.3})", node.tag, node.x, node.y, node.z);
                } else {
                    println!("  2nd Surface block does not have a 3rd node.");
                }
            },
            NodeBlock::Volume { nodes, .. } => {
                if let Some(node) = nodes.get(2) {
                    println!("  Accessed 3rd node in 10th Volume block: Tag {}, Coord ({:.3}, {:.3}, {:.3})", node.tag, node.x, node.y, node.z);
                } else {
                    println!("  2nd Volume block does not have a 3rd node.");
                }
            },
            _ => println!("  2nd node block is of a parametric or other specialized type. Skipping detailed print."),
        }
    } else {
        println!("  Mesh does not contain a 10th node block.");
    }

    // 6. Access Elements
    // Elements are also grouped into blocks based on entity and element type.
    println!("\nElement Blocks:");
    for (i, block) in mesh.element_blocks.iter().enumerate() {
        let ElementBlock {
            entity_dim,
            entity_tag,
            element_type,
            elements,
        } = block;

        println!(
            "  Block {}: Entity (Dim {}, Tag {}) has {} elements of type {}",
            i,
            entity_dim,
            entity_tag,
            elements.len(),
            element_type
        );

        // Example: Print the first element's connectivity (node tags)
        if let Some(first_elem) = elements.first() {
            println!(
                "    First Element: Tag {}, Nodes {:?}",
                first_elem.tag, first_elem.nodes
            );
        }
    }

    Ok(())
}
