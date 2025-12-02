//! Partitioned entities section types
//!
//! Defines partitioned entities for parallel mesh processing.

use crate::types::EntityDimension;

/// Ghost entity information
#[derive(Debug, Clone)]
pub struct GhostEntity {
    pub tag: i32,
    pub partition: i32,
}

/// Partitioned point entity
#[derive(Debug, Clone)]
pub struct PartitionedPoint {
    pub tag: i32,
    pub parent_dim: EntityDimension,
    pub parent_tag: i32,
    pub partition_tags: Vec<i32>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub physical_tags: Vec<i32>,
}

/// Partitioned curve entity
#[derive(Debug, Clone)]
pub struct PartitionedCurve {
    pub tag: i32,
    pub parent_dim: EntityDimension,
    pub parent_tag: i32,
    pub partition_tags: Vec<i32>,
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
    pub physical_tags: Vec<i32>,
    pub bounding_points: Vec<i32>,
}

/// Partitioned surface entity
#[derive(Debug, Clone)]
pub struct PartitionedSurface {
    pub tag: i32,
    pub parent_dim: EntityDimension,
    pub parent_tag: i32,
    pub partition_tags: Vec<i32>,
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
    pub physical_tags: Vec<i32>,
    pub bounding_curves: Vec<i32>,
}

/// Partitioned volume entity
#[derive(Debug, Clone)]
pub struct PartitionedVolume {
    pub tag: i32,
    pub parent_dim: EntityDimension,
    pub parent_tag: i32,
    pub partition_tags: Vec<i32>,
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
    pub physical_tags: Vec<i32>,
    pub bounding_surfaces: Vec<i32>,
}

/// Complete partitioned entities information
#[derive(Debug, Clone, Default)]
pub struct PartitionedEntities {
    pub num_partitions: usize,
    pub ghost_entities: Vec<GhostEntity>,
    pub points: Vec<PartitionedPoint>,
    pub curves: Vec<PartitionedCurve>,
    pub surfaces: Vec<PartitionedSurface>,
    pub volumes: Vec<PartitionedVolume>,
}
