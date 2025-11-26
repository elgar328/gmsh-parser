use gmsh_parser::parse_msh_file;

#[test]
fn test_parse_box_msh() {
    let result = parse_msh_file("tests/data/box.msh");
    assert!(result.is_ok(), "Failed to parse box.msh: {:?}", result.err());

    let mesh = result.unwrap();

    // Check format
    assert_eq!(mesh.format.version, 4.1);
    assert_eq!(mesh.format.file_type, 0); // ASCII

    // Check physical names
    assert_eq!(mesh.physical_names.len(), 3);
    assert_eq!(mesh.physical_name(2, 13), Some("top"));
    assert_eq!(mesh.physical_name(2, 14), Some("bottom"));
    assert_eq!(mesh.physical_name(3, 15), Some("volume1"));

    // Check entities
    assert_eq!(mesh.entities.points.len(), 8);
    assert_eq!(mesh.entities.curves.len(), 12);
    assert_eq!(mesh.entities.surfaces.len(), 6);
    assert_eq!(mesh.entities.volumes.len(), 1);

    // Check nodes
    println!("Total nodes: {}", mesh.num_nodes());
    assert!(mesh.num_nodes() > 0);

    // Check elements
    println!("Total elements: {}", mesh.num_elements());
    assert!(mesh.num_elements() > 0);

    // Verify we can access nodes by tag
    let node1 = mesh.get_node(1);
    assert!(node1.is_some());
    let node1 = node1.unwrap();
    assert_eq!(node1.x, -0.5);
    assert_eq!(node1.y, -0.5);
    assert_eq!(node1.z, 0.5);

    // Print summary
    println!("\n=== Mesh Summary ===");
    println!("Nodes: {}", mesh.num_nodes());
    println!("Elements: {}", mesh.num_elements());
    println!("Node blocks: {}", mesh.node_blocks.len());
    println!("Element blocks: {}", mesh.element_blocks.len());

    // Print element type distribution
    let mut type_counts = std::collections::HashMap::new();
    for block in &mesh.element_blocks {
        *type_counts.entry(block.element_type).or_insert(0) += block.elements.len();
    }
    println!("\nElement types:");
    for (elem_type, count) in type_counts {
        println!("  {:?}: {}", elem_type, count);
    }
}

#[test]
fn test_entity_based_queries() {
    let mesh = parse_msh_file("tests/data/box.msh").unwrap();

    // Test entity-based node queries
    for block in &mesh.node_blocks {
        let nodes = mesh.nodes_in_entity(block.entity_dim, block.entity_tag);
        assert_eq!(nodes.len(), block.nodes.len());
    }

    // Test entity-based element queries
    for block in &mesh.element_blocks {
        let elements = mesh.elements_in_entity(block.entity_dim, block.entity_tag);
        assert_eq!(elements.len(), block.elements.len());
    }
}

#[test]
fn test_iterators() {
    let mesh = parse_msh_file("tests/data/box.msh").unwrap();

    // Test node iterator
    let node_count = mesh.nodes_iter().count();
    assert_eq!(node_count, mesh.num_nodes());

    // Test element iterator
    let element_count = mesh.elements_iter().count();
    assert_eq!(element_count, mesh.num_elements());
}
