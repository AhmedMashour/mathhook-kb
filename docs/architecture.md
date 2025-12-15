# System Architecture

> Deep dive into the MathHook Knowledge Base Engine - a multi-format documentation generation system.

## Overview

The MathHook Knowledge Base Engine transforms **single schema definitions** into **12+ output formats**:

```
Schema (YAML) → Generators → {Jupyter, mdBook, Vue, LaTeX, ...}
```

**Core Principle**: Single Source of Truth - one schema file generates all formats.

## Repository Structure

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
├── docs/                  # Documentation (this folder)
└── scripts/               # Build and validation scripts
```

## Output Formats

### Interactive Formats
1. **Jupyter Notebooks** (`.ipynb`) - Interactive Python tutorials
2. **Google Colab Notebooks** - Zero-install onboarding with Colab metadata
3. **Observable Notebooks** (`.js`) - Interactive visualizations

### Documentation Formats
4. **mdBook** (`.md`) - Rust-style documentation
5. **Vue SSR Static Site** - Marketing/learning hub with Vue components
6. **Interactive API Docs** - VitePress-compatible with live playground

### Academic Formats
7. **LaTeX/PDF** - Academic paper templates
8. **LLM-Optimized RAG Markdown** - Chunked for AI agent retrieval

### Integration Formats
9. **Benchmark Dashboard** (JSON) - Performance comparisons
10. **LangChain Integration Guides** - Agent framework docs
11. **MCP Server** - AI code navigation
12. **Video Tutorial Companions** - Transcripts + notebooks

## Technology Stack

### Core Engine (Rust)

| Category | Dependencies |
|----------|-------------|
| Schema & Serialization | `serde`, `serde_yaml`, `serde_json` |
| Templating | `tera` (Jinja2-like) |
| CLI | `clap` (derive-based) |
| Code Analysis | `syn`, `tree-sitter` |
| Error Handling | `anyhow`, `thiserror` |

### Vue SSR Builder

- **Nuxt 3** - Vue SSR framework
- **Tailwind CSS** - Utility-first styling
- **KaTeX** - Math rendering
- **Prism.js** - Syntax highlighting

## Generator Trait Pattern

All generators implement a common trait:

```rust
pub trait OutputGenerator {
    fn name(&self) -> &str;
    fn file_extension(&self) -> &str;
    fn generate(&self, schema: &Schema) -> Result<String>;
    fn validate_output(&self, output: &str) -> Result<()>;
}
```

Example implementation:

```rust
impl OutputGenerator for JupyterGenerator {
    fn name(&self) -> &str { "jupyter" }
    fn file_extension(&self) -> &str { "ipynb" }

    fn generate(&self, schema: &Schema) -> Result<String> {
        // Generate .ipynb JSON using Tera templates
    }

    fn validate_output(&self, output: &str) -> Result<()> {
        // Validate notebook JSON structure
    }
}
```

## Data Flow Pipeline

```
1. Parse YAML Schema
   ├── serde_yaml::from_str()
   └── Deserialize into Schema struct

2. Validate Schema
   ├── Check required fields
   ├── Validate code references
   └── Check example syntax

3. For Each Generator
   ├── Load Tera templates
   ├── Build Context from Schema
   ├── Render template
   └── Validate output

4. Write Output Files
   ├── derivative.ipynb
   ├── derivative.md
   ├── derivative.json (for Vue)
   └── derivative.tex
```

## Template Engine (Tera)

Templates separate content from presentation:

```
schemas/calculus/derivative.yaml  → Content (what to show)
templates/jupyter/notebook.tera   → Presentation (how to show)
```

Example template:

```jinja2
# {{ title }}

{{ description }}

{% if mathematical_definition %}
## Definition

{{ mathematical_definition }}
{% endif %}

## Examples

{% for example in examples %}
### {{ example.title }}

{{ example.explanation }}

```python
{{ example.code.python }}
```
{% endfor %}
```

## Extension Points

### Adding a New Generator

1. Create new crate: `crates/kb-{format}/`
2. Implement `OutputGenerator` trait
3. Create templates: `templates/{format}/`
4. Add to generator registry in CLI
5. Update documentation

### Adding Schema Fields

1. Update `Schema` struct in `kb-core`
2. Add validation rules
3. Update all generators to use new field
4. Update templates
5. Document in [Schema Reference](./schema-reference.md)

## Performance Targets

- Schema parsing and generation: **< 1 second** per schema
- Optimization strategies:
  - Parse schemas in parallel (rayon)
  - Cache template compilation
  - Lazy evaluation where possible

## Related Documentation

- [CLAUDE.md](../CLAUDE.md) - AI agent development guide
- [Schema Reference](./schema-reference.md) - Schema format specification
- [Generator Guide](./generator-guide.md) - Writing new generators
