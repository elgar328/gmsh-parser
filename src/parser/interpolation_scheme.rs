//! Parser for $InterpolationScheme section

use crate::error::Result;
use crate::types::{ElementTopologyInterpolation, InterpolationMatrix, InterpolationScheme, Mesh};

use super::LineReader;

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    // Read scheme name (all tokens on the line combined)
    let token_line = reader.read_token_line()?;
    let name = token_line
        .iter()
        .map(|t| t.value.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    // Read number of element topologies
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();

    let num_element_topologies = iter.parse_usize("numElementTopologies")?;

    let mut topologies = Vec::with_capacity(num_element_topologies);

    for _ in 0..num_element_topologies {
        // Read element topology ID
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();

        let element_topology = iter.parse_element_topology("elementTopology")?;
        iter.expect_no_more()?;

        // Read number of interpolation matrices
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();

        let num_interpolation_matrices = iter.parse_usize("numInterpolationMatrices")?;
        iter.expect_no_more()?;

        let mut matrices = Vec::with_capacity(num_interpolation_matrices);

        for _ in 0..num_interpolation_matrices {
            // Read matrix dimensions and values (all on the same line)
            let token_line = reader.read_token_line()?;
            let mut iter = token_line.iter();

            let num_rows = iter.parse_usize("numRows")?;
            let num_columns = iter.parse_usize("numColumns")?;

            // Read matrix values (row by row)
            let total_values = num_rows * num_columns;
            let values = iter.parse_floats(total_values, "matrixValue")?;
            iter.expect_no_more()?;

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
