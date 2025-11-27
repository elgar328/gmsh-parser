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
    assert!(mesh.physical_names.iter().any(|pn| pn.dimension == 2 && pn.tag == 13 && pn.name == "top"));
    assert!(mesh.physical_names.iter().any(|pn| pn.dimension == 2 && pn.tag == 14 && pn.name == "bottom"));
    assert!(mesh.physical_names.iter().any(|pn| pn.dimension == 3 && pn.tag == 15 && pn.name == "volume1"));

    // Check entities
    let entities = mesh.entities.as_ref().unwrap();
    assert_eq!(entities.points.len(), 8);
    assert_eq!(entities.curves.len(), 12);
    assert_eq!(entities.surfaces.len(), 6);
    assert_eq!(entities.volumes.len(), 1);

    // Check node blocks
    assert!(mesh.node_blocks.len() > 0);

    // Check element blocks
    assert!(mesh.element_blocks.len() > 0);

    // Print summary
    println!("\n=== Mesh Summary ===");
    println!("Node blocks: {}", mesh.node_blocks.len());
    println!("Element blocks: {}", mesh.element_blocks.len());
}

#[test]
fn test_basic_structure() {
    let mesh = parse_msh_file("tests/data/box.msh").unwrap();

    // Verify basic structure exists
    assert!(mesh.entities.is_some());
    assert!(!mesh.physical_names.is_empty());
    assert!(!mesh.node_blocks.is_empty());
    assert!(!mesh.element_blocks.is_empty());
}
