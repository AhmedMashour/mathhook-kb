# Contributing Documentation YAML Schemas

Generated: 2025-12-15T03:20:00Z

## Overview

This directory contains YAML schema representations of the MathHook contributing documentation. Each schema follows the hybrid format defined in the mathhook-kb project.

## Files

| Schema File | Source Markdown | Topic |
|-------------|-----------------|-------|
| `development_20251215_032000.yaml` | `development.md` | `contributing.development` |
| `architecture-reference_20251215_032000.yaml` | `architecture-reference.md` | `contributing.architecture-reference` |
| `testing_20251215_032000.yaml` | `testing.md` | `contributing.testing` |
| `style_20251215_032000.yaml` | `style.md` | `contributing.style` |
| `documentation_20251215_032000.yaml` | `documentation.md` | `contributing.documentation` |
| `correctness_20251215_032000.yaml` | `correctness.md` | `contributing.correctness` |

## Schema Structure

Each YAML file follows this structure:

```yaml
topic: "contributing.{topic-name}"
title: "{Document Title}"
description: |
  {Brief description}

article:
  content: |
    {Full markdown content from source file}

related_topics:
  - {links to related contributing topics}

metadata:
  schema_version: "1.0"
  source_file: "contributing/{filename}.md"
  generated_at: "2025-12-15T03:20:00Z"
```

## Schema Validation

All schemas should validate according to the mathhook-kb core validation rules:

- Required fields: `topic`, `title`, `description`, `article.content`, `metadata`
- Related topics properly cross-referenced
- Metadata includes source file tracking
- Timestamp in ISO 8601 format

## Generation Pipeline

These schemas serve as input to the mathhook-kb generator which produces:

1. Jupyter Notebooks (`.ipynb`)
2. Google Colab Notebooks
3. mdBook documentation
4. Vue SSR Static Site
5. Interactive API Docs
6. LLM-Optimized RAG Markdown
7. LaTeX/PDF
8. Observable Notebooks
9. Benchmark Dashboard
10. LangChain Integration Guides
11. MCP Server
12. Video Tutorial Companions

## Related Topics Graph

```
contributing.development
  ├─→ contributing.testing
  ├─→ contributing.style
  ├─→ contributing.documentation
  ├─→ contributing.correctness
  └─→ contributing.architecture-reference

contributing.architecture-reference
  ├─→ contributing.development
  ├─→ contributing.testing
  ├─→ contributing.style
  └─→ contributing.correctness

contributing.testing
  ├─→ contributing.development
  ├─→ contributing.correctness
  └─→ contributing.style

contributing.style
  ├─→ contributing.development
  ├─→ contributing.testing
  └─→ contributing.documentation

contributing.documentation
  ├─→ contributing.development
  ├─→ contributing.style
  └─→ contributing.testing

contributing.correctness
  ├─→ contributing.testing
  ├─→ contributing.development
  └─→ contributing.architecture-reference
```

## Usage

To generate outputs from these schemas:

```bash
# Build all outputs
mathhook-kb build --schema schemas/contributing/

# Build specific format
mathhook-kb build --schema schemas/contributing/ --output jupyter
mathhook-kb build --schema schemas/contributing/ --output mdbook

# Validate schemas
mathhook-kb validate --schema schemas/contributing/
```

## Maintenance

When updating source markdown files in `~/Documents/work/math/mathhook/docs/src/contributing/`:

1. Re-run the conversion script
2. Update the timestamp in filename
3. Update `generated_at` in metadata
4. Validate the new schema
5. Regenerate all outputs

## Notes

- Filenames include timestamp for versioning: `{topic}_YYYYMMDD_HHMMSS.yaml`
- All content is preserved verbatim from source markdown
- Code examples maintain original formatting
- LaTeX formulas preserved in markdown format for downstream processing
