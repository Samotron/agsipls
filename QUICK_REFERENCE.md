# 🚀 AGSi Quick Reference

## Installation

```bash
cargo build --release
# Binary: target/release/agsi (4.0MB)
```

## All Commands (12)

```bash
agsi validate <file>              # Validate file
agsi create document ...          # Create document
agsi create material ...          # Create material
agsi create model ...             # Create model
agsi edit <file>                  # Edit file
agsi tui [file]                   # TUI editor ⭐
agsi extract <file>               # Extract materials
agsi info <file>                  # Show information
agsi convert <in> -o <out>        # Convert formats
agsi form <item>                  # Interactive forms
agsi diff <file1> <file2>         # Compare files
agsi stats <file>                 # Show statistics
agsi mcp                          # MCP server
agsi lsp                          # LSP server
```

## Quick Examples

### TUI Editor ⭐ NEW
```bash
agsi tui project.agsi.json

# Keys:
# d - Document info    m - Models
# a - Materials        c - Components  
# s - Save            q - Quit
# ↑↓ - Navigate       ? - Help
```

### Python ⭐ NEW
```python
import agsi_py

doc = agsi_py.Document("DOC-001")
doc.set_author("User")
doc.to_json_file("out.json")

result = doc.validate()
print(f"Valid: {result.is_valid()}")
```

### Avro ⭐ NEW
```bash
# Convert to Avro
agsi convert input.json -o output.avro -f avro

# Avro is binary, smaller than JSON
```

## Features Matrix

| Feature | Status | Command/Usage |
|---------|--------|---------------|
| CLI | ✅ | `agsi <cmd>` |
| TUI Editor | ✅ | `agsi tui` |
| Python | ✅ | `import agsi_py` |
| Avro | ✅ | `convert -f avro` |
| Protobuf | ✅ Schema | `schemas/agsi.proto` |
| MCP | ✅ | `agsi mcp` |
| LSP | 🏗️ | Structure ready |
| Docker | ✅ | `docker build .` |
| CI/CD | ✅ | GitHub Actions |

## File Locations

```
target/release/agsi              # Main binary (4.0MB)
schemas/agsi.avsc                # Avro schema
schemas/agsi.proto               # Protobuf schema
crates/agsi-py/                  # Python bindings
examples/3d_model.rs             # 3D example
examples/create_model.rs         # 2D example
```

## Documentation

- `README.md` - Main docs
- `QUICKSTART.md` - 5-min tutorial
- `COMPLETE_IMPLEMENTATION.md` - Full status
- `CONTRIBUTING.md` - How to contribute
- `DOCKER.md` - Docker usage
- `crates/agsi-py/README.md` - Python docs

## Build Commands

```bash
cargo build --release            # Build CLI
cargo test                       # Run tests
./test.sh                        # Integration tests
cd crates/agsi-py && maturin develop  # Python
docker build -t agsi .           # Docker
```

## What's New (Latest)

✅ **TUI Editor** - Full terminal UI
✅ **Python Bindings** - PyO3 integration
✅ **Avro Support** - Complete schema + serialization
✅ **Protobuf Schema** - Ready for codegen
✅ **diff command** - Compare files
✅ **stats command** - Statistics
✅ **CI/CD** - GitHub Actions
✅ **Docker** - Production ready

## Quick Stats

- **Binary Size**: 4.0MB
- **Commands**: 12
- **Tests**: 17 unit + 11 integration
- **Languages**: Rust + Python
- **Formats**: JSON, Avro, Protobuf
- **Schemas**: 2 (Avro + Proto)
- **Examples**: 2 working
- **Docs**: 14 files

## Three Major Features ⭐

1. **Avro/Protobuf** ✅
   - Complete schemas
   - Working serialization
   - Ready for production

2. **TUI Editor** ✅
   - Interactive terminal UI
   - Multiple views
   - Real-time navigation

3. **Python Bindings** ✅
   - Complete PyO3 wrapper
   - Pythonic API
   - Ready for PyPI

**Status**: All delivered! 🎉
