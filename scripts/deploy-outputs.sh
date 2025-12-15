#!/bin/bash
# deploy-outputs.sh - Organize KB outputs for docs site deployment
# Usage: ./scripts/deploy-outputs.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
OUTPUTS_SRC="$ROOT_DIR/outputs"
DOCS_PUBLIC="$ROOT_DIR/mathhook-docs-site/public/outputs"

echo "📦 Deploying MathHook KB outputs..."
echo "   Source: $OUTPUTS_SRC"
echo "   Target: $DOCS_PUBLIC"

# Create output directories
mkdir -p "$DOCS_PUBLIC/latex"
mkdir -p "$DOCS_PUBLIC/llm-rag"
mkdir -p "$DOCS_PUBLIC/colab"
mkdir -p "$DOCS_PUBLIC/jupyter"

# Copy LaTeX files
echo "📄 Copying LaTeX files..."
cp "$OUTPUTS_SRC"/*.tex "$DOCS_PUBLIC/latex/" 2>/dev/null || echo "   No .tex files found"
TEX_COUNT=$(ls -1 "$DOCS_PUBLIC/latex"/*.tex 2>/dev/null | wc -l | tr -d ' ')
echo "   Copied $TEX_COUNT LaTeX files"

# Copy RAG markdown files
echo "🤖 Copying LLM-RAG files..."
cp "$OUTPUTS_SRC"/*.rag.md "$DOCS_PUBLIC/llm-rag/" 2>/dev/null || echo "   No .rag.md files found"
RAG_COUNT=$(ls -1 "$DOCS_PUBLIC/llm-rag"/*.rag.md 2>/dev/null | wc -l | tr -d ' ')
echo "   Copied $RAG_COUNT RAG files"

# Copy Colab notebooks
echo "📓 Copying Colab notebooks..."
cp "$OUTPUTS_SRC"/*.colab.ipynb "$DOCS_PUBLIC/colab/" 2>/dev/null || echo "   No .colab.ipynb files found"
COLAB_COUNT=$(ls -1 "$DOCS_PUBLIC/colab"/*.colab.ipynb 2>/dev/null | wc -l | tr -d ' ')
echo "   Copied $COLAB_COUNT Colab notebooks"

# Copy regular Jupyter notebooks (excluding colab ones)
echo "📔 Copying Jupyter notebooks..."
for f in "$OUTPUTS_SRC"/*.ipynb; do
  if [[ ! "$f" =~ \.colab\.ipynb$ ]]; then
    cp "$f" "$DOCS_PUBLIC/jupyter/"
  fi
done
JUPYTER_COUNT=$(ls -1 "$DOCS_PUBLIC/jupyter"/*.ipynb 2>/dev/null | wc -l | tr -d ' ')
echo "   Copied $JUPYTER_COUNT Jupyter notebooks"

echo ""
echo "✅ Output deployment complete!"
echo "   LaTeX:   $TEX_COUNT files"
echo "   RAG:     $RAG_COUNT files"
echo "   Colab:   $COLAB_COUNT files"
echo "   Jupyter: $JUPYTER_COUNT files"
echo ""
echo "📝 Next: Run 'npm run generate' in mathhook-docs-site to build the site"
