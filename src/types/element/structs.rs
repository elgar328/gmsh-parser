//! Element structures for all 140 Gmsh element types
//!
//! Fixed-size elements use arrays `[usize; N]` for compile-time safety.
//! Variable-size elements use `Vec<usize>` for flexibility.

// ============================================================================
// MACROS
// ============================================================================

macro_rules! define_fixed_elements {
    (
        $(
            $name:ident($count:expr)
        ),* $(,)?
    ) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                pub tag: usize,
                pub nodes: [usize; $count],
            }
        )*
    };
}

macro_rules! define_variable_elements {
    ($($name:ident),* $(,)?) => {
        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                pub tag: usize,
                pub nodes: Vec<usize>,
            }
        )*
    };
}

macro_rules! define_point_element {
    ($name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub tag: usize,
            pub node: usize,
        }
    };
}

// ============================================================================
// FIXED SIZE ELEMENTS (128 types)
// ============================================================================

define_fixed_elements!(
    // Basic elements (1-15)
    Line2Element(2),
    Triangle3Element(3),
    Quadrangle4Element(4),
    Tetrahedron4Element(4),
    Hexahedron8Element(8),
    Prism6Element(6),
    Pyramid5Element(5),
    Line3Element(3),
    Triangle6Element(6),
    Quadrangle9Element(9),
    Tetrahedron10Element(10),
    Hexahedron27Element(27),
    Prism18Element(18),
    Pyramid14Element(14),

    // Second order elements (16-33)
    Quadrangle8Element(8),
    Hexahedron20Element(20),
    Prism15Element(15),
    Pyramid13Element(13),
    Triangle9Element(9),
    Triangle10Element(10),
    Triangle12Element(12),
    Triangle15Element(15),
    Triangle15IElement(15),
    Triangle21Element(21),
    Line4Element(4),
    Line5Element(5),
    Line6Element(6),
    Tetrahedron20Element(20),
    Tetrahedron35Element(35),
    Tetrahedron56Element(56),
    Tetrahedron22Element(22),
    Tetrahedron28Element(28),

    // Higher order elements (36-66)
    Quadrangle16Element(16),
    Quadrangle25Element(25),
    Quadrangle36Element(36),
    Quadrangle12Element(12),
    Quadrangle16IElement(16),
    Quadrangle20Element(20),
    Triangle28Element(28),
    Triangle36Element(36),
    Triangle45Element(45),
    Triangle55Element(55),
    Triangle66Element(66),
    Quadrangle49Element(49),
    Quadrangle64Element(64),
    Quadrangle81Element(81),
    Quadrangle100Element(100),
    Quadrangle121Element(121),
    Triangle18Element(18),
    Triangle21IElement(21),
    Triangle24Element(24),
    Triangle27Element(27),
    Triangle30Element(30),
    Quadrangle24Element(24),
    Quadrangle28Element(28),
    Quadrangle32Element(32),
    Quadrangle36IElement(36),
    Quadrangle40Element(40),
    Line7Element(7),
    Line8Element(8),
    Line9Element(9),
    Line10Element(10),
    Line11Element(11),

    // Higher order tetrahedra (71-75, 79-83)
    Tetrahedron84Element(84),
    Tetrahedron120Element(120),
    Tetrahedron165Element(165),
    Tetrahedron220Element(220),
    Tetrahedron286Element(286),
    Tetrahedron34Element(34),
    Tetrahedron40Element(40),
    Tetrahedron46Element(46),
    Tetrahedron52Element(52),
    Tetrahedron58Element(58),

    // 1-node elements (84-89)
    Line1Element(1),
    Triangle1Element(1),
    Quadrangle1Element(1),
    Tetrahedron1Element(1),
    Hexahedron1Element(1),
    Prism1Element(1),

    // Higher order prisms (90-91, 106-117)
    Prism40Element(40),
    Prism75Element(75),
    Prism126Element(126),
    Prism196Element(196),
    Prism288Element(288),
    Prism405Element(405),
    Prism550Element(550),
    Prism24Element(24),
    Prism33Element(33),
    Prism42Element(42),
    Prism51Element(51),
    Prism60Element(60),
    Prism69Element(69),
    Prism78Element(78),

    // Higher order hexahedra (92-105)
    Hexahedron64Element(64),
    Hexahedron125Element(125),
    Hexahedron216Element(216),
    Hexahedron343Element(343),
    Hexahedron512Element(512),
    Hexahedron729Element(729),
    Hexahedron1000Element(1000),
    Hexahedron32Element(32),
    Hexahedron44Element(44),
    Hexahedron56Element(56),
    Hexahedron68Element(68),
    Hexahedron80Element(80),
    Hexahedron92Element(92),
    Hexahedron104Element(104),

    // Higher order pyramids (118-132)
    Pyramid30Element(30),
    Pyramid55Element(55),
    Pyramid91Element(91),
    Pyramid140Element(140),
    Pyramid204Element(204),
    Pyramid285Element(285),
    Pyramid385Element(385),
    Pyramid21Element(21),
    Pyramid29Element(29),
    Pyramid37Element(37),
    Pyramid45Element(45),
    Pyramid53Element(53),
    Pyramid61Element(61),
    Pyramid69Element(69),
    Pyramid1Element(1),

    // Miscellaneous (137, 140)
    Tetrahedron16Element(16),
    TriHedron4Element(4),
);

// Point element (special case - no nodes array)
define_point_element!(PointElement);

// ============================================================================
// VARIABLE SIZE ELEMENTS (12 types)
// ============================================================================

define_variable_elements!(
    PolygonElement,
    PolyhedronElement,
    LineBElement,
    TriangleBElement,
    PolygonBElement,
    LineCElement,
    PointSubElement,
    LineSubElement,
    TriangleSubElement,
    TetrahedronSubElement,
    TriangleMiniElement,
    TetrahedronMiniElement,
);
