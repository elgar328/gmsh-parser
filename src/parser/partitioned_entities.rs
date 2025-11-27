//! Parser for $PartitionedEntities section

use std::io::BufReader;
use std::io::Lines;

use crate::error::{ParseError, Result};
use crate::types::{Mesh, PartitionedEntities, GhostEntity, PartitionedPoint, PartitionedCurve, PartitionedSurface, PartitionedVolume};

use super::{read_line, expect_end_marker};

pub fn parse<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    let mut partitioned = PartitionedEntities::default();

    // Read: numPartitions
    let header1 = read_line(lines)?;
    partitioned.num_partitions = header1.parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid numPartitions: {}", header1))
    })?;

    // Read: numGhostEntities
    let header2 = read_line(lines)?;
    let num_ghost_entities: usize = header2.parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid numGhostEntities: {}", header2))
    })?;

    // Read ghost entities
    for _ in 0..num_ghost_entities {
        let line = read_line(lines)?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(ParseError::InvalidFormat(format!(
                "Invalid ghost entity line: {}",
                line
            )));
        }

        partitioned.ghost_entities.push(GhostEntity {
            tag: parts[0].parse().map_err(|_| {
                ParseError::InvalidFormat(format!("Invalid ghost entity tag: {}", parts[0]))
            })?,
            partition: parts[1].parse().map_err(|_| {
                ParseError::InvalidFormat(format!("Invalid ghost entity partition: {}", parts[1]))
            })?,
        });
    }

    // Read: numPoints numCurves numSurfaces numVolumes
    let counts_line = read_line(lines)?;
    let counts: Vec<usize> = counts_line
        .split_whitespace()
        .map(|s| {
            s.parse()
                .map_err(|_| ParseError::InvalidFormat(format!("Invalid entity count: {}", s)))
        })
        .collect::<Result<Vec<_>>>()?;

    if counts.len() < 4 {
        return Err(ParseError::InvalidFormat(
            "Invalid entity counts line".to_string(),
        ));
    }

    let (num_points, num_curves, num_surfaces, num_volumes) =
        (counts[0], counts[1], counts[2], counts[3]);

    // Parse points
    for _ in 0..num_points {
        partitioned.points.push(parse_partitioned_point(lines)?);
    }

    // Parse curves
    for _ in 0..num_curves {
        partitioned.curves.push(parse_partitioned_curve(lines)?);
    }

    // Parse surfaces
    for _ in 0..num_surfaces {
        partitioned.surfaces.push(parse_partitioned_surface(lines)?);
    }

    // Parse volumes
    for _ in 0..num_volumes {
        partitioned.volumes.push(parse_partitioned_volume(lines)?);
    }

    mesh.partitioned_entities = Some(partitioned);
    expect_end_marker(lines, "PartitionedEntities")?;
    Ok(())
}

fn parse_partitioned_point<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
) -> Result<PartitionedPoint> {
    // Line 1: pointTag parentDim parentTag numPartitions partitionTag ...
    let line1 = read_line(lines)?;
    let parts1: Vec<&str> = line1.split_whitespace().collect();
    if parts1.len() < 4 {
        return Err(ParseError::InvalidFormat(format!(
            "Invalid partitioned point line 1: {}",
            line1
        )));
    }

    let tag: i32 = parts1[0].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid point tag: {}", parts1[0]))
    })?;
    let parent_dim: i32 = parts1[1].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid parent_dim: {}", parts1[1]))
    })?;
    let parent_tag: i32 = parts1[2].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid parent_tag: {}", parts1[2]))
    })?;
    let num_partitions: usize = parts1[3].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid numPartitions: {}", parts1[3]))
    })?;

    let mut partition_tags = Vec::with_capacity(num_partitions);
    for i in 0..num_partitions {
        if 4 + i >= parts1.len() {
            return Err(ParseError::InvalidFormat(
                "Not enough partition tags".to_string(),
            ));
        }
        partition_tags.push(parts1[4 + i].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid partition tag: {}", parts1[4 + i]))
        })?);
    }

    // Line 2: X Y Z numPhysicalTags physicalTag ...
    let line2 = read_line(lines)?;
    let parts2: Vec<&str> = line2.split_whitespace().collect();
    if parts2.len() < 4 {
        return Err(ParseError::InvalidFormat(format!(
            "Invalid partitioned point line 2: {}",
            line2
        )));
    }

    let x: f64 = parts2[0].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid X: {}", parts2[0]))
    })?;
    let y: f64 = parts2[1].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid Y: {}", parts2[1]))
    })?;
    let z: f64 = parts2[2].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid Z: {}", parts2[2]))
    })?;
    let num_physical_tags: usize = parts2[3].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid numPhysicalTags: {}", parts2[3]))
    })?;

    let mut physical_tags = Vec::with_capacity(num_physical_tags);
    for i in 0..num_physical_tags {
        if 4 + i >= parts2.len() {
            return Err(ParseError::InvalidFormat(
                "Not enough physical tags".to_string(),
            ));
        }
        physical_tags.push(parts2[4 + i].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid physical tag: {}", parts2[4 + i]))
        })?);
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

fn parse_partitioned_curve<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
) -> Result<PartitionedCurve> {
    // Line 1: curveTag parentDim parentTag numPartitions partitionTag ...
    let line1 = read_line(lines)?;
    let parts1: Vec<&str> = line1.split_whitespace().collect();
    if parts1.len() < 4 {
        return Err(ParseError::InvalidFormat(format!(
            "Invalid partitioned curve line 1: {}",
            line1
        )));
    }

    let tag: i32 = parts1[0].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid curve tag: {}", parts1[0]))
    })?;
    let parent_dim: i32 = parts1[1].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid parent_dim: {}", parts1[1]))
    })?;
    let parent_tag: i32 = parts1[2].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid parent_tag: {}", parts1[2]))
    })?;
    let num_partitions: usize = parts1[3].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid numPartitions: {}", parts1[3]))
    })?;

    let mut partition_tags = Vec::with_capacity(num_partitions);
    for i in 0..num_partitions {
        if 4 + i >= parts1.len() {
            return Err(ParseError::InvalidFormat(
                "Not enough partition tags".to_string(),
            ));
        }
        partition_tags.push(parts1[4 + i].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid partition tag: {}", parts1[4 + i]))
        })?);
    }

    // Line 2: minX minY minZ maxX maxY maxZ numPhysicalTags physicalTag ...
    let line2 = read_line(lines)?;
    let parts2: Vec<&str> = line2.split_whitespace().collect();
    if parts2.len() < 7 {
        return Err(ParseError::InvalidFormat(format!(
            "Invalid partitioned curve line 2: {}",
            line2
        )));
    }

    let min_x: f64 = parts2[0].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid minX: {}", parts2[0]))
    })?;
    let min_y: f64 = parts2[1].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid minY: {}", parts2[1]))
    })?;
    let min_z: f64 = parts2[2].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid minZ: {}", parts2[2]))
    })?;
    let max_x: f64 = parts2[3].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid maxX: {}", parts2[3]))
    })?;
    let max_y: f64 = parts2[4].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid maxY: {}", parts2[4]))
    })?;
    let max_z: f64 = parts2[5].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid maxZ: {}", parts2[5]))
    })?;
    let num_physical_tags: usize = parts2[6].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid numPhysicalTags: {}", parts2[6]))
    })?;

    let mut physical_tags = Vec::with_capacity(num_physical_tags);
    for i in 0..num_physical_tags {
        if 7 + i >= parts2.len() {
            return Err(ParseError::InvalidFormat(
                "Not enough physical tags".to_string(),
            ));
        }
        physical_tags.push(parts2[7 + i].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid physical tag: {}", parts2[7 + i]))
        })?);
    }

    // Line 3: numBoundingPoints pointTag ...
    let line3 = read_line(lines)?;
    let parts3: Vec<&str> = line3.split_whitespace().collect();
    if parts3.is_empty() {
        return Err(ParseError::InvalidFormat(
            "Missing numBoundingPoints".to_string(),
        ));
    }

    let num_bounding_points: usize = parts3[0].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid numBoundingPoints: {}", parts3[0]))
    })?;

    let mut bounding_points = Vec::with_capacity(num_bounding_points);
    for i in 0..num_bounding_points {
        if 1 + i >= parts3.len() {
            return Err(ParseError::InvalidFormat(
                "Not enough bounding points".to_string(),
            ));
        }
        bounding_points.push(parts3[1 + i].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid bounding point: {}", parts3[1 + i]))
        })?);
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

fn parse_partitioned_surface<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
) -> Result<PartitionedSurface> {
    // Line 1: surfaceTag parentDim parentTag numPartitions partitionTag ...
    let line1 = read_line(lines)?;
    let parts1: Vec<&str> = line1.split_whitespace().collect();
    if parts1.len() < 4 {
        return Err(ParseError::InvalidFormat(format!(
            "Invalid partitioned surface line 1: {}",
            line1
        )));
    }

    let tag: i32 = parts1[0].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid surface tag: {}", parts1[0]))
    })?;
    let parent_dim: i32 = parts1[1].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid parent_dim: {}", parts1[1]))
    })?;
    let parent_tag: i32 = parts1[2].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid parent_tag: {}", parts1[2]))
    })?;
    let num_partitions: usize = parts1[3].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid numPartitions: {}", parts1[3]))
    })?;

    let mut partition_tags = Vec::with_capacity(num_partitions);
    for i in 0..num_partitions {
        if 4 + i >= parts1.len() {
            return Err(ParseError::InvalidFormat(
                "Not enough partition tags".to_string(),
            ));
        }
        partition_tags.push(parts1[4 + i].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid partition tag: {}", parts1[4 + i]))
        })?);
    }

    // Line 2: minX minY minZ maxX maxY maxZ numPhysicalTags physicalTag ...
    let line2 = read_line(lines)?;
    let parts2: Vec<&str> = line2.split_whitespace().collect();
    if parts2.len() < 7 {
        return Err(ParseError::InvalidFormat(format!(
            "Invalid partitioned surface line 2: {}",
            line2
        )));
    }

    let min_x: f64 = parts2[0].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid minX: {}", parts2[0]))
    })?;
    let min_y: f64 = parts2[1].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid minY: {}", parts2[1]))
    })?;
    let min_z: f64 = parts2[2].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid minZ: {}", parts2[2]))
    })?;
    let max_x: f64 = parts2[3].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid maxX: {}", parts2[3]))
    })?;
    let max_y: f64 = parts2[4].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid maxY: {}", parts2[4]))
    })?;
    let max_z: f64 = parts2[5].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid maxZ: {}", parts2[5]))
    })?;
    let num_physical_tags: usize = parts2[6].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid numPhysicalTags: {}", parts2[6]))
    })?;

    let mut physical_tags = Vec::with_capacity(num_physical_tags);
    for i in 0..num_physical_tags {
        if 7 + i >= parts2.len() {
            return Err(ParseError::InvalidFormat(
                "Not enough physical tags".to_string(),
            ));
        }
        physical_tags.push(parts2[7 + i].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid physical tag: {}", parts2[7 + i]))
        })?);
    }

    // Line 3: numBoundingCurves curveTag ...
    let line3 = read_line(lines)?;
    let parts3: Vec<&str> = line3.split_whitespace().collect();
    if parts3.is_empty() {
        return Err(ParseError::InvalidFormat(
            "Missing numBoundingCurves".to_string(),
        ));
    }

    let num_bounding_curves: usize = parts3[0].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid numBoundingCurves: {}", parts3[0]))
    })?;

    let mut bounding_curves = Vec::with_capacity(num_bounding_curves);
    for i in 0..num_bounding_curves {
        if 1 + i >= parts3.len() {
            return Err(ParseError::InvalidFormat(
                "Not enough bounding curves".to_string(),
            ));
        }
        bounding_curves.push(parts3[1 + i].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid bounding curve: {}", parts3[1 + i]))
        })?);
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

fn parse_partitioned_volume<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
) -> Result<PartitionedVolume> {
    // Line 1: volumeTag parentDim parentTag numPartitions partitionTag ...
    let line1 = read_line(lines)?;
    let parts1: Vec<&str> = line1.split_whitespace().collect();
    if parts1.len() < 4 {
        return Err(ParseError::InvalidFormat(format!(
            "Invalid partitioned volume line 1: {}",
            line1
        )));
    }

    let tag: i32 = parts1[0].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid volume tag: {}", parts1[0]))
    })?;
    let parent_dim: i32 = parts1[1].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid parent_dim: {}", parts1[1]))
    })?;
    let parent_tag: i32 = parts1[2].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid parent_tag: {}", parts1[2]))
    })?;
    let num_partitions: usize = parts1[3].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid numPartitions: {}", parts1[3]))
    })?;

    let mut partition_tags = Vec::with_capacity(num_partitions);
    for i in 0..num_partitions {
        if 4 + i >= parts1.len() {
            return Err(ParseError::InvalidFormat(
                "Not enough partition tags".to_string(),
            ));
        }
        partition_tags.push(parts1[4 + i].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid partition tag: {}", parts1[4 + i]))
        })?);
    }

    // Line 2: minX minY minZ maxX maxY maxZ numPhysicalTags physicalTag ...
    let line2 = read_line(lines)?;
    let parts2: Vec<&str> = line2.split_whitespace().collect();
    if parts2.len() < 7 {
        return Err(ParseError::InvalidFormat(format!(
            "Invalid partitioned volume line 2: {}",
            line2
        )));
    }

    let min_x: f64 = parts2[0].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid minX: {}", parts2[0]))
    })?;
    let min_y: f64 = parts2[1].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid minY: {}", parts2[1]))
    })?;
    let min_z: f64 = parts2[2].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid minZ: {}", parts2[2]))
    })?;
    let max_x: f64 = parts2[3].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid maxX: {}", parts2[3]))
    })?;
    let max_y: f64 = parts2[4].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid maxY: {}", parts2[4]))
    })?;
    let max_z: f64 = parts2[5].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid maxZ: {}", parts2[5]))
    })?;
    let num_physical_tags: usize = parts2[6].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid numPhysicalTags: {}", parts2[6]))
    })?;

    let mut physical_tags = Vec::with_capacity(num_physical_tags);
    for i in 0..num_physical_tags {
        if 7 + i >= parts2.len() {
            return Err(ParseError::InvalidFormat(
                "Not enough physical tags".to_string(),
            ));
        }
        physical_tags.push(parts2[7 + i].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid physical tag: {}", parts2[7 + i]))
        })?);
    }

    // Line 3: numBoundingSurfaces surfaceTag ...
    let line3 = read_line(lines)?;
    let parts3: Vec<&str> = line3.split_whitespace().collect();
    if parts3.is_empty() {
        return Err(ParseError::InvalidFormat(
            "Missing numBoundingSurfaces".to_string(),
        ));
    }

    let num_bounding_surfaces: usize = parts3[0].parse().map_err(|_| {
        ParseError::InvalidFormat(format!("Invalid numBoundingSurfaces: {}", parts3[0]))
    })?;

    let mut bounding_surfaces = Vec::with_capacity(num_bounding_surfaces);
    for i in 0..num_bounding_surfaces {
        if 1 + i >= parts3.len() {
            return Err(ParseError::InvalidFormat(
                "Not enough bounding surfaces".to_string(),
            ));
        }
        bounding_surfaces.push(parts3[1 + i].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid bounding surface: {}", parts3[1 + i]))
        })?);
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
