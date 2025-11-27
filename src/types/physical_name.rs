//! Physical group name representation

#[derive(Debug, Clone)]
pub struct PhysicalName {
    pub dimension: i32,
    pub tag: i32,
    pub name: String,
}

impl PhysicalName {
    pub fn new(dimension: i32, tag: i32, name: String) -> Self {
        Self {
            dimension,
            tag,
            name,
        }
    }
}
