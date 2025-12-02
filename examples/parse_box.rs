use gmsh_parser::parse_msh_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mesh = parse_msh_file("tests/data/custom/box.msh")?;

    // Use the new print_summary method
    mesh.print_summary();

    Ok(())
}
