# CLAUDE.md - MathHook Knowledge Base Engine

This file provides critical guidance to AI agents (Claude Code and similar) working on the MathHook Knowledge Base (KB) Engine codebase.

---

## 🔴 CRITICAL: Core Directives

**This is the absolute highest priority. All other guidelines are secondary.**

### Non-Negotiable Requirements

1. **Schema Validation First**: Every schema MUST validate before generation. Invalid schemas MUST error loudly with actionable messages.
   - Use JSON Schema or similar for strict validation
   - Check required fields exist
   - Validate code references point to real functions
   - Verify examples are syntactically correct

2. **No Silent Failures**: Generator errors MUST be explicit and actionable.
   - Never skip failing schemas silently
   - Always report which schema failed and why
   - Provide file path, line number, field name in errors
   - Suggest fixes when possible

3. **Multi-Language Consistency**: Code examples MUST be mathematically equivalent across Rust/Python/JavaScript.
   - Same input → same output across languages
   - Mathematical operations must match (e.g., `x^2` in all three)
   - Test cross-language consistency automatically

4. **Output Correctness**: Generated files MUST be syntactically valid.
   - `.ipynb` files must be valid Jupyter JSON
   - Markdown must render correctly in target system
   - LaTeX must compile without errors
   - Vue components must be valid SFC syntax

5. **Idempotency**: Running generator twice on same schema produces identical output.
   - No timestamps in generated files (unless explicitly requested)
   - Deterministic ordering (sorted alphabetically)
   - No random IDs or UUIDs

---

## 📋 Project Overview

The MathHook Knowledge Base Engine is a multi-format documentation generation system that transforms single schema definitions into 12+ output formats:

1. **Jupyter Notebooks** (`.ipynb`) - Interactive Python tutorials
2. **Google Colab Notebooks** (`.ipynb` + Colab metadata) - Zero-install onboarding
3. **mdBook** (`.md`) - Rust documentation
4. **Vue SSR Static Site** (`.vue` components + JSON data) - Marketing/learning hub
5. **Interactive API Docs** (VitePress-compatible `.md`) - Live code playground
6. **LLM-Optimized RAG Markdown** - Chunked for AI agent retrieval
7. **LaTeX/PDF** - Academic paper templates
8. **Observable Notebooks** (`.js`) - Interactive visualizations
9. **Benchmark Dashboard** (JSON data) - Performance comparisons
10. **LangChain Integration Guides** - Agent framework docs
11. **MCP Server** - AI code navigation
12. **Video Tutorial Companions** - Transcripts + notebooks

**Core Principle**: **Single Source of Truth** → One schema file generates all formats

---

## 🏗️ Architecture

### Repository Structure

```
mathhook-kb/
├── crates/
│   ├── kb-core/           # Core schema parsing & validation (Rust)
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

### Schema Format (Hybrid Option C)

**Design Philosophy**: Core content (required) + Output hints (optional) + Engine fills gaps

**Schema Structure** (`schemas/topic.yaml`):

```yaml
# === CORE CONTENT (Required) ===
topic: "calculus.derivative"
title: "Symbolic Differentiation"

description: |
  Computes the derivative of an expression with respect to a variable
  using symbolic differentiation rules (power rule, chain rule, etc.).

mathematical_definition: |
  $$\frac{d}{dx} f(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$$

# === CODE REFERENCES (Required) ===
code_refs:
  rust: "mathhook_core::calculus::derivative"
  python: "mathhook.calculus.derivative"
  nodejs: "mathhook.calculus.derivative"

# === EXAMPLES (Required, multi-language) ===
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

# === METADATA (Optional but recommended) ===
use_cases:
  - "Physics: Computing velocity from position"
  - "Machine Learning: Backpropagation gradients"

related_topics:
  - "calculus.integral"
  - "calculus.chain_rule"

performance:
  complexity: "O(n)"
  typical_time: "0.5ms"

# === OUTPUT HINTS (Optional, customize per format) ===
outputs:
  jupyter:
    include_interactive_plots: true
    include_performance_section: false
  mdbook:
    include_mathematical_proof: true
    runnable_code: true
  vue_site:
    include_live_demo: true
    seo_keywords: ["symbolic differentiation", "calculus"]
  api_docs:
    include_playground: true
    show_all_languages: true
  llm_rag:
    chunk_strategy: "by_example"
    max_chunk_size: 512
```

---

## 🦀 Technology Stack

### Core Engine (Rust)

**Dependencies** (use latest stable versions):

```toml
[dependencies]
# Schema & Serialization
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1.0"

# Templating
tera = "1.19"              # Jinja2-like templates

# CLI
clap = { version = "4.5", features = ["derive"] }
colored = "2.1"

# Code Analysis (future: AST parsing for validation)
syn = "2.0"                # Rust AST parsing
tree-sitter = "0.20"       # Multi-language parsing

# Error Handling
anyhow = "1.0"
thiserror = "1.0"

# Utilities
walkdir = "2.4"
regex = "1.0"
```

### Generator-Specific Tech

- **Jupyter**: Direct `.ipynb` JSON generation (no external deps)
- **mdBook**: Generate markdown, use `mdbook` CLI for building
- **Vue SSR**: Nuxt 3 consumes JSON data from Rust generator
- **Templates**: Tera (Rust-native, Jinja2-compatible)

---

## 🔴 Critical Development Rules

### Rule Priority System

**🔴 CRITICAL**: Violating these causes system failures or data corruption
**🟡 IMPORTANT**: Strong preference, exceptions require justification
**🟢 RECOMMENDED**: Best practices, follow when practical

---

### 🔴 CRITICAL: Schema Validation

**Rule**: All schemas MUST validate before any generation begins.

**Implementation**:
- Define strict schema structure using serde types
- Validate on parse: required fields, correct types, valid references
- Check code examples are syntactically correct (parser check)
- Verify cross-language consistency (same logic across Rust/Python/JS)

**Example**:
```rust
// ❌ WRONG: Skip validation
fn generate_jupyter(schema: Schema) -> Notebook {
    // Directly use schema fields without checking
}

// ✅ CORRECT: Validate first
fn generate_jupyter(schema: Schema) -> Result<Notebook, ValidationError> {
    schema.validate()?;  // Fails early if invalid
    // Now safe to use schema
}
```

**Error Messages**:
```
❌ BAD: "Schema invalid"
✅ GOOD: "Schema validation failed in 'schemas/calculus/derivative.yaml':
  - Missing required field 'description' (line 5)
  - Code reference 'rust: mathhook_core::calculus::deriv' not found
    Did you mean 'mathhook_core::calculus::derivative'?
  - Python example syntax error (line 23): unexpected EOF"
```

---

### 🔴 CRITICAL: Multi-Language Consistency

**Rule**: Code examples in Rust/Python/JavaScript MUST produce equivalent results.

**Testing Strategy**:
```rust
#[test]
fn test_cross_language_consistency() {
    let schema = load_schema("derivative.yaml");

    for example in schema.examples {
        let rust_result = execute_rust(example.code.rust);
        let python_result = execute_python(example.code.python);
        let js_result = execute_js(example.code.nodejs);

        assert_eq!(rust_result, python_result,
            "Rust and Python outputs differ for example '{}'", example.title);
        assert_eq!(python_result, js_result,
            "Python and JavaScript outputs differ for example '{}'", example.title);
    }
}
```

**Common Pitfalls**:
- Floating point precision differences (use rational arithmetic in examples)
- Off-by-one in array indexing (0-based vs 1-based)
- Different function names across bindings

---

### 🔴 CRITICAL: Output File Validity

**Rule**: Generated files MUST be syntactically valid in their target format.

**Validation Requirements**:

| Output Format | Validation Method |
|---------------|-------------------|
| `.ipynb` | Parse JSON, validate against nbformat schema |
| `.md` (mdBook) | Markdown linter + check code blocks compile |
| `.vue` | Vue SFC parser (eslint-plugin-vue) |
| `.tex` | LaTeX compiler (`pdflatex --interaction=nonstopmode`) |
| `.js` (Observable) | ESLint + syntax check |

**Implementation**:
```rust
fn generate_jupyter(schema: &Schema) -> Result<Notebook> {
    let nb = build_notebook(schema)?;

    // Validate before writing
    validate_notebook_json(&nb)?;

    Ok(nb)
}

fn validate_notebook_json(nb: &Notebook) -> Result<()> {
    // Check nbformat version
    if nb.nbformat != 4 {
        return Err(anyhow!("Only nbformat 4 supported"));
    }

    // Validate all cells
    for cell in &nb.cells {
        validate_cell(cell)?;
    }

    // Ensure JSON serializes correctly
    serde_json::to_string(nb)?;

    Ok(())
}
```

---

### 🔴 CRITICAL: Idempotency

**Rule**: Running generator twice on unchanged schema produces identical output.

**Why It Matters**:
- Version control diffs should only show real changes
- Reproducible builds for CI/CD
- Predictable behavior for users

**What to Avoid**:
```rust
// ❌ WRONG: Timestamps make output non-deterministic
fn generate_header() -> String {
    format!("Generated at {}", Utc::now())
}

// ❌ WRONG: Random IDs differ each run
fn generate_cell_id() -> String {
    Uuid::new_v4().to_string()
}

// ✅ CORRECT: Deterministic generation
fn generate_header(schema: &Schema) -> String {
    format!("Generated from schema: {}", schema.topic)
}

// ✅ CORRECT: Hash-based stable IDs
fn generate_cell_id(content: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("cell-{:x}", hasher.finish())
}
```

**Testing**:
```bash
# Generate twice, diff should be empty
mathhook-kb build
cp -r output output-v1
mathhook-kb build
diff -r output output-v1  # Should show no differences
```

---

### 🟡 IMPORTANT: Template Separation

**Rule**: Content (schemas) separate from presentation (templates).

**Architecture**:
```
schemas/calculus/derivative.yaml  ← Content (what to show)
templates/jupyter/function.tera   ← Presentation (how to show)
```

**Why**:
- Change template design without touching schemas
- Reuse templates across similar schemas
- A/B test different presentation styles

**Example**:
```rust
// ❌ WRONG: Hardcoded formatting in code
fn generate_title(schema: &Schema) -> String {
    format!("# {}\n\n**Category**: {}", schema.title, schema.category)
}

// ✅ CORRECT: Use template
fn generate_title(schema: &Schema, template: &Template) -> String {
    let mut context = Context::new();
    context.insert("title", &schema.title);
    context.insert("category", &schema.category);
    template.render("title.tera", &context)
}
```

---

### 🟡 IMPORTANT: Generator Extensibility

**Rule**: New output formats should be implementable via trait, not modifying core.

**Design**:
```rust
// Define generator trait
pub trait OutputGenerator {
    fn name(&self) -> &str;
    fn file_extension(&self) -> &str;
    fn generate(&self, schema: &Schema) -> Result<String>;
    fn validate_output(&self, output: &str) -> Result<()>;
}

// Implement for each format
pub struct JupyterGenerator { /* ... */ }

impl OutputGenerator for JupyterGenerator {
    fn name(&self) -> &str { "jupyter" }
    fn file_extension(&self) -> &str { "ipynb" }

    fn generate(&self, schema: &Schema) -> Result<String> {
        // Generate .ipynb JSON
    }

    fn validate_output(&self, output: &str) -> Result<()> {
        // Validate JSON structure
    }
}

// Register generators
let generators: Vec<Box<dyn OutputGenerator>> = vec![
    Box::new(JupyterGenerator::new()),
    Box::new(MdBookGenerator::new()),
    Box::new(VueGenerator::new()),
];
```

---

### 🟡 IMPORTANT: Error Messages

**Rule**: Errors must be actionable with context (file, line, suggestion).

**Good Error Messages**:
```
❌ Schema validation failed

✅ Schema validation failed in 'schemas/calculus/derivative.yaml':

   Line 23: Missing required field 'description'

   Add a description field:

     description: |
       Brief explanation of what this function does

   Line 45: Invalid code reference 'mathhook_core::calculus::deriv'

   Function not found in mathhook-core.
   Did you mean 'mathhook_core::calculus::derivative'?
```

**Implementation**:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("Missing required field '{field}' at line {line} in {file}\n\nSuggestion: {suggestion}")]
    MissingField {
        field: String,
        line: usize,
        file: String,
        suggestion: String,
    },

    #[error("Invalid code reference '{reference}' at line {line} in {file}\n\nDid you mean '{suggestion}'?")]
    InvalidCodeRef {
        reference: String,
        line: usize,
        file: String,
        suggestion: String,
    },
}
```

---

### 🟡 IMPORTANT: Performance

**Rule**: Schema parsing and generation should be <1s for typical schema.

**Optimization Strategies**:
- Parse schemas in parallel (rayon)
- Cache template compilation
- Lazy evaluation where possible
- Profile hot paths with `cargo flamegraph`

**Benchmarking**:
```bash
# Add benchmarks for critical paths
cargo bench --bench generation_speed
```

---

### 🟢 RECOMMENDED: Code Quality

**Rule**: All code must pass `cargo fmt` and `cargo clippy` with no warnings.

**Enforcement**:
```bash
# Before committing
cargo fmt --check
cargo clippy -- -D warnings

# CI should fail on warnings
```

**Clippy Configuration** (`.clippy.toml`):
```toml
# Enforce strict lints
cognitive-complexity-threshold = 15
```

---

### 🟢 RECOMMENDED: Documentation

**Rule**: All public APIs have rustdoc with examples.

**Example**:
```rust
/// Generates a Jupyter notebook from a schema.
///
/// # Arguments
///
/// * `schema` - The validated schema to generate from
/// * `template_dir` - Directory containing Tera templates
///
/// # Examples
///
/// ```rust
/// use kb_jupyter::JupyterGenerator;
/// use kb_core::schema::Schema;
///
/// let schema = Schema::load("derivative.yaml")?;
/// let generator = JupyterGenerator::new("templates/")?;
/// let notebook = generator.generate(&schema)?;
/// ```
///
/// # Errors
///
/// Returns error if:
/// - Schema is invalid
/// - Template rendering fails
/// - Output validation fails
pub fn generate(schema: &Schema) -> Result<Notebook> {
    // Implementation
}
```

---

## 🧪 Testing Strategy

### Test Categories

1. **Unit Tests**: Test individual functions in each crate
2. **Integration Tests**: Test full schema → output pipeline
3. **Validation Tests**: Ensure outputs are syntactically valid
4. **Cross-Language Tests**: Verify code example consistency
5. **Regression Tests**: Prevent breaking changes

### Testing Requirements

**Every generator MUST have**:
- Unit tests for core logic
- Integration test with example schema
- Output validation (syntax check)

**Example Integration Test**:
```rust
#[test]
fn test_jupyter_generation_derivative() {
    // Load example schema
    let schema = Schema::load("tests/schemas/derivative.yaml")
        .expect("Failed to load schema");

    // Generate notebook
    let generator = JupyterGenerator::new("templates/")
        .expect("Failed to initialize generator");
    let notebook = generator.generate(&schema)
        .expect("Failed to generate notebook");

    // Validate output
    assert!(notebook.nbformat == 4);
    assert!(notebook.cells.len() > 0);

    // Check content
    let first_cell = &notebook.cells[0];
    assert!(first_cell.source.contains("Symbolic Differentiation"));

    // Validate as JSON
    let json = serde_json::to_string(&notebook)
        .expect("Failed to serialize notebook");
    serde_json::from_str::<serde_json::Value>(&json)
        .expect("Generated invalid JSON");
}
```

---

## 📁 File Organization

### Module Size Limit

**Rule**: Maximum 500 lines per file. Split larger modules into focused sub-modules.

### Naming Conventions

- **Files**: `snake_case.rs`
- **Types**: `PascalCase`
- **Functions**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`

### Import Organization

```rust
// Standard library
use std::fs;
use std::path::Path;

// External crates (alphabetical)
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tera::Tera;

// Internal crates
use kb_core::schema::Schema;
use kb_core::generator::OutputGenerator;

// Current crate
use crate::notebook::Notebook;
```

---

## 🚀 Build and Development Commands

### Development Workflow

```bash
# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Test
cargo test --all --no-fail-fast

# Build
cargo build --release

# Install CLI locally
cargo install --path crates/kb-cli

# Use CLI
mathhook-kb build --schema schemas/derivative.yaml
mathhook-kb validate --schema schemas/
mathhook-kb watch --schema schemas/ --output output/
```

### CI/CD

```bash
# Run in CI
cargo fmt --check
cargo clippy -- -D warnings
cargo test --all --no-fail-fast
cargo build --release
```

---

## 🎯 Common AI Agent Pitfalls

### Mistakes to Avoid

1. **Generating invalid JSON**: Always validate `.ipynb` structure
2. **Inconsistent code examples**: Test Rust/Python/JS equivalence
3. **Hardcoding paths**: Use configurable template/output directories
4. **Silent errors**: Never swallow validation failures
5. **Non-deterministic output**: No timestamps, random IDs, or UUIDs
6. **Template in code**: Keep formatting in `.tera` files, not Rust
7. **Missing error context**: Always include file path, line number

---

## 📚 References

### Key Dependencies Documentation

- **Serde**: https://serde.rs/
- **Tera Templates**: https://tera.netlify.app/
- **Clap CLI**: https://docs.rs/clap/
- **Anyhow Errors**: https://docs.rs/anyhow/

### Related Projects

- **mdBook**: https://rust-lang.github.io/mdBook/
- **Jupyter nbformat**: https://nbformat.readthedocs.io/
- **Nuxt 3**: https://nuxt.com/

---

## 🔄 When to Update This Document

Update CLAUDE.md when:
- A new critical rule emerges (validation requirement, output format)
- A new generator is added (document trait implementation)
- Common mistakes are identified (add to pitfalls section)
- Schema format changes (update examples)
- Testing strategy evolves (new test categories)

**Tell the user immediately when you update this file and explain why.**

---

## 🎓 Quick Start for AI Agents

**First time working on mathhook-kb?**

1. Read 🔴 CRITICAL sections first
2. Understand the schema format (Hybrid Option C)
3. Look at example schema in `schemas/examples/`
4. Review one generator implementation (`kb-jupyter`)
5. Run tests to understand expected behavior: `cargo test`

**When adding a new generator**:

1. Implement `OutputGenerator` trait
2. Create templates in `templates/{generator-name}/`
3. Add integration test in `tests/integration/`
4. Validate output in test (syntax check)
5. Update CLI to include new generator
6. Document in README

**When modifying schema format**:

1. Update schema types in `kb-core/src/schema/`
2. Update example schemas
3. Update all generators to handle new fields
4. Add validation for new fields
5. Update tests
6. Update CLAUDE.md schema example

---

**Remember**: This project powers documentation for a mathematical library. **Correctness and clarity are paramount.**
