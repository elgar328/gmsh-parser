use gmsh_parser::parse_msh_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mesh = parse_msh_file("tests/data/box.msh")?;

    println!("=== Gmsh MSH Parser Example ===\n");

    // Format information
    println!("Format Information:");
    println!("  Version: {}", mesh.format.version);
    println!("  File Type: {} (0=ASCII, 1=binary)", mesh.format.file_type);
    println!("  Data Size: {}", mesh.format.data_size);

    // Physical names
    println!("\nPhysical Groups: {}", mesh.physical_names.len());
    for pn in &mesh.physical_names {
        println!("  [dim={}, tag={}]: \"{}\"", pn.dimension, pn.tag, pn.name);
    }

    // Entities
    if let Some(entities) = &mesh.entities {
        println!("\nEntities:");
        println!("  Points: {}", entities.points.len());
        println!("  Curves: {}", entities.curves.len());
        println!("  Surfaces: {}", entities.surfaces.len());
        println!("  Volumes: {}", entities.volumes.len());
    }

    // Nodes
    println!("\nNodes:");
    println!("  Node blocks: {}", mesh.node_blocks.len());

    // Count total nodes
    let total_nodes: usize = mesh.node_blocks.iter().map(|block| {
        use gmsh_parser::NodeBlock;
        match block {
            NodeBlock::Point { nodes, .. } => nodes.len(),
            NodeBlock::Curve { nodes, .. } => nodes.len(),
            NodeBlock::CurveParametric { nodes, .. } => nodes.len(),
            NodeBlock::Surface { nodes, .. } => nodes.len(),
            NodeBlock::SurfaceParametric { nodes, .. } => nodes.len(),
            NodeBlock::Volume { nodes, .. } => nodes.len(),
            NodeBlock::VolumeParametric { nodes, .. } => nodes.len(),
        }
    }).sum();
    println!("  Total nodes: {}", total_nodes);

    // Elements
    println!("\nElements:");
    println!("  Element blocks: {}", mesh.element_blocks.len());

    // Count total elements
    let total_elements: usize = mesh.element_blocks.iter().map(|block| {
        use gmsh_parser::ElementBlock;
        match block {
            ElementBlock::Line2 { elements, .. } => elements.len(),
            ElementBlock::Triangle3 { elements, .. } => elements.len(),
            ElementBlock::Quadrangle4 { elements, .. } => elements.len(),
            ElementBlock::Tetrahedron4 { elements, .. } => elements.len(),
            ElementBlock::Hexahedron8 { elements, .. } => elements.len(),
            _ => 0,  // Add more cases as needed
        }
    }).sum();
    println!("  Total elements: {}", total_elements);

    Ok(())
}
