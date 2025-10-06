# AGSi Python Bindings

Python bindings for the AGSi ground model library.

## Installation

### From Source

```bash
cd crates/agsi-py
pip install maturin
maturin develop
```

### From PyPI (when published)

```bash
pip install agsi-py
```

## Usage

### Basic Example

```python
import agsi_py

# Create a new document
doc = agsi_py.Document("PROJECT-001")
doc.set_author("Python User")

# Save to file
doc.to_json_file("output.agsi.json")

# Load from file
doc2 = agsi_py.Document.from_json_file("output.agsi.json")

# Validate
result = doc2.validate()
if result.is_valid():
    print("✅ Document is valid!")
else:
    print(f"❌ {result.error_count()} errors found")
    for error in result.errors():
        print(f"  - {error}")
```

### Create Materials

```python
import agsi_py

# Create material
clay = agsi_py.Material("MAT001", "London Clay", "SOIL")
clay.set_description("Stiff clay with occasional sand lenses")

print(clay)  # Material(id='MAT001', name='London Clay', type='Soil')
print(f"Properties: {clay.property_count()}")
```

### Work with Documents

```python
import agsi_py

# Load document
doc = agsi_py.Document.from_json_file("project.agsi.json")

# Get info
print(f"File ID: {doc.file_id}")
print(f"Models: {doc.model_count()}")

# Convert to JSON string
json_str = doc.to_json_string()
print(json_str)

# Validate
result = doc.validate()
print(f"Valid: {result.is_valid()}")
print(f"Errors: {result.error_count()}")
print(f"Warnings: {result.warning_count()}")
```

## API Reference

### Document

- `Document(file_id: str)` - Create new document
- `Document.from_json_file(path: str)` - Load from file
- `Document.from_json_str(json: str)` - Load from JSON string
- `to_json_file(path: str)` - Save to file
- `to_json_string()` - Convert to JSON string
- `file_id` - Get file ID (property)
- `set_author(author: str)` - Set author
- `model_count()` - Get number of models
- `validate()` - Validate document, returns ValidationResult

### ValidationResult

- `is_valid()` - Check if valid
- `error_count()` - Get number of errors
- `warning_count()` - Get number of warnings
- `errors()` - Get list of error messages
- `warnings()` - Get list of warning messages

### Material

- `Material(id: str, name: str, material_type: str)` - Create material
  - Types: "SOIL", "ROCK", "FILL", "MADE_GROUND", "ANTHROPOGENIC", "WATER", "VOID"
- `id` - Get material ID (property)
- `name` - Get material name (property)
- `set_description(desc: str)` - Set description
- `property_count()` - Get number of properties

## Building

### Development

```bash
# Install maturin
pip install maturin

# Build and install in development mode
maturin develop

# Run tests
pytest tests/
```

### Release

```bash
# Build wheel
maturin build --release

# Build and publish to PyPI
maturin publish
```

## Requirements

- Python 3.8+
- Rust 1.75+ (for building from source)

## License

MIT OR Apache-2.0
