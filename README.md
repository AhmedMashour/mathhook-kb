# MathHook Knowledge Base Engine

Multi-format documentation generation engine for [MathHook CAS](https://github.com/AhmedMashour/mathhook).

## Overview

The MathHook KB Engine transforms **single schema definitions** into **12+ documentation formats**:

- 📓 **Jupyter Notebooks** - Interactive Python tutorials
- 🌐 **Google Colab** - Zero-install onboarding
- 📚 **mdBook** - Rust documentation
- 🎨 **Vue SSR Site** - Marketing and learning hub (mathhook.org)
- 🔍 **API Documentation** - Interactive reference with live playground
- 🤖 **LLM-Optimized RAG** - AI agent-friendly chunks
- 📄 **LaTeX/PDF** - Academic paper templates
- 📊 **Observable Notebooks** - Interactive visualizations
- ⚡ **Benchmark Dashboard** - Performance comparisons
- 🔗 **LangChain Integration** - Agent framework guides
- 🛠️ **MCP Server Docs** - AI code navigation
- 🎥 **Video Companions** - Tutorial transcripts + notebooks

**Core Principle**: Write documentation **once** in YAML schemas, generate **all formats** automatically.

## Quick Start

### Installation

```bash
# From source
cargo install --path crates/kb-cli

# Or via cargo
cargo install mathhook-kb
```

### Usage

```bash
# Generate all formats from schemas
mathhook-kb build --schemas schemas/mathhook/

# Validate schemas
mathhook-kb validate --schemas schemas/

# Watch mode for development
mathhook-kb watch --schemas schemas/ --output output/

# Generate specific format
mathhook-kb build --format jupyter --output notebooks/
```

## Schema Example

`schemas/calculus/derivative.yaml`:

```yaml
topic: "calculus.derivative"
title: "Symbolic Differentiation"

description: |
  Computes the derivative of an expression with respect to a variable
  using symbolic differentiation rules (power rule, chain rule, etc.).

code_refs:
  rust: "mathhook_core::calculus::derivative"
  python: "mathhook.calculus.derivative"
  nodejs: "mathhook.calculus.derivative"

examples:
  - title: "Power Rule"
    explanation: "Derivative of x^n is n*x^(n-1)"
    code:
      rust: |
        let x = symbol!(x);
        let f = expr!(x ^ 3);
        let df = f.derivative(&x, 1);
      python: |
        x = symbol('x')
        f = expr('x^3')
        df = f.derivative(x)
      nodejs: |
        const x = symbol('x');
        const f = expr('x^3');
        const df = f.derivative(x);
```

**Output**: This single schema generates:
- Jupyter notebook with executable Python code
- mdBook page with Rust examples
- Vue component with tabbed code samples
- API documentation with live playground
- LLM-optimized RAG chunks
- And 7+ more formats!

## Architecture

```
mathhook-kb/
├── crates/
│   ├── kb-core           # Schema parsing & validation
│   ├── kb-cli            # CLI tool
│   ├── kb-jupyter        # Jupyter generator
│   ├── kb-mdbook         # mdBook generator
│   ├── kb-vue            # Vue SSR data generator
│   ├── kb-api-docs       # API docs generator
│   ├── kb-llm-rag        # LLM-optimized markdown
│   └── kb-latex          # LaTeX generator
├── templates/            # Tera templates
├── schemas/              # Schema definitions
└── generators/
    └── vue-site/         # Nuxt 3 SSR site
```

## Documentation

- [Architecture](./docs/architecture.md) - System design
- [Schema Reference](./docs/schema-reference.md) - Schema format specification
- [Generator Guide](./docs/generator-guide.md) - Writing new generators

## Development

```bash
# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Test
cargo test --all --no-fail-fast

# Build
cargo build --release
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md).

## Related Projects

- [MathHook](https://github.com/AhmedMashour/mathhook) - Symbolic mathematics CAS in Rust
- [mdBook](https://rust-lang.github.io/mdBook/) - Markdown book generator
- [Jupyter](https://jupyter.org/) - Interactive computing notebooks
- [Nuxt](https://nuxt.com/) - Vue SSR framework
