//! Parser for $InterpolationScheme section

use crate::error::Result;
use crate::types::{ElementTopologyInterpolation, InterpolationMatrix, InterpolationScheme, Mesh};

use super::LineReader;

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    // Read scheme name
    let token_line = reader.read_token_line()?;
    let name = token_line.content.trim().to_string();

    // Read number of element topologies
    let token_line = reader.read_token_line()?;
    let num_element_topologies = token_line.tokens[0].parse_usize("numElementTopologies")?;

    let mut topologies = Vec::with_capacity(num_element_topologies);

    for _ in 0..num_element_topologies {
        // Read element topology ID
        let token_line = reader.read_token_line()?;
        let element_topology = token_line.tokens[0].parse_element_topology("elementTopology")?;

        // Read number of interpolation matrices
        let token_line = reader.read_token_line()?;
        let num_interpolation_matrices =
            token_line.tokens[0].parse_usize("numInterpolationMatrices")?;

        let mut matrices = Vec::with_capacity(num_interpolation_matrices);

        for _ in 0..num_interpolation_matrices {
            // Read matrix dimensions
            let token_line = reader.read_token_line()?;
            token_line.expect_min_len(2)?;

            let num_rows = token_line.tokens[0].parse_usize("numRows")?;
            let num_columns = token_line.tokens[1].parse_usize("numColumns")?;

            // Read matrix values (row by row)
            let total_values = num_rows * num_columns;
            let mut values = Vec::with_capacity(total_values);

            // Values can be on the same line as dimensions or on subsequent lines
            // Try to read remaining values from the first line
            for k in 2..token_line.len() {
                if values.len() >= total_values {
                    break;
                }
                let value = token_line.tokens[k].parse_float("matrixValue")?;
                values.push(value);
            }

            // Read remaining values
            while values.len() < total_values {
                let token_line = reader.read_token_line()?;
                for token in &token_line.tokens {
                    if values.len() >= total_values {
                        break;
                    }
                    let value = token.parse_float("matrixValue")?;
                    values.push(value);
                }
            }

            matrices.push(InterpolationMatrix {
                num_rows,
                num_columns,
                values,
            });
        }

        topologies.push(ElementTopologyInterpolation {
            element_topology,
            matrices,
        });
    }

    mesh.interpolation_schemes
        .push(InterpolationScheme { name, topologies });

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("InterpolationScheme")?;

    Ok(())
}
