#!/bin/bash

# MathHook Documentation Generation Script
# This script converts all YAML schemas to JSON and updates the documentation website

set -e  # Exit on error

SCHEMA_DIR="/Users/ahmedmashhour/Documents/work/math/mathhook-kb/schemas"
OUTPUT_DIR="/Users/ahmedmashhour/Documents/work/math/mathhook-kb/mathhook-docs-site/public/data"
CONVERTER="/Users/ahmedmashhour/Documents/work/math/mathhook-kb/convert_schema.py"

echo "🚀 MathHook Documentation Generator"
echo "===================================="
echo ""

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Counter for converted files
converted=0

# Find all YAML files recursively
while IFS= read -r yaml_file; do
    # Get the filename without extension
    filename=$(basename "$yaml_file" .yaml)

    # Get the relative directory path from schemas/
    rel_dir=$(dirname "$yaml_file" | sed "s|$SCHEMA_DIR/||")

    # Create output filename (convert directory structure to hyphenated)
    if [ "$rel_dir" = "." ]; then
        output_name="$filename.json"
    else
        # Convert path to hyphenated format: ode/first_order-linear -> ode-first_order-linear
        output_name="${rel_dir//\//-}-${filename}.json"
    fi

    output_file="$OUTPUT_DIR/$output_name"

    echo "📄 Converting: $yaml_file"
    echo "   → $output_file"

    # Run the converter
    python3 "$CONVERTER" "$yaml_file" "$output_file"

    if [ $? -eq 0 ]; then
        ((converted++))
        echo "   ✅ Success"
    else
        echo "   ❌ Failed"
    fi
    echo ""
done < <(find "$SCHEMA_DIR" -type f -name "*.yaml")

echo "===================================="
echo "✨ Converted $converted documentation files"
echo ""
echo "Next steps:"
echo "1. Update mathhook-docs-site/pages/docs/index.vue with links to all articles"
echo "2. Run: cd mathhook-docs-site && npm run dev"
echo "3. Visit: http://localhost:3000/docs"
