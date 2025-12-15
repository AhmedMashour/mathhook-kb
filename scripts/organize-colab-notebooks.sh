#!/bin/bash
# organize-colab-notebooks.sh - Organize Colab notebooks for GitHub hosting
# Usage: ./scripts/organize-colab-notebooks.sh
#
# This script:
# 1. Organizes .colab.ipynb files by category
# 2. Creates a structured directory for GitHub
# 3. Generates index with "Open in Colab" links
# 4. Prepares for push to GitHub

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
OUTPUTS_DIR="$ROOT_DIR/outputs"
COLAB_DIR="$ROOT_DIR/colab-notebooks"  # Will be pushed to GitHub
DOCS_COLAB="$ROOT_DIR/mathhook-docs-site/public/outputs/colab"

# GitHub repo info (update if different)
GITHUB_USER="AhmedMashour"
GITHUB_REPO="mathhook"
GITHUB_BRANCH="master"
COLAB_PATH="colab-notebooks"  # Path in repo where notebooks will live

echo "📓 Organizing Colab notebooks for GitHub..."

# Clean and create directories
rm -rf "$COLAB_DIR"
mkdir -p "$COLAB_DIR"

# Get unique categories
categories=$(ls "$OUTPUTS_DIR"/*.colab.ipynb 2>/dev/null | xargs -I {} basename {} .colab.ipynb | cut -d'-' -f1 | sort -u)

echo "  Found categories: $(echo $categories | tr '\n' ' ')"

# Organize notebooks by category
for category in $categories; do
    mkdir -p "$COLAB_DIR/$category"

    for file in "$OUTPUTS_DIR"/${category}-*.colab.ipynb; do
        if [ -f "$file" ]; then
            # Get clean filename without category prefix
            basename_full=$(basename "$file")
            # Keep original name for now
            cp "$file" "$COLAB_DIR/$category/$basename_full"
        fi
    done

    count=$(ls -1 "$COLAB_DIR/$category"/*.ipynb 2>/dev/null | wc -l | tr -d ' ')
    echo "  $category: $count notebooks"
done

# Create category README files
for category in $categories; do
    cap_category=$(echo "$category" | awk '{print toupper(substr($0,1,1)) substr($0,2)}')

    cat > "$COLAB_DIR/$category/README.md" << EOF
# $cap_category Notebooks

MathHook $cap_category tutorials and examples for Google Colab.

## Notebooks

| Notebook | Open in Colab |
|----------|---------------|
EOF

    for file in "$COLAB_DIR/$category"/*.ipynb; do
        if [ -f "$file" ]; then
            filename=$(basename "$file")
            title=$(echo "$filename" | sed 's/\.colab\.ipynb$//' | sed "s/${category}-//" | sed 's/-/ /g' | sed 's/_/ /g' | awk '{for(i=1;i<=NF;i++)$i=toupper(substr($i,1,1))substr($i,2)}1')
            colab_url="https://colab.research.google.com/github/${GITHUB_USER}/${GITHUB_REPO}/blob/${GITHUB_BRANCH}/${COLAB_PATH}/${category}/${filename}"
            echo "| $title | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)]($colab_url) |" >> "$COLAB_DIR/$category/README.md"
        fi
    done
done

# Create main README
cat > "$COLAB_DIR/README.md" << EOF
# MathHook Colab Notebooks

Interactive Google Colab notebooks for learning MathHook symbolic mathematics.

## Categories

EOF

for category in $categories; do
    cap_category=$(echo "$category" | awk '{print toupper(substr($0,1,1)) substr($0,2)}')
    count=$(ls -1 "$COLAB_DIR/$category"/*.ipynb 2>/dev/null | wc -l | tr -d ' ')
    echo "- [**$cap_category**](./$category/) - $count notebooks" >> "$COLAB_DIR/README.md"
done

cat >> "$COLAB_DIR/README.md" << 'EOF'

## Quick Start

Click any "Open in Colab" badge to launch the notebook directly in Google Colab with zero setup.

## Usage

1. Click the Colab badge on any notebook
2. The notebook opens in Google Colab
3. Run cells with Shift+Enter
4. All dependencies are pre-configured

## Generated From

These notebooks are auto-generated from MathHook Knowledge Base schemas.
EOF

# Generate styled index.html for docs site with Colab links
echo "  Generating Colab index page with Open in Colab buttons..."

cat > "$DOCS_COLAB/index.html" << 'HTMLHEAD'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Google Colab Notebooks - MathHook</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <script>
    tailwind.config = {
      theme: {
        extend: {
          colors: {
            'logic-navy': { 700: '#1e3a5f', 800: '#152a45', 900: '#0d1b2a' },
            'rust-core': '#E64524',
            'solve-cyan': '#06B6D4',
            'chalk': { DEFAULT: '#F5F5F5', 400: '#A3A3A3', 500: '#737373', 600: '#525252' }
          }
        }
      }
    }
  </script>
  <style>
    @keyframes fade-up { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
    .animate-fade-up { animation: fade-up 0.6s ease-out forwards; }
    .notebook-card { animation: fade-up 0.4s ease-out forwards; opacity: 0; }
  </style>
</head>
<body class="min-h-screen bg-logic-navy-900 text-chalk">
  <div class="fixed inset-0 pointer-events-none">
    <div class="absolute inset-0 bg-gradient-to-br from-logic-navy-900 via-logic-navy-800 to-logic-navy-900"></div>
    <div class="absolute top-0 right-0 w-96 h-96 bg-amber-500/10 rounded-full blur-3xl"></div>
    <div class="absolute bottom-0 left-0 w-96 h-96 bg-solve-cyan/10 rounded-full blur-3xl"></div>
  </div>

  <nav class="sticky top-0 z-50 bg-logic-navy-900/95 backdrop-blur-xl border-b border-logic-navy-700/50">
    <div class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="flex items-center justify-between h-14">
        <a href="/" class="flex items-center gap-2">
          <svg class="w-7 h-7" viewBox="0 0 48 48" fill="none">
            <defs><linearGradient id="nav-logo" x1="0%" y1="0%" x2="100%" y2="100%"><stop offset="0%" stop-color="#E64524"/><stop offset="100%" stop-color="#06B6D4"/></linearGradient></defs>
            <path d="M12 40 C12 40 12 12 12 10 C12 6 16 4 20 8 L24 16 L28 8 C32 4 36 6 36 10 C36 12 36 40 36 40" stroke="url(#nav-logo)" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
          </svg>
          <span class="text-lg font-semibold"><span class="text-rust-core">Math</span><span class="text-solve-cyan">Hook</span></span>
        </a>
        <a href="/outputs" class="text-chalk-400 hover:text-chalk transition-colors">← Back to Outputs</a>
      </div>
    </div>
  </nav>

  <main class="max-w-6xl mx-auto px-4 py-10 relative z-10">
    <div class="mb-10 animate-fade-up">
      <div class="flex items-center gap-4 mb-4">
        <div class="w-14 h-14 rounded-xl bg-amber-500/10 flex items-center justify-center">
          <svg class="w-7 h-7 text-amber-500" fill="currentColor" viewBox="0 0 24 24">
            <path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm0 22C6.486 22 2 17.514 2 12S6.486 2 12 2s10 4.486 10 10-4.486 10-10 10z"/>
            <path d="M10 8l6 4-6 4V8z"/>
          </svg>
        </div>
        <div>
          <h1 class="text-3xl font-bold text-chalk">Google Colab Notebooks</h1>
          <p class="text-chalk-400">One-click execution in the cloud</p>
        </div>
      </div>
      <p class="text-chalk-400 max-w-2xl">Click "Open in Colab" to launch any notebook directly in Google Colab with zero setup required.</p>
    </div>

    <div class="mb-6 animate-fade-up" style="animation-delay: 0.1s">
      <input type="text" id="search" placeholder="Search notebooks..." class="w-full max-w-md px-4 py-2 rounded-xl bg-logic-navy-800/50 border border-logic-navy-700/50 text-chalk placeholder-chalk-500 focus:outline-none focus:border-amber-500/50 transition-colors">
    </div>

HTMLHEAD

# Add category sections
delay=0
for category in $categories; do
    cap_category=$(echo "$category" | awk '{print toupper(substr($0,1,1)) substr($0,2)}')

    cat >> "$DOCS_COLAB/index.html" << EOF
    <div class="mb-8">
      <h2 class="text-xl font-semibold text-amber-500 mb-4">$cap_category</h2>
      <div class="grid gap-3">
EOF

    for file in "$COLAB_DIR/$category"/*.ipynb; do
        if [ -f "$file" ]; then
            filename=$(basename "$file")
            title=$(echo "$filename" | sed 's/\.colab\.ipynb$//' | sed "s/${category}-//" | sed 's/-/ /g' | sed 's/_/ /g' | awk '{for(i=1;i<=NF;i++)$i=toupper(substr($i,1,1))substr($i,2)}1')
            colab_url="https://colab.research.google.com/github/${GITHUB_USER}/${GITHUB_REPO}/blob/${GITHUB_BRANCH}/${COLAB_PATH}/${category}/${filename}"

            cat >> "$DOCS_COLAB/index.html" << EOF
        <div class="notebook-card flex items-center justify-between p-4 rounded-xl bg-logic-navy-800/30 border border-logic-navy-700/50 hover:bg-logic-navy-800/50 hover:border-amber-500/30 transition-all" style="animation-delay: ${delay}ms" data-filename="$filename">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-lg bg-amber-500/10 flex items-center justify-center">
              <svg class="w-5 h-5 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
              </svg>
            </div>
            <div>
              <h3 class="font-medium text-chalk">$title</h3>
              <p class="text-sm text-chalk-500">$filename</p>
            </div>
          </div>
          <a href="$colab_url" target="_blank" class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-amber-500 to-amber-600 text-logic-navy-900 rounded-lg hover:from-amber-400 hover:to-amber-500 transition-all font-medium text-sm">
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm0 22C6.486 22 2 17.514 2 12S6.486 2 12 2s10 4.486 10 10-4.486 10-10 10z"/></svg>
            Open in Colab
          </a>
        </div>
EOF
            delay=$((delay + 15))
            if [ $delay -gt 300 ]; then delay=300; fi
        fi
    done

    echo "      </div>" >> "$DOCS_COLAB/index.html"
    echo "    </div>" >> "$DOCS_COLAB/index.html"
done

# Close HTML
cat >> "$DOCS_COLAB/index.html" << 'HTMLFOOT'
  </main>

  <script>
    const cards = document.querySelectorAll('.notebook-card');
    document.getElementById('search').addEventListener('input', function(e) {
      const query = e.target.value.toLowerCase();
      cards.forEach(card => {
        const filename = card.dataset.filename.toLowerCase();
        card.style.display = filename.includes(query) ? 'flex' : 'none';
      });
    });
  </script>
</body>
</html>
HTMLFOOT

# Copy organized notebooks to docs site as well (for download)
cp -r "$COLAB_DIR"/* "$DOCS_COLAB/" 2>/dev/null || true

total_notebooks=$(find "$COLAB_DIR" -name "*.ipynb" | wc -l | tr -d ' ')

echo ""
echo "✅ Colab notebooks organized!"
echo "   Total: $total_notebooks notebooks in $(echo $categories | wc -w | tr -d ' ') categories"
echo ""
echo "📁 Organized structure: $COLAB_DIR"
echo "📄 Index page: $DOCS_COLAB/index.html"
echo ""
echo "🚀 Next steps to enable 'Open in Colab':"
echo "   1. Copy $COLAB_DIR to your main mathhook repo"
echo "   2. git add colab-notebooks && git commit -m 'Add Colab notebooks'"
echo "   3. git push origin $GITHUB_BRANCH"
echo ""
echo "   Or run: cp -r $COLAB_DIR /path/to/mathhook/"
