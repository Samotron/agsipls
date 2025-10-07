# Contributing to AGSi Rust Implementation

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the project.

## 🎯 Ways to Contribute

- **Bug Reports**: Found a bug? Open an issue with details to reproduce
- **Feature Requests**: Have an idea? Open an issue to discuss it
- **Code Contributions**: Submit pull requests for bug fixes or features
- **Documentation**: Improve docs, add examples, fix typos
- **Testing**: Add tests, report test failures, improve test coverage
- **Performance**: Profile and optimize hot paths
- **Examples**: Add real-world usage examples

## 🚀 Getting Started

### Prerequisites

- Rust 1.75 or later
- Git
- Basic familiarity with geotechnical data (helpful but not required)

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/agsipls.git
cd agsipls

# Build the project
cargo build

# Run tests
cargo test

# Run the CLI
cargo run --bin agsi -- --help
```

## 📝 Development Workflow

### 1. Fork and Clone

```bash
# Fork on GitHub, then:
git clone https://github.com/YOUR_USERNAME/agsipls.git
cd agsipls
git remote add upstream https://github.com/ORIGINAL_OWNER/agsipls.git
```

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 3. Make Changes

- Write code following the style guide below
- Add tests for new functionality
- Update documentation as needed
- Run tests and linting

### 4. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests
./test.sh

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings
```

### 5. Commit

Write clear, descriptive commit messages:

```bash
git add .
git commit -m "feat: add diff command for comparing AGSi files"
```

Commit message format:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test additions/changes
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `chore:` Maintenance tasks

### 6. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## 💻 Code Style Guide

### Rust Code

- Follow standard Rust style (use `cargo fmt`)
- Use meaningful variable names
- Add doc comments for public APIs
- Keep functions focused and small
- Use `Result` for fallible operations
- Prefer `&str` over `String` for parameters

```rust
/// Validates an AGSi document against the schema.
///
/// # Examples
///
/// ```
/// use agsi_core::{Document, validation};
///
/// let doc = Document::new("DOC001");
/// let result = validation::validate_document(&doc)?;
/// assert!(result.is_valid());
/// ```
///
/// # Errors
///
/// Returns `Err` if the document fails validation.
pub fn validate_document(doc: &Document) -> Result<ValidationResult> {
    // Implementation
}
```

### File Organization

```
crates/
├── agsi-core/           # Core library
│   └── src/
│       ├── lib.rs       # Public API
│       ├── document.rs  # Document types
│       ├── material.rs  # Material types
│       └── ...
└── agsi/                # CLI binary
    └── src/
        ├── main.rs      # Entry point
        ├── commands/    # CLI commands
        ├── mcp/         # MCP server
        └── lsp/         # LSP server
```

### Testing

- Unit tests in the same file as the code
- Integration tests in `tests/` directory
- Use descriptive test names

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        let material = Material::new("MAT001", "Clay", MaterialType::Soil);
        assert_eq!(material.id, "MAT001");
    }

    #[test]
    fn test_validation_catches_duplicate_ids() {
        // Test implementation
    }
}
```

### Documentation

- Add doc comments to all public items
- Include examples in doc comments
- Update README.md for user-facing changes
- Update CHANGELOG.md for all changes

## 🔍 Pull Request Guidelines

### Before Submitting

- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Commit messages are clear

### PR Description

Include in your PR:

1. **What**: Brief description of changes
2. **Why**: Reason for the changes
3. **How**: Technical approach (if complex)
4. **Testing**: How you tested the changes
5. **Breaking Changes**: Any API changes (if applicable)

Example:

```markdown
## What
Adds `agsi diff` command to compare two AGSi files.

## Why
Users need to track changes between versions of ground models.

## How
- Loads both documents
- Compares schemas, models, materials, and components
- Displays differences in a user-friendly format

## Testing
- Added unit tests for diff logic
- Tested with example files
- Integration test added to test.sh

## Breaking Changes
None
```

## 🐛 Bug Reports

Good bug reports include:

- **Title**: Clear, specific description
- **Steps to Reproduce**: Minimal example
- **Expected Behavior**: What should happen
- **Actual Behavior**: What actually happens
- **Environment**: OS, Rust version, agsi version
- **Logs/Output**: Relevant error messages

Example:

```markdown
## Bug: Validation fails on valid AGSi file

**Steps to Reproduce:**
1. Create file with: `agsi create document --id TEST --output test.json`
2. Validate with: `agsi validate test.json`

**Expected:** Validation passes
**Actual:** Error: "Missing required field: agsProject"

**Environment:**
- OS: Ubuntu 22.04
- Rust: 1.75.0
- agsi: 0.1.0

**Error Output:**
```
Error: Validation failed
...
```
```

## 💡 Feature Requests

For feature requests, please include:

- **Problem**: What problem does this solve?
- **Solution**: Your proposed solution
- **Alternatives**: Other approaches considered
- **Additional Context**: Examples, mockups, etc.

## 📚 Adding Examples

Examples go in `examples/` directory:

```rust
// examples/my_example.rs
use agsi_core::{Document, Material};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Your example code
    Ok(())
}
```

Add to `Cargo.toml`:

```toml
[[example]]
name = "my_example"
path = "examples/my_example.rs"
```

## 🎓 Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [AGSi Documentation](https://ags-data-format-wg.gitlab.io/agsi/)
- [Project Architecture](ARCHITECTURE.md)
- [Quick Start Guide](QUICKSTART.md)

## 🤝 Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions
- Keep discussions on-topic

## ❓ Questions?

- Open an issue with the `question` label
- Check existing issues and discussions
- Read the documentation first

## 🎉 Recognition

Contributors will be:
- Listed in CHANGELOG.md
- Mentioned in release notes
- Credited in documentation (if desired)

Thank you for contributing! 🚀
