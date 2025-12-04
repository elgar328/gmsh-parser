//! Interpolation scheme section types
//!
//! Defines interpolation schemes for post-processing views.

/// Element topology types for interpolation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ElementTopology {
    Points = 1,
    Lines = 2,
    Triangles = 3,
    Quadrangles = 4,
    Tetrahedra = 5,
    Pyramids = 6,
    Prisms = 7,
    Hexahedra = 8,
    Polygons = 9,
    Polyhedra = 10,
}

impl std::fmt::Display for ElementTopology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ElementTopology {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(ElementTopology::Points),
            2 => Some(ElementTopology::Lines),
            3 => Some(ElementTopology::Triangles),
            4 => Some(ElementTopology::Quadrangles),
            5 => Some(ElementTopology::Tetrahedra),
            6 => Some(ElementTopology::Pyramids),
            7 => Some(ElementTopology::Prisms),
            8 => Some(ElementTopology::Hexahedra),
            9 => Some(ElementTopology::Polygons),
            10 => Some(ElementTopology::Polyhedra),
            _ => None,
        }
    }
}

/// Interpolation matrix
#[derive(Debug, Clone)]
pub struct InterpolationMatrix {
    pub num_rows: usize,
    pub num_columns: usize,
    pub values: Vec<f64>,
}

/// Element topology interpolation
#[derive(Debug, Clone)]
pub struct ElementTopologyInterpolation {
    /// Element topology type
    pub element_topology: ElementTopology,
    /// Interpolation matrices for this topology
    pub matrices: Vec<InterpolationMatrix>,
}

/// Complete interpolation scheme
#[derive(Debug, Clone)]
pub struct InterpolationScheme {
    /// Name of the interpolation scheme
    pub name: String,
    /// Interpolations for different element topologies
    pub topologies: Vec<ElementTopologyInterpolation>,
}
