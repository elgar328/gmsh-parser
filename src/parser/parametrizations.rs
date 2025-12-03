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
    let mut iter = header.iter();

    let num_curve_param = iter.parse_usize("numCurveParam")?;
    let num_surface_param = iter.parse_usize("numSurfaceParam")?;

    iter.expect_no_more()?;

    // Parse curve parametrizations
    for _ in 0..num_curve_param {
        // curveTag (on its own line)
        let curve_tag_line = reader.read_token_line()?;
        let mut iter = curve_tag_line.iter();
        let curve_tag = iter.parse_int("curveTag")?;
        iter.expect_no_more()?;

        // numNodes (on next line)
        let num_nodes_line = reader.read_token_line()?;
        let mut iter = num_nodes_line.iter();
        let num_nodes = iter.parse_usize("numNodes")?;
        iter.expect_no_more()?;

        let mut nodes = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            // nodeX nodeY nodeZ nodeU
            let node_line = reader.read_token_line()?;
            let mut iter = node_line.iter();

            nodes.push(CurveParametrizationNode {
                x: iter.parse_float("x")?,
                y: iter.parse_float("y")?,
                z: iter.parse_float("z")?,
                u: iter.parse_float("u")?,
            });
            iter.expect_no_more()?;
        }

        parametrizations
            .curves
            .push(CurveParametrization { curve_tag, nodes });
    }

    // Parse surface parametrizations
    for _ in 0..num_surface_param {
        // surfaceTag (on its own line)
        let surface_tag_line = reader.read_token_line()?;
        let mut iter = surface_tag_line.iter();
        let surface_tag = iter.parse_int("surfaceTag")?;
        iter.expect_no_more()?;

        // numNodes numTriangles (on next line)
        let counts_line = reader.read_token_line()?;
        let mut iter = counts_line.iter();
        let num_nodes = iter.parse_usize("numNodes")?;
        let num_triangles = iter.parse_usize("numTriangles")?;
        iter.expect_no_more()?;

        let mut nodes = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            // nodeX nodeY nodeZ nodeU nodeV curvMaxX curvMaxY curvMaxZ curvMinX curvMinY curvMinZ
            let node_line = reader.read_token_line()?;
            let mut iter = node_line.iter();

            nodes.push(SurfaceParametrizationNode {
                x: iter.parse_float("x")?,
                y: iter.parse_float("y")?,
                z: iter.parse_float("z")?,
                u: iter.parse_float("u")?,
                v: iter.parse_float("v")?,
                curv_max_x: iter.parse_float("curvMaxX")?,
                curv_max_y: iter.parse_float("curvMaxY")?,
                curv_max_z: iter.parse_float("curvMaxZ")?,
                curv_min_x: iter.parse_float("curvMinX")?,
                curv_min_y: iter.parse_float("curvMinY")?,
                curv_min_z: iter.parse_float("curvMinZ")?,
            });
            iter.expect_no_more()?;
        }

        let mut triangles = Vec::with_capacity(num_triangles);
        for _ in 0..num_triangles {
            // nodeIndex1 nodeIndex2 nodeIndex3
            let triangle_line = reader.read_token_line()?;
            let mut iter = triangle_line.iter();

            triangles.push(ParametrizationTriangle {
                node_index1: iter.parse_usize("nodeIndex1")?,
                node_index2: iter.parse_usize("nodeIndex2")?,
                node_index3: iter.parse_usize("nodeIndex3")?,
            });
            iter.expect_no_more()?;
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
