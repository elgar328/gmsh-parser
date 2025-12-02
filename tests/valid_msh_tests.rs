use gmsh_parser::parse_msh_file;
use std::fs;
use std::path::Path;

/// Test all valid MSH files in tests/data/valid/
/// These files should successfully parse
#[test]
fn test_all_valid_msh_files() {
    let valid_dir = Path::new("tests/data/valid");

    if !valid_dir.exists() {
        panic!("Valid test directory not found: {:?}", valid_dir);
    }

    let entries = fs::read_dir(valid_dir)
        .expect("Failed to read valid directory")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "msh")
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    if entries.is_empty() {
        panic!("No .msh files found in {:?}", valid_dir);
    }

    println!("\nTesting {} valid MSH files:", entries.len());

    for entry in entries {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();

        print!("  Testing {}... ", file_name);

        match parse_msh_file(&path) {
            Ok(mesh) => {
                println!("✓ (nodes: {}, elements: {})",
                    mesh.node_blocks.len(),
                    mesh.element_blocks.len()
                );
            }
            Err(e) => {
                println!("✗");
                panic!("Failed to parse valid file {:?}:\n{:?}", path, e);
            }
        }
    }
}
