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
    partitioned.num_partitions = token_line.tokens[0].parse_usize("numPartitions")?;

    // Read: numGhostEntities
    let token_line = reader.read_token_line()?;
    let num_ghost_entities = token_line.tokens[0].parse_usize("numGhostEntities")?;

    // Read ghost entities
    for _ in 0..num_ghost_entities {
        let token_line = reader.read_token_line()?;
        token_line.expect_len(2)?;

        partitioned.ghost_entities.push(GhostEntity {
            tag: token_line.tokens[0].parse_int("ghostEntityTag")?,
            partition: token_line.tokens[1].parse_int("ghostEntityPartition")?,
        });
    }

    // Read: numPoints numCurves numSurfaces numVolumes
    let token_line = reader.read_token_line()?;

    token_line.expect_len(4)?;
    let num_points = token_line.tokens[0].parse_usize("numPoints")?;
    let num_curves = token_line.tokens[1].parse_usize("numCurves")?;
    let num_surfaces = token_line.tokens[2].parse_usize("numSurfaces")?;
    let num_volumes = token_line.tokens[3].parse_usize("numVolumes")?;

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

    token_line.expect_min_len(4)?;

    let tag = token_line.tokens[0].parse_int("tag")?;
    let parent_dim = token_line.tokens[1].parse_entity_dimension("parent_dim")?;
    let parent_tag = token_line.tokens[2].parse_int("parent_tag")?;
    let num_partitions = token_line.tokens[3].parse_usize("numPartitions")?;

    token_line.expect_min_len(4 + num_partitions)?;
    let mut partition_tags = Vec::with_capacity(num_partitions);
    for i in 0..num_partitions {
        partition_tags.push(token_line.tokens[4 + i].parse_int("partitionTag")?);
    }

    // Continue parsing from the same line: x y z numPhysicalTags physicalTag ...
    let coord_offset = 4 + num_partitions;
    token_line.expect_min_len(coord_offset + 4)?;

    let x = token_line.tokens[coord_offset].parse_float("x")?;
    let y = token_line.tokens[coord_offset + 1].parse_float("y")?;
    let z = token_line.tokens[coord_offset + 2].parse_float("z")?;
    let num_physical_tags = token_line.tokens[coord_offset + 3].parse_usize("numPhysicalTags")?;

    token_line.expect_min_len(coord_offset + 4 + num_physical_tags)?;
    let mut physical_tags = Vec::with_capacity(num_physical_tags);
    for i in 0..num_physical_tags {
        let idx = coord_offset + 4 + i;
        physical_tags.push(token_line.tokens[idx].parse_int("physicalTag")?);
    }

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
    token_line.expect_min_len(4)?;

    let tag = token_line.tokens[0].parse_int("tag")?;
    let parent_dim = token_line.tokens[1].parse_entity_dimension("parent_dim")?;
    let parent_tag = token_line.tokens[2].parse_int("parent_tag")?;
    let num_partitions = token_line.tokens[3].parse_usize("numPartitions")?;

    token_line.expect_min_len(4 + num_partitions)?;
    let mut partition_tags = Vec::with_capacity(num_partitions);
    for i in 0..num_partitions {
        partition_tags.push(token_line.tokens[4 + i].parse_int("partitionTag")?);
    }

    // Continue parsing: minX minY minZ maxX maxY maxZ numPhysicalTags physicalTag ...
    let bbox_offset = 4 + num_partitions;
    token_line.expect_min_len(bbox_offset + 7)?;

    let min_x = token_line.tokens[bbox_offset].parse_float("minX")?;
    let min_y = token_line.tokens[bbox_offset + 1].parse_float("minY")?;
    let min_z = token_line.tokens[bbox_offset + 2].parse_float("minZ")?;
    let max_x = token_line.tokens[bbox_offset + 3].parse_float("maxX")?;
    let max_y = token_line.tokens[bbox_offset + 4].parse_float("maxY")?;
    let max_z = token_line.tokens[bbox_offset + 5].parse_float("maxZ")?;
    let num_physical_tags = token_line.tokens[bbox_offset + 6].parse_usize("numPhysicalTags")?;

    token_line.expect_min_len(bbox_offset + 7 + num_physical_tags)?;
    let mut physical_tags = Vec::with_capacity(num_physical_tags);
    for i in 0..num_physical_tags {
        let idx = bbox_offset + 7 + i;
        physical_tags.push(token_line.tokens[idx].parse_int("physicalTag")?);
    }

    // Continue parsing: numBoundingPoints pointTag ...
    let boundary_offset = bbox_offset + 7 + num_physical_tags;
    token_line.expect_min_len(boundary_offset)?;

    let num_bounding_points =
        token_line.tokens[boundary_offset].parse_usize("numBoundingPoints")?;

    token_line.expect_min_len(boundary_offset + 1 + num_bounding_points)?;
    let mut bounding_points = Vec::with_capacity(num_bounding_points);
    for i in 0..num_bounding_points {
        let idx = boundary_offset + 1 + i;
        bounding_points.push(token_line.tokens[idx].parse_int("boundingPoint")?);
    }

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

    token_line.expect_min_len(4)?;

    let tag = token_line.tokens[0].parse_int("tag")?;
    let parent_dim = token_line.tokens[1].parse_entity_dimension("parent_dim")?;
    let parent_tag = token_line.tokens[2].parse_int("parent_tag")?;
    let num_partitions = token_line.tokens[3].parse_usize("numPartitions")?;

    token_line.expect_min_len(4 + num_partitions)?;
    let mut partition_tags = Vec::with_capacity(num_partitions);
    for i in 0..num_partitions {
        partition_tags.push(token_line.tokens[4 + i].parse_int("partitionTag")?);
    }

    // Continue parsing: minX minY minZ maxX maxY maxZ numPhysicalTags physicalTag ...
    let bbox_offset = 4 + num_partitions;
    token_line.expect_min_len(bbox_offset + 7)?;

    let min_x = token_line.tokens[bbox_offset].parse_float("minX")?;
    let min_y = token_line.tokens[bbox_offset + 1].parse_float("minY")?;
    let min_z = token_line.tokens[bbox_offset + 2].parse_float("minZ")?;
    let max_x = token_line.tokens[bbox_offset + 3].parse_float("maxX")?;
    let max_y = token_line.tokens[bbox_offset + 4].parse_float("maxY")?;
    let max_z = token_line.tokens[bbox_offset + 5].parse_float("maxZ")?;
    let num_physical_tags = token_line.tokens[bbox_offset + 6].parse_usize("numPhysicalTags")?;

    token_line.expect_min_len(bbox_offset + 7 + num_physical_tags)?;
    let mut physical_tags = Vec::with_capacity(num_physical_tags);
    for i in 0..num_physical_tags {
        let idx = bbox_offset + 7 + i;
        physical_tags.push(token_line.tokens[idx].parse_int("physicalTag")?);
    }

    // Continue parsing: numBoundingCurves curveTag ...
    let boundary_offset = bbox_offset + 7 + num_physical_tags;
    token_line.expect_min_len(boundary_offset)?;

    let num_bounding_curves =
        token_line.tokens[boundary_offset].parse_usize("numBoundingCurves")?;

    token_line.expect_min_len(boundary_offset + 1 + num_bounding_curves)?;
    let mut bounding_curves = Vec::with_capacity(num_bounding_curves);
    for i in 0..num_bounding_curves {
        let idx = boundary_offset + 1 + i;
        bounding_curves.push(token_line.tokens[idx].parse_int("boundingCurve")?);
    }

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
    token_line.expect_min_len(4)?;

    let tag = token_line.tokens[0].parse_int("tag")?;
    let parent_dim = token_line.tokens[1].parse_entity_dimension("parent_dim")?;
    let parent_tag = token_line.tokens[2].parse_int("parent_tag")?;
    let num_partitions = token_line.tokens[3].parse_usize("numPartitions")?;

    token_line.expect_min_len(4 + num_partitions)?;
    let mut partition_tags = Vec::with_capacity(num_partitions);
    for i in 0..num_partitions {
        partition_tags.push(token_line.tokens[4 + i].parse_int("partitionTag")?);
    }

    // Continue parsing: minX minY minZ maxX maxY maxZ numPhysicalTags physicalTag ...
    let bbox_offset = 4 + num_partitions;
    token_line.expect_min_len(bbox_offset + 7)?;

    let min_x = token_line.tokens[bbox_offset].parse_float("minX")?;
    let min_y = token_line.tokens[bbox_offset + 1].parse_float("minY")?;
    let min_z = token_line.tokens[bbox_offset + 2].parse_float("minZ")?;
    let max_x = token_line.tokens[bbox_offset + 3].parse_float("maxX")?;
    let max_y = token_line.tokens[bbox_offset + 4].parse_float("maxY")?;
    let max_z = token_line.tokens[bbox_offset + 5].parse_float("maxZ")?;
    let num_physical_tags = token_line.tokens[bbox_offset + 6].parse_usize("numPhysicalTags")?;

    token_line.expect_min_len(bbox_offset + 7 + num_physical_tags)?;
    let mut physical_tags = Vec::with_capacity(num_physical_tags);
    for i in 0..num_physical_tags {
        let idx = bbox_offset + 7 + i;
        physical_tags.push(token_line.tokens[idx].parse_int("physicalTag")?);
    }

    // Continue parsing: numBoundingSurfaces surfaceTag ...
    let boundary_offset = bbox_offset + 7 + num_physical_tags;
    token_line.expect_min_len(boundary_offset)?;

    let num_bounding_surfaces =
        token_line.tokens[boundary_offset].parse_usize("numBoundingSurfaces")?;

    token_line.expect_min_len(boundary_offset + 1 + num_bounding_surfaces)?;
    let mut bounding_surfaces = Vec::with_capacity(num_bounding_surfaces);
    for i in 0..num_bounding_surfaces {
        let idx = boundary_offset + 1 + i;
        bounding_surfaces.push(token_line.tokens[idx].parse_int("boundingSurface")?);
    }

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
