# Quick Start Guide - MathHook Knowledge Base Engine

For AI agents continuing this project.

---

## 🎯 Current Status

**Phase 1 Complete**: Core infrastructure, schema system, validation
**Next Phase**: Implement MVP generators (Jupyter, mdBook, LLM RAG)

---

## 🚀 Getting Started in 5 Minutes

### 1. Verify Setup

```bash
cd ../mathhook-kb
cargo check  # Should compile without errors
cargo test   # Should pass 9 tests
```

### 2. Understand the Schema

```bash
cat schemas/examples/derivative.yaml
```

This is your reference. Every schema follows this structure.

### 3. Load and Validate the Schema

```rust
use kb_core::Schema;
use std::path::Path;

let schema = Schema::load_from_file(
    Path::new("schemas/examples/derivative.yaml")
)?;

// Schema is now validated and ready to use
println!("Topic: {}", schema.topic);
println!("Examples: {}", schema.examples.len());
```

### 4. Next Task: Jupyter Generator

Your job is to implement `kb-jupyter` crate.

**Goal**: Convert `schemas/examples/derivative.yaml` → valid `.ipynb` file

---

## 📝 Implementing Jupyter Generator (Step-by-Step)

### Step 1: Understand Jupyter Format

A `.ipynb` file is JSON with this structure:

```json
{
  "nbformat": 4,
  "nbformat_minor": 5,
  "metadata": {},
  "cells": [
    {
      "cell_type": "markdown",
      "metadata": {},
      "source": ["# Title\n", "Description"]
    },
    {
      "cell_type": "code",
      "execution_count": null,
      "metadata": {},
      "outputs": [],
      "source": ["# Python code"]
    }
  ]
}
```

### Step 2: Define Jupyter Types

**File**: `crates/kb-jupyter/src/notebook.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Notebook {
    pub nbformat: u32,
    pub nbformat_minor: u32,
    pub metadata: serde_json::Value,
    pub cells: Vec<Cell>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {
    pub cell_type: String,  // "code" or "markdown"
    pub metadata: serde_json::Value,
    pub source: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_count: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<serde_json::Value>>,
}
```

### Step 3: Implement Generator

**File**: `crates/kb-jupyter/src/lib.rs`

```rust
use kb_core::{OutputGenerator, Result, Schema, KbError};
use serde_json;

mod notebook;
use notebook::{Notebook, Cell};

pub struct JupyterGenerator;

impl JupyterGenerator {
    pub fn new() -> Self {
        Self
    }

    fn create_markdown_cell(&self, content: &str) -> Cell {
        Cell {
            cell_type: "markdown".to_string(),
            metadata: serde_json::json!({}),
            source: content.lines()
                .map(|l| format!("{}\n", l))
                .collect(),
            execution_count: None,
            outputs: None,
        }
    }

    fn create_code_cell(&self, code: &str) -> Cell {
        Cell {
            cell_type: "code".to_string(),
            metadata: serde_json::json!({}),
            source: code.lines()
                .map(|l| format!("{}\n", l))
                .collect(),
            execution_count: None,
            outputs: Some(vec![]),
        }
    }
}

impl OutputGenerator for JupyterGenerator {
    fn name(&self) -> &str {
        "jupyter"
    }

    fn file_extension(&self) -> &str {
        "ipynb"
    }

    fn generate(&self, schema: &Schema) -> Result<String> {
        let mut cells = Vec::new();

        // Title cell
        cells.push(self.create_markdown_cell(&format!(
            "# {}\n\n{}",
            schema.title,
            schema.description
        )));

        // Mathematical definition if present
        if let Some(math_def) = &schema.mathematical_definition {
            cells.push(self.create_markdown_cell(&format!(
                "## Mathematical Definition\n\n{}",
                math_def
            )));
        }

        // Examples
        for example in &schema.examples {
            cells.push(self.create_markdown_cell(&format!(
                "## Example: {}\n\n{}",
                example.title,
                example.explanation
            )));

            cells.push(self.create_code_cell(&example.code.python));

            if let Some(output) = &example.expected_output {
                cells.push(self.create_markdown_cell(&format!(
                    "**Expected output:** `{}`",
                    output
                )));
            }
        }

        // Use cases
        if !schema.use_cases.is_empty() {
            let use_cases_text = schema.use_cases
                .iter()
                .map(|uc| format!("- {}", uc))
                .collect::<Vec<_>>()
                .join("\n");

            cells.push(self.create_markdown_cell(&format!(
                "## Use Cases\n\n{}",
                use_cases_text
            )));
        }

        let notebook = Notebook {
            nbformat: 4,
            nbformat_minor: 5,
            metadata: serde_json::json!({}),
            cells,
        };

        let json = serde_json::to_string_pretty(&notebook)?;
        Ok(json)
    }

    fn validate_output(&self, output: &str) -> Result<()> {
        // Validate JSON structure
        let _notebook: Notebook = serde_json::from_str(output)
            .map_err(|e| KbError::InvalidOutput {
                format: "jupyter".to_string(),
                file: "output.ipynb".to_string(),
                error: format!("Invalid JSON: {}", e),
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::schema::*;

    #[test]
    fn test_jupyter_generation() {
        let schema = Schema {
            topic: "test.function".to_string(),
            title: "Test Function".to_string(),
            description: "A test description".to_string(),
            mathematical_definition: None,
            code_refs: CodeReferences {
                rust: "test::func".to_string(),
                python: "test.func".to_string(),
                nodejs: "test.func".to_string(),
            },
            examples: vec![Example {
                title: "Example 1".to_string(),
                explanation: "An example".to_string(),
                code: CodeSnippets {
                    rust: "test()".to_string(),
                    python: "test()".to_string(),
                    nodejs: "test()".to_string(),
                },
                expected_output: Some("42".to_string()),
            }],
            use_cases: vec!["Use case 1".to_string()],
            related_topics: vec![],
            performance: None,
            interactive_playground: None,
            outputs: OutputHints::default(),
            metadata: None,
        };

        let generator = JupyterGenerator::new();
        let output = generator.generate(&schema).expect("Generation failed");

        // Should be valid JSON
        assert!(generator.validate_output(&output).is_ok());

        // Should contain title
        assert!(output.contains("Test Function"));

        // Should contain Python code
        assert!(output.contains("test()"));

        // Should have correct nbformat
        assert!(output.contains("\"nbformat\": 4"));
    }
}
```

### Step 4: Update Cargo.toml

**File**: `crates/kb-jupyter/Cargo.toml`

```toml
[package]
name = "kb-jupyter"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
kb-core = { path = "../kb-core" }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
```

### Step 5: Integration Test

**File**: `crates/kb-jupyter/tests/generate_derivative.rs`

```rust
use kb_jupyter::JupyterGenerator;
use kb_core::{OutputGenerator, Schema};
use std::path::PathBuf;

#[test]
fn test_generate_derivative_notebook() {
    // Load derivative schema
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("schemas/examples/derivative.yaml");

    let schema = Schema::load_from_file(&path)
        .expect("Failed to load derivative.yaml");

    // Generate notebook
    let generator = JupyterGenerator::new();
    let notebook_json = generator.generate(&schema)
        .expect("Failed to generate notebook");

    // Validate output
    assert!(generator.validate_output(&notebook_json).is_ok());

    // Check content
    assert!(notebook_json.contains("Symbolic Differentiation"));
    assert!(notebook_json.contains("Power Rule"));
    assert!(notebook_json.contains("symbol('x')"));

    println!("✅ Successfully generated Jupyter notebook from derivative.yaml");
}
```

### Step 6: Test

```bash
cargo test -p kb-jupyter
```

Should pass all tests!

### Step 7: Manual Verification

```bash
# Generate notebook
cargo run -p kb-cli -- build \
  --schema schemas/examples/derivative.yaml \
  --format jupyter \
  --output output/

# Open in Jupyter
jupyter notebook output/calculus-derivative.ipynb
```

---

## ✅ Completion Checklist

- [ ] Types defined in `notebook.rs`
- [ ] Generator implements `OutputGenerator` trait
- [ ] Markdown cells for title, description, examples
- [ ] Code cells with Python code
- [ ] Mathematical definitions handled (LaTeX)
- [ ] Use cases section included
- [ ] JSON validation in `validate_output()`
- [ ] Unit tests pass
- [ ] Integration test with derivative.yaml passes
- [ ] Manual verification in Jupyter works

---

## 🎯 After Jupyter Generator

Once Jupyter generator is complete:

1. **mdBook Generator** - Similar process, generates markdown
2. **LLM RAG Generator** - Chunked markdown for AI retrieval
3. **CLI Tool** - Wire up generators to command-line interface
4. **Templates** - Move hardcoded formatting to Tera templates
5. **Vue SSR** - Node.js generator for website

---

## 📚 Resources

**Jupyter Format**:
- https://nbformat.readthedocs.io/en/latest/format_description.html

**Testing**:
```bash
cargo test -p kb-jupyter          # Run tests
cargo test -p kb-jupyter -- --nocapture  # With output
cargo test test_load_derivative_schema   # Specific test
```

**Build**:
```bash
cargo build -p kb-jupyter         # Build only Jupyter generator
cargo build --release             # Build all (optimized)
```

---

## 🚨 Common Pitfalls

1. **Forgot to validate JSON**: Always call `validate_output()` in tests
2. **Line endings in cells**: Jupyter wants `"line\n"` not `"line"`
3. **Empty outputs array**: Code cells need `"outputs": []` not `null`
4. **nbformat version**: Must be exactly `4`, not `"4"`
5. **cell_type**: Must be `"code"` or `"markdown"`, not `"Code"`

---

**Good luck! 🚀**

If stuck, check:
1. `CLAUDE.md` for critical rules
2. `PROJECT_STATUS.md` for architecture decisions
3. `schemas/examples/derivative.yaml` for schema structure
4. `crates/kb-core/src/schema/mod.rs` for type definitions
