//! Parser for $InterpolationScheme section

use std::io::BufReader;
use std::io::Lines;

use crate::error::{ParseError, Result};
use crate::types::{Mesh, InterpolationScheme, ElementTopologyInterpolation, InterpolationMatrix};

use super::{read_line, expect_end_marker};

pub fn parse<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    // Read scheme name
    let name = read_line(lines)?;

    // Read number of element topologies
    let num_element_topologies: usize = read_line(lines)?
        .parse()
        .map_err(|_| ParseError::InvalidFormat("Invalid numElementTopologies".to_string()))?;

    let mut topologies = Vec::with_capacity(num_element_topologies);

    for _ in 0..num_element_topologies {
        // Read element topology ID
        let element_topology: i32 = read_line(lines)?
            .parse()
            .map_err(|_| ParseError::InvalidFormat("Invalid elementTopology".to_string()))?;

        // Read number of interpolation matrices
        let num_interpolation_matrices: usize = read_line(lines)?
            .parse()
            .map_err(|_| {
                ParseError::InvalidFormat("Invalid numInterpolationMatrices".to_string())
            })?;

        let mut matrices = Vec::with_capacity(num_interpolation_matrices);

        for _ in 0..num_interpolation_matrices {
            // Read matrix dimensions
            let dim_line = read_line(lines)?;
            let dim_parts: Vec<&str> = dim_line.split_whitespace().collect();
            if dim_parts.len() < 2 {
                return Err(ParseError::InvalidFormat(
                    "Invalid matrix dimensions".to_string(),
                ));
            }

            let num_rows: usize = dim_parts[0].parse().map_err(|_| {
                ParseError::InvalidFormat(format!("Invalid numRows: {}", dim_parts[0]))
            })?;
            let num_columns: usize = dim_parts[1].parse().map_err(|_| {
                ParseError::InvalidFormat(format!("Invalid numColumns: {}", dim_parts[1]))
            })?;

            // Read matrix values (row by row)
            let total_values = num_rows * num_columns;
            let mut values = Vec::with_capacity(total_values);

            // Values can be on the same line as dimensions or on subsequent lines
            // Try to read remaining values from the first line
            for i in 2..dim_parts.len() {
                if values.len() >= total_values {
                    break;
                }
                let value: f64 = dim_parts[i].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid matrix value: {}", dim_parts[i]))
                })?;
                values.push(value);
            }

            // Read remaining values
            while values.len() < total_values {
                let value_line = read_line(lines)?;
                for part in value_line.split_whitespace() {
                    if values.len() >= total_values {
                        break;
                    }
                    let value: f64 = part.parse().map_err(|_| {
                        ParseError::InvalidFormat(format!("Invalid matrix value: {}", part))
                    })?;
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

    mesh.interpolation_schemes.push(InterpolationScheme { name, topologies });
    expect_end_marker(lines, "InterpolationScheme")?;
    Ok(())
}
