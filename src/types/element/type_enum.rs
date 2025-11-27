//! Element type enum for parsing and identification
//!
//! This enum is used during parsing to identify element types by their ID.
//! The actual element data is stored in individual typed structs and ElementBlock variants.

use crate::error::{ParseError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementType {
    // Basic elements (1-15)
    Line2,              // ID 1, 2 nodes
    Triangle3,          // ID 2, 3 nodes
    Quadrangle4,        // ID 3, 4 nodes
    Tetrahedron4,       // ID 4, 4 nodes
    Hexahedron8,        // ID 5, 8 nodes
    Prism6,             // ID 6, 6 nodes
    Pyramid5,           // ID 7, 5 nodes
    Line3,              // ID 8, 3 nodes
    Triangle6,          // ID 9, 6 nodes
    Quadrangle9,        // ID 10, 9 nodes
    Tetrahedron10,      // ID 11, 10 nodes
    Hexahedron27,       // ID 12, 27 nodes
    Prism18,            // ID 13, 18 nodes
    Pyramid14,          // ID 14, 14 nodes
    Point,              // ID 15, 1 node

    // Second order elements (16-33)
    Quadrangle8,        // ID 16, 8 nodes
    Hexahedron20,       // ID 17, 20 nodes
    Prism15,            // ID 18, 15 nodes
    Pyramid13,          // ID 19, 13 nodes
    Triangle9,          // ID 20, 9 nodes
    Triangle10,         // ID 21, 10 nodes
    Triangle12,         // ID 22, 12 nodes
    Triangle15,         // ID 23, 15 nodes (complete)
    Triangle15I,        // ID 24, 15 nodes (incomplete)
    Triangle21,         // ID 25, 21 nodes
    Line4,              // ID 26, 4 nodes
    Line5,              // ID 27, 5 nodes
    Line6,              // ID 28, 6 nodes
    Tetrahedron20,      // ID 29, 20 nodes
    Tetrahedron35,      // ID 30, 35 nodes
    Tetrahedron56,      // ID 31, 56 nodes
    Tetrahedron22,      // ID 32, 22 nodes
    Tetrahedron28,      // ID 33, 28 nodes

    // Variable size elements (34-35)
    Polygon,            // ID 34, variable nodes
    Polyhedron,         // ID 35, variable nodes

    // High order quadrangles (36-41)
    Quadrangle16,       // ID 36, 16 nodes
    Quadrangle25,       // ID 37, 25 nodes
    Quadrangle36,       // ID 38, 36 nodes
    Quadrangle12,       // ID 39, 12 nodes
    Quadrangle16I,      // ID 40, 16 nodes (incomplete)
    Quadrangle20,       // ID 41, 20 nodes

    // High order triangles (42-46)
    Triangle28,         // ID 42, 28 nodes
    Triangle36,         // ID 43, 36 nodes
    Triangle45,         // ID 44, 45 nodes
    Triangle55,         // ID 45, 55 nodes
    Triangle66,         // ID 46, 66 nodes

    // Very high order quadrangles (47-51)
    Quadrangle49,       // ID 47, 49 nodes
    Quadrangle64,       // ID 48, 64 nodes
    Quadrangle81,       // ID 49, 81 nodes
    Quadrangle100,      // ID 50, 100 nodes
    Quadrangle121,      // ID 51, 121 nodes

    // More high order triangles (52-56)
    Triangle18,         // ID 52, 18 nodes
    Triangle21I,        // ID 53, 21 nodes (incomplete)
    Triangle24,         // ID 54, 24 nodes
    Triangle27,         // ID 55, 27 nodes
    Triangle30,         // ID 56, 30 nodes

    // More high order quadrangles (57-61)
    Quadrangle24,       // ID 57, 24 nodes
    Quadrangle28,       // ID 58, 28 nodes
    Quadrangle32,       // ID 59, 32 nodes
    Quadrangle36I,      // ID 60, 36 nodes (incomplete)
    Quadrangle40,       // ID 61, 40 nodes

    // High order lines (62-66)
    Line7,              // ID 62, 7 nodes
    Line8,              // ID 63, 8 nodes
    Line9,              // ID 64, 9 nodes
    Line10,             // ID 65, 10 nodes
    Line11,             // ID 66, 11 nodes

    // Bezier/Bubble and Composite (67-70)
    LineB,              // ID 67, variable nodes (uncertain)
    TriangleB,          // ID 68, variable nodes (uncertain)
    PolygonB,           // ID 69, variable nodes
    LineC,              // ID 70, variable nodes (uncertain)

    // Very high order tetrahedra (71-75)
    Tetrahedron84,      // ID 71, 84 nodes
    Tetrahedron120,     // ID 72, 120 nodes
    Tetrahedron165,     // ID 73, 165 nodes
    Tetrahedron220,     // ID 74, 220 nodes
    Tetrahedron286,     // ID 75, 286 nodes

    // Incomplete tetrahedra (79-83)
    Tetrahedron34,      // ID 79, 34 nodes
    Tetrahedron40,      // ID 80, 40 nodes
    Tetrahedron46,      // ID 81, 46 nodes
    Tetrahedron52,      // ID 82, 52 nodes
    Tetrahedron58,      // ID 83, 58 nodes

    // Single node elements (84-89)
    Line1,              // ID 84, 1 node
    Triangle1,          // ID 85, 1 node
    Quadrangle1,        // ID 86, 1 node
    Tetrahedron1,       // ID 87, 1 node
    Hexahedron1,        // ID 88, 1 node
    Prism1,             // ID 89, 1 node

    // High order prisms (90-91)
    Prism40,            // ID 90, 40 nodes
    Prism75,            // ID 91, 75 nodes

    // Very high order hexahedra (92-98)
    Hexahedron64,       // ID 92, 64 nodes
    Hexahedron125,      // ID 93, 125 nodes
    Hexahedron216,      // ID 94, 216 nodes
    Hexahedron343,      // ID 95, 343 nodes
    Hexahedron512,      // ID 96, 512 nodes
    Hexahedron729,      // ID 97, 729 nodes
    Hexahedron1000,     // ID 98, 1000 nodes

    // Incomplete hexahedra (99-105)
    Hexahedron32,       // ID 99, 32 nodes
    Hexahedron44,       // ID 100, 44 nodes
    Hexahedron56,       // ID 101, 56 nodes
    Hexahedron68,       // ID 102, 68 nodes
    Hexahedron80,       // ID 103, 80 nodes
    Hexahedron92,       // ID 104, 92 nodes
    Hexahedron104,      // ID 105, 104 nodes

    // Very high order prisms (106-110)
    Prism126,           // ID 106, 126 nodes
    Prism196,           // ID 107, 196 nodes
    Prism288,           // ID 108, 288 nodes
    Prism405,           // ID 109, 405 nodes
    Prism550,           // ID 110, 550 nodes

    // Incomplete prisms (111-117)
    Prism24,            // ID 111, 24 nodes
    Prism33,            // ID 112, 33 nodes
    Prism42,            // ID 113, 42 nodes
    Prism51,            // ID 114, 51 nodes
    Prism60,            // ID 115, 60 nodes
    Prism69,            // ID 116, 69 nodes
    Prism78,            // ID 117, 78 nodes

    // Very high order pyramids (118-124)
    Pyramid30,          // ID 118, 30 nodes
    Pyramid55,          // ID 119, 55 nodes
    Pyramid91,          // ID 120, 91 nodes
    Pyramid140,         // ID 121, 140 nodes
    Pyramid204,         // ID 122, 204 nodes
    Pyramid285,         // ID 123, 285 nodes
    Pyramid385,         // ID 124, 385 nodes

    // Incomplete pyramids (125-131)
    Pyramid21,          // ID 125, 21 nodes
    Pyramid29,          // ID 126, 29 nodes
    Pyramid37,          // ID 127, 37 nodes
    Pyramid45,          // ID 128, 45 nodes
    Pyramid53,          // ID 129, 53 nodes
    Pyramid61,          // ID 130, 61 nodes
    Pyramid69,          // ID 131, 69 nodes

    // Single node pyramid (132)
    Pyramid1,           // ID 132, 1 node

    // Sub-elements (133-136)
    PointSub,           // ID 133, variable nodes (uncertain)
    LineSub,            // ID 134, variable nodes (uncertain)
    TriangleSub,        // ID 135, variable nodes (uncertain)
    TetrahedronSub,     // ID 136, variable nodes (uncertain)

    // Remaining elements (137-140)
    Tetrahedron16,      // ID 137, 16 nodes
    TriangleMini,       // ID 138, variable nodes (uncertain)
    TetrahedronMini,    // ID 139, variable nodes (uncertain)
    TriHedron4,         // ID 140, 4 nodes
}

impl ElementType {
    /// Convert from Gmsh element type ID to ElementType enum
    pub fn from_i32(id: i32) -> Result<Self> {
        match id {
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
            23 => Ok(ElementType::Triangle15),
            24 => Ok(ElementType::Triangle15I),
            25 => Ok(ElementType::Triangle21),
            26 => Ok(ElementType::Line4),
            27 => Ok(ElementType::Line5),
            28 => Ok(ElementType::Line6),
            29 => Ok(ElementType::Tetrahedron20),
            30 => Ok(ElementType::Tetrahedron35),
            31 => Ok(ElementType::Tetrahedron56),
            32 => Ok(ElementType::Tetrahedron22),
            33 => Ok(ElementType::Tetrahedron28),
            34 => Ok(ElementType::Polygon),
            35 => Ok(ElementType::Polyhedron),
            36 => Ok(ElementType::Quadrangle16),
            37 => Ok(ElementType::Quadrangle25),
            38 => Ok(ElementType::Quadrangle36),
            39 => Ok(ElementType::Quadrangle12),
            40 => Ok(ElementType::Quadrangle16I),
            41 => Ok(ElementType::Quadrangle20),
            42 => Ok(ElementType::Triangle28),
            43 => Ok(ElementType::Triangle36),
            44 => Ok(ElementType::Triangle45),
            45 => Ok(ElementType::Triangle55),
            46 => Ok(ElementType::Triangle66),
            47 => Ok(ElementType::Quadrangle49),
            48 => Ok(ElementType::Quadrangle64),
            49 => Ok(ElementType::Quadrangle81),
            50 => Ok(ElementType::Quadrangle100),
            51 => Ok(ElementType::Quadrangle121),
            52 => Ok(ElementType::Triangle18),
            53 => Ok(ElementType::Triangle21I),
            54 => Ok(ElementType::Triangle24),
            55 => Ok(ElementType::Triangle27),
            56 => Ok(ElementType::Triangle30),
            57 => Ok(ElementType::Quadrangle24),
            58 => Ok(ElementType::Quadrangle28),
            59 => Ok(ElementType::Quadrangle32),
            60 => Ok(ElementType::Quadrangle36I),
            61 => Ok(ElementType::Quadrangle40),
            62 => Ok(ElementType::Line7),
            63 => Ok(ElementType::Line8),
            64 => Ok(ElementType::Line9),
            65 => Ok(ElementType::Line10),
            66 => Ok(ElementType::Line11),
            67 => Ok(ElementType::LineB),
            68 => Ok(ElementType::TriangleB),
            69 => Ok(ElementType::PolygonB),
            70 => Ok(ElementType::LineC),
            71 => Ok(ElementType::Tetrahedron84),
            72 => Ok(ElementType::Tetrahedron120),
            73 => Ok(ElementType::Tetrahedron165),
            74 => Ok(ElementType::Tetrahedron220),
            75 => Ok(ElementType::Tetrahedron286),
            79 => Ok(ElementType::Tetrahedron34),
            80 => Ok(ElementType::Tetrahedron40),
            81 => Ok(ElementType::Tetrahedron46),
            82 => Ok(ElementType::Tetrahedron52),
            83 => Ok(ElementType::Tetrahedron58),
            84 => Ok(ElementType::Line1),
            85 => Ok(ElementType::Triangle1),
            86 => Ok(ElementType::Quadrangle1),
            87 => Ok(ElementType::Tetrahedron1),
            88 => Ok(ElementType::Hexahedron1),
            89 => Ok(ElementType::Prism1),
            90 => Ok(ElementType::Prism40),
            91 => Ok(ElementType::Prism75),
            92 => Ok(ElementType::Hexahedron64),
            93 => Ok(ElementType::Hexahedron125),
            94 => Ok(ElementType::Hexahedron216),
            95 => Ok(ElementType::Hexahedron343),
            96 => Ok(ElementType::Hexahedron512),
            97 => Ok(ElementType::Hexahedron729),
            98 => Ok(ElementType::Hexahedron1000),
            99 => Ok(ElementType::Hexahedron32),
            100 => Ok(ElementType::Hexahedron44),
            101 => Ok(ElementType::Hexahedron56),
            102 => Ok(ElementType::Hexahedron68),
            103 => Ok(ElementType::Hexahedron80),
            104 => Ok(ElementType::Hexahedron92),
            105 => Ok(ElementType::Hexahedron104),
            106 => Ok(ElementType::Prism126),
            107 => Ok(ElementType::Prism196),
            108 => Ok(ElementType::Prism288),
            109 => Ok(ElementType::Prism405),
            110 => Ok(ElementType::Prism550),
            111 => Ok(ElementType::Prism24),
            112 => Ok(ElementType::Prism33),
            113 => Ok(ElementType::Prism42),
            114 => Ok(ElementType::Prism51),
            115 => Ok(ElementType::Prism60),
            116 => Ok(ElementType::Prism69),
            117 => Ok(ElementType::Prism78),
            118 => Ok(ElementType::Pyramid30),
            119 => Ok(ElementType::Pyramid55),
            120 => Ok(ElementType::Pyramid91),
            121 => Ok(ElementType::Pyramid140),
            122 => Ok(ElementType::Pyramid204),
            123 => Ok(ElementType::Pyramid285),
            124 => Ok(ElementType::Pyramid385),
            125 => Ok(ElementType::Pyramid21),
            126 => Ok(ElementType::Pyramid29),
            127 => Ok(ElementType::Pyramid37),
            128 => Ok(ElementType::Pyramid45),
            129 => Ok(ElementType::Pyramid53),
            130 => Ok(ElementType::Pyramid61),
            131 => Ok(ElementType::Pyramid69),
            132 => Ok(ElementType::Pyramid1),
            133 => Ok(ElementType::PointSub),
            134 => Ok(ElementType::LineSub),
            135 => Ok(ElementType::TriangleSub),
            136 => Ok(ElementType::TetrahedronSub),
            137 => Ok(ElementType::Tetrahedron16),
            138 => Ok(ElementType::TriangleMini),
            139 => Ok(ElementType::TetrahedronMini),
            140 => Ok(ElementType::TriHedron4),
            _ => Err(ParseError::InvalidData(
                "Elements".to_string(),
                format!("Unknown element type ID: {}", id),
            )),
        }
    }

    /// Get the fixed node count for this element type, or None if variable
    pub fn fixed_node_count(&self) -> Option<usize> {
        match self {
            ElementType::Line2 => Some(2),
            ElementType::Triangle3 => Some(3),
            ElementType::Quadrangle4 => Some(4),
            ElementType::Tetrahedron4 => Some(4),
            ElementType::Hexahedron8 => Some(8),
            ElementType::Prism6 => Some(6),
            ElementType::Pyramid5 => Some(5),
            ElementType::Line3 => Some(3),
            ElementType::Triangle6 => Some(6),
            ElementType::Quadrangle9 => Some(9),
            ElementType::Tetrahedron10 => Some(10),
            ElementType::Hexahedron27 => Some(27),
            ElementType::Prism18 => Some(18),
            ElementType::Pyramid14 => Some(14),
            ElementType::Point => Some(1),
            ElementType::Quadrangle8 => Some(8),
            ElementType::Hexahedron20 => Some(20),
            ElementType::Prism15 => Some(15),
            ElementType::Pyramid13 => Some(13),
            ElementType::Triangle9 => Some(9),
            ElementType::Triangle10 => Some(10),
            ElementType::Triangle12 => Some(12),
            ElementType::Triangle15 => Some(15),
            ElementType::Triangle15I => Some(15),
            ElementType::Triangle21 => Some(21),
            ElementType::Line4 => Some(4),
            ElementType::Line5 => Some(5),
            ElementType::Line6 => Some(6),
            ElementType::Tetrahedron20 => Some(20),
            ElementType::Tetrahedron35 => Some(35),
            ElementType::Tetrahedron56 => Some(56),
            ElementType::Tetrahedron22 => Some(22),
            ElementType::Tetrahedron28 => Some(28),
            ElementType::Polygon => None,
            ElementType::Polyhedron => None,
            ElementType::Quadrangle16 => Some(16),
            ElementType::Quadrangle25 => Some(25),
            ElementType::Quadrangle36 => Some(36),
            ElementType::Quadrangle12 => Some(12),
            ElementType::Quadrangle16I => Some(16),
            ElementType::Quadrangle20 => Some(20),
            ElementType::Triangle28 => Some(28),
            ElementType::Triangle36 => Some(36),
            ElementType::Triangle45 => Some(45),
            ElementType::Triangle55 => Some(55),
            ElementType::Triangle66 => Some(66),
            ElementType::Quadrangle49 => Some(49),
            ElementType::Quadrangle64 => Some(64),
            ElementType::Quadrangle81 => Some(81),
            ElementType::Quadrangle100 => Some(100),
            ElementType::Quadrangle121 => Some(121),
            ElementType::Triangle18 => Some(18),
            ElementType::Triangle21I => Some(21),
            ElementType::Triangle24 => Some(24),
            ElementType::Triangle27 => Some(27),
            ElementType::Triangle30 => Some(30),
            ElementType::Quadrangle24 => Some(24),
            ElementType::Quadrangle28 => Some(28),
            ElementType::Quadrangle32 => Some(32),
            ElementType::Quadrangle36I => Some(36),
            ElementType::Quadrangle40 => Some(40),
            ElementType::Line7 => Some(7),
            ElementType::Line8 => Some(8),
            ElementType::Line9 => Some(9),
            ElementType::Line10 => Some(10),
            ElementType::Line11 => Some(11),
            ElementType::LineB => None,
            ElementType::TriangleB => None,
            ElementType::PolygonB => None,
            ElementType::LineC => None,
            ElementType::Tetrahedron84 => Some(84),
            ElementType::Tetrahedron120 => Some(120),
            ElementType::Tetrahedron165 => Some(165),
            ElementType::Tetrahedron220 => Some(220),
            ElementType::Tetrahedron286 => Some(286),
            ElementType::Tetrahedron34 => Some(34),
            ElementType::Tetrahedron40 => Some(40),
            ElementType::Tetrahedron46 => Some(46),
            ElementType::Tetrahedron52 => Some(52),
            ElementType::Tetrahedron58 => Some(58),
            ElementType::Line1 => Some(1),
            ElementType::Triangle1 => Some(1),
            ElementType::Quadrangle1 => Some(1),
            ElementType::Tetrahedron1 => Some(1),
            ElementType::Hexahedron1 => Some(1),
            ElementType::Prism1 => Some(1),
            ElementType::Prism40 => Some(40),
            ElementType::Prism75 => Some(75),
            ElementType::Hexahedron64 => Some(64),
            ElementType::Hexahedron125 => Some(125),
            ElementType::Hexahedron216 => Some(216),
            ElementType::Hexahedron343 => Some(343),
            ElementType::Hexahedron512 => Some(512),
            ElementType::Hexahedron729 => Some(729),
            ElementType::Hexahedron1000 => Some(1000),
            ElementType::Hexahedron32 => Some(32),
            ElementType::Hexahedron44 => Some(44),
            ElementType::Hexahedron56 => Some(56),
            ElementType::Hexahedron68 => Some(68),
            ElementType::Hexahedron80 => Some(80),
            ElementType::Hexahedron92 => Some(92),
            ElementType::Hexahedron104 => Some(104),
            ElementType::Prism126 => Some(126),
            ElementType::Prism196 => Some(196),
            ElementType::Prism288 => Some(288),
            ElementType::Prism405 => Some(405),
            ElementType::Prism550 => Some(550),
            ElementType::Prism24 => Some(24),
            ElementType::Prism33 => Some(33),
            ElementType::Prism42 => Some(42),
            ElementType::Prism51 => Some(51),
            ElementType::Prism60 => Some(60),
            ElementType::Prism69 => Some(69),
            ElementType::Prism78 => Some(78),
            ElementType::Pyramid30 => Some(30),
            ElementType::Pyramid55 => Some(55),
            ElementType::Pyramid91 => Some(91),
            ElementType::Pyramid140 => Some(140),
            ElementType::Pyramid204 => Some(204),
            ElementType::Pyramid285 => Some(285),
            ElementType::Pyramid385 => Some(385),
            ElementType::Pyramid21 => Some(21),
            ElementType::Pyramid29 => Some(29),
            ElementType::Pyramid37 => Some(37),
            ElementType::Pyramid45 => Some(45),
            ElementType::Pyramid53 => Some(53),
            ElementType::Pyramid61 => Some(61),
            ElementType::Pyramid69 => Some(69),
            ElementType::Pyramid1 => Some(1),
            ElementType::PointSub => None,
            ElementType::LineSub => None,
            ElementType::TriangleSub => None,
            ElementType::TetrahedronSub => None,
            ElementType::Tetrahedron16 => Some(16),
            ElementType::TriangleMini => None,
            ElementType::TetrahedronMini => None,
            ElementType::TriHedron4 => Some(4),
        }
    }
}
