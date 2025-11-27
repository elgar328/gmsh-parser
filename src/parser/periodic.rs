//! Parser for $Periodic section

use std::io::BufReader;
use std::io::Lines;

use crate::error::{ParseError, Result};
use crate::types::{Mesh, PeriodicLink};

use super::{read_line, expect_end_marker};

pub fn parse<R: std::io::Read>(
    lines: &mut Lines<BufReader<R>>,
    mesh: &mut Mesh,
) -> Result<()> {
    // Read number of periodic links
    let header = read_line(lines)?;
    let num_periodic_links: usize = header
        .parse()
        .map_err(|_| ParseError::InvalidFormat(format!("Invalid numPeriodicLinks: {}", header)))?;

    for _ in 0..num_periodic_links {
        // Read entity info: entityDim entityTag entityTagMaster
        let entity_line = read_line(lines)?;
        let parts: Vec<&str> = entity_line.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(ParseError::InvalidFormat(format!(
                "Invalid periodic entity line: {}",
                entity_line
            )));
        }

        let entity_dim: i32 = parts[0].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid entity_dim: {}", parts[0]))
        })?;
        let entity_tag: i32 = parts[1].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid entity_tag: {}", parts[1]))
        })?;
        let entity_tag_master: i32 = parts[2].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid entity_tag_master: {}", parts[2]))
        })?;

        // Read affine transform
        let affine_line = read_line(lines)?;
        let affine_parts: Vec<&str> = affine_line.split_whitespace().collect();
        if affine_parts.is_empty() {
            return Err(ParseError::InvalidFormat(
                "Missing numAffine value".to_string(),
            ));
        }

        let num_affine: usize = affine_parts[0].parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid numAffine: {}", affine_parts[0]))
        })?;

        let mut affine_transform = Vec::with_capacity(num_affine);
        for i in 1..=num_affine {
            if i >= affine_parts.len() {
                return Err(ParseError::InvalidFormat(
                    "Not enough affine transform values".to_string(),
                ));
            }
            let value: f64 = affine_parts[i].parse().map_err(|_| {
                ParseError::InvalidFormat(format!("Invalid affine value: {}", affine_parts[i]))
            })?;
            affine_transform.push(value);
        }

        // Read node correspondences
        let corr_line = read_line(lines)?;
        let num_corresponding_nodes: usize = corr_line.parse().map_err(|_| {
            ParseError::InvalidFormat(format!("Invalid numCorrespondingNodes: {}", corr_line))
        })?;

        let mut node_correspondences = Vec::with_capacity(num_corresponding_nodes);
        for _ in 0..num_corresponding_nodes {
            let node_line = read_line(lines)?;
            let node_parts: Vec<&str> = node_line.split_whitespace().collect();
            if node_parts.len() < 2 {
                return Err(ParseError::InvalidFormat(format!(
                    "Invalid node correspondence line: {}",
                    node_line
                )));
            }

            let node_tag: usize = node_parts[0].parse().map_err(|_| {
                ParseError::InvalidFormat(format!("Invalid node_tag: {}", node_parts[0]))
            })?;
            let node_tag_master: usize = node_parts[1].parse().map_err(|_| {
                ParseError::InvalidFormat(format!("Invalid node_tag_master: {}", node_parts[1]))
            })?;

            node_correspondences.push((node_tag, node_tag_master));
        }

        mesh.periodic_links.push(PeriodicLink {
            entity_dim,
            entity_tag,
            entity_tag_master,
            affine_transform,
            node_correspondences,
        });
    }

    expect_end_marker(lines, "Periodic")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, Cursor};

    #[test]
    fn test_parse_periodic() {
        let data = r#"1
2 1 2
0
2
1 2
3 4
$EndPeriodic
"#;
        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        let mut lines = reader.lines();
        let mut mesh = Mesh::default();

        parse(&mut lines, &mut mesh).unwrap();

        assert_eq!(mesh.periodic_links.len(), 1);
        let link = &mesh.periodic_links[0];
        assert_eq!(link.entity_dim, 2);
        assert_eq!(link.entity_tag, 1);
        assert_eq!(link.entity_tag_master, 2);
        assert_eq!(link.affine_transform.len(), 0);
        assert_eq!(link.node_correspondences.len(), 2);
        assert_eq!(link.node_correspondences[0], (1, 2));
        assert_eq!(link.node_correspondences[1], (3, 4));
    }
}
