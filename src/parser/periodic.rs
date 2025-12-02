//! Parser for $Periodic section

use crate::error::Result;
use crate::types::{Mesh, PeriodicLink};

use super::LineReader;

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    // Read number of periodic links
    let token_line = reader.read_token_line()?;
    let num_periodic_links = token_line.tokens[0].parse_usize("numPeriodicLinks")?;

    for _ in 0..num_periodic_links {
        // Read entity info: entityDim entityTag entityTagMaster
        let token_line = reader.read_token_line()?;
        token_line.expect_len(3)?;

        let entity_dim = token_line.tokens[0].parse_entity_dimension("entityDim")?;
        let entity_tag = token_line.tokens[1].parse_int("entityTag")?;
        let entity_tag_master = token_line.tokens[2].parse_int("entityTagMaster")?;

        // Read affine transform
        let token_line = reader.read_token_line()?;

        let num_affine = token_line.tokens[0].parse_usize("numAffine")?;

        token_line.expect_min_len(1 + num_affine)?;
        let mut affine_transform = Vec::with_capacity(num_affine);
        for j in 0..num_affine {
            let value = token_line.tokens[1 + j].parse_float("affineValue")?;
            affine_transform.push(value);
        }

        // Read node correspondences
        let token_line = reader.read_token_line()?;
        let num_corresponding_nodes = token_line.tokens[0].parse_usize("numCorrespondingNodes")?;

        let mut node_correspondences = Vec::with_capacity(num_corresponding_nodes);
        for _ in 0..num_corresponding_nodes {
            let token_line = reader.read_token_line()?;
            token_line.expect_len(2)?;

            let node_tag = token_line.tokens[0].parse_usize("nodeTag")?;
            let node_tag_master = token_line.tokens[1].parse_usize("nodeTagMaster")?;

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

    let token_line = reader.read_token_line()?;
    token_line.expect_end_marker("Periodic")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::super::*;
    use super::super::*;
    use super::*;

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

        let source_file = SourceFile::new(data.into());
        let mut reader = LineReader::new(source_file);
        let mut mesh = Mesh::default();

        parse(&mut reader, &mut mesh).unwrap();

        assert_eq!(mesh.periodic_links.len(), 1);
        let link = &mesh.periodic_links[0];
        assert_eq!(link.entity_dim, EntityDimension::Surface);
        assert_eq!(link.entity_tag, 1);
        assert_eq!(link.entity_tag_master, 2);
        assert_eq!(link.affine_transform.len(), 0);
        assert_eq!(link.node_correspondences.len(), 2);
        assert_eq!(link.node_correspondences[0], (1, 2));
        assert_eq!(link.node_correspondences[1], (3, 4));
    }
}
