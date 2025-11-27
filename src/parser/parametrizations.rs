//! Parser for $Parametrizations section

use std::io::BufReader;
use std::io::Lines;

use crate::error::{ParseError, Result};
use crate::types::{Mesh, Parametrizations, CurveParametrization, SurfaceParametrization, CurveParametrizationNode, SurfaceParametrizationNode, ParametrizationTriangle};

use super::{read_line, expect_end_marker};

pub fn parse<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    let mut parametrizations = Parametrizations::default();

    // Read: numCurveParam numSurfaceParam
    let header = read_line(lines)?;
    let counts: Vec<usize> = header
        .split_whitespace()
        .map(|s| {
            s.parse()
                .map_err(|_| ParseError::InvalidFormat(format!("Invalid count: {}", s)))
        })
        .collect::<Result<Vec<_>>>()?;

    if counts.len() < 2 {
        return Err(ParseError::InvalidFormat(
            "Invalid parametrizations header".to_string(),
        ));
    }

    let (num_curve_param, num_surface_param) = (counts[0], counts[1]);

    // Parse curve parametrizations
    for _ in 0..num_curve_param {
        // curveTag numNodes
        let curve_header = read_line(lines)?;
        let curve_parts: Vec<&str> = curve_header.split_whitespace().collect();
        if curve_parts.len() < 2 {
            return Err(ParseError::InvalidFormat(format!(
                "Invalid curve parametrization header: {}",
                curve_header
            )));
        }

        let curve_tag: i32 = curve_parts[0].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid curve tag: {}", curve_parts[0]))
        })?;
        let num_nodes: usize = curve_parts[1].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid numNodes: {}", curve_parts[1]))
        })?;

        let mut nodes = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            // nodeX nodeY nodeZ nodeU
            let node_line = read_line(lines)?;
            let node_parts: Vec<&str> = node_line.split_whitespace().collect();
            if node_parts.len() < 4 {
                return Err(ParseError::InvalidFormat(format!(
                    "Invalid curve node line: {}",
                    node_line
                )));
            }

            nodes.push(CurveParametrizationNode {
                x: node_parts[0].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid x: {}", node_parts[0]))
                })?,
                y: node_parts[1].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid y: {}", node_parts[1]))
                })?,
                z: node_parts[2].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid z: {}", node_parts[2]))
                })?,
                u: node_parts[3].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid u: {}", node_parts[3]))
                })?,
            });
        }

        parametrizations.curves.push(CurveParametrization { curve_tag, nodes });
    }

    // Parse surface parametrizations
    for _ in 0..num_surface_param {
        // surfaceTag numNodes numTriangles
        let surface_header = read_line(lines)?;
        let surface_parts: Vec<&str> = surface_header.split_whitespace().collect();
        if surface_parts.len() < 3 {
            return Err(ParseError::InvalidFormat(format!(
                "Invalid surface parametrization header: {}",
                surface_header
            )));
        }

        let surface_tag: i32 = surface_parts[0].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid surface tag: {}", surface_parts[0]))
        })?;
        let num_nodes: usize = surface_parts[1].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid numNodes: {}", surface_parts[1]))
        })?;
        let num_triangles: usize = surface_parts[2].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid numTriangles: {}", surface_parts[2]))
        })?;

        let mut nodes = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            // nodeX nodeY nodeZ nodeU nodeV curvMaxX curvMaxY curvMaxZ curvMinX curvMinY curvMinZ
            let node_line = read_line(lines)?;
            let node_parts: Vec<&str> = node_line.split_whitespace().collect();
            if node_parts.len() < 11 {
                return Err(ParseError::InvalidFormat(format!(
                    "Invalid surface node line: {}",
                    node_line
                )));
            }

            nodes.push(SurfaceParametrizationNode {
                x: node_parts[0].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid x: {}", node_parts[0]))
                })?,
                y: node_parts[1].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid y: {}", node_parts[1]))
                })?,
                z: node_parts[2].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid z: {}", node_parts[2]))
                })?,
                u: node_parts[3].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid u: {}", node_parts[3]))
                })?,
                v: node_parts[4].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid v: {}", node_parts[4]))
                })?,
                curv_max_x: node_parts[5].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid curvMaxX: {}", node_parts[5]))
                })?,
                curv_max_y: node_parts[6].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid curvMaxY: {}", node_parts[6]))
                })?,
                curv_max_z: node_parts[7].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid curvMaxZ: {}", node_parts[7]))
                })?,
                curv_min_x: node_parts[8].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid curvMinX: {}", node_parts[8]))
                })?,
                curv_min_y: node_parts[9].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid curvMinY: {}", node_parts[9]))
                })?,
                curv_min_z: node_parts[10].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid curvMinZ: {}", node_parts[10]))
                })?,
            });
        }

        let mut triangles = Vec::with_capacity(num_triangles);
        for _ in 0..num_triangles {
            // nodeIndex1 nodeIndex2 nodeIndex3
            let triangle_line = read_line(lines)?;
            let triangle_parts: Vec<&str> = triangle_line.split_whitespace().collect();
            if triangle_parts.len() < 3 {
                return Err(ParseError::InvalidFormat(format!(
                    "Invalid triangle line: {}",
                    triangle_line
                )));
            }

            triangles.push(ParametrizationTriangle {
                node_index1: triangle_parts[0].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid index1: {}", triangle_parts[0]))
                })?,
                node_index2: triangle_parts[1].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid index2: {}", triangle_parts[1]))
                })?,
                node_index3: triangle_parts[2].parse().map_err(|_| {
                    ParseError::InvalidFormat(format!("Invalid index3: {}", triangle_parts[2]))
                })?,
            });
        }

        parametrizations.surfaces.push(SurfaceParametrization {
            surface_tag,
            nodes,
            triangles,
        });
    }

    mesh.parametrizations = Some(parametrizations);
    expect_end_marker(lines, "Parametrizations")?;
    Ok(())
}
