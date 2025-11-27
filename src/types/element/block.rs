//! ElementBlock enum with all 140 variants
//!
//! Each variant contains entity information and a vector of elements of that specific type.

use super::structs::*;

// ============================================================================
// MACRO
// ============================================================================

macro_rules! define_element_blocks {
    (
        $(
            $variant:ident => $element_type:ty
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone)]
        pub enum ElementBlock {
            $(
                $variant {
                    entity_dim: i32,
                    entity_tag: i32,
                    elements: Vec<$element_type>,
                },
            )*
        }
    };
}

// ============================================================================
// ELEMENT BLOCK ENUM
// ============================================================================

define_element_blocks!(
    // Basic elements (1-15)
    Line2 => Line2Element,
    Triangle3 => Triangle3Element,
    Quadrangle4 => Quadrangle4Element,
    Tetrahedron4 => Tetrahedron4Element,
    Hexahedron8 => Hexahedron8Element,
    Prism6 => Prism6Element,
    Pyramid5 => Pyramid5Element,
    Line3 => Line3Element,
    Triangle6 => Triangle6Element,
    Quadrangle9 => Quadrangle9Element,
    Tetrahedron10 => Tetrahedron10Element,
    Hexahedron27 => Hexahedron27Element,
    Prism18 => Prism18Element,
    Pyramid14 => Pyramid14Element,
    Point => PointElement,

    // Second order (16-33)
    Quadrangle8 => Quadrangle8Element,
    Hexahedron20 => Hexahedron20Element,
    Prism15 => Prism15Element,
    Pyramid13 => Pyramid13Element,
    Triangle9 => Triangle9Element,
    Triangle10 => Triangle10Element,
    Triangle12 => Triangle12Element,
    Triangle15 => Triangle15Element,
    Triangle15I => Triangle15IElement,
    Triangle21 => Triangle21Element,
    Line4 => Line4Element,
    Line5 => Line5Element,
    Line6 => Line6Element,
    Tetrahedron20 => Tetrahedron20Element,
    Tetrahedron35 => Tetrahedron35Element,
    Tetrahedron56 => Tetrahedron56Element,
    Tetrahedron22 => Tetrahedron22Element,
    Tetrahedron28 => Tetrahedron28Element,

    // Variable size (34-35)
    Polygon => PolygonElement,
    Polyhedron => PolyhedronElement,

    // Higher order (36-66)
    Quadrangle16 => Quadrangle16Element,
    Quadrangle25 => Quadrangle25Element,
    Quadrangle36 => Quadrangle36Element,
    Quadrangle12 => Quadrangle12Element,
    Quadrangle16I => Quadrangle16IElement,
    Quadrangle20 => Quadrangle20Element,
    Triangle28 => Triangle28Element,
    Triangle36 => Triangle36Element,
    Triangle45 => Triangle45Element,
    Triangle55 => Triangle55Element,
    Triangle66 => Triangle66Element,
    Quadrangle49 => Quadrangle49Element,
    Quadrangle64 => Quadrangle64Element,
    Quadrangle81 => Quadrangle81Element,
    Quadrangle100 => Quadrangle100Element,
    Quadrangle121 => Quadrangle121Element,
    Triangle18 => Triangle18Element,
    Triangle21I => Triangle21IElement,
    Triangle24 => Triangle24Element,
    Triangle27 => Triangle27Element,
    Triangle30 => Triangle30Element,
    Quadrangle24 => Quadrangle24Element,
    Quadrangle28 => Quadrangle28Element,
    Quadrangle32 => Quadrangle32Element,
    Quadrangle36I => Quadrangle36IElement,
    Quadrangle40 => Quadrangle40Element,
    Line7 => Line7Element,
    Line8 => Line8Element,
    Line9 => Line9Element,
    Line10 => Line10Element,
    Line11 => Line11Element,

    // Variable size (67-70)
    LineB => LineBElement,
    TriangleB => TriangleBElement,
    PolygonB => PolygonBElement,
    LineC => LineCElement,

    // Higher order tetrahedra (71-75)
    Tetrahedron84 => Tetrahedron84Element,
    Tetrahedron120 => Tetrahedron120Element,
    Tetrahedron165 => Tetrahedron165Element,
    Tetrahedron220 => Tetrahedron220Element,
    Tetrahedron286 => Tetrahedron286Element,

    // Incomplete tetrahedra (79-83)
    Tetrahedron34 => Tetrahedron34Element,
    Tetrahedron40 => Tetrahedron40Element,
    Tetrahedron46 => Tetrahedron46Element,
    Tetrahedron52 => Tetrahedron52Element,
    Tetrahedron58 => Tetrahedron58Element,

    // 1-node elements (84-89)
    Line1 => Line1Element,
    Triangle1 => Triangle1Element,
    Quadrangle1 => Quadrangle1Element,
    Tetrahedron1 => Tetrahedron1Element,
    Hexahedron1 => Hexahedron1Element,
    Prism1 => Prism1Element,

    // Prisms (90-91)
    Prism40 => Prism40Element,
    Prism75 => Prism75Element,

    // Higher order hexahedra (92-105)
    Hexahedron64 => Hexahedron64Element,
    Hexahedron125 => Hexahedron125Element,
    Hexahedron216 => Hexahedron216Element,
    Hexahedron343 => Hexahedron343Element,
    Hexahedron512 => Hexahedron512Element,
    Hexahedron729 => Hexahedron729Element,
    Hexahedron1000 => Hexahedron1000Element,
    Hexahedron32 => Hexahedron32Element,
    Hexahedron44 => Hexahedron44Element,
    Hexahedron56 => Hexahedron56Element,
    Hexahedron68 => Hexahedron68Element,
    Hexahedron80 => Hexahedron80Element,
    Hexahedron92 => Hexahedron92Element,
    Hexahedron104 => Hexahedron104Element,

    // Higher order prisms (106-117)
    Prism126 => Prism126Element,
    Prism196 => Prism196Element,
    Prism288 => Prism288Element,
    Prism405 => Prism405Element,
    Prism550 => Prism550Element,
    Prism24 => Prism24Element,
    Prism33 => Prism33Element,
    Prism42 => Prism42Element,
    Prism51 => Prism51Element,
    Prism60 => Prism60Element,
    Prism69 => Prism69Element,
    Prism78 => Prism78Element,

    // Higher order pyramids (118-132)
    Pyramid30 => Pyramid30Element,
    Pyramid55 => Pyramid55Element,
    Pyramid91 => Pyramid91Element,
    Pyramid140 => Pyramid140Element,
    Pyramid204 => Pyramid204Element,
    Pyramid285 => Pyramid285Element,
    Pyramid385 => Pyramid385Element,
    Pyramid21 => Pyramid21Element,
    Pyramid29 => Pyramid29Element,
    Pyramid37 => Pyramid37Element,
    Pyramid45 => Pyramid45Element,
    Pyramid53 => Pyramid53Element,
    Pyramid61 => Pyramid61Element,
    Pyramid69 => Pyramid69Element,
    Pyramid1 => Pyramid1Element,

    // Variable size (133-136)
    PointSub => PointSubElement,
    LineSub => LineSubElement,
    TriangleSub => TriangleSubElement,
    TetrahedronSub => TetrahedronSubElement,

    // Miscellaneous (137-140)
    Tetrahedron16 => Tetrahedron16Element,
    TriangleMini => TriangleMiniElement,
    TetrahedronMini => TetrahedronMiniElement,
    TriHedron4 => TriHedron4Element,
);
