//! Parser for $Parametrizations section

use crate::error::Result;
use crate::types::{
    CurveParametrization, CurveParametrizationNode, Mesh, ParametrizationTriangle,
    Parametrizations, SurfaceParametrization, SurfaceParametrizationNode,
};

use super::LineReader;

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    let mut parametrizations = Parametrizations::default();

    // Read: numCurveParam numSurfaceParam
    let header = reader.read_token_line()?;
    header.expect_len(2)?;

    let num_curve_param = header.tokens[0].parse_usize("numCurveParam")?;
    let num_surface_param = header.tokens[1].parse_usize("numSurfaceParam")?;

    // Parse curve parametrizations
    for _ in 0..num_curve_param {
        // curveTag (on its own line)
        let curve_tag_line = reader.read_token_line()?;
        let curve_tag = curve_tag_line.tokens[0].parse_int("curveTag")?;

        // numNodes (on next line)
        let num_nodes_line = reader.read_token_line()?;
        let num_nodes = num_nodes_line.tokens[0].parse_usize("numNodes")?;

        let mut nodes = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            // nodeX nodeY nodeZ nodeU
            let node_line = reader.read_token_line()?;
            node_line.expect_len(4)?;

            nodes.push(CurveParametrizationNode {
                x: node_line.tokens[0].parse_float("x")?,
                y: node_line.tokens[1].parse_float("y")?,
                z: node_line.tokens[2].parse_float("z")?,
                u: node_line.tokens[3].parse_float("u")?,
            });
        }

        parametrizations
            .curves
            .push(CurveParametrization { curve_tag, nodes });
    }

    // Parse surface parametrizations
    for _ in 0..num_surface_param {
        // surfaceTag (on its own line)
        let surface_tag_line = reader.read_token_line()?;
        let surface_tag = surface_tag_line.tokens[0].parse_int("surfaceTag")?;

        // numNodes numTriangles (on next line)
        let counts_line = reader.read_token_line()?;
        counts_line.expect_len(2)?;
        let num_nodes = counts_line.tokens[0].parse_usize("numNodes")?;
        let num_triangles = counts_line.tokens[1].parse_usize("numTriangles")?;

        let mut nodes = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            // nodeX nodeY nodeZ nodeU nodeV curvMaxX curvMaxY curvMaxZ curvMinX curvMinY curvMinZ
            let node_line = reader.read_token_line()?;
            node_line.expect_len(11)?;

            nodes.push(SurfaceParametrizationNode {
                x: node_line.tokens[0].parse_float("x")?,
                y: node_line.tokens[1].parse_float("y")?,
                z: node_line.tokens[2].parse_float("z")?,
                u: node_line.tokens[3].parse_float("u")?,
                v: node_line.tokens[4].parse_float("v")?,
                curv_max_x: node_line.tokens[5].parse_float("curvMaxX")?,
                curv_max_y: node_line.tokens[6].parse_float("curvMaxY")?,
                curv_max_z: node_line.tokens[7].parse_float("curvMaxZ")?,
                curv_min_x: node_line.tokens[8].parse_float("curvMinX")?,
                curv_min_y: node_line.tokens[9].parse_float("curvMinY")?,
                curv_min_z: node_line.tokens[10].parse_float("curvMinZ")?,
            });
        }

        let mut triangles = Vec::with_capacity(num_triangles);
        for _ in 0..num_triangles {
            // nodeIndex1 nodeIndex2 nodeIndex3
            let triangle_line = reader.read_token_line()?;
            triangle_line.expect_len(3)?;

            triangles.push(ParametrizationTriangle {
                node_index1: triangle_line.tokens[0].parse_usize("nodeIndex1")?,
                node_index2: triangle_line.tokens[1].parse_usize("nodeIndex2")?,
                node_index3: triangle_line.tokens[2].parse_usize("nodeIndex3")?,
            });
        }

        parametrizations.surfaces.push(SurfaceParametrization {
            surface_tag,
            nodes,
            triangles,
        });
    }

    mesh.parametrizations = Some(parametrizations);

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("Parametrizations")?;

    Ok(())
}
