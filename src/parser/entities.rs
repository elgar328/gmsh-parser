use super::LineReader;
use crate::error::Result;
use crate::types::{CurveEntity, Entities, Mesh, PointEntity, SurfaceEntity, VolumeEntity};

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();

    let num_points = iter.parse_usize("numPoints")?;
    let num_curves = iter.parse_usize("numCurves")?;
    let num_surfaces = iter.parse_usize("numSurfaces")?;
    let num_volumes = iter.parse_usize("numVolumes")?;

    iter.expect_no_more()?;

    // Entities section should only appear once
    if mesh.entities.is_some() {
        return Err(token_line.invalid_format(
            "Duplicate $Entities section found - this section should only appear once",
        ));
    }
    mesh.entities = Some(Entities::new());
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
    let mut iter = token_line.iter();

    let tag = iter.parse_int("tag")?;
    let x = iter.parse_float("x")?;
    let y = iter.parse_float("y")?;
    let z = iter.parse_float("z")?;

    let num_physical_tags = iter.parse_usize("numPhysicalTags")?;
    let physical_tags = iter.parse_ints(num_physical_tags, "physicalTag")?;

    iter.expect_no_more()?;

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
    let mut iter = token_line.iter();

    let tag = iter.parse_int("tag")?;
    let min_x = iter.parse_float("minX")?;
    let min_y = iter.parse_float("minY")?;
    let min_z = iter.parse_float("minZ")?;
    let max_x = iter.parse_float("maxX")?;
    let max_y = iter.parse_float("maxY")?;
    let max_z = iter.parse_float("maxZ")?;

    let num_physical_tags = iter.parse_usize("numPhysicalTags")?;
    let physical_tags: Vec<i32> = iter.parse_ints(num_physical_tags, "physicalTag")?;

    let num_bounding_points = iter.parse_usize("numBoundingPoints")?;
    let bounding_points: Vec<i32> = iter.parse_ints(num_bounding_points, "boundingPoint")?;

    iter.expect_no_more()?;

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
    let mut iter = token_line.iter();

    let tag = iter.parse_int("tag")?;
    let min_x = iter.parse_float("minX")?;
    let min_y = iter.parse_float("minY")?;
    let min_z = iter.parse_float("minZ")?;
    let max_x = iter.parse_float("maxX")?;
    let max_y = iter.parse_float("maxY")?;
    let max_z = iter.parse_float("maxZ")?;

    let num_physical_tags = iter.parse_usize("numPhysicalTags")?;
    let physical_tags: Vec<i32> = iter.parse_ints(num_physical_tags, "physicalTag")?;

    let num_bounding_curves = iter.parse_usize("numBoundingCurves")?;
    let bounding_curves: Vec<i32> = iter.parse_ints(num_bounding_curves, "boundingCurve")?;

    iter.expect_no_more()?;

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
    let mut iter = token_line.iter();

    let tag = iter.parse_int("tag")?;
    let min_x = iter.parse_float("minX")?;
    let min_y = iter.parse_float("minY")?;
    let min_z = iter.parse_float("minZ")?;
    let max_x = iter.parse_float("maxX")?;
    let max_y = iter.parse_float("maxY")?;
    let max_z = iter.parse_float("maxZ")?;

    let num_physical_tags = iter.parse_usize("numPhysicalTags")?;
    let physical_tags: Vec<i32> = iter.parse_ints(num_physical_tags, "physicalTag")?;

    let num_bounding_surfaces = iter.parse_usize("numBoundingSurfaces")?;
    let bounding_surfaces: Vec<i32> = iter.parse_ints(num_bounding_surfaces, "boundingSurface")?;

    iter.expect_no_more()?;

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
        let mut mesh = Mesh::dummy();

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
