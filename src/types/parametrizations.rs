//! Parametrizations section types
//!
//! Defines parametrizations for curves and surfaces.

/// Node parametrization for curves
#[derive(Debug, Clone)]
pub struct CurveParametrizationNode {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u: f64,
}

/// Curve parametrization
#[derive(Debug, Clone)]
pub struct CurveParametrization {
    pub curve_tag: i32,
    pub nodes: Vec<CurveParametrizationNode>,
}

/// Node parametrization for surfaces
#[derive(Debug, Clone)]
pub struct SurfaceParametrizationNode {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u: f64,
    pub v: f64,
    pub curv_max_x: f64,
    pub curv_max_y: f64,
    pub curv_max_z: f64,
    pub curv_min_x: f64,
    pub curv_min_y: f64,
    pub curv_min_z: f64,
}

/// Triangle for surface parametrization
#[derive(Debug, Clone)]
pub struct ParametrizationTriangle {
    pub node_index1: usize,
    pub node_index2: usize,
    pub node_index3: usize,
}

/// Surface parametrization
#[derive(Debug, Clone)]
pub struct SurfaceParametrization {
    pub surface_tag: i32,
    pub nodes: Vec<SurfaceParametrizationNode>,
    pub triangles: Vec<ParametrizationTriangle>,
}

/// Complete parametrizations information
#[derive(Debug, Clone, Default)]
pub struct Parametrizations {
    pub curves: Vec<CurveParametrization>,
    pub surfaces: Vec<SurfaceParametrization>,
}
