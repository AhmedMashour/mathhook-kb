#!/bin/bash
# Generate all outputs from all schema files
# Generated: 2025-12-15T19:30:00Z

set -e

SCHEMA_DIR="${1:-schemas}"
OUTPUT_BASE="${2:-outputs}"

echo "🚀 Starting bulk output generation..."
echo "   Schema directory: $SCHEMA_DIR"
echo "   Output base: $OUTPUT_BASE"
echo ""

# Create output directories for each format
mkdir -p "$OUTPUT_BASE/jupyter"
mkdir -p "$OUTPUT_BASE/mdbook"
mkdir -p "$OUTPUT_BASE/llm-rag"
mkdir -p "$OUTPUT_BASE/vue"
mkdir -p "$OUTPUT_BASE/api-docs"
mkdir -p "$OUTPUT_BASE/colab"
mkdir -p "$OUTPUT_BASE/latex"

# Count files
TOTAL=$(find "$SCHEMA_DIR" -name "*.yaml" | wc -l | tr -d ' ')
PROCESSED=0
FAILED=0
FAILED_FILES=""

echo "📊 Found $TOTAL schema files to process"
echo ""

# Process each schema file
find "$SCHEMA_DIR" -name "*.yaml" | while read -r schema_file; do
    PROCESSED=$((PROCESSED + 1))

    # Extract relative path for subdirectory structure
    rel_path="${schema_file#$SCHEMA_DIR/}"
    dir_path=$(dirname "$rel_path")

    echo "[$PROCESSED/$TOTAL] Processing: $rel_path"

    # Create output subdirectories
    mkdir -p "$OUTPUT_BASE/jupyter/$dir_path"
    mkdir -p "$OUTPUT_BASE/mdbook/$dir_path"
    mkdir -p "$OUTPUT_BASE/llm-rag/$dir_path"
    mkdir -p "$OUTPUT_BASE/vue/$dir_path"
    mkdir -p "$OUTPUT_BASE/api-docs/$dir_path"
    mkdir -p "$OUTPUT_BASE/colab/$dir_path"
    mkdir -p "$OUTPUT_BASE/latex/$dir_path"

    # Run generators
    if ./target/release/kb-cli build "$schema_file" --output "$OUTPUT_BASE/tmp" --generators all 2>/dev/null; then
        # Move generated files to appropriate subdirectories
        mv "$OUTPUT_BASE/tmp"/*.ipynb "$OUTPUT_BASE/jupyter/$dir_path/" 2>/dev/null || true
        mv "$OUTPUT_BASE/tmp"/*.md "$OUTPUT_BASE/mdbook/$dir_path/" 2>/dev/null || true
        mv "$OUTPUT_BASE/tmp"/*.rag.md "$OUTPUT_BASE/llm-rag/$dir_path/" 2>/dev/null || true
        mv "$OUTPUT_BASE/tmp"/*.vue "$OUTPUT_BASE/vue/$dir_path/" 2>/dev/null || true
        mv "$OUTPUT_BASE/tmp"/*.json "$OUTPUT_BASE/api-docs/$dir_path/" 2>/dev/null || true
        mv "$OUTPUT_BASE/tmp"/*.colab.ipynb "$OUTPUT_BASE/colab/$dir_path/" 2>/dev/null || true
        mv "$OUTPUT_BASE/tmp"/*.tex "$OUTPUT_BASE/latex/$dir_path/" 2>/dev/null || true
        echo "   ✅ Success"
    else
        echo "   ❌ Failed: $schema_file"
        FAILED=$((FAILED + 1))
        FAILED_FILES="$FAILED_FILES\n  - $schema_file"
    fi
done

# Cleanup
rm -rf "$OUTPUT_BASE/tmp"

echo ""
echo "📈 Generation Complete!"
echo "   Processed: $TOTAL schemas"
if [ -n "$FAILED_FILES" ]; then
    echo "   Failed: $FAILED"
    echo "   Failed files:$FAILED_FILES"
fi
