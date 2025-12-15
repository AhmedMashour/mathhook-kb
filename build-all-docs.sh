#!/bin/bash

# MathHook Documentation Build Script
# Generates all documentation formats from YAML schemas with proper organization

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUTS_DIR="$SCRIPT_DIR/outputs"
SCHEMAS_DIR="$SCRIPT_DIR/schemas"

echo "🔨 Building MathHook Documentation"
echo "=================================="
echo ""

# Define output base directories
MDBOOK_DIR="$OUTPUTS_DIR/mdbook/src"
JUPYTER_DIR="$OUTPUTS_DIR/jupyter-book"
VUE_DIR="$OUTPUTS_DIR/vue-site/pages"
API_DIR="$OUTPUTS_DIR/api-docs"
COLAB_DIR="$OUTPUTS_DIR/colab"
RAG_DIR="$OUTPUTS_DIR/llm-rag"
LATEX_DIR="$OUTPUTS_DIR/latex"

# Create category subdirectories
for dir in "$MDBOOK_DIR" "$JUPYTER_DIR" "$VUE_DIR" "$API_DIR" "$COLAB_DIR" "$RAG_DIR" "$LATEX_DIR"; do
    mkdir -p "$dir"/{calculus,algebra,ode,pde,special_functions,getting-started}
done

# Function to build docs for a schema
build_schema() {
    local schema_file=$1
    local category=$2
    local topic=$(basename "$schema_file" .yaml)

    echo "📄 Processing: $category/$topic"

    # Build to temporary directory
    cargo run --release -p kb-cli -- build "$schema_file" \
        --output "$OUTPUTS_DIR/temp" \
        --generators all 2>&1 | grep -E "(✅|Error)" || true

    # Move generated files to proper locations
    if [ -d "$OUTPUTS_DIR/temp" ]; then
        # mdBook
        if [ -f "$OUTPUTS_DIR/temp"/*.md ] && [ ! -f "$OUTPUTS_DIR/temp"/*.rag.md ]; then
            mv "$OUTPUTS_DIR/temp"/*.md "$MDBOOK_DIR/$category/" 2>/dev/null || true
        fi

        # Jupyter
        if ls "$OUTPUTS_DIR/temp"/*.ipynb 2>/dev/null | grep -v ".colab.ipynb" > /dev/null; then
            mv "$OUTPUTS_DIR/temp"/*.ipynb "$JUPYTER_DIR/$category/" 2>/dev/null || true
        fi

        # Vue
        if [ -f "$OUTPUTS_DIR/temp"/*.vue ]; then
            mv "$OUTPUTS_DIR/temp"/*.vue "$VUE_DIR/$category/" 2>/dev/null || true
        fi

        # API docs
        if [ -f "$OUTPUTS_DIR/temp"/*.json ]; then
            mv "$OUTPUTS_DIR/temp"/*.json "$API_DIR/$category/" 2>/dev/null || true
        fi

        # Colab
        if [ -f "$OUTPUTS_DIR/temp"/*.colab.ipynb ]; then
            mv "$OUTPUTS_DIR/temp"/*.colab.ipynb "$COLAB_DIR/$category/" 2>/dev/null || true
        fi

        # LLM-RAG
        if [ -f "$OUTPUTS_DIR/temp"/*.rag.md ]; then
            mv "$OUTPUTS_DIR/temp"/*.rag.md "$RAG_DIR/$category/" 2>/dev/null || true
        fi

        # LaTeX
        if [ -f "$OUTPUTS_DIR/temp"/*.tex ]; then
            mv "$OUTPUTS_DIR/temp"/*.tex "$LATEX_DIR/$category/" 2>/dev/null || true
        fi

        # Cleanup
        rm -rf "$OUTPUTS_DIR/temp"
    fi
}

# Build calculus schemas
echo "📐 Building Calculus Documentation"
echo "-----------------------------------"
for schema in "$SCHEMAS_DIR/calculus"/*.yaml; do
    if [ -f "$schema" ]; then
        build_schema "$schema" "calculus"
    fi
done

# Build algebra schemas
echo ""
echo "🔷 Building Algebra Documentation"
echo "-----------------------------------"
for schema in "$SCHEMAS_DIR/algebra"/*.yaml; do
    if [ -f "$schema" ]; then
        build_schema "$schema" "algebra"
    fi
done

# Build ODE schemas
echo ""
echo "📊 Building ODE Documentation"
echo "-----------------------------------"
for schema in "$SCHEMAS_DIR/ode"/*.yaml; do
    if [ -f "$schema" ]; then
        build_schema "$schema" "ode"
    fi
done

# Build PDE schemas
echo ""
echo "🌊 Building PDE Documentation"
echo "-----------------------------------"
for schema in "$SCHEMAS_DIR/pde"/*.yaml; do
    if [ -f "$schema" ]; then
        build_schema "$schema" "pde"
    fi
done

# Build special functions schemas
echo ""
echo "⚡ Building Special Functions Documentation"
echo "-----------------------------------"
for schema in "$SCHEMAS_DIR/special_functions"/*.yaml; do
    if [ -f "$schema" ]; then
        build_schema "$schema" "special_functions"
    fi
done

echo ""
echo "✨ Build Complete!"
echo "=================="
echo ""
echo "📚 Outputs:"
echo "  - mdBook:        $MDBOOK_DIR"
echo "  - Jupyter Book:  $JUPYTER_DIR"
echo "  - Vue Site:      $VUE_DIR"
echo "  - API Docs:      $API_DIR"
echo "  - Colab:         $COLAB_DIR"
echo "  - LLM-RAG:       $RAG_DIR"
echo "  - LaTeX:         $LATEX_DIR"
echo ""
echo "📘 Building mdBook..."
cd "$OUTPUTS_DIR/mdbook" && mdbook build
echo "   ✅ mdBook built successfully"

echo ""
echo "📋 Copying outputs to docs website..."
DOCS_PUBLIC="$SCRIPT_DIR/mathhook-docs-site/public/outputs"
mkdir -p "$DOCS_PUBLIC"
cp -r "$OUTPUTS_DIR/mdbook/book" "$DOCS_PUBLIC/mdbook"
echo "   ✅ mdBook copied to website"

echo ""
echo "🚀 Next Steps:"
echo "  - Build Jupyter Book: cd outputs/jupyter-book && jupyter-book build ."
echo "  - View outputs page:  http://localhost:3000/outputs"
echo "  - View mdBook:        http://localhost:3000/outputs/mdbook/index.html"
