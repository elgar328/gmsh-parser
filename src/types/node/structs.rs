//! Node structures for different entity dimensions and parametric modes

// 0D: Point entities
#[derive(Debug, Clone)]
pub struct Node0D {
    pub tag: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// 1D: Curve entities (non-parametric)
#[derive(Debug, Clone)]
pub struct Node1D {
    pub tag: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// 1D: Curve entities (parametric)
#[derive(Debug, Clone)]
pub struct Node1DParametric {
    pub tag: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u: f64,
}

// 2D: Surface entities (non-parametric)
#[derive(Debug, Clone)]
pub struct Node2D {
    pub tag: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// 2D: Surface entities (parametric)
#[derive(Debug, Clone)]
pub struct Node2DParametric {
    pub tag: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u: f64,
    pub v: f64,
}

// 3D: Volume entities (non-parametric)
#[derive(Debug, Clone)]
pub struct Node3D {
    pub tag: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// 3D: Volume entities (parametric)
#[derive(Debug, Clone)]
pub struct Node3DParametric {
    pub tag: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u: f64,
    pub v: f64,
    pub w: f64,
}
