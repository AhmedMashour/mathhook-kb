# Contributing to MathHook Knowledge Base Engine

Thank you for your interest in contributing to the MathHook KB Engine! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contribution Workflow](#contribution-workflow)
- [Code Style Guidelines](#code-style-guidelines)
- [Testing Requirements](#testing-requirements)
- [Schema Contributions](#schema-contributions)
- [Adding New Generators](#adding-new-generators)
- [Pull Request Process](#pull-request-process)
- [Reporting Issues](#reporting-issues)

---

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment. Be kind, constructive, and professional in all interactions.

---

## Getting Started

### Prerequisites

- **Rust** (latest stable, 1.70+): [Install Rust](https://rustup.rs/)
- **Node.js** (18+): For Vue SSR site generation
- **Git**: Version control

### Quick Start

```bash
# Clone the repository
git clone https://github.com/your-org/mathhook-kb.git
cd mathhook-kb

# Build the project
cargo build

# Run tests
cargo test --all

# Install CLI locally (optional)
cargo install --path crates/kb-cli
```

---

## Development Setup

### Repository Structure

```
mathhook-kb/
├── crates/
│   ├── kb-core/           # Core schema parsing & validation
│   ├── kb-cli/            # CLI tool (mathhook-kb command)
│   ├── kb-jupyter/        # Jupyter notebook generator
│   ├── kb-mdbook/         # mdBook generator
│   ├── kb-vue/            # Vue SSR data generator
│   ├── kb-api-docs/       # API docs generator
│   ├── kb-llm-rag/        # LLM-optimized markdown
│   └── kb-latex/          # LaTeX generator
├── templates/             # Tera templates for all outputs
├── schemas/               # Schema definitions (YAML)
├── generators/
│   └── vue-site/          # Node.js/Nuxt SSR builder
└── scripts/               # Build and validation scripts
```

### Recommended IDE Setup

- **VS Code** with rust-analyzer extension
- **RustRover** or **CLion** with Rust plugin

### Environment Configuration

```bash
# Enable verbose logging during development
export RUST_LOG=debug

# Run with backtrace for debugging
export RUST_BACKTRACE=1
```

---

## Contribution Workflow

### 1. Find or Create an Issue

- Check [existing issues](https://github.com/your-org/mathhook-kb/issues) before starting work
- For new features, open an issue first to discuss the approach
- For bugs, include reproduction steps and expected behavior

### 2. Fork and Branch

```bash
# Fork the repo on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/mathhook-kb.git

# Create a feature branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/issue-description
```

### 3. Make Changes

- Write code following our [style guidelines](#code-style-guidelines)
- Add tests for new functionality
- Update documentation as needed

### 4. Submit Pull Request

- Push your branch and open a PR against `master`
- Fill out the PR template completely
- Link related issues

---

## Code Style Guidelines

### Rust Code Style

All Rust code must pass formatting and linting checks:

```bash
# Format code (run before committing)
cargo fmt

# Check for lint warnings (must pass with no warnings)
cargo clippy -- -D warnings
```

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Files | `snake_case.rs` | `schema_parser.rs` |
| Types/Structs | `PascalCase` | `SchemaValidator` |
| Functions | `snake_case` | `validate_schema()` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_FILE_SIZE` |
| Modules | `snake_case` | `mod output_generator;` |

### Import Organization

Organize imports in this order, with blank lines between groups:

```rust
// 1. Standard library
use std::fs;
use std::path::Path;

// 2. External crates (alphabetical)
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tera::Tera;

// 3. Internal crates (kb-* crates)
use kb_core::schema::Schema;

// 4. Current crate
use crate::notebook::Notebook;
```

### File Size Limit

Keep files under **500 lines**. If a module grows beyond this, split it into focused sub-modules.

### Documentation

All public APIs must have rustdoc comments with:
- Brief description
- Arguments explanation
- Example usage
- Possible errors

```rust
/// Generates a Jupyter notebook from a validated schema.
///
/// # Arguments
///
/// * `schema` - The validated schema to generate from
///
/// # Examples
///
/// ```rust
/// let schema = Schema::load("derivative.yaml")?;
/// let notebook = generate_notebook(&schema)?;
/// ```
///
/// # Errors
///
/// Returns error if schema is invalid or template rendering fails.
pub fn generate_notebook(schema: &Schema) -> Result<Notebook> {
    // ...
}
```

---

## Testing Requirements

### Test Categories

1. **Unit Tests**: Test individual functions (`#[test]` in same file)
2. **Integration Tests**: Full pipeline tests (`tests/` directory)
3. **Validation Tests**: Ensure output file validity

### Running Tests

```bash
# Run all tests
cargo test --all

# Run tests with output
cargo test --all -- --nocapture

# Run specific test
cargo test test_jupyter_generation

# Run tests for specific crate
cargo test -p kb-jupyter
```

### Test Requirements for Contributions

| Contribution Type | Required Tests |
|-------------------|----------------|
| Bug fix | Regression test proving the fix |
| New function | Unit tests covering edge cases |
| New generator | Integration test + output validation |
| Schema changes | Validation tests + example update |

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = create_test_input();

        // Act
        let result = function_under_test(input);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().field, expected_value);
    }

    #[test]
    fn test_error_handling() {
        let invalid_input = create_invalid_input();
        let result = function_under_test(invalid_input);
        assert!(result.is_err());
    }
}
```

---

## Schema Contributions

### Schema Format Overview

Schemas are YAML files that define documentation content. The engine generates 12+ output formats from a single schema.

### Required Fields

```yaml
# Core content (REQUIRED)
topic: "calculus.derivative"        # Unique identifier
title: "Symbolic Differentiation"   # Human-readable title
description: |                      # What this does
  Computes derivatives using symbolic differentiation rules.

# Code references (REQUIRED)
code_refs:
  rust: "mathhook_core::calculus::derivative"
  python: "mathhook.calculus.derivative"
  nodejs: "mathhook.calculus.derivative"

# Examples (REQUIRED, all three languages)
examples:
  - title: "Power Rule"
    explanation: "Derivative of x^n is n*x^(n-1)"
    code:
      rust: |
        let x = symbol!(x);
        let f = expr!(x ^ 3);
        let df = f.derivative(&x, 1);
        // Result: 3*x^2
      python: |
        x = symbol('x')
        f = expr('x^3')
        df = f.derivative(x)
        # Result: 3*x^2
      nodejs: |
        const x = symbol('x');
        const f = expr('x^3');
        const df = f.derivative(x);
        // Result: 3*x^2
```

### Critical Schema Rules

1. **Multi-Language Consistency**: Code examples in Rust/Python/JavaScript must produce mathematically equivalent results
2. **Valid Syntax**: All code examples must be syntactically correct
3. **Real References**: `code_refs` must point to actual functions in the MathHook library
4. **LaTeX Validity**: Mathematical definitions must be valid LaTeX

### Validating Schemas

```bash
# Validate a single schema
mathhook-kb validate --schema schemas/your-schema.yaml

# Validate all schemas
mathhook-kb validate --schema schemas/
```

### Schema Contribution Checklist

- [ ] Topic ID follows `category.function_name` pattern
- [ ] All required fields present
- [ ] Code examples in all three languages
- [ ] Examples produce equivalent mathematical results
- [ ] LaTeX compiles without errors
- [ ] Schema validates with `mathhook-kb validate`

---

## Adding New Generators

To add a new output format generator:

### 1. Create New Crate

```bash
cd crates
cargo new kb-your-format --lib
```

### 2. Implement OutputGenerator Trait

```rust
use kb_core::{Schema, OutputGenerator, Result};

pub struct YourFormatGenerator {
    template_dir: PathBuf,
}

impl OutputGenerator for YourFormatGenerator {
    fn name(&self) -> &str {
        "your-format"
    }

    fn file_extension(&self) -> &str {
        "ext"
    }

    fn generate(&self, schema: &Schema) -> Result<String> {
        // Generate output from schema
        schema.validate()?;  // Always validate first!
        // ... generation logic
    }

    fn validate_output(&self, output: &str) -> Result<()> {
        // Verify output is syntactically valid
    }
}
```

### 3. Create Templates

```
templates/your-format/
├── main.tera
├── header.tera
└── example.tera
```

### 4. Add Tests

```rust
#[test]
fn test_your_format_generation() {
    let schema = Schema::load("tests/schemas/example.yaml").unwrap();
    let generator = YourFormatGenerator::new("templates/").unwrap();

    let output = generator.generate(&schema).unwrap();

    // Validate output
    generator.validate_output(&output).unwrap();
}
```

### 5. Register in CLI

Update `crates/kb-cli/src/main.rs` to include your generator.

### Generator Checklist

- [ ] Implements `OutputGenerator` trait
- [ ] Validates schema before generation
- [ ] Validates output after generation
- [ ] Has unit tests
- [ ] Has integration test with example schema
- [ ] Templates in `templates/{generator-name}/`
- [ ] Registered in CLI
- [ ] Documentation in README

---

## Pull Request Process

### Before Submitting

1. **Run all checks locally**:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test --all
   ```

2. **Verify idempotency** (for generator changes):
   ```bash
   mathhook-kb build
   cp -r output output-v1
   mathhook-kb build
   diff -r output output-v1  # Should show no differences
   ```

3. **Update documentation** if your changes affect:
   - Public APIs
   - Schema format
   - CLI usage
   - Build process

### PR Requirements

- [ ] All CI checks pass
- [ ] Tests added for new functionality
- [ ] Documentation updated
- [ ] Commits have clear, descriptive messages
- [ ] PR description explains the changes and motivation

### Review Process

1. A maintainer will review your PR within a few days
2. Address any feedback by pushing additional commits
3. Once approved, a maintainer will merge your PR

### Commit Message Format

```
<type>: <short summary>

<detailed description if needed>

Fixes #123
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

Example:
```
feat: add Observable notebook generator

Implements OutputGenerator for Observable .js format.
Includes templates and validation for Observable syntax.

Fixes #45
```

---

## Reporting Issues

### Bug Reports

Include:
- **Description**: What happened vs. what you expected
- **Reproduction steps**: Minimal steps to reproduce
- **Environment**: OS, Rust version, relevant dependencies
- **Error messages**: Full error output with `RUST_BACKTRACE=1`

### Feature Requests

Include:
- **Use case**: Why you need this feature
- **Proposed solution**: How you envision it working
- **Alternatives considered**: Other approaches you thought of

### Security Issues

For security vulnerabilities, please email security@mathook.org directly instead of opening a public issue.

---

## Questions?

- Open a [Discussion](https://github.com/your-org/mathhook-kb/discussions) for general questions
- Check existing issues and discussions before asking
- Join our community chat (link in README)

---

## Recognition

Contributors are recognized in our [CONTRIBUTORS.md](CONTRIBUTORS.md) file and release notes. Thank you for helping make MathHook better!
