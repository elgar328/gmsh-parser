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

impl NodeBlock {
    /// Returns the number of nodes in this block
    pub fn num_nodes(&self) -> usize {
        match self {
            Self::Point { nodes, .. } => nodes.len(),
            Self::Curve { nodes, .. } => nodes.len(),
            Self::CurveParametric { nodes, .. } => nodes.len(),
            Self::Surface { nodes, .. } => nodes.len(),
            Self::SurfaceParametric { nodes, .. } => nodes.len(),
            Self::Volume { nodes, .. } => nodes.len(),
            Self::VolumeParametric { nodes, .. } => nodes.len(),
        }
    }

    /// Iterates over the tag of each node in this block
    pub fn for_each_tag<F>(&self, mut f: F)
    where
        F: FnMut(usize),
    {
        match self {
            Self::Point { nodes, .. } => nodes.iter().for_each(|n| f(n.tag)),
            Self::Curve { nodes, .. } => nodes.iter().for_each(|n| f(n.tag)),
            Self::CurveParametric { nodes, .. } => nodes.iter().for_each(|n| f(n.tag)),
            Self::Surface { nodes, .. } => nodes.iter().for_each(|n| f(n.tag)),
            Self::SurfaceParametric { nodes, .. } => nodes.iter().for_each(|n| f(n.tag)),
            Self::Volume { nodes, .. } => nodes.iter().for_each(|n| f(n.tag)),
            Self::VolumeParametric { nodes, .. } => nodes.iter().for_each(|n| f(n.tag)),
        }
    }
}
