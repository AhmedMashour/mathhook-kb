# Generator Guide

> Step-by-step guide for implementing new output format generators.

## Overview

Generators transform YAML schemas into specific documentation formats. Each generator implements the `OutputGenerator` trait and uses Tera templates for rendering.

```
Generator: Schema → OutputFormat
```

## Generator Checklist

### Before You Start

1. **Choose your format**: What documentation format are you targeting?
2. **Study existing generators**: Look at `kb-jupyter` or `kb-mdbook` for patterns
3. **Plan your templates**: What sections will your output include?

### Implementation Steps

1. Create crate: `cargo new --lib crates/kb-{format}`
2. Add dependencies: `serde`, `tera`, `anyhow`, `kb-core`
3. Implement trait: `OutputGenerator` with all 4 methods
4. Create templates: In `templates/{format}/`
5. Write tests: Unit + integration tests
6. Register generator: Add to CLI command
7. Document: Update README with new format

## Implementing the Trait

### Trait Definition

```rust
pub trait OutputGenerator {
    /// Generator name (e.g., "jupyter")
    fn name(&self) -> &str;

    /// Output file extension (e.g., "ipynb")
    fn file_extension(&self) -> &str;

    /// Generate output from validated schema
    fn generate(&self, schema: &Schema) -> Result<String>;

    /// Validate the generated output
    fn validate_output(&self, output: &str) -> Result<()>;
}
```

### Example Implementation

```rust
use anyhow::Result;
use kb_core::schema::Schema;
use std::path::PathBuf;
use tera::{Context, Tera};

pub struct MarkdownGenerator {
    template_dir: PathBuf,
    tera: Tera,
}

impl MarkdownGenerator {
    pub fn new(template_dir: &str) -> Result<Self> {
        let tera = Tera::new(&format!("{}/**/*.tera", template_dir))?;
        Ok(Self {
            template_dir: PathBuf::from(template_dir),
            tera,
        })
    }
}

impl OutputGenerator for MarkdownGenerator {
    fn name(&self) -> &str {
        "markdown"
    }

    fn file_extension(&self) -> &str {
        "md"
    }

    fn generate(&self, schema: &Schema) -> Result<String> {
        let mut context = Context::new();
        context.insert("title", &schema.title);
        context.insert("description", &schema.description);
        context.insert("examples", &schema.examples);
        context.insert("math_def", &schema.mathematical_definition);

        self.tera.render("markdown/doc.tera", &context)
            .map_err(|e| anyhow::anyhow!("Template error: {}", e))
    }

    fn validate_output(&self, output: &str) -> Result<()> {
        // Check markdown structure
        if !output.starts_with("# ") {
            anyhow::bail!("Markdown must start with H1 header");
        }
        Ok(())
    }
}
```

## Creating Templates

### Template Organization

```
templates/markdown/
├── doc.tera           # Main document template
├── example.tera       # Example section partial
├── sidebar.tera       # Sidebar partial
└── macros.tera        # Reusable macros
```

### Example Template

```jinja2
{# templates/markdown/doc.tera #}

# {{ title }}

{{ description }}

{% if math_def %}
## Definition

{{ math_def }}
{% endif %}

## Examples

{% for example in examples %}
### {{ example.title }}

{{ example.explanation }}

#### Python
```python
{{ example.code.python }}
```

#### Rust
```rust
{{ example.code.rust }}
```

#### JavaScript
```javascript
{{ example.code.nodejs }}
```

{% if example.expected_output %}
**Output:**
```
{{ example.expected_output }}
```
{% endif %}

{% endfor %}

{% if use_cases %}
## Use Cases

{% for use_case in use_cases %}
- {{ use_case }}
{% endfor %}
{% endif %}
```

### Template Guidelines

1. **Keep templates simple**: Logic in Rust, presentation in templates
2. **Use partials**: Break complex templates into smaller pieces
3. **Handle missing fields**: Use `{% if field %}` for optional content
4. **Escape properly**: Use appropriate escaping for target format
5. **Test with edge cases**: Empty arrays, missing fields, special characters

### Custom Filters

```rust
fn register_custom_filters(tera: &mut Tera) {
    // Escape LaTeX special characters
    tera.register_filter("escape_latex", |value, _| {
        let s = value.as_str().unwrap_or("");
        Ok(tera::Value::String(
            s.replace("\\", "\\\\")
             .replace("$", "\\$")
             .replace("%", "\\%")
        ))
    });

    // Convert to code block
    tera.register_filter("code_block", |value, args| {
        let code = value.as_str().unwrap_or("");
        let lang = args.get("lang")
            .and_then(|v| v.as_str())
            .unwrap_or("text");
        Ok(tera::Value::String(format!("```{}\n{}\n```", lang, code)))
    });
}
```

## Validation Strategies

### Output Validation by Format

| Format | Validation Method |
|--------|-------------------|
| `.ipynb` | Parse JSON, check nbformat schema |
| `.md` | Check headers, validate code blocks |
| `.tex` | Run `pdflatex --interaction=nonstopmode` |
| `.vue` | Vue SFC parser (eslint-plugin-vue) |
| `.json` | JSON.parse() + schema validation |

### Common Validation Checks

1. **Structure**: Required sections present
2. **Syntax**: Valid for target format
3. **Content**: Schema content appears in output
4. **Encoding**: UTF-8, proper escaping
5. **Idempotency**: Same input = same output

### Example Validation

```rust
fn validate_output(&self, output: &str) -> Result<()> {
    // 1. Parse as target format
    let parsed: serde_json::Value = serde_json::from_str(output)
        .context("Invalid JSON output")?;

    // 2. Check required structure
    let cells = parsed.get("cells")
        .context("Missing 'cells' field")?
        .as_array()
        .context("'cells' must be an array")?;

    if cells.is_empty() {
        anyhow::bail!("Notebook must have at least one cell");
    }

    // 3. Validate each cell
    for (i, cell) in cells.iter().enumerate() {
        validate_cell(cell)
            .with_context(|| format!("Invalid cell at index {}", i))?;
    }

    Ok(())
}
```

## Writing Tests

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_returns_correct_value() {
        let gen = MarkdownGenerator::new("templates/").unwrap();
        assert_eq!(gen.name(), "markdown");
    }

    #[test]
    fn test_file_extension() {
        let gen = MarkdownGenerator::new("templates/").unwrap();
        assert_eq!(gen.file_extension(), "md");
    }
}
```

### Integration Tests

```rust
#[test]
fn test_generate_creates_valid_output() {
    // Load example schema
    let schema = Schema::load("tests/schemas/derivative.yaml")
        .expect("Failed to load schema");

    // Generate output
    let generator = MarkdownGenerator::new("templates/")
        .expect("Failed to initialize generator");
    let output = generator.generate(&schema)
        .expect("Failed to generate output");

    // Validate structure
    assert!(output.starts_with("# "));
    assert!(output.contains("## Examples"));

    // Validate content
    assert!(output.contains(&schema.title));
    assert!(output.contains(&schema.description));

    // Validate with generator's validator
    generator.validate_output(&output)
        .expect("Output validation failed");
}

#[test]
fn test_idempotency() {
    let schema = Schema::load("tests/schemas/derivative.yaml").unwrap();
    let generator = MarkdownGenerator::new("templates/").unwrap();

    let output1 = generator.generate(&schema).unwrap();
    let output2 = generator.generate(&schema).unwrap();

    assert_eq!(output1, output2, "Generator must be idempotent");
}
```

## CLI Integration

### Registering Your Generator

```rust
// crates/kb-cli/src/main.rs
use kb_markdown::MarkdownGenerator;

fn build_generators(template_dir: &str) -> Vec<Box<dyn OutputGenerator>> {
    vec![
        Box::new(JupyterGenerator::new(&format!("{}/jupyter", template_dir))),
        Box::new(MdBookGenerator::new(&format!("{}/mdbook", template_dir))),
        Box::new(MarkdownGenerator::new(&format!("{}/markdown", template_dir))),
        // Add your generator here
    ]
}
```

### CLI Commands

```bash
# Generate all formats
mathhook-kb build --schema schemas/derivative.yaml

# Generate specific format
mathhook-kb build --schema schemas/derivative.yaml --format markdown

# Validate only (no generation)
mathhook-kb validate --schema schemas/

# Watch mode
mathhook-kb watch --schema schemas/ --output output/
```

## Critical Requirements

Your generator **MUST**:

1. **Validate output before writing** - Never write invalid files
2. **Produce identical output on repeated runs** - No timestamps, random IDs
3. **Handle all schema fields** - Required and optional
4. **Report errors with context** - File path, line number, field name
5. **Use templates for formatting** - Keep logic in Rust, presentation in templates

## Common Pitfalls

1. **Non-deterministic output**: Using timestamps, random UUIDs
2. **Silent failures**: Not validating output before writing
3. **Hardcoded paths**: Use configurable template/output directories
4. **Missing escaping**: Special characters in target format
5. **Template logic**: Complex logic belongs in Rust, not templates

## Example Generators to Study

| Generator | Format | Key Features |
|-----------|--------|--------------|
| `kb-jupyter` | `.ipynb` | JSON output, complex cell structure |
| `kb-mdbook` | `.md` | Markdown with mdBook syntax |
| `kb-vue` | `.json` | Data for Vue consumption |
| `kb-latex` | `.tex` | Escaped special characters |

## Related Documentation

- [CLAUDE.md](../CLAUDE.md) - AI agent development guide
- [Architecture](./architecture.md) - System design
- [Schema Reference](./schema-reference.md) - Schema format specification
