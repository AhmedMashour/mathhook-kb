# MathHook API Documentation Schemas

Generated: 2025-12-15T14:56

This directory contains YAML schemas converted from MathHook's mdBook documentation for the knowledge base engine.

## Converted Schemas

| Schema | Source | Description | Size |
|--------|--------|-------------|------|
| `core_2025-12-15T1456.yaml` | `core/expressions.md`, `getting-started/basic-usage.md` | Expression creation, immutability, canonical forms | 7.5K |
| `algebra_2025-12-15T1456.yaml` | `operations/simplification.md` | Symbolic simplification, noncommutative algebra, power rule | 9.9K |
| `calculus_2025-12-15T1456.yaml` | `operations/differentiation.md`, `operations/integration.md` | Derivatives, integrals, 8-layer integration strategy | 12K |
| `solver_2025-12-15T1456.yaml` | `operations/solving.md` | Equation solving: linear, quadratic, polynomial, transcendental | 12K |
| `matrix_2025-12-15T1456.yaml` | `advanced/matrices.md` | Matrix operations, noncommutative multiplication, left/right division | 13K |
| `parser_2025-12-15T1456.yaml` | `parser/latex.md` | LaTeX parsing and generation, implicit multiplication, type inference | 12K |

## Schema Format

Each schema follows the mathhook-kb standard format:

```yaml
topic: "api.{category}.{feature}"
title: "Human-readable Title"
description: |
  Brief overview

article:
  content: |
    # Full markdown documentation

code_refs:
  rust: "mathhook_core::{module}"
  python: "mathhook.{module}"
  nodejs: "mathhook.{module}"

examples:
  - title: "Example Title"
    explanation: "What this demonstrates"
    code:
      rust: |
        # Rust code
      python: |
        # Python code
      nodejs: |
        # Node.js code

use_cases:
  - "Use case 1"
  - "Use case 2"

related_topics:
  - "api.other.topic"

performance:
  complexity: "O(n)"
  typical_time: "1ms"

metadata:
  schema_version: "1.0"
  source_file: "path/to/source.md"
  timestamp: "2025-12-15T14:56"
```

## Key Features

### Multi-Language Code Examples
All examples include equivalent code in:
- **Rust**: Using `expr!()` and `symbol!()` macros
- **Python**: Using idiomatic Python API
- **Node.js**: Using JavaScript/TypeScript API

### Mathematical Rigor
- LaTeX formulas for mathematical concepts
- Algorithm explanations and complexity analysis
- Performance characteristics documented
- Limitations explicitly stated

### Real-World Context
- Physics applications (velocity, acceleration)
- Engineering examples (circuits, transformations)
- Economics (break-even analysis)
- Quantum mechanics (Pauli matrices)

### Cross-References
Each schema includes `related_topics` linking to other API components.

## Original Sources

The schemas were converted from MathHook's mdBook documentation at:
`~/Documents/work/math/mathhook/docs/src/`

Original API stubs (minimal content) were at:
`~/Documents/work/math/mathhook/docs/src/api/*.md`

Full documentation extracted from:
- `operations/` - Mathematical operations
- `core/` - Core concepts
- `advanced/` - Advanced features
- `parser/` - Parsing and formatting
- `getting-started/` - Basic usage

## Timestamp Format

All files follow the naming convention:
`{topic}_2025-12-15T1456.yaml`

This AI-friendly timestamp format ensures:
- Unique filenames
- Clear versioning
- Chronological sorting

## Usage

These schemas will be consumed by the mathhook-kb engine to generate:
1. Jupyter notebooks (`.ipynb`)
2. Google Colab notebooks
3. mdBook documentation
4. Vue SSR static site
5. Interactive API docs (VitePress)
6. LLM-optimized RAG markdown
7. LaTeX/PDF
8. Observable notebooks
9. Benchmark dashboards
10. LangChain integration guides
11. MCP server metadata
12. Video tutorial companions

## Validation

To validate a schema:
```bash
cd /Users/ahmedmashhour/Documents/work/math/mathhook-kb
cargo run --bin mathhook-kb validate --schema schemas/api/core_2025-12-15T1456.yaml
```

To generate outputs:
```bash
cargo run --bin mathhook-kb build --schema schemas/api/ --output output/
```

## Notes

- All code examples are runnable and tested
- No placeholders or stub code
- Mathematical correctness validated
- Cross-language equivalence maintained
- Educational value emphasized
