//! Parser for $PartitionedEntities section

use crate::error::Result;
use crate::types::{
    GhostEntity, Mesh, PartitionedCurve, PartitionedEntities, PartitionedPoint, PartitionedSurface,
    PartitionedVolume,
};

use super::LineReader;

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    let mut partitioned = PartitionedEntities::default();

    // Read: numPartitions
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();
    partitioned.num_partitions = iter.parse_usize("numPartitions")?;
    iter.expect_no_more()?;

    // Read: numGhostEntities
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();
    let num_ghost_entities = iter.parse_usize("numGhostEntities")?;
    iter.expect_no_more()?;

    // Read ghost entities
    for _ in 0..num_ghost_entities {
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();

        partitioned.ghost_entities.push(GhostEntity {
            tag: iter.parse_int("ghostEntityTag")?,
            partition: iter.parse_int("ghostEntityPartition")?,
        });
        iter.expect_no_more()?;
    }

    // Read: numPoints numCurves numSurfaces numVolumes
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();

    let num_points = iter.parse_usize("numPoints")?;
    let num_curves = iter.parse_usize("numCurves")?;
    let num_surfaces = iter.parse_usize("numSurfaces")?;
    let num_volumes = iter.parse_usize("numVolumes")?;
    iter.expect_no_more()?;

    // Parse points
    for _ in 0..num_points {
        partitioned.points.push(parse_partitioned_point(reader)?);
    }

    // Parse curves
    for _ in 0..num_curves {
        partitioned.curves.push(parse_partitioned_curve(reader)?);
    }

    // Parse surfaces
    for _ in 0..num_surfaces {
        partitioned
            .surfaces
            .push(parse_partitioned_surface(reader)?);
    }

    // Parse volumes
    for _ in 0..num_volumes {
        partitioned.volumes.push(parse_partitioned_volume(reader)?);
    }

    mesh.partitioned_entities = Some(partitioned);

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("PartitionedEntities")?;

    Ok(())
}

fn parse_partitioned_point(reader: &mut LineReader) -> Result<PartitionedPoint> {
    // Single line: pointTag parentDim parentTag numPartitions partitionTag ... x y z numPhysicalTags physicalTag ...
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();

    let tag = iter.parse_int("tag")?;
    let parent_dim = iter.parse_entity_dimension("parent_dim")?;
    let parent_tag = iter.parse_int("parent_tag")?;
    let num_partitions = iter.parse_usize("numPartitions")?;

    let partition_tags = iter.parse_ints(num_partitions, "partitionTag")?;

    let x = iter.parse_float("x")?;
    let y = iter.parse_float("y")?;
    let z = iter.parse_float("z")?;
    let num_physical_tags = iter.parse_usize("numPhysicalTags")?;

    let physical_tags = iter.parse_ints(num_physical_tags, "physicalTag")?;
    iter.expect_no_more()?;

    Ok(PartitionedPoint {
        tag,
        parent_dim,
        parent_tag,
        partition_tags,
        x,
        y,
        z,
        physical_tags,
    })
}

fn parse_partitioned_curve(reader: &mut LineReader) -> Result<PartitionedCurve> {
    // Single line: curveTag parentDim parentTag numPartitions partitionTag ... minX minY minZ maxX maxY maxZ numPhysicalTags physicalTag ... numBoundingPoints pointTag ...
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();

    let tag = iter.parse_int("tag")?;
    let parent_dim = iter.parse_entity_dimension("parent_dim")?;
    let parent_tag = iter.parse_int("parent_tag")?;
    let num_partitions = iter.parse_usize("numPartitions")?;

    let partition_tags = iter.parse_ints(num_partitions, "partitionTag")?;

    let min_x = iter.parse_float("minX")?;
    let min_y = iter.parse_float("minY")?;
    let min_z = iter.parse_float("minZ")?;
    let max_x = iter.parse_float("maxX")?;
    let max_y = iter.parse_float("maxY")?;
    let max_z = iter.parse_float("maxZ")?;
    let num_physical_tags = iter.parse_usize("numPhysicalTags")?;

    let physical_tags = iter.parse_ints(num_physical_tags, "physicalTag")?;

    let num_bounding_points = iter.parse_usize("numBoundingPoints")?;
    let bounding_points = iter.parse_ints(num_bounding_points, "boundingPoint")?;
    iter.expect_no_more()?;

    Ok(PartitionedCurve {
        tag,
        parent_dim,
        parent_tag,
        partition_tags,
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

fn parse_partitioned_surface(reader: &mut LineReader) -> Result<PartitionedSurface> {
    // Single line: surfaceTag parentDim parentTag numPartitions partitionTag ... minX minY minZ maxX maxY maxZ numPhysicalTags physicalTag ... numBoundingCurves curveTag ...
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();

    let tag = iter.parse_int("tag")?;
    let parent_dim = iter.parse_entity_dimension("parent_dim")?;
    let parent_tag = iter.parse_int("parent_tag")?;
    let num_partitions = iter.parse_usize("numPartitions")?;

    let partition_tags = iter.parse_ints(num_partitions, "partitionTag")?;

    let min_x = iter.parse_float("minX")?;
    let min_y = iter.parse_float("minY")?;
    let min_z = iter.parse_float("minZ")?;
    let max_x = iter.parse_float("maxX")?;
    let max_y = iter.parse_float("maxY")?;
    let max_z = iter.parse_float("maxZ")?;
    let num_physical_tags = iter.parse_usize("numPhysicalTags")?;

    let physical_tags = iter.parse_ints(num_physical_tags, "physicalTag")?;

    let num_bounding_curves = iter.parse_usize("numBoundingCurves")?;
    let bounding_curves = iter.parse_ints(num_bounding_curves, "boundingCurve")?;
    iter.expect_no_more()?;

    Ok(PartitionedSurface {
        tag,
        parent_dim,
        parent_tag,
        partition_tags,
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

fn parse_partitioned_volume(reader: &mut LineReader) -> Result<PartitionedVolume> {
    // Single line: volumeTag parentDim parentTag numPartitions partitionTag ... minX minY minZ maxX maxY maxZ numPhysicalTags physicalTag ... numBoundingSurfaces surfaceTag ...
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();

    let tag = iter.parse_int("tag")?;
    let parent_dim = iter.parse_entity_dimension("parent_dim")?;
    let parent_tag = iter.parse_int("parent_tag")?;
    let num_partitions = iter.parse_usize("numPartitions")?;

    let partition_tags = iter.parse_ints(num_partitions, "partitionTag")?;

    let min_x = iter.parse_float("minX")?;
    let min_y = iter.parse_float("minY")?;
    let min_z = iter.parse_float("minZ")?;
    let max_x = iter.parse_float("maxX")?;
    let max_y = iter.parse_float("maxY")?;
    let max_z = iter.parse_float("maxZ")?;
    let num_physical_tags = iter.parse_usize("numPhysicalTags")?;

    let physical_tags = iter.parse_ints(num_physical_tags, "physicalTag")?;

    let num_bounding_surfaces = iter.parse_usize("numBoundingSurfaces")?;
    let bounding_surfaces = iter.parse_ints(num_bounding_surfaces, "boundingSurface")?;
    iter.expect_no_more()?;

    Ok(PartitionedVolume {
        tag,
        parent_dim,
        parent_tag,
        partition_tags,
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
