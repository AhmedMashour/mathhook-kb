#!/bin/bash
# build-mdbook.sh - Build mdbook from KB-generated markdown files
# Usage: ./scripts/build-mdbook.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
OUTPUTS_DIR="$ROOT_DIR/outputs"
BUILD_DIR="$ROOT_DIR/build/mdbook"
DOCS_PUBLIC="$ROOT_DIR/mathhook-docs-site/public/outputs/mdbook"

echo "📚 Building mdBook from KB outputs..."

# Check if mdbook is installed
if ! command -v mdbook &> /dev/null; then
    echo "❌ mdbook not installed. Install with: cargo install mdbook"
    exit 1
fi

# Clean and create build directory
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/src"

# Create book.toml (matching mathhook/docs settings)
cat > "$BUILD_DIR/book.toml" << 'EOF'
[book]
title = "MathHook KB Documentation"
authors = ["MathHook Contributors"]
description = "Comprehensive documentation for MathHook symbolic mathematics library"
language = "en"
src = "src"

[build]
build-dir = "book"
create-missing = false

[output.html]
default-theme = "ayu"
preferred-dark-theme = "ayu"
mathjax-support = false
git-repository-url = "https://github.com/AhmedMashour/mathhook"
git-repository-icon = "fa-github"
edit-url-template = "https://github.com/AhmedMashour/mathhook-kb/edit/master/outputs/{path}"
additional-css = ["custom.css"]

[output.html.fold]
enable = true
level = 1

[output.html.search]
enable = true
limit-results = 30
teaser-word-count = 30
use-boolean-and = true
boost-title = 2
boost-hierarchy = 1
boost-paragraph = 1
expand = true
heading-split-level = 3

[output.html.print]
enable = true

[output.html.playground]
runnable = true
editable = true

# KaTeX for math rendering (install with: cargo install mdbook-katex)
[preprocessor.katex]
after = ["links"]

[rust]
edition = "2021"
EOF

# Create custom CSS for MathHook theme
cat > "$BUILD_DIR/custom.css" << 'EOF'
:root {
  --rust-core: #E64524;
  --solve-cyan: #06B6D4;
}

.sidebar .sidebar-scrollbox {
  background: #0d1b2a;
}

.menu-title {
  color: var(--rust-core) !important;
}

h1, h2, h3 {
  color: #F5F5F5;
}

a {
  color: var(--solve-cyan);
}

code {
  background: rgba(6, 182, 212, 0.1);
  border-radius: 4px;
}

.ayu {
  --bg: #0d1b2a;
  --sidebar-bg: #152a45;
  --sidebar-active: var(--rust-core);
}
EOF

# Copy all .md files (not .rag.md) to src
echo "  Copying markdown files..."
for file in "$OUTPUTS_DIR"/*.md; do
    if [ -f "$file" ] && [[ ! "$file" =~ \.rag\.md$ ]]; then
        cp "$file" "$BUILD_DIR/src/"
    fi
done

MD_COUNT=$(ls -1 "$BUILD_DIR/src"/*.md 2>/dev/null | wc -l | tr -d ' ')
echo "  Copied $MD_COUNT markdown files"

# Generate SUMMARY.md
echo "  Generating SUMMARY.md..."
cat > "$BUILD_DIR/src/SUMMARY.md" << 'EOF'
# Summary

[Introduction](./README.md)

---

EOF

# Create README/intro
cat > "$BUILD_DIR/src/README.md" << 'EOF'
# MathHook Documentation

<div style="text-align: center; margin: 2em 0;">
<strong style="color: #E64524;">Symbolic Power</strong> ·
<strong style="color: #06B6D4;">Educational Clarity</strong> ·
<strong>Native Speed</strong>
</div>

Welcome to the MathHook documentation! This comprehensive guide covers all symbolic mathematics operations available in the MathHook library.

## Quick Navigation

Use the sidebar to browse topics by category:

- **Algebra** - Simplification, solving, matrices, polynomials
- **Calculus** - Derivatives, integrals, limits, series
- **ODE** - Ordinary differential equations and solving methods
- **PDE** - Partial differential equations
- **Advanced** - Complex numbers, assumptions, special functions
- **Core** - Expression building, symbols, evaluation

## Code Examples

Every topic includes working code examples in three languages:

```rust
// Rust
let x = symbol!(x);
let f = expr!(x^2 + 2*x + 1);
let df = f.derivative(&x);
```

```python
# Python
x = symbol('x')
f = x**2 + 2*x + 1
df = f.diff(x)
```

```javascript
// JavaScript
const x = symbol('x');
const f = expr('x^2 + 2*x + 1');
const df = f.derivative(x);
```

## Getting Started

1. Browse topics in the sidebar
2. Each page includes mathematical definitions and examples
3. Use the search function (🔍) to find specific topics
4. Click the expand arrows to see code in different languages

---

*Generated from MathHook Knowledge Base schemas*
EOF

# Group files by category and generate summary sections
echo "  Organizing by category..."

# Get unique categories
categories=$(ls "$BUILD_DIR/src"/*.md 2>/dev/null | xargs -I {} basename {} .md | grep -v "SUMMARY\|README" | cut -d'-' -f1 | sort -u)

for category in $categories; do
    # Capitalize category name
    cap_category=$(echo "$category" | awk '{print toupper(substr($0,1,1)) substr($0,2)}')

    echo "" >> "$BUILD_DIR/src/SUMMARY.md"
    echo "# $cap_category" >> "$BUILD_DIR/src/SUMMARY.md"
    echo "" >> "$BUILD_DIR/src/SUMMARY.md"

    # Add all files in this category
    for file in "$BUILD_DIR/src"/${category}-*.md; do
        if [ -f "$file" ]; then
            filename=$(basename "$file" .md)
            # Create readable title from filename
            title=$(echo "$filename" | sed "s/${category}-//" | sed 's/-/ /g' | sed 's/_/ /g' | awk '{for(i=1;i<=NF;i++)$i=toupper(substr($i,1,1))substr($i,2)}1')
            echo "- [$title](./${filename}.md)" >> "$BUILD_DIR/src/SUMMARY.md"
        fi
    done
done

echo "  Generated SUMMARY.md with $(echo "$categories" | wc -w | tr -d ' ') categories"

# Build mdbook
echo "  Running mdbook build..."
cd "$BUILD_DIR"
mdbook build

# Deploy to docs site
echo "  Deploying to docs site..."
rm -rf "$DOCS_PUBLIC"
cp -r "$BUILD_DIR/book" "$DOCS_PUBLIC"

# Count output files
HTML_COUNT=$(find "$DOCS_PUBLIC" -name "*.html" | wc -l | tr -d ' ')

echo ""
echo "✅ mdBook build complete!"
echo "   Source: $MD_COUNT markdown files"
echo "   Output: $HTML_COUNT HTML pages"
echo "   Location: $DOCS_PUBLIC"
