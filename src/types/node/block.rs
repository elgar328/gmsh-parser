//! NodeBlock enum for different entity dimensions and parametric modes

use super::structs::*;

#[derive(Debug, Clone)]
pub enum NodeBlock {
    Point {
        entity_tag: i32,
        nodes: Vec<Node0D>,
    },
    Curve {
        entity_tag: i32,
        nodes: Vec<Node1D>,
    },
    CurveParametric {
        entity_tag: i32,
        nodes: Vec<Node1DParametric>,
    },
    Surface {
        entity_tag: i32,
        nodes: Vec<Node2D>,
    },
    SurfaceParametric {
        entity_tag: i32,
        nodes: Vec<Node2DParametric>,
    },
    Volume {
        entity_tag: i32,
        nodes: Vec<Node3D>,
    },
    VolumeParametric {
        entity_tag: i32,
        nodes: Vec<Node3DParametric>,
    },
}
