#!/bin/bash
# build-all-outputs.sh - Complete build pipeline for all KB outputs
# Usage: ./scripts/build-all-outputs.sh
#
# This script:
# 1. Runs KB CLI to generate raw outputs from schemas
# 2. Builds mdBook from generated .md files
# 3. Builds Jupyter Book from generated .ipynb files
# 4. Deploys all outputs to docs site public folder

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
SCHEMAS_DIR="$ROOT_DIR/schemas"
OUTPUTS_DIR="$ROOT_DIR/outputs"
BUILD_DIR="$ROOT_DIR/build"
DOCS_PUBLIC="$ROOT_DIR/mathhook-docs-site/public/outputs"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}╔════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║     MathHook KB - Complete Output Build Pipeline    ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════╝${NC}"
echo ""

# Step 1: Generate raw outputs using KB CLI
echo -e "${YELLOW}[1/5] Generating raw outputs from schemas...${NC}"
if [ -d "$SCHEMAS_DIR" ]; then
    # Build the CLI if not already built
    cargo build --release -p kb-cli 2>/dev/null || {
        echo -e "${RED}Failed to build KB CLI. Skipping generation step.${NC}"
        echo -e "${YELLOW}Using existing outputs in $OUTPUTS_DIR${NC}"
    }

    # Run generation for all schemas
    if [ -x "$ROOT_DIR/target/release/kb-cli" ]; then
        find "$SCHEMAS_DIR" -name "*.yaml" -o -name "*.yml" | while read schema; do
            echo "  Processing: $(basename "$schema")"
            "$ROOT_DIR/target/release/kb-cli" build "$schema" --output "$OUTPUTS_DIR" 2>/dev/null || true
        done
    fi
else
    echo -e "${YELLOW}No schemas directory found. Using existing outputs.${NC}"
fi

# Step 2: Build mdBook
echo -e "${YELLOW}[2/5] Building mdBook...${NC}"
MDBOOK_SRC="$BUILD_DIR/mdbook"
mkdir -p "$MDBOOK_SRC/src"

# Check if mdbook is installed
if command -v mdbook &> /dev/null; then
    # Create book.toml
    cat > "$MDBOOK_SRC/book.toml" << 'BOOKTOML'
[book]
title = "MathHook Documentation"
authors = ["MathHook Team"]
description = "Comprehensive documentation for MathHook symbolic mathematics library"
language = "en"

[build]
build-dir = "book"

[output.html]
default-theme = "ayu"
preferred-dark-theme = "ayu"
git-repository-url = "https://github.com/AhmedMashour/mathhook"
edit-url-template = "https://github.com/AhmedMashour/mathhook/edit/master/docs/{path}"

[output.html.fold]
enable = true
level = 1
BOOKTOML

    # Copy all .md files to src
    cp "$OUTPUTS_DIR"/*.md "$MDBOOK_SRC/src/" 2>/dev/null || echo "  No .md files found"

    # Generate SUMMARY.md
    echo "# Summary" > "$MDBOOK_SRC/src/SUMMARY.md"
    echo "" >> "$MDBOOK_SRC/src/SUMMARY.md"
    echo "- [Introduction](./README.md)" >> "$MDBOOK_SRC/src/SUMMARY.md"
    echo "" >> "$MDBOOK_SRC/src/SUMMARY.md"

    # Group files by category and write summary
    current_category=""
    for file in $(ls "$MDBOOK_SRC/src"/*.md 2>/dev/null | sort); do
        if [ -f "$file" ]; then
            filename=$(basename "$file" .md)
            if [ "$filename" = "SUMMARY" ] || [ "$filename" = "README" ]; then
                continue
            fi
            # Extract category from filename (before first -)
            category=$(echo "$filename" | cut -d'-' -f1)

            # Start new category section if needed
            if [ "$category" != "$current_category" ]; then
                current_category="$category"
                # Capitalize first letter
                cap_category=$(echo "$category" | awk '{print toupper(substr($0,1,1)) substr($0,2)}')
                echo "" >> "$MDBOOK_SRC/src/SUMMARY.md"
                echo "## $cap_category" >> "$MDBOOK_SRC/src/SUMMARY.md"
                echo "" >> "$MDBOOK_SRC/src/SUMMARY.md"
            fi

            # Create title from filename
            title=$(echo "$filename" | sed 's/-/ /g' | awk '{for(i=1;i<=NF;i++)$i=toupper(substr($i,1,1))substr($i,2)}1')
            echo "- [$title](./${filename}.md)" >> "$MDBOOK_SRC/src/SUMMARY.md"
        fi
    done

    # Create README
    cat > "$MDBOOK_SRC/src/README.md" << 'README'
# MathHook Documentation

Welcome to the MathHook documentation! This book contains comprehensive guides for symbolic mathematics operations.

## Getting Started

- Browse the topics in the sidebar
- Each topic includes examples in Rust, Python, and JavaScript
- Use the search function to find specific topics

## Topics

The documentation is organized by mathematical domain:
- **Algebra**: Simplification, solving equations, matrices
- **Calculus**: Derivatives, integrals, limits, series
- **Advanced**: Complex numbers, assumptions, special functions
README

    # Build mdBook
    cd "$MDBOOK_SRC"
    mdbook build

    # Copy to docs public
    rm -rf "$DOCS_PUBLIC/mdbook"
    cp -r "$MDBOOK_SRC/book" "$DOCS_PUBLIC/mdbook"
    echo -e "${GREEN}  ✓ mdBook built and deployed${NC}"
else
    echo -e "${RED}  mdbook not installed. Install with: cargo install mdbook${NC}"
    echo -e "${YELLOW}  Keeping existing mdBook output${NC}"
fi

# Step 3: Build Jupyter Book
echo -e "${YELLOW}[3/5] Building Jupyter Book...${NC}"
JBOOK_SRC="$BUILD_DIR/jupyter-book"
mkdir -p "$JBOOK_SRC"

if command -v jupyter-book &> /dev/null; then
    # Copy notebooks
    cp "$OUTPUTS_DIR"/*.ipynb "$JBOOK_SRC/" 2>/dev/null || echo "  No .ipynb files found"

    # Create _config.yml
    cat > "$JBOOK_SRC/_config.yml" << 'JBCONFIG'
title: MathHook Notebooks
author: MathHook Team
logo: ""
execute:
  execute_notebooks: "off"
repository:
  url: https://github.com/AhmedMashour/mathhook
  branch: master
html:
  use_issues_button: true
  use_repository_button: true
JBCONFIG

    # Create _toc.yml
    echo "format: jb-book" > "$JBOOK_SRC/_toc.yml"
    echo "root: intro" >> "$JBOOK_SRC/_toc.yml"
    echo "chapters:" >> "$JBOOK_SRC/_toc.yml"

    for nb in "$JBOOK_SRC"/*.ipynb; do
        if [ -f "$nb" ] && [[ ! "$(basename "$nb")" =~ \.colab\. ]]; then
            name=$(basename "$nb" .ipynb)
            echo "  - file: $name" >> "$JBOOK_SRC/_toc.yml"
        fi
    done

    # Create intro notebook
    cat > "$JBOOK_SRC/intro.md" << 'INTRO'
# MathHook Jupyter Notebooks

Interactive Python notebooks for learning MathHook symbolic mathematics.

## Contents

Browse the notebooks in the sidebar to explore different mathematical topics.
INTRO

    # Build Jupyter Book
    cd "$JBOOK_SRC"
    jupyter-book build . --all

    # Copy to docs public
    rm -rf "$DOCS_PUBLIC/jupyter-book"
    mkdir -p "$DOCS_PUBLIC/jupyter-book"
    cp -r "$JBOOK_SRC/_build" "$DOCS_PUBLIC/jupyter-book/"
    echo -e "${GREEN}  ✓ Jupyter Book built and deployed${NC}"
else
    echo -e "${RED}  jupyter-book not installed. Install with: pip install jupyter-book${NC}"
    echo -e "${YELLOW}  Keeping existing Jupyter Book output${NC}"
fi

# Step 4: Deploy raw outputs
echo -e "${YELLOW}[4/5] Deploying raw outputs...${NC}"
"$SCRIPT_DIR/deploy-outputs.sh"

# Step 5: Generate index pages
echo -e "${YELLOW}[5/5] Generating index pages...${NC}"
"$SCRIPT_DIR/generate-index-pages.sh"

echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║            Build Complete!                          ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Output locations:"
echo "  • mdBook:       $DOCS_PUBLIC/mdbook/"
echo "  • Jupyter Book: $DOCS_PUBLIC/jupyter-book/"
echo "  • API Docs:     $DOCS_PUBLIC/api-docs/"
echo "  • LaTeX:        $DOCS_PUBLIC/latex/"
echo "  • Colab:        $DOCS_PUBLIC/colab/"
echo "  • Jupyter:      $DOCS_PUBLIC/jupyter/"
echo "  • LLM-RAG:      $DOCS_PUBLIC/llm-rag/"
echo ""
echo "Next: cd mathhook-docs-site && npm run dev"
