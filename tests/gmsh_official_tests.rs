use gmsh_parser::parse_msh_file;

// Individual tests for each Gmsh tutorial file (t1-t21)
// These tests verify that the parser can handle real-world Gmsh files

#[test]
fn test_tutorial_01() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t1.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_02() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t2.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_03() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t3.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_04() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t4.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_05() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t5.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_06() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t6.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_07() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t7.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_08() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t8.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_09() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t9.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_10() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t10.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_11() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t11.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_12() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t12.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_13() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t13.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_14() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t14.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_15() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t15.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_16() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t16.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_17() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t17.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_18() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t18.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_19() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t19.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_20() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t20.msh")?;
    Ok(())
}

#[test]
fn test_tutorial_21() -> miette::Result<()> {
    parse_msh_file("tests/data/gmsh_official/t21.msh")?;
    Ok(())
}
