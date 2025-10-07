# AGSi Rust Implementation - Technical Documentation

## Overview

This project provides a complete Rust implementation of the AGSi (Association of Geotechnical & Geoenvironmental Specialists interchange format) standard for ground model and interpreted geotechnical data exchange.

## Architecture

### Component Overview

```
┌─────────────────────────────────────────────────────────────┐
│                        User Applications                      │
│  (Scripts, LLMs via MCP, IDEs via LSP, Terminal via CLI)   │
└────────────────┬────────────────────────────────────────────┘
                 │
    ┌────────────┴────────────┬──────────────┬─────────────┐
    │                         │              │             │
┌───▼────┐              ┌─────▼──────┐  ┌───▼────┐  ┌────▼────┐
│agsi-cli│              │  agsi-mcp  │  │agsi-lsp│  │User Code│
│        │              │  (Server)  │  │(Server)│  │         │
└───┬────┘              └─────┬──────┘  └───┬────┘  └────┬────┘
    │                         │              │            │
    └─────────────────────────┴──────────────┴────────────┘
                              │
                    ┌─────────▼──────────┐
                    │    agsi-core       │
                    │  (Core Library)    │
                    │                    │
                    │  • Data Structures │
                    │  • Serialization   │
                    │  • Validation      │
                    │  • Geometry        │
                    └────────────────────┘
```

### Core Library (`agsi-core`)

The foundation of the project, providing:

#### Data Structures

1. **Document** (`document.rs`)
   - Top-level AGSi document container
   - Schema and file metadata
   - Project information
   - Collection of ground models

2. **Material** (`material.rs`)
   - Independent material definitions
   - Material types (Soil, Rock, Fill, etc.)
   - Properties with values, units, and sources
   - Can exist without being part of a model

3. **Ground Model** (`model.rs`)
   - Stratigraphic, structural, or other model types
   - 1D, 2D, or 3D representations
   - Collection of materials and components
   - Spatial extent definition

4. **Geometry** (`geometry.rs`)
   - Point, LineString, Polygon geometries
   - 3D surface support via OBJ format
   - WKT/WKB text representations
   - Coordinate reference system support

5. **Project** (`project.rs`)
   - Project metadata
   - Client and contractor information
   - Location data

#### Serialization (`serialization.rs`)

Supports multiple formats:
- **JSON**: Primary format per AGSi specification
- **Apache Avro**: Compact binary format (planned)
- **Protocol Buffers**: Efficient binary format (planned)

Geometry embedding strategy:
- **1D/2D geometries**: WKT (Well-Known Text) as embedded text, optional WKB (Well-Known Binary) as base64
- **3D surfaces**: OBJ format encoded as base64 binary within the JSON

#### Validation (`validation.rs`)

Comprehensive validation including:
- Schema structure validation
- Required field checks
- Reference integrity (material IDs, model IDs)
- Value range validation
- Duplicate detection
- Extent consistency checks

### CLI Application (`agsi-cli`)

Terminal-based interface with commands:

#### Core Commands

1. **validate** - Validate AGSi files
   ```bash
   agsipls validate file.agsi.json --detailed
   ```

2. **create** - Create documents, materials, or models
   ```bash
   agsipls create document --id DOC001 --output doc.json
   agsipls create material --id MAT001 --name "Clay" --output mat.json
   ```

3. **info** - Display document information
   ```bash
   agsipls info file.agsi.json --materials --models
   ```

4. **extract** - Extract materials from models
   ```bash
   agsipls extract file.agsi.json --model MODEL001 --output materials.json
   ```

5. **convert** - Format conversion
   ```bash
   agsipls convert input.json --output output.avro --format avro
   ```

6. **form** - Interactive form-based creation
   ```bash
   agsipls form material
   agsipls form model
   agsipls form document
   ```

### MCP Server (`agsi-mcp`)

Model Context Protocol server for LLM integration.

#### Available Tools

1. **agsi_validate**
   - Input: file_path
   - Output: Validation result with errors and warnings

2. **agsi_extract_materials**
   - Input: file_path, model_id (optional)
   - Output: Array of materials with properties

3. **agsi_get_info**
   - Input: file_path
   - Output: Document metadata and structure

4. **agsi_query_materials**
   - Input: file_path, material_type (optional), property_name (optional)
   - Output: Filtered materials matching criteria

#### Usage

The MCP server communicates via JSON-RPC 2.0 over stdin/stdout:

```bash
agsi-mcp < requests.jsonl > responses.jsonl
```

Example request:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "agsi_extract_materials",
    "arguments": {
      "file_path": "/path/to/file.agsi.json"
    }
  }
}
```

### LSP Server (`agsi-lsp`)

Language Server Protocol implementation for editor integration (planned).

Features:
- Syntax validation on-the-fly
- Auto-completion for material types, properties
- Hover documentation
- Go-to-definition for material references
- Diagnostics for validation errors

## Data Model Design

### Material Independence

Materials are designed as first-class, independent entities:

```rust
// Create a material without a model
let material = Material::new("MAT001", "Clay", MaterialType::Soil)
    .with_property(
        MaterialProperty::numeric("cohesion", 50.0, Some("kPa".to_string()))
    );

// Serialize independently
let json = serde_json::to_string(&material)?;
```

This allows:
- Material libraries to be shared across projects
- Reusable material definitions
- Independent material databases

### Geometry Embedding

Geometries are embedded directly in the JSON file as binary data:

```json
{
  "type": "LineString",
  "coordinates": [[0, 0, 10], [100, 0, 10]],
  "wkt": "LINESTRING (0 0, 100 0)",
  "wkb": "base64encodeddata..."
}
```

For 3D surfaces:
```json
{
  "type": "Surface",
  "objData": "base64_encoded_obj_file_content",
  "metadata": {
    "vertexCount": 1024,
    "faceCount": 2048
  }
}
```

This approach:
- Keeps all data in a single file
- Avoids external file dependencies
- Maintains compatibility with AGSi JSON schema
- Allows efficient binary storage within JSON

## Validation Strategy

Multi-level validation approach:

1. **Structural Validation** (via `validator` crate)
   - Field presence and types
   - String length constraints
   - Basic format checks

2. **Semantic Validation** (custom logic)
   - Reference integrity
   - Value ranges and bounds
   - Cross-field consistency

3. **Schema Validation** (via `jsonschema` crate)
   - Against official AGSi JSON Schema
   - Ensures specification compliance

## Performance Considerations

### Serialization

- JSON: Human-readable, debuggable, spec-compliant
- Avro: ~40% size reduction, faster parsing
- Protobuf: ~60% size reduction, fastest parsing

### Caching

The MCP server caches loaded documents to avoid repeated parsing:
```rust
documents: HashMap<String, Document>
```

### Geometry Optimization

- WKT for simple geometries (human-readable)
- WKB for complex geometries (binary, compact)
- OBJ with binary encoding for 3D surfaces

## Testing

### Unit Tests

Each module has comprehensive unit tests:
- `material.rs`: 2 tests covering creation and properties
- `geometry.rs`: 4 tests covering all geometry types
- `model.rs`: 3 tests covering model creation and extent
- `document.rs`: 3 tests covering serialization
- `validation.rs`: 3 tests covering validation scenarios

Run tests:
```bash
cargo test                    # All tests
cargo test --package agsi-core  # Core library only
cargo test -- --nocapture      # With output
```

### Integration Tests

Located in `examples/`:
- `create_model.rs`: Complete ground model creation
- Tests end-to-end workflow from creation to validation

### Property-Based Testing

Uses `proptest` for generative testing (planned):
- Random document generation
- Serialization round-trip testing
- Validation consistency checks

## Future Enhancements

### Short Term

1. Complete Avro serialization implementation
2. Complete Protobuf serialization with .proto files
3. LSP server implementation
4. TUI editor with ratatui
5. More comprehensive validation rules

### Medium Term

1. Geometry visualization (2D/3D rendering)
2. Performance benchmarks
3. Streaming parser for large files
4. Python bindings via PyO3
5. WASM compilation for web use

### Long Term

1. Real-time collaboration features
2. Version control integration
3. Cloud storage adapters
4. Machine learning integration for material classification
5. GIS system integration

## Contributing

See main README.md for contribution guidelines.

## References

- AGSi Documentation: https://ags-data-format-wg.gitlab.io/agsi/
- AGSi Standard: v1.0.1
- Rust Edition: 2021
- MSRV: 1.75

## License

MIT OR Apache-2.0
