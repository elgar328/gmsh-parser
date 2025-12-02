use gmsh_parser::parse_msh_file;
use miette::{GraphicalReportHandler, GraphicalTheme};

// ============================================================
// Invalid MSH file list
// Add new invalid test files here - one test will be generated for each
// Format: (test_name, "filename.msh")
// ============================================================
macro_rules! generate_invalid_tests {
    ($(($test_name:ident, $file:expr)),* $(,)?) => {
        $(
            #[test]
            fn $test_name() {
                let file_path = format!("tests/data/invalid/{}", $file);

                match parse_msh_file(&file_path) {
                    Ok(_) => {
                        panic!("Expected {} to fail parsing, but it succeeded", $file);
                    }
                    Err(e) => {
                        // Use miette's fancy graphical report handler
                        let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode());
                        let mut output = String::new();

                        // Add file header
                        output.push_str(&format!("\n📄 {}\n", $file));

                        // Render the miette report
                        handler.render_report(&mut output, &e as &dyn miette::Diagnostic).unwrap();

                        // Print output (will be captured by test runner by default)
                        // Use --nocapture to see this output
                        println!("{}", output);
                    }
                }
            }
        )*
    };
}

// ============================================================
// Generate tests - Add new invalid files here
// ============================================================
generate_invalid_tests!(
    (test_invalid_box, "box.msh"),
    // Add more invalid test files here:
    // (test_invalid_bad_version, "bad_version.msh"),
    // (test_invalid_missing_end, "missing_end.msh"),
);
