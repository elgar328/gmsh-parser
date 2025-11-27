use std::io::{BufReader, Lines};

use crate::error::{ParseError, Result};
use crate::types::{Mesh, Entities, PointEntity, CurveEntity, SurfaceEntity, VolumeEntity};
use super::{read_line, expect_end_marker};

pub fn parse<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    let line = read_line(lines)?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 4 {
        return Err(ParseError::InvalidData(
            "Entities".to_string(),
            format!("Expected 4 counts, got {}", parts.len()),
        ));
    }

    let num_points: usize = parts[0].parse()?;
    let num_curves: usize = parts[1].parse()?;
    let num_surfaces: usize = parts[2].parse()?;
    let num_volumes: usize = parts[3].parse()?;

    // Initialize entities if not already present
    if mesh.entities.is_none() {
        mesh.entities = Some(Entities::new());
    }
    let entities = mesh.entities.as_mut().unwrap();

    // Parse points
    for _ in 0..num_points {
        let point = parse_point_entity(lines)?;
        entities.points.push(point);
    }

    // Parse curves
    for _ in 0..num_curves {
        let curve = parse_curve_entity(lines)?;
        entities.curves.push(curve);
    }

    // Parse surfaces
    for _ in 0..num_surfaces {
        let surface = parse_surface_entity(lines)?;
        entities.surfaces.push(surface);
    }

    // Parse volumes
    for _ in 0..num_volumes {
        let volume = parse_volume_entity(lines)?;
        entities.volumes.push(volume);
    }

    expect_end_marker(lines, "Entities")?;

    Ok(())
}

fn parse_point_entity<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
) -> Result<PointEntity> {
    let line = read_line(lines)?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 5 {
        return Err(ParseError::InvalidData(
            "Entities/Point".to_string(),
            format!("Expected at least 5 values, got {}", parts.len()),
        ));
    }

    let tag: i32 = parts[0].parse()?;
    let x: f64 = parts[1].parse()?;
    let y: f64 = parts[2].parse()?;
    let z: f64 = parts[3].parse()?;
    let num_physical_tags: usize = parts[4].parse()?;

    if parts.len() < 5 + num_physical_tags {
        return Err(ParseError::InvalidData(
            "Entities/Point".to_string(),
            format!(
                "Expected {} physical tags, but not enough values",
                num_physical_tags
            ),
        ));
    }

    let physical_tags: Result<Vec<i32>> = parts[5..5 + num_physical_tags]
        .iter()
        .map(|s| s.parse().map_err(Into::into))
        .collect();

    Ok(PointEntity {
        tag,
        x,
        y,
        z,
        physical_tags: physical_tags?,
    })
}

fn parse_curve_entity<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
) -> Result<CurveEntity> {
    let line = read_line(lines)?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 8 {
        return Err(ParseError::InvalidData(
            "Entities/Curve".to_string(),
            format!("Expected at least 8 values, got {}", parts.len()),
        ));
    }

    let tag: i32 = parts[0].parse()?;
    let min_x: f64 = parts[1].parse()?;
    let min_y: f64 = parts[2].parse()?;
    let min_z: f64 = parts[3].parse()?;
    let max_x: f64 = parts[4].parse()?;
    let max_y: f64 = parts[5].parse()?;
    let max_z: f64 = parts[6].parse()?;
    let num_physical_tags: usize = parts[7].parse()?;

    let idx = 8;
    if parts.len() < idx + num_physical_tags + 1 {
        return Err(ParseError::InvalidData(
            "Entities/Curve".to_string(),
            "Not enough values for physical tags and bounding points".to_string(),
        ));
    }

    let physical_tags: Result<Vec<i32>> = parts[idx..idx + num_physical_tags]
        .iter()
        .map(|s| s.parse().map_err(Into::into))
        .collect();

    let idx = idx + num_physical_tags;
    let num_bounding_points: usize = parts[idx].parse()?;

    let idx = idx + 1;
    if parts.len() < idx + num_bounding_points {
        return Err(ParseError::InvalidData(
            "Entities/Curve".to_string(),
            "Not enough values for bounding points".to_string(),
        ));
    }

    let bounding_points: Result<Vec<i32>> = parts[idx..idx + num_bounding_points]
        .iter()
        .map(|s| s.parse().map_err(Into::into))
        .collect();

    Ok(CurveEntity {
        tag,
        min_x,
        min_y,
        min_z,
        max_x,
        max_y,
        max_z,
        physical_tags: physical_tags?,
        bounding_points: bounding_points?,
    })
}

fn parse_surface_entity<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
) -> Result<SurfaceEntity> {
    let line = read_line(lines)?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 8 {
        return Err(ParseError::InvalidData(
            "Entities/Surface".to_string(),
            format!("Expected at least 8 values, got {}", parts.len()),
        ));
    }

    let tag: i32 = parts[0].parse()?;
    let min_x: f64 = parts[1].parse()?;
    let min_y: f64 = parts[2].parse()?;
    let min_z: f64 = parts[3].parse()?;
    let max_x: f64 = parts[4].parse()?;
    let max_y: f64 = parts[5].parse()?;
    let max_z: f64 = parts[6].parse()?;
    let num_physical_tags: usize = parts[7].parse()?;

    let idx = 8;
    if parts.len() < idx + num_physical_tags + 1 {
        return Err(ParseError::InvalidData(
            "Entities/Surface".to_string(),
            "Not enough values for physical tags and bounding curves".to_string(),
        ));
    }

    let physical_tags: Result<Vec<i32>> = parts[idx..idx + num_physical_tags]
        .iter()
        .map(|s| s.parse().map_err(Into::into))
        .collect();

    let idx = idx + num_physical_tags;
    let num_bounding_curves: usize = parts[idx].parse()?;

    let idx = idx + 1;
    if parts.len() < idx + num_bounding_curves {
        return Err(ParseError::InvalidData(
            "Entities/Surface".to_string(),
            "Not enough values for bounding curves".to_string(),
        ));
    }

    let bounding_curves: Result<Vec<i32>> = parts[idx..idx + num_bounding_curves]
        .iter()
        .map(|s| s.parse().map_err(Into::into))
        .collect();

    Ok(SurfaceEntity {
        tag,
        min_x,
        min_y,
        min_z,
        max_x,
        max_y,
        max_z,
        physical_tags: physical_tags?,
        bounding_curves: bounding_curves?,
    })
}

fn parse_volume_entity<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
) -> Result<VolumeEntity> {
    let line = read_line(lines)?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 8 {
        return Err(ParseError::InvalidData(
            "Entities/Volume".to_string(),
            format!("Expected at least 8 values, got {}", parts.len()),
        ));
    }

    let tag: i32 = parts[0].parse()?;
    let min_x: f64 = parts[1].parse()?;
    let min_y: f64 = parts[2].parse()?;
    let min_z: f64 = parts[3].parse()?;
    let max_x: f64 = parts[4].parse()?;
    let max_y: f64 = parts[5].parse()?;
    let max_z: f64 = parts[6].parse()?;
    let num_physical_tags: usize = parts[7].parse()?;

    let idx = 8;
    if parts.len() < idx + num_physical_tags + 1 {
        return Err(ParseError::InvalidData(
            "Entities/Volume".to_string(),
            "Not enough values for physical tags and bounding surfaces".to_string(),
        ));
    }

    let physical_tags: Result<Vec<i32>> = parts[idx..idx + num_physical_tags]
        .iter()
        .map(|s| s.parse().map_err(Into::into))
        .collect();

    let idx = idx + num_physical_tags;
    let num_bounding_surfaces: usize = parts[idx].parse()?;

    let idx = idx + 1;
    if parts.len() < idx + num_bounding_surfaces {
        return Err(ParseError::InvalidData(
            "Entities/Volume".to_string(),
            "Not enough values for bounding surfaces".to_string(),
        ));
    }

    let bounding_surfaces: Result<Vec<i32>> = parts[idx..idx + num_bounding_surfaces]
        .iter()
        .map(|s| s.parse().map_err(Into::into))
        .collect();

    Ok(VolumeEntity {
        tag,
        min_x,
        min_y,
        min_z,
        max_x,
        max_y,
        max_z,
        physical_tags: physical_tags?,
        bounding_surfaces: bounding_surfaces?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, Cursor};

    #[test]
    fn test_parse_entities() {
        let data = r#"1 0 0 0
1 0.0 0.0 0.0 0
$EndEntities
"#;
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        let mut lines = reader.lines();
        let mut mesh = Mesh::default();

        let result = parse(&mut lines, &mut mesh);
        assert!(result.is_ok());

        let entities = mesh.entities.as_ref().unwrap();
        assert_eq!(entities.points.len(), 1);

        let point = &entities.points[0];
        assert_eq!(point.tag, 1);
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
        assert_eq!(point.z, 0.0);
    }
}
