#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EntityDimension {
    Point = 0,
    Curve = 1,
    Surface = 2,
    Volume = 3,
}

impl EntityDimension {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(EntityDimension::Point),
            1 => Some(EntityDimension::Curve),
            2 => Some(EntityDimension::Surface),
            3 => Some(EntityDimension::Volume),
            _ => None,
        }
    }
}

impl std::fmt::Display for EntityDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityDimension::Point => write!(f, "Point"),
            EntityDimension::Curve => write!(f, "Curve"),
            EntityDimension::Surface => write!(f, "Surface"),
            EntityDimension::Volume => write!(f, "Volume"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PointEntity {
    pub tag: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub physical_tags: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct CurveEntity {
    pub tag: i32,
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
    pub physical_tags: Vec<i32>,
    pub bounding_points: Vec<i32>,  // Sign encodes orientation
}

#[derive(Debug, Clone)]
pub struct SurfaceEntity {
    pub tag: i32,
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
    pub physical_tags: Vec<i32>,
    pub bounding_curves: Vec<i32>,  // Sign encodes orientation
}

#[derive(Debug, Clone)]
pub struct VolumeEntity {
    pub tag: i32,
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
    pub physical_tags: Vec<i32>,
    pub bounding_surfaces: Vec<i32>,  // Sign encodes orientation
}

#[derive(Debug, Clone, Default)]
pub struct Entities {
    pub points: Vec<PointEntity>,
    pub curves: Vec<CurveEntity>,
    pub surfaces: Vec<SurfaceEntity>,
    pub volumes: Vec<VolumeEntity>,
}

impl Entities {
    pub fn new() -> Self {
        Self::default()
    }
}
