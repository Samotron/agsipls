# Quick Start Guide

## Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd agsipls

# Build the project
cargo build --release

# The binaries will be in target/release/
# - agsipls (CLI)
# - agsi-mcp (MCP server)
# - agsi-lsp (LSP server)
```

## 5-Minute Tutorial

### 1. Create Your First Document

```bash
# Using the CLI
./target/release/agsipls create document \
  --id "MYPROJECT-001" \
  --output my-first-project.agsi.json

# View what was created
cat my-first-project.agsi.json | jq .
```

### 2. Add a Material Interactively

```bash
# Use the interactive form
./target/release/agsipls form material --output material.json

# The form will guide you through:
# - Material ID
# - Material name
# - Material type (Soil, Rock, etc.)
# - Properties (density, strength, etc.)
```

### 3. Validate Your File

```bash
./target/release/agsipls validate material.json --detailed
```

### 4. View Information

```bash
./target/release/agsipls info material.json --materials
```

### 5. Create a Complete Ground Model

```bash
# Use the interactive form for a model
./target/release/agsipls form model --output my-model.json

# Or use the Rust API
cargo run --example create_model
```

## Using the Library in Your Code

Add to your `Cargo.toml`:
```toml
[dependencies]
agsi-core = { path = "path/to/agsipls/crates/agsi-core" }
```

Simple example:
```rust
use agsi_core::{Document, Material, material::MaterialType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a document
    let mut doc = Document::new("PROJECT-001");
    
    // Create a material
    let clay = Material::new("CLAY-01", "London Clay", MaterialType::Soil);
    
    // Save
    doc.to_json_file("output.agsi.json")?;
    
    Ok(())
}
```

## MCP Server Usage

The MCP server enables LLM interaction with AGSi data.

### Starting the Server

```bash
./target/release/agsi-mcp
```

### Example MCP Request

```bash
echo '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "agsi_get_info",
    "arguments": {
      "file_path": "/path/to/file.agsi.json"
    }
  }
}' | ./target/release/agsi-mcp
```

### Available MCP Tools

1. **agsi_validate** - Validate files
2. **agsi_extract_materials** - Get materials from models
3. **agsi_get_info** - Get document info
4. **agsi_query_materials** - Filter materials by type/properties

## Common Workflows

### Creating a Site Investigation Model

```bash
# 1. Create the document
agsipls form document --output site-investigation.agsi.json

# 2. Add materials for each soil/rock type
agsipls form material  # Repeat for each material

# 3. Create the ground model
agsipls form model

# 4. Validate everything
agsipls validate final-document.agsi.json --detailed

# 5. Extract materials for analysis
agsipls extract final-document.agsi.json --output materials.json
```

### Extracting Materials from Existing Files

```bash
# Extract all materials
agsipls extract project.agsi.json --output all-materials.json

# Extract from specific model
agsipls extract project.agsi.json --model STRAT-001 --output strat-materials.json

# View material properties
cat all-materials.json | jq '.[] | {name, type, properties: .properties | length}'
```

### Format Conversion

```bash
# JSON to Avro (compact)
agsipls convert large-file.agsi.json --output compact.avro --format avro

# JSON to Protobuf (efficient)
agsipls convert large-file.agsi.json --output efficient.pb --format protobuf
```

## Tips and Tricks

### Pretty Printing

```bash
# Use jq for nice formatting
agsipls info file.agsi.json | jq .
```

### Validation Pipeline

```bash
# Validate multiple files
for file in *.agsi.json; do
  echo "Validating $file"
  agsipls validate "$file" || echo "FAILED: $file"
done
```

### Material Properties

Common property names:
- `density` (kg/m3)
- `undrained_shear_strength` (kPa)
- `friction_angle` (degrees)
- `cohesion` (kPa)
- `plasticity_index` (%)
- `permeability` (m/s)

### Coordinate Systems

Common CRS values:
- `EPSG:27700` - British National Grid
- `EPSG:4326` - WGS 84 (lat/lon)
- `EPSG:3857` - Web Mercator

## Troubleshooting

### Build Errors

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Validation Errors

```bash
# Get detailed validation report
agsipls validate file.agsi.json --detailed

# Check specific issues
agsipls info file.agsi.json --materials  # Check materials
agsipls info file.agsi.json --models     # Check models
```

### MCP Server Issues

```bash
# Check if server is receiving requests
RUST_LOG=debug ./target/release/agsi-mcp

# Validate JSON-RPC format
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | \
  ./target/release/agsi-mcp | jq .
```

## Next Steps

1. Read the full [README.md](README.md)
2. Check the [ARCHITECTURE.md](ARCHITECTURE.md) for design details
3. Explore the [examples/](examples/) directory
4. Run the test suite: `cargo test`
5. Try the interactive forms: `agsipls form <item>`

## Getting Help

- Check existing documentation
- Run `agsipls --help` or `agsipls <command> --help`
- Look at examples in `examples/`
- Check the tests for usage patterns

## Contributing

See [README.md](README.md) for contribution guidelines.

Happy modeling! 🗺️
