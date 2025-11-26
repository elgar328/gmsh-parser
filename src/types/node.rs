#[derive(Debug, Clone)]
pub struct Node {
    pub tag: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    // Optional parametric coordinates (if entity is parametric)
    pub u: Option<f64>,
    pub v: Option<f64>,
    pub w: Option<f64>,
}

impl Node {
    pub fn new(tag: usize, x: f64, y: f64, z: f64) -> Self {
        Self {
            tag,
            x,
            y,
            z,
            u: None,
            v: None,
            w: None,
        }
    }

    pub fn with_parametric(mut self, u: Option<f64>, v: Option<f64>, w: Option<f64>) -> Self {
        self.u = u;
        self.v = v;
        self.w = w;
        self
    }
}

#[derive(Debug, Clone)]
pub struct NodeBlock {
    pub entity_dim: i32,
    pub entity_tag: i32,
    pub parametric: bool,
    pub nodes: Vec<Node>,
}

impl NodeBlock {
    pub fn new(entity_dim: i32, entity_tag: i32, parametric: bool) -> Self {
        Self {
            entity_dim,
            entity_tag,
            parametric,
            nodes: Vec::new(),
        }
    }
}
