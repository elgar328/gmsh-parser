//! Interpolation scheme section types
//!
//! Defines interpolation schemes for post-processing views.

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
    /// Element topology ID:
    /// 1=points, 2=lines, 3=triangles, 4=quadrangles,
    /// 5=tetrahedra, 6=pyramids, 7=prisms, 8=hexahedra,
    /// 9=polygons, 10=polyhedra
    pub element_topology: i32,
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
