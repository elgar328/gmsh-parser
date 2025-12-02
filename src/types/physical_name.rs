//! Physical group name representation

use crate::types::EntityDimension;

#[derive(Debug, Clone)]
pub struct PhysicalName {
    pub dimension: EntityDimension,
    pub tag: i32,
    pub name: String,
}

impl PhysicalName {
    pub fn new(dimension: EntityDimension, tag: i32, name: String) -> Self {
        Self {
            dimension,
            tag,
            name,
        }
    }
}
