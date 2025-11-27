use std::io::{BufReader, Lines};

use crate::error::{ParseError, Result};
use crate::types::{Mesh, ElementBlock, ElementType};
use crate::types::element::*;
use super::{read_line, expect_end_marker};

pub fn parse<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    let line = read_line(lines)?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 4 {
        return Err(ParseError::InvalidData(
            "Elements".to_string(),
            format!("Expected 4 values in header, got {}", parts.len()),
        ));
    }

    let num_entity_blocks: usize = parts[0].parse()?;
    let _num_elements: usize = parts[1].parse()?;
    let _min_element_tag: usize = parts[2].parse()?;
    let _max_element_tag: usize = parts[3].parse()?;

    // Parse each entity block
    for _ in 0..num_entity_blocks {
        let block = parse_element_block(lines)?;
        mesh.element_blocks.push(block);
    }

    expect_end_marker(lines, "Elements")?;

    Ok(())
}

fn parse_element_block<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
) -> Result<ElementBlock> {
    let line = read_line(lines)?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 4 {
        return Err(ParseError::InvalidData(
            "Elements/Block".to_string(),
            format!("Expected 4 values in block header, got {}", parts.len()),
        ));
    }

    let entity_dim: i32 = parts[0].parse()?;
    let entity_tag: i32 = parts[1].parse()?;
    let element_type_id: i32 = parts[2].parse()?;
    let num_elements_in_block: usize = parts[3].parse()?;

    let element_type = ElementType::from_i32(element_type_id)?;

    // Parse elements based on type
    match element_type {
        ElementType::Line2 => parse_fixed_element_block::<2, _, R>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Line2Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Line2 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle3 => parse_fixed_element_block::<3, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle3Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle3 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle4 => parse_fixed_element_block::<4, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle4Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle4 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron4 => parse_fixed_element_block::<4, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron4Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron4 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron8 => parse_fixed_element_block::<8, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron8Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron8 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism6 => parse_fixed_element_block::<6, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism6Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism6 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid5 => parse_fixed_element_block::<5, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid5Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid5 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Line3 => parse_fixed_element_block::<3, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Line3Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Line3 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle6 => parse_fixed_element_block::<6, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle6Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle6 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle9 => parse_fixed_element_block::<9, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle9Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle9 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron10 => parse_fixed_element_block::<10, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron10Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron10 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron27 => parse_fixed_element_block::<27, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron27Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron27 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism18 => parse_fixed_element_block::<18, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism18Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism18 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid14 => parse_fixed_element_block::<14, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid14Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid14 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Point => parse_point_element_block(
            lines, num_elements_in_block, entity_dim, entity_tag
        ),
        ElementType::Quadrangle8 => parse_fixed_element_block::<8, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle8Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle8 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron20 => parse_fixed_element_block::<20, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron20Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron20 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism15 => parse_fixed_element_block::<15, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism15Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism15 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid13 => parse_fixed_element_block::<13, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid13Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid13 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle9 => parse_fixed_element_block::<9, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle9Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle9 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle10 => parse_fixed_element_block::<10, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle10Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle10 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle12 => parse_fixed_element_block::<12, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle12Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle12 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle15 => parse_fixed_element_block::<15, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle15Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle15 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle15I => parse_fixed_element_block::<15, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle15IElement { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle15I { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle21 => parse_fixed_element_block::<21, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle21Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle21 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Line4 => parse_fixed_element_block::<4, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Line4Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Line4 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Line5 => parse_fixed_element_block::<5, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Line5Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Line5 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Line6 => parse_fixed_element_block::<6, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Line6Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Line6 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron20 => parse_fixed_element_block::<20, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron20Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron20 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron35 => parse_fixed_element_block::<35, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron35Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron35 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron56 => parse_fixed_element_block::<56, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron56Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron56 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron22 => parse_fixed_element_block::<22, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron22Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron22 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron28 => parse_fixed_element_block::<28, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron28Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron28 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Polygon => parse_variable_element_block(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| PolygonElement { tag, nodes },
            |dim, tag, elements| ElementBlock::Polygon { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Polyhedron => parse_variable_element_block(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| PolyhedronElement { tag, nodes },
            |dim, tag, elements| ElementBlock::Polyhedron { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle16 => parse_fixed_element_block::<16, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle16Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle16 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle25 => parse_fixed_element_block::<25, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle25Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle25 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle36 => parse_fixed_element_block::<36, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle36Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle36 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle12 => parse_fixed_element_block::<12, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle12Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle12 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle16I => parse_fixed_element_block::<16, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle16IElement { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle16I { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle20 => parse_fixed_element_block::<20, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle20Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle20 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle28 => parse_fixed_element_block::<28, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle28Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle28 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle36 => parse_fixed_element_block::<36, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle36Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle36 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle45 => parse_fixed_element_block::<45, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle45Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle45 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle55 => parse_fixed_element_block::<55, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle55Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle55 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle66 => parse_fixed_element_block::<66, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle66Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle66 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle49 => parse_fixed_element_block::<49, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle49Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle49 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle64 => parse_fixed_element_block::<64, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle64Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle64 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle81 => parse_fixed_element_block::<81, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle81Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle81 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle100 => parse_fixed_element_block::<100, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle100Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle100 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle121 => parse_fixed_element_block::<121, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle121Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle121 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle18 => parse_fixed_element_block::<18, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle18Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle18 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle21I => parse_fixed_element_block::<21, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle21IElement { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle21I { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle24 => parse_fixed_element_block::<24, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle24Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle24 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle27 => parse_fixed_element_block::<27, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle27Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle27 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle30 => parse_fixed_element_block::<30, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle30Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle30 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle24 => parse_fixed_element_block::<24, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle24Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle24 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle28 => parse_fixed_element_block::<28, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle28Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle28 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle32 => parse_fixed_element_block::<32, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle32Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle32 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle36I => parse_fixed_element_block::<36, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle36IElement { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle36I { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle40 => parse_fixed_element_block::<40, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle40Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle40 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Line7 => parse_fixed_element_block::<7, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Line7Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Line7 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Line8 => parse_fixed_element_block::<8, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Line8Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Line8 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Line9 => parse_fixed_element_block::<9, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Line9Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Line9 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Line10 => parse_fixed_element_block::<10, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Line10Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Line10 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Line11 => parse_fixed_element_block::<11, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Line11Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Line11 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::LineB => parse_variable_element_block(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| LineBElement { tag, nodes },
            |dim, tag, elements| ElementBlock::LineB { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::TriangleB => parse_variable_element_block(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| TriangleBElement { tag, nodes },
            |dim, tag, elements| ElementBlock::TriangleB { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::PolygonB => parse_variable_element_block(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| PolygonBElement { tag, nodes },
            |dim, tag, elements| ElementBlock::PolygonB { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::LineC => parse_variable_element_block(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| LineCElement { tag, nodes },
            |dim, tag, elements| ElementBlock::LineC { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron84 => parse_fixed_element_block::<84, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron84Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron84 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron120 => parse_fixed_element_block::<120, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron120Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron120 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron165 => parse_fixed_element_block::<165, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron165Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron165 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron220 => parse_fixed_element_block::<220, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron220Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron220 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron286 => parse_fixed_element_block::<286, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron286Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron286 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron34 => parse_fixed_element_block::<34, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron34Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron34 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron40 => parse_fixed_element_block::<40, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron40Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron40 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron46 => parse_fixed_element_block::<46, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron46Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron46 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron52 => parse_fixed_element_block::<52, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron52Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron52 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron58 => parse_fixed_element_block::<58, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron58Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron58 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Line1 => parse_fixed_element_block::<1, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Line1Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Line1 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Triangle1 => parse_fixed_element_block::<1, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Triangle1Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Triangle1 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Quadrangle1 => parse_fixed_element_block::<1, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Quadrangle1Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Quadrangle1 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron1 => parse_fixed_element_block::<1, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron1Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron1 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron1 => parse_fixed_element_block::<1, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron1Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron1 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism1 => parse_fixed_element_block::<1, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism1Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism1 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism40 => parse_fixed_element_block::<40, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism40Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism40 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism75 => parse_fixed_element_block::<75, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism75Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism75 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron64 => parse_fixed_element_block::<64, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron64Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron64 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron125 => parse_fixed_element_block::<125, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron125Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron125 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron216 => parse_fixed_element_block::<216, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron216Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron216 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron343 => parse_fixed_element_block::<343, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron343Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron343 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron512 => parse_fixed_element_block::<512, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron512Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron512 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron729 => parse_fixed_element_block::<729, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron729Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron729 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron1000 => parse_fixed_element_block::<1000, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron1000Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron1000 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron32 => parse_fixed_element_block::<32, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron32Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron32 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron44 => parse_fixed_element_block::<44, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron44Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron44 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron56 => parse_fixed_element_block::<56, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron56Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron56 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron68 => parse_fixed_element_block::<68, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron68Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron68 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron80 => parse_fixed_element_block::<80, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron80Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron80 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron92 => parse_fixed_element_block::<92, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron92Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron92 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Hexahedron104 => parse_fixed_element_block::<104, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Hexahedron104Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Hexahedron104 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism126 => parse_fixed_element_block::<126, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism126Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism126 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism196 => parse_fixed_element_block::<196, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism196Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism196 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism288 => parse_fixed_element_block::<288, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism288Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism288 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism405 => parse_fixed_element_block::<405, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism405Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism405 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism550 => parse_fixed_element_block::<550, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism550Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism550 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism24 => parse_fixed_element_block::<24, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism24Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism24 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism33 => parse_fixed_element_block::<33, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism33Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism33 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism42 => parse_fixed_element_block::<42, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism42Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism42 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism51 => parse_fixed_element_block::<51, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism51Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism51 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism60 => parse_fixed_element_block::<60, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism60Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism60 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism69 => parse_fixed_element_block::<69, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism69Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism69 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Prism78 => parse_fixed_element_block::<78, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Prism78Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Prism78 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid30 => parse_fixed_element_block::<30, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid30Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid30 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid55 => parse_fixed_element_block::<55, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid55Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid55 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid91 => parse_fixed_element_block::<91, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid91Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid91 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid140 => parse_fixed_element_block::<140, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid140Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid140 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid204 => parse_fixed_element_block::<204, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid204Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid204 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid285 => parse_fixed_element_block::<285, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid285Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid285 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid385 => parse_fixed_element_block::<385, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid385Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid385 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid21 => parse_fixed_element_block::<21, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid21Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid21 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid29 => parse_fixed_element_block::<29, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid29Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid29 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid37 => parse_fixed_element_block::<37, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid37Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid37 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid45 => parse_fixed_element_block::<45, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid45Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid45 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid53 => parse_fixed_element_block::<53, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid53Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid53 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid61 => parse_fixed_element_block::<61, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid61Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid61 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid69 => parse_fixed_element_block::<69, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid69Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid69 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Pyramid1 => parse_fixed_element_block::<1, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Pyramid1Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Pyramid1 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::PointSub => parse_variable_element_block(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| PointSubElement { tag, nodes },
            |dim, tag, elements| ElementBlock::PointSub { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::LineSub => parse_variable_element_block(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| LineSubElement { tag, nodes },
            |dim, tag, elements| ElementBlock::LineSub { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::TriangleSub => parse_variable_element_block(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| TriangleSubElement { tag, nodes },
            |dim, tag, elements| ElementBlock::TriangleSub { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::TetrahedronSub => parse_variable_element_block(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| TetrahedronSubElement { tag, nodes },
            |dim, tag, elements| ElementBlock::TetrahedronSub { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::Tetrahedron16 => parse_fixed_element_block::<16, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| Tetrahedron16Element { tag, nodes },
            |dim, tag, elements| ElementBlock::Tetrahedron16 { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::TriangleMini => parse_variable_element_block(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| TriangleMiniElement { tag, nodes },
            |dim, tag, elements| ElementBlock::TriangleMini { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::TetrahedronMini => parse_variable_element_block(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| TetrahedronMiniElement { tag, nodes },
            |dim, tag, elements| ElementBlock::TetrahedronMini { entity_dim: dim, entity_tag: tag, elements }
        ),
        ElementType::TriHedron4 => parse_fixed_element_block::<4, _, _>(
            lines, num_elements_in_block, entity_dim, entity_tag,
            |tag, nodes| TriHedron4Element { tag, nodes },
            |dim, tag, elements| ElementBlock::TriHedron4 { entity_dim: dim, entity_tag: tag, elements }
        ),
    }
}

/// Helper function to parse fixed-size elements
fn parse_fixed_element_block<const N: usize, E, R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    num_elements: usize,
    entity_dim: i32,
    entity_tag: i32,
    create_element: impl Fn(usize, [usize; N]) -> E,
    create_block: impl FnOnce(i32, i32, Vec<E>) -> ElementBlock,
) -> Result<ElementBlock> {
    let mut elements = Vec::with_capacity(num_elements);

    for _ in 0..num_elements {
        let line = read_line(lines)?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 1 + N {
            return Err(ParseError::InvalidData(
                "Elements/Block".to_string(),
                format!("Expected {} values (1 tag + {} nodes), got {}", 1 + N, N, parts.len()),
            ));
        }

        let tag: usize = parts[0].parse()?;

        let nodes: Vec<usize> = parts[1..1 + N]
            .iter()
            .map(|s| s.parse())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e: std::num::ParseIntError| ParseError::InvalidData("Elements/Block".to_string(), e.to_string()))?;

        let nodes_array: [usize; N] = nodes.try_into()
            .map_err(|_| ParseError::InvalidData("Elements/Block".to_string(), "Failed to convert to array".to_string()))?;

        elements.push(create_element(tag, nodes_array));
    }

    Ok(create_block(entity_dim, entity_tag, elements))
}

/// Helper function to parse variable-size elements
fn parse_variable_element_block<E, R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    num_elements: usize,
    entity_dim: i32,
    entity_tag: i32,
    create_element: impl Fn(usize, Vec<usize>) -> E,
    create_block: impl FnOnce(i32, i32, Vec<E>) -> ElementBlock,
) -> Result<ElementBlock> {
    let mut elements = Vec::with_capacity(num_elements);

    for _ in 0..num_elements {
        let line = read_line(lines)?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            return Err(ParseError::InvalidData(
                "Elements/Block".to_string(),
                "Expected at least tag value".to_string(),
            ));
        }

        let tag: usize = parts[0].parse()?;

        let nodes: Vec<usize> = parts[1..]
            .iter()
            .map(|s| s.parse())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e: std::num::ParseIntError| ParseError::InvalidData("Elements/Block".to_string(), e.to_string()))?;

        elements.push(create_element(tag, nodes));
    }

    Ok(create_block(entity_dim, entity_tag, elements))
}

/// Special helper for Point elements (no nodes in data, just tag)
fn parse_point_element_block<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    num_elements: usize,
    entity_dim: i32,
    entity_tag: i32,
) -> Result<ElementBlock> {
    let mut elements = Vec::with_capacity(num_elements);

    for _ in 0..num_elements {
        let line = read_line(lines)?;
        let tag: usize = line.trim().parse()?;
        elements.push(PointElement { tag });
    }

    Ok(ElementBlock::Point { entity_dim, entity_tag, elements })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, Cursor};

    #[test]
    fn test_parse_elements() {
        let data = r#"1 2 1 2
2 1 3 2
1 1 2 3 4
2 2 5 6 3
$EndElements
"#;
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        let mut lines = reader.lines();
        let mut mesh = Mesh::default();

        let result = parse(&mut lines, &mut mesh);
        assert!(result.is_ok());
        assert_eq!(mesh.element_blocks.len(), 1);

        match &mesh.element_blocks[0] {
            ElementBlock::Quadrangle4 { entity_dim, entity_tag, elements } => {
                assert_eq!(*entity_dim, 2);
                assert_eq!(*entity_tag, 1);
                assert_eq!(elements.len(), 2);
                assert_eq!(elements[0].tag, 1);
                assert_eq!(elements[0].nodes, [1, 2, 3, 4]);
            }
            _ => panic!("Expected Quadrangle4 block"),
        }
    }
}
