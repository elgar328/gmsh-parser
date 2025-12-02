use super::LineReader;
use crate::error::Result;
use crate::types::{CurveEntity, Entities, Mesh, PointEntity, SurfaceEntity, VolumeEntity};

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    let token_line = reader.read_token_line()?;

    token_line.expect_len(4)?;

    let num_points = token_line.tokens[0].parse_usize("numPoints")?;
    let num_curves = token_line.tokens[1].parse_usize("numCurves")?;
    let num_surfaces = token_line.tokens[2].parse_usize("numSurfaces")?;
    let num_volumes = token_line.tokens[3].parse_usize("numVolumes")?;

    // Initialize entities if not already present
    if mesh.entities.is_none() {
        mesh.entities = Some(Entities::new());
    }
    let entities = mesh.entities.as_mut().unwrap();

    // Parse points
    for _ in 0..num_points {
        let point = parse_point_entity(reader)?;
        entities.points.push(point);
    }

    // Parse curves
    for _ in 0..num_curves {
        let curve = parse_curve_entity(reader)?;
        entities.curves.push(curve);
    }

    // Parse surfaces
    for _ in 0..num_surfaces {
        let surface = parse_surface_entity(reader)?;
        entities.surfaces.push(surface);
    }

    // Parse volumes
    for _ in 0..num_volumes {
        let volume = parse_volume_entity(reader)?;
        entities.volumes.push(volume);
    }

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("Entities")?;

    Ok(())
}

fn parse_point_entity(reader: &mut LineReader) -> Result<PointEntity> {
    let token_line = reader.read_token_line()?;

    token_line.expect_min_len(5)?;

    let tag = token_line.tokens[0].parse_int("tag")?;
    let x = token_line.tokens[1].parse_float("x")?;
    let y = token_line.tokens[2].parse_float("y")?;
    let z = token_line.tokens[3].parse_float("z")?;
    let num_physical_tags = token_line.tokens[4].parse_usize("numPhysicalTags")?;

    token_line.expect_min_len(5 + num_physical_tags)?;

    let physical_tags: Vec<i32> = token_line.tokens[5..5 + num_physical_tags]
        .iter()
        .map(|t| t.parse_int("physicalTag"))
        .collect::<Result<Vec<i32>>>()?;

    Ok(PointEntity {
        tag,
        x,
        y,
        z,
        physical_tags,
    })
}

fn parse_curve_entity(reader: &mut LineReader) -> Result<CurveEntity> {
    let token_line = reader.read_token_line()?;

    token_line.expect_min_len(8)?;

    let tag = token_line.tokens[0].parse_int("tag")?;
    let min_x = token_line.tokens[1].parse_float("minX")?;
    let min_y = token_line.tokens[2].parse_float("minY")?;
    let min_z = token_line.tokens[3].parse_float("minZ")?;
    let max_x = token_line.tokens[4].parse_float("maxX")?;
    let max_y = token_line.tokens[5].parse_float("maxY")?;
    let max_z = token_line.tokens[6].parse_float("maxZ")?;
    let num_physical_tags = token_line.tokens[7].parse_usize("numPhysicalTags")?;

    let physical_tags_idx = 8;
    token_line.expect_min_len(physical_tags_idx + num_physical_tags + 1)?;

    let physical_tags: Vec<i32> = token_line.tokens
        [physical_tags_idx..physical_tags_idx + num_physical_tags]
        .iter()
        .map(|t| t.parse_int("physicalTag"))
        .collect::<Result<Vec<i32>>>()?;

    let num_bounding_points_idx = physical_tags_idx + num_physical_tags;
    let num_bounding_points =
        token_line.tokens[num_bounding_points_idx].parse_usize("numBoundingPoints")?;

    let bounding_points_idx = num_bounding_points_idx + 1;
    token_line.expect_min_len(bounding_points_idx + num_bounding_points)?;

    let bounding_points: Vec<i32> = token_line.tokens
        [bounding_points_idx..bounding_points_idx + num_bounding_points]
        .iter()
        .map(|t| t.parse_int("boundingPoint"))
        .collect::<Result<Vec<i32>>>()?;

    Ok(CurveEntity {
        tag,
        min_x,
        min_y,
        min_z,
        max_x,
        max_y,
        max_z,
        physical_tags,
        bounding_points,
    })
}

fn parse_surface_entity(reader: &mut LineReader) -> Result<SurfaceEntity> {
    let token_line = reader.read_token_line()?;

    token_line.expect_min_len(8)?;

    let tag = token_line.tokens[0].parse_int("tag")?;
    let min_x = token_line.tokens[1].parse_float("minX")?;
    let min_y = token_line.tokens[2].parse_float("minY")?;
    let min_z = token_line.tokens[3].parse_float("minZ")?;
    let max_x = token_line.tokens[4].parse_float("maxX")?;
    let max_y = token_line.tokens[5].parse_float("maxY")?;
    let max_z = token_line.tokens[6].parse_float("maxZ")?;
    let num_physical_tags = token_line.tokens[7].parse_usize("numPhysicalTags")?;

    let physical_tags_idx = 8;
    token_line.expect_min_len(physical_tags_idx + num_physical_tags + 1)?;

    let physical_tags: Vec<i32> = token_line.tokens
        [physical_tags_idx..physical_tags_idx + num_physical_tags]
        .iter()
        .map(|t| t.parse_int("physicalTag"))
        .collect::<Result<Vec<i32>>>()?;

    let num_bounding_curves_idx = physical_tags_idx + num_physical_tags;
    let num_bounding_curves =
        token_line.tokens[num_bounding_curves_idx].parse_usize("numBoundingCurves")?;

    let bounding_curves_idx = num_bounding_curves_idx + 1;
    token_line.expect_min_len(bounding_curves_idx + num_bounding_curves)?;

    let bounding_curves: Vec<i32> = token_line.tokens
        [bounding_curves_idx..bounding_curves_idx + num_bounding_curves]
        .iter()
        .map(|t| t.parse_int("boundingCurve"))
        .collect::<Result<Vec<i32>>>()?;

    Ok(SurfaceEntity {
        tag,
        min_x,
        min_y,
        min_z,
        max_x,
        max_y,
        max_z,
        physical_tags,
        bounding_curves,
    })
}

fn parse_volume_entity(reader: &mut LineReader) -> Result<VolumeEntity> {
    let token_line = reader.read_token_line()?;

    token_line.expect_min_len(8)?;

    let tag = token_line.tokens[0].parse_int("tag")?;
    let min_x = token_line.tokens[1].parse_float("minX")?;
    let min_y = token_line.tokens[2].parse_float("minY")?;
    let min_z = token_line.tokens[3].parse_float("minZ")?;
    let max_x = token_line.tokens[4].parse_float("maxX")?;
    let max_y = token_line.tokens[5].parse_float("maxY")?;
    let max_z = token_line.tokens[6].parse_float("maxZ")?;
    let num_physical_tags = token_line.tokens[7].parse_usize("numPhysicalTags")?;

    let physical_tags_idx = 8;
    token_line.expect_min_len(physical_tags_idx + num_physical_tags + 1)?;

    let physical_tags: Vec<i32> = token_line.tokens
        [physical_tags_idx..physical_tags_idx + num_physical_tags]
        .iter()
        .map(|t| t.parse_int("physicalTag"))
        .collect::<Result<Vec<i32>>>()?;

    let num_bounding_surfaces_idx = physical_tags_idx + num_physical_tags;
    let num_bounding_surfaces =
        token_line.tokens[num_bounding_surfaces_idx].parse_usize("numBoundingSurfaces")?;

    let bounding_surfaces_idx = num_bounding_surfaces_idx + 1;
    token_line.expect_min_len(bounding_surfaces_idx + num_bounding_surfaces)?;

    let bounding_surfaces: Vec<i32> = token_line.tokens
        [bounding_surfaces_idx..bounding_surfaces_idx + num_bounding_surfaces]
        .iter()
        .map(|t| t.parse_int("boundingSurface"))
        .collect::<Result<Vec<i32>>>()?;

    Ok(VolumeEntity {
        tag,
        min_x,
        min_y,
        min_z,
        max_x,
        max_y,
        max_z,
        physical_tags,
        bounding_surfaces,
    })
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn test_parse_entities() {
        let data = r#"1 0 0 0
1 0.0 0.0 0.0 0
$EndEntities
"#;

        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);
        let mut mesh = Mesh::default();

        let result = parse(&mut reader, &mut mesh);
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
