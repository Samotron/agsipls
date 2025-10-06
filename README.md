# AGSi Rust Library and CLI

A comprehensive Rust library and **single unified binary** for working with **AGSi** (Association of Geotechnical & Geoenvironmental Specialists interchange format) ground model data.

## 🌟 Features

### Core Library (`agsi-core`)
- **Type-safe data structures** for AGSi components (materials, models, documents)
- **Material-centric design** - materials can be used independently
- **Multiple serialization formats**:
  - JSON (primary format per AGSi spec)
  - Apache Avro (compact binary)
  - Protocol Buffers (efficient binary)
- **Geometry support**:
  - 1D/2D geometries: WKT/WKB text formats
  - 3D surfaces: OBJ format embedded as binary
- **Comprehensive validation** against AGSi v1.0.1 standard
- **Full test coverage** with unit tests

### Unified Binary (`agsipls`) - 3.2MB
**One binary with everything:**
- **CLI commands** - Validate, create, extract, convert, info
- **Interactive forms** - Form-based input for easy data entry
- **MCP server** - Model Context Protocol for LLM integration
- **LSP server** - Language Server Protocol (structure ready)

## 📦 Installation

### From Source

```bash
git clone <repository-url>
cd agsipls
cargo build --release
```

The single `agsipls` binary will be available at `target/release/agsipls` (only 3.2MB!).

### Using Cargo

```bash
cargo install --path crates/agsi
```

## 🚀 Quick Start

### CLI Usage

```bash
# Create a new AGSi document
agsipls create document --id DOC001 --output my-project.agsi.json

# Validate a file
agsipls validate my-project.agsi.json

# Display file information
agsipls info my-project.agsi.json --materials --models

# Extract materials from a model
agsipls extract my-project.agsi.json --model MODEL001 --output materials.json

# Interactive form-based creation
agsipls form material --output my-material.agsi.json
agsipls form model --output my-model.agsi.json
agsipls form document

# Convert between formats
agsipls convert input.agsi.json --output output.avro --format avro
```

### MCP Server

```bash
# Run the MCP server (JSON-RPC over stdin/stdout)
agsipls mcp

# With debug output
agsipls mcp --debug

# Test it
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | agsipls mcp
```

### LSP Server

```bash
# Run the LSP server (stdio)
agsipls lsp

# With TCP port
agsipls lsp --port 9257
```

### Library Usage

```rust
use agsi_core::{
    Document, Material, MaterialProperty, GroundModel,
    material::{MaterialType, PropertySource},
    model::{ModelType, ModelDimension, ComponentType, ModelComponent},
    geometry::Geometry,
    validation,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new document
    let mut doc = Document::new("PROJECT001")
        .with_file_name("site-investigation.agsi.json")
        .with_author("Engineering Team");

    // Create a material
    let material = Material::new("MAT001", "Dense Sand", MaterialType::Soil)
        .with_description("Medium to coarse sand, dense")
        .with_property(
            MaterialProperty::numeric("density", 1900.0, Some("kg/m3".to_string()))
                .with_source(PropertySource::Tested)
        )
        .with_property(
            MaterialProperty::numeric("friction_angle", 35.0, Some("degrees".to_string()))
                .with_source(PropertySource::Tested)
        );

    // Create a ground model
    let mut model = GroundModel::new(
        "MODEL001",
        "Site Stratigraphy",
        ModelType::Stratigraphic,
        ModelDimension::TwoD,
    ).with_crs("EPSG:27700");

    // Add material to model
    model.add_material(material);

    // Create a layer component
    let layer = ModelComponent::new(
        "LAYER001",
        "Sand Layer",
        ComponentType::Layer,
        "MAT001",
        Geometry::polygon(
            vec![
                [0.0, 0.0, -5.0],
                [100.0, 0.0, -5.0],
                [100.0, 100.0, -10.0],
                [0.0, 100.0, -10.0],
                [0.0, 0.0, -5.0],
            ],
            vec![],
        )?,
    ).with_elevations(-5.0, -10.0);

    model.add_component(layer);

    // Add model to document
    doc.add_model(model);

    // Validate
    let validation_result = validation::validate_document(&doc)?;
    if validation_result.is_valid() {
        println!("✅ Document is valid");
    }

    // Save to file
    doc.to_json_file("output.agsi.json")?;

    Ok(())
}
```

### Working with Materials Independently

Materials are first-class citizens and can be used independently:

```rust
use agsi_core::{Material, MaterialProperty, material::{MaterialType, PropertySource}};

let clay = Material::new("CLAY001", "London Clay", MaterialType::Soil)
    .with_property(
        MaterialProperty::numeric("undrained_shear_strength", 75.0, Some("kPa".to_string()))
            .with_source(PropertySource::Tested)
            .with_method("Triaxial UU test")
    )
    .with_property(
        MaterialProperty::range("plasticity_index", 30.0, 50.0, Some("%".to_string()))
    );

// Serialize just the material
let json = serde_json::to_string_pretty(&clay)?;
```

## 📚 Documentation

- **AGSi Standard**: https://ags-data-format-wg.gitlab.io/agsi/AGSi_Documentation/
- **AGSi Schema**: v1.0.1 (JSON Schema draft 2020-12)
- **API Documentation**: Run `cargo doc --open`

## 🏗️ Architecture

```
agsipls/
├── crates/
│   ├── agsi-core/        # Core library with data structures
│   ├── agsi-cli/         # Command-line interface
│   ├── agsi-mcp/         # MCP server
│   └── agsi-lsp/         # LSP server
├── Cargo.toml            # Workspace configuration
└── README.md
```

### Core Design Principles

1. **Material Independence**: Materials can exist and be used without being part of a model
2. **Geometry Embedding**: Geometries are embedded as binary data in the file
   - 1D/2D: WKT text format, optional WKB binary
   - 3D surfaces: OBJ format as base64-encoded binary
3. **Multi-format Support**: JSON for interoperability, Avro/Protobuf for efficiency
4. **Validation First**: Comprehensive validation against AGSi schema
5. **Type Safety**: Leverage Rust's type system for correctness

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test --package agsi-core

# Run with output
cargo test -- --nocapture
```

## 🛠️ Development

### Build

```bash
# Build all crates
cargo build

# Build in release mode
cargo build --release

# Build specific crate
cargo build --package agsi-cli
```

### Lint and Format

```bash
# Check formatting
cargo fmt --check

# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings
```

## 📝 Examples

### Create a Complete Site Model

```bash
# 1. Create the document
agsipls form document

# 2. Add materials
agsipls form material --output materials.json

# 3. Create a model
agsipls form model --output model.json

# 4. Validate everything
agsipls validate final-document.agsi.json --detailed
```

### Extract and Analyze Materials

```bash
# Extract all materials from a model
agsipls extract project.agsi.json --model STRAT_MODEL --output materials.json

# View material properties
cat materials.json | jq '.[] | {id, name, type, properties: .properties | length}'
```

## 🗺️ Roadmap

- [x] Core library with data structures
- [x] JSON serialization/deserialization
- [x] Validation framework
- [x] CLI with basic commands
- [x] Interactive form-based input
- [ ] Complete Avro serialization
- [ ] Complete Protobuf serialization
- [ ] MCP server implementation
- [ ] LSP server implementation
- [ ] TUI editor with ratatui
- [ ] Geometry visualization
- [ ] Advanced validation rules
- [ ] Performance benchmarks

## 🤝 Contributing

Contributions are welcome! Please ensure:

1. Tests pass: `cargo test`
2. Code is formatted: `cargo fmt`
3. No clippy warnings: `cargo clippy`
4. Documentation is updated

## 📄 License

MIT OR Apache-2.0

## 🙏 Acknowledgments

- AGS Data Format Working Group for the AGSi standard
- The Rust community for excellent crates and tools

## 📧 Contact

For questions or issues, please open an issue on the repository.
