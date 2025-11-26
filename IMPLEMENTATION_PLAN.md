# Gmsh MSH 4.1 Parser Implementation Plan

## 1. Overview

Rust library for parsing Gmsh MSH 4.1 format files (ASCII only). The parser supports entity-based mesh organization introduced in MSH 4.1.

### Supported Sections
- `$MeshFormat` - File format version and metadata
- `$PhysicalNames` - Physical group names
- `$Entities` - Geometric entities (points, curves, surfaces, volumes)
- `$Nodes` - Mesh nodes organized by entity blocks
- `$Elements` - Mesh elements organized by entity blocks

### Supported Element Types
- 1-31: Standard element types (lines, triangles, quads, tets, hexes, prisms, pyramids)
- 92-93: Higher-order hexahedra

### Future Consideration
- `$Periodic` - Periodicity relations
- `$NodeData` - Node-based post-processing data
- `$ElementData` - Element-based post-processing data
- `$ElementNodeData` - Element-node-based post-processing data

## 2. Data Structure Design

### 2.1 Entity-Based Organization

MSH 4.1 uses an entity-based structure where:
- Entities represent geometric features (points, curves, surfaces, volumes)
- Nodes are grouped into blocks, each associated with an entity
- Elements are grouped into blocks, each associated with an entity

### 2.2 Core Data Structures

```rust
// File format metadata
struct MeshFormat {
    version: f64,           // Should be 4.1
    file_type: i32,         // 0 for ASCII
    data_size: i32,         // sizeof(size_t)
}

// Physical group names
struct PhysicalName {
    dimension: i32,
    physical_tag: i32,
    name: String,
}

// Entity types
enum EntityDimension {
    Point = 0,
    Curve = 1,
    Surface = 2,
    Volume = 3,
}

// Point entity
struct PointEntity {
    tag: i32,
    x: f64,
    y: f64,
    z: f64,
    physical_tags: Vec<i32>,
}

// Curve entity
struct CurveEntity {
    tag: i32,
    min_x: f64, min_y: f64, min_z: f64,
    max_x: f64, max_y: f64, max_z: f64,
    physical_tags: Vec<i32>,
    bounding_points: Vec<i32>,  // Sign encodes orientation
}

// Surface entity
struct SurfaceEntity {
    tag: i32,
    min_x: f64, min_y: f64, min_z: f64,
    max_x: f64, max_y: f64, max_z: f64,
    physical_tags: Vec<i32>,
    bounding_curves: Vec<i32>,  // Sign encodes orientation
}

// Volume entity
struct VolumeEntity {
    tag: i32,
    min_x: f64, min_y: f64, min_z: f64,
    max_x: f64, max_y: f64, max_z: f64,
    physical_tags: Vec<i32>,
    bounding_surfaces: Vec<i32>,  // Sign encodes orientation
}

// All entities
struct Entities {
    points: HashMap<i32, PointEntity>,
    curves: HashMap<i32, CurveEntity>,
    surfaces: HashMap<i32, SurfaceEntity>,
    volumes: HashMap<i32, VolumeEntity>,
}

// Node representation
struct Node {
    tag: usize,
    x: f64,
    y: f64,
    z: f64,
    // Optional parametric coordinates (if entity is parametric)
    u: Option<f64>,
    v: Option<f64>,
    w: Option<f64>,
}

// Node block (grouped by entity)
struct NodeBlock {
    entity_dim: i32,
    entity_tag: i32,
    parametric: bool,
    nodes: Vec<Node>,
}

// Element types (1-31, 92-93)
#[repr(i32)]
enum ElementType {
    Line2 = 1,
    Triangle3 = 2,
    Quadrangle4 = 3,
    Tetrahedron4 = 4,
    Hexahedron8 = 5,
    Prism6 = 6,
    Pyramid5 = 7,
    Line3 = 8,
    Triangle6 = 9,
    Quadrangle9 = 10,
    Tetrahedron10 = 11,
    Hexahedron27 = 12,
    Prism18 = 13,
    Pyramid14 = 14,
    Point = 15,
    Quadrangle8 = 16,
    Hexahedron20 = 17,
    Prism15 = 18,
    Pyramid13 = 19,
    Triangle9 = 20,
    Triangle10 = 21,
    Triangle12 = 22,
    Triangle15a = 23,
    Triangle15b = 24,
    Triangle21 = 25,
    Line4 = 26,
    Line5 = 27,
    Line6 = 28,
    Tetrahedron20 = 29,
    Tetrahedron35 = 30,
    Tetrahedron56 = 31,
    Hexahedron64 = 92,
    Hexahedron125 = 93,
}

// Element representation
struct Element {
    tag: usize,
    element_type: ElementType,
    node_tags: Vec<usize>,
}

// Element block (grouped by entity)
struct ElementBlock {
    entity_dim: i32,
    entity_tag: i32,
    element_type: ElementType,
    elements: Vec<Element>,
}

// Top-level mesh structure
struct Mesh {
    format: MeshFormat,
    physical_names: HashMap<(i32, i32), String>,  // (dim, tag) -> name
    entities: Entities,
    node_blocks: Vec<NodeBlock>,
    element_blocks: Vec<ElementBlock>,

    // Optional: Flat access structures for performance
    nodes_by_tag: HashMap<usize, Node>,
    elements_by_tag: HashMap<usize, Element>,
}
```

### 2.3 Design Rationale

**Hybrid Structure:**
- Entity-based blocks preserve file structure and entity relationships
- Flat lookup maps (`nodes_by_tag`, `elements_by_tag`) provide O(1) access
- Users can choose iteration by entity or direct lookup by tag

**Trade-offs:**
- Memory: Some duplication (nodes stored in blocks and flat map)
- Flexibility: Easy to query by entity or by tag
- Future-proof: Can add entity-specific queries later

## 3. Parser Architecture

### 3.1 Module Structure

```
src/
├── lib.rs              # Public API
├── parser/
│   ├── mod.rs          # Parser orchestration
│   ├── mesh_format.rs  # $MeshFormat parser
│   ├── physical_names.rs # $PhysicalNames parser
│   ├── entities.rs     # $Entities parser
│   ├── nodes.rs        # $Nodes parser
│   └── elements.rs     # $Elements parser
├── types/
│   ├── mod.rs
│   ├── mesh.rs         # Main Mesh struct
│   ├── entity.rs       # Entity types
│   ├── node.rs         # Node types
│   └── element.rs      # Element types
├── error.rs            # Error types
└── utils.rs            # Helper functions
```

### 3.2 Parser Strategy

1. **Line-by-line parsing:** Use `BufReader` for efficient file reading
2. **Section detection:** Look for `$SectionName` markers
3. **Section routing:** Route to appropriate section parser
4. **Validation:** Validate section order and data integrity

### 3.3 Parser Flow

```rust
pub fn parse_msh_file(path: &Path) -> Result<Mesh, ParseError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut mesh = Mesh::default();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line?;
        match line.trim() {
            "$MeshFormat" => parse_mesh_format(&mut lines, &mut mesh)?,
            "$PhysicalNames" => parse_physical_names(&mut lines, &mut mesh)?,
            "$Entities" => parse_entities(&mut lines, &mut mesh)?,
            "$Nodes" => parse_nodes(&mut lines, &mut mesh)?,
            "$Elements" => parse_elements(&mut lines, &mut mesh)?,
            _ => skip_unknown_section(&mut lines)?,
        }
    }

    mesh.build_indices()?;  // Build flat lookup maps
    Ok(mesh)
}
```

## 4. Implementation Phases

### Phase 1: Foundation (Week 1)
- [ ] Set up project structure
- [ ] Define core data structures
- [ ] Implement error types
- [ ] Write basic file I/O skeleton

### Phase 2: Basic Parsing (Week 2)
- [ ] Implement `$MeshFormat` parser
- [ ] Implement `$PhysicalNames` parser
- [ ] Add section detection and routing
- [ ] Write unit tests for basic sections

### Phase 3: Entity Parsing (Week 3)
- [ ] Implement `$Entities` parser
  - [ ] Point entities
  - [ ] Curve entities
  - [ ] Surface entities
  - [ ] Volume entities
- [ ] Write unit tests with sample entities

### Phase 4: Node Parsing (Week 4)
- [ ] Implement `$Nodes` parser
  - [ ] Parse entity blocks
  - [ ] Handle parametric coordinates
  - [ ] Build node lookup map
- [ ] Write unit tests with various node configurations

### Phase 5: Element Parsing (Week 5)
- [ ] Implement element type enum (1-31, 92-93)
- [ ] Implement `$Elements` parser
  - [ ] Parse entity blocks
  - [ ] Handle different element types
  - [ ] Build element lookup map
- [ ] Write unit tests for all supported element types

### Phase 6: Integration & Testing (Week 6)
- [ ] Integration tests with real MSH files
- [ ] Performance benchmarking
- [ ] Documentation
- [ ] Example programs

### Phase 7: Polish (Week 7)
- [ ] Error message improvements
- [ ] API refinement
- [ ] Code review and refactoring
- [ ] Prepare for release

## 5. Testing Strategy

### 5.1 Unit Tests
- Test each section parser independently
- Test edge cases (empty sections, single elements, etc.)
- Test error handling (malformed input, invalid tags, etc.)

### 5.2 Integration Tests
- Use real MSH 4.1 files from Gmsh
- Test files with different entity types
- Test files with various element types
- Test large files for performance

### 5.3 Test Data

Create minimal test files:
```
tests/
├── data/
│   ├── minimal.msh          # Simplest valid file
│   ├── triangles.msh        # 2D triangular mesh
│   ├── tetrahedra.msh       # 3D tetrahedral mesh
│   ├── mixed_elements.msh   # Multiple element types
│   ├── higher_order.msh     # Higher-order elements
│   └── physical_groups.msh  # With physical names
```

### 5.4 Reference Implementation

Compare results with Python gmshparser library when possible.

## 6. Error Handling

```rust
#[derive(Debug)]
pub enum ParseError {
    IoError(std::io::Error),
    InvalidFormat(String),
    UnsupportedVersion(f64),
    UnsupportedFileType(i32),
    InvalidSection(String),
    InvalidEntityDimension(i32),
    InvalidElementType(i32),
    MissingSection(String),
    InvalidData(String),
    DuplicateTag(String),
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IoError(err)
    }
}
```

## 7. API Design

### 7.1 Public API

```rust
// Main parsing function
pub fn parse_msh_file(path: &Path) -> Result<Mesh, ParseError>;

// Mesh queries
impl Mesh {
    pub fn get_node(&self, tag: usize) -> Option<&Node>;
    pub fn get_element(&self, tag: usize) -> Option<&Element>;

    pub fn nodes_in_entity(&self, dim: i32, tag: i32) -> Vec<&Node>;
    pub fn elements_in_entity(&self, dim: i32, tag: i32) -> Vec<&Element>;

    pub fn nodes_iter(&self) -> impl Iterator<Item = &Node>;
    pub fn elements_iter(&self) -> impl Iterator<Item = &Element>;

    pub fn physical_name(&self, dim: i32, tag: i32) -> Option<&str>;
}
```

### 7.2 Example Usage

```rust
use gmsh_parser::parse_msh_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mesh = parse_msh_file("example.msh")?;

    println!("Version: {}", mesh.format.version);
    println!("Total nodes: {}", mesh.nodes_by_tag.len());
    println!("Total elements: {}", mesh.elements_by_tag.len());

    // Iterate by entity
    for block in &mesh.element_blocks {
        println!("Entity {}-{}: {} elements",
            block.entity_dim, block.entity_tag, block.elements.len());
    }

    // Direct lookup
    if let Some(node) = mesh.get_node(1) {
        println!("Node 1: ({}, {}, {})", node.x, node.y, node.z);
    }

    Ok(())
}
```

## 8. Performance Considerations

### 8.1 Memory Efficiency
- Use `Vec` instead of `HashMap` when tags are continuous (check min/max tags)
- Option to skip building flat maps if not needed
- Lazy evaluation for expensive queries

### 8.2 Parsing Speed
- Use `BufReader` with appropriate buffer size
- Minimize allocations during parsing
- Consider parallel parsing for large files (future)

### 8.3 Benchmarks
- Parse time vs file size
- Memory usage vs mesh complexity
- Query performance (entity-based vs tag-based)

## 9. Documentation

- [ ] API documentation with rustdoc
- [ ] Usage examples
- [ ] MSH 4.1 format reference
- [ ] Migration guide from Python gmshparser
- [ ] Contributing guidelines

## 10. Future Enhancements

### 10.1 Near-term
- Binary MSH 4.1 support
- `$Periodic` section parsing
- Post-processing data sections (`$NodeData`, etc.)

### 10.2 Long-term
- MSH file writing
- Mesh manipulation utilities
- Integration with FEM libraries
- Python bindings (PyO3)

## 11. Dependencies

```toml
[dependencies]
# Minimal dependencies for parsing
thiserror = "1.0"  # Error handling

[dev-dependencies]
criterion = "0.5"   # Benchmarking
tempfile = "3.0"    # Testing
```

## 12. Success Criteria

- [ ] Parse all test MSH 4.1 files correctly
- [ ] Match Python gmshparser output (where applicable)
- [ ] Handle files up to 1GB in reasonable time (<1min)
- [ ] Clean, documented API
- [ ] >80% test coverage
- [ ] Zero unsafe code
