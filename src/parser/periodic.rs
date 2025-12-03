//! Parser for $Periodic section

use crate::error::Result;
use crate::types::{Mesh, PeriodicLink};

use super::LineReader;

pub fn parse(reader: &mut LineReader, mesh: &mut Mesh) -> Result<()> {
    // Read number of periodic links
    let token_line = reader.read_token_line()?;
    let mut iter = token_line.iter();
    let num_periodic_links = iter.parse_usize("numPeriodicLinks")?;
    iter.expect_no_more()?;

    for _ in 0..num_periodic_links {
        // Read entity info: entityDim entityTag entityTagMaster
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();

        let entity_dim = iter.parse_entity_dimension("entityDim")?;
        let entity_tag = iter.parse_int("entityTag")?;
        let entity_tag_master = iter.parse_int("entityTagMaster")?;
        iter.expect_no_more()?;

        // Read affine transform
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();

        let num_affine = iter.parse_usize("numAffine")?;
        let affine_transform = iter.parse_floats(num_affine, "affineValue")?;
        iter.expect_no_more()?;

        // Read node correspondences
        let token_line = reader.read_token_line()?;
        let mut iter = token_line.iter();
        let num_corresponding_nodes = iter.parse_usize("numCorrespondingNodes")?;
        iter.expect_no_more()?;

        let mut node_correspondences = Vec::with_capacity(num_corresponding_nodes);
        for _ in 0..num_corresponding_nodes {
            let token_line = reader.read_token_line()?;
            let mut iter = token_line.iter();

            let node_tag = iter.parse_usize("nodeTag")?;
            let node_tag_master = iter.parse_usize("nodeTagMaster")?;
            iter.expect_no_more()?;

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
        let mut mesh = Mesh::dummy();

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
