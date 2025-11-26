use crate::error::{ParseError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ElementType {
    // Linear elements
    Line2 = 1,
    Triangle3 = 2,
    Quadrangle4 = 3,
    Tetrahedron4 = 4,
    Hexahedron8 = 5,
    Prism6 = 6,
    Pyramid5 = 7,

    // Second order elements
    Line3 = 8,
    Triangle6 = 9,
    Quadrangle9 = 10,
    Tetrahedron10 = 11,
    Hexahedron27 = 12,
    Prism18 = 13,
    Pyramid14 = 14,

    // Point
    Point = 15,

    // More second order
    Quadrangle8 = 16,
    Hexahedron20 = 17,
    Prism15 = 18,
    Pyramid13 = 19,

    // Higher order triangles
    Triangle9 = 20,
    Triangle10 = 21,
    Triangle12 = 22,
    Triangle15a = 23,
    Triangle15b = 24,
    Triangle21 = 25,

    // Higher order lines
    Line4 = 26,
    Line5 = 27,
    Line6 = 28,

    // Higher order tetrahedra
    Tetrahedron20 = 29,
    Tetrahedron35 = 30,
    Tetrahedron56 = 31,

    // Higher order hexahedra
    Hexahedron64 = 92,
    Hexahedron125 = 93,
}

impl ElementType {
    pub fn from_i32(value: i32) -> Result<Self> {
        match value {
            1 => Ok(ElementType::Line2),
            2 => Ok(ElementType::Triangle3),
            3 => Ok(ElementType::Quadrangle4),
            4 => Ok(ElementType::Tetrahedron4),
            5 => Ok(ElementType::Hexahedron8),
            6 => Ok(ElementType::Prism6),
            7 => Ok(ElementType::Pyramid5),
            8 => Ok(ElementType::Line3),
            9 => Ok(ElementType::Triangle6),
            10 => Ok(ElementType::Quadrangle9),
            11 => Ok(ElementType::Tetrahedron10),
            12 => Ok(ElementType::Hexahedron27),
            13 => Ok(ElementType::Prism18),
            14 => Ok(ElementType::Pyramid14),
            15 => Ok(ElementType::Point),
            16 => Ok(ElementType::Quadrangle8),
            17 => Ok(ElementType::Hexahedron20),
            18 => Ok(ElementType::Prism15),
            19 => Ok(ElementType::Pyramid13),
            20 => Ok(ElementType::Triangle9),
            21 => Ok(ElementType::Triangle10),
            22 => Ok(ElementType::Triangle12),
            23 => Ok(ElementType::Triangle15a),
            24 => Ok(ElementType::Triangle15b),
            25 => Ok(ElementType::Triangle21),
            26 => Ok(ElementType::Line4),
            27 => Ok(ElementType::Line5),
            28 => Ok(ElementType::Line6),
            29 => Ok(ElementType::Tetrahedron20),
            30 => Ok(ElementType::Tetrahedron35),
            31 => Ok(ElementType::Tetrahedron56),
            92 => Ok(ElementType::Hexahedron64),
            93 => Ok(ElementType::Hexahedron125),
            _ => Err(ParseError::InvalidElementType(value)),
        }
    }

    pub fn num_nodes(&self) -> usize {
        match self {
            ElementType::Point => 1,
            ElementType::Line2 => 2,
            ElementType::Triangle3 => 3,
            ElementType::Quadrangle4 => 4,
            ElementType::Tetrahedron4 => 4,
            ElementType::Hexahedron8 => 8,
            ElementType::Prism6 => 6,
            ElementType::Pyramid5 => 5,
            ElementType::Line3 => 3,
            ElementType::Triangle6 => 6,
            ElementType::Quadrangle9 => 9,
            ElementType::Tetrahedron10 => 10,
            ElementType::Hexahedron27 => 27,
            ElementType::Prism18 => 18,
            ElementType::Pyramid14 => 14,
            ElementType::Quadrangle8 => 8,
            ElementType::Hexahedron20 => 20,
            ElementType::Prism15 => 15,
            ElementType::Pyramid13 => 13,
            ElementType::Triangle9 => 9,
            ElementType::Triangle10 => 10,
            ElementType::Triangle12 => 12,
            ElementType::Triangle15a => 15,
            ElementType::Triangle15b => 15,
            ElementType::Triangle21 => 21,
            ElementType::Line4 => 4,
            ElementType::Line5 => 5,
            ElementType::Line6 => 6,
            ElementType::Tetrahedron20 => 20,
            ElementType::Tetrahedron35 => 35,
            ElementType::Tetrahedron56 => 56,
            ElementType::Hexahedron64 => 64,
            ElementType::Hexahedron125 => 125,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    pub tag: usize,
    pub element_type: ElementType,
    pub node_tags: Vec<usize>,
}

impl Element {
    pub fn new(tag: usize, element_type: ElementType, node_tags: Vec<usize>) -> Self {
        Self {
            tag,
            element_type,
            node_tags,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElementBlock {
    pub entity_dim: i32,
    pub entity_tag: i32,
    pub element_type: ElementType,
    pub elements: Vec<Element>,
}

impl ElementBlock {
    pub fn new(entity_dim: i32, entity_tag: i32, element_type: ElementType) -> Self {
        Self {
            entity_dim,
            entity_tag,
            element_type,
            elements: Vec::new(),
        }
    }
}
