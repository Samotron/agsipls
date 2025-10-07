# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-10-06

### Added

#### Core Library (`agsi-core`)
- Initial implementation of AGSi v1.0.1 data structures
- `Document` type for top-level AGSi documents
- `Material` type with properties and metadata
- `GroundModel` type with components and spatial extent
- `Geometry` types: Point, LineString, Polygon, Surface (OBJ)
- `Project` type for project metadata
- `ModelComponent` for model building blocks
- JSON serialization/deserialization support
- Comprehensive validation framework
- WKT/WKB support for 1D/2D geometries
- OBJ format support for 3D surfaces (base64 embedded)
- Error handling with custom error types
- 17 unit tests covering all major functionality

#### CLI Application (`agsi-cli`)
- `validate` command for file validation
- `create` command for documents, materials, and models
- `info` command for displaying document information
- `extract` command for extracting materials from models
- `convert` command for format conversion
- `form` command for interactive form-based creation
- `edit` command for editing files (basic)
- Beautiful terminal output with emojis and formatting
- Async/await support with tokio
- Interactive forms using inquire crate

#### MCP Server (`agsi-mcp`)
- JSON-RPC 2.0 server over stdin/stdout
- `agsi_validate` tool for file validation
- `agsi_extract_materials` tool for material extraction
- `agsi_get_info` tool for document information
- `agsi_query_materials` tool for querying materials
- Document caching for performance
- Full MCP protocol compliance

#### LSP Server (`agsi-lsp`)
- Project structure and skeleton
- Ready for implementation

#### Documentation
- Comprehensive README.md with examples
- ARCHITECTURE.md with technical design
- QUICKSTART.md with 5-minute tutorial
- PROJECT_SUMMARY.md with deliverables
- Example code in `examples/create_model.rs`
- Inline documentation throughout codebase

#### Infrastructure
- Cargo workspace configuration
- Multi-crate architecture
- GitHub-ready .gitignore
- Release build optimization (LTO, strip)

### Design Decisions
- Material-centric design allowing independent material use
- Geometry embedding as binary data in JSON files
- Multi-format support (JSON, Avro, Protobuf)
- Three-level validation strategy
- Type-safe API leveraging Rust's type system

### Technical Stack
- Rust 2021 edition
- MSRV: 1.75
- serde for serialization
- clap for CLI
- tokio for async
- geo/wkt for geometry
- validator for validation

## [Unreleased]

### Planned

#### Short Term
- Complete Avro serialization implementation
- Complete Protobuf serialization with .proto files
- Full LSP server implementation
- TUI editor with ratatui
- Advanced validation rules
- More comprehensive test coverage

#### Medium Term
- Geometry visualization (2D/3D rendering)
- Performance benchmarks
- Streaming parser for large files
- Python bindings via PyO3
- WASM compilation for web use
- Additional examples and tutorials

#### Long Term
- Real-time collaboration features
- Version control integration
- Cloud storage adapters
- Machine learning integration for material classification
- GIS system integration
- Advanced geometry operations
- Material database/library system
- Cross-section generation from 3D models

### Known Limitations
- Avro serialization not yet fully implemented
- Protobuf serialization not yet fully implemented
- LSP server is placeholder only
- TUI editor not yet implemented
- No visualization capabilities yet
- Limited geometry operations

### Performance Notes
- Build time: ~2 minutes (release)
- Binary size: ~8MB (optimized)
- Memory usage: Efficient (no unnecessary cloning)
- Test suite: < 1 second

## Version History

- **0.1.0** (2024-10-06): Initial release with core functionality

---

[0.1.0]: https://github.com/yourusername/agsipls/releases/tag/v0.1.0
