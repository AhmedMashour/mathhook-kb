#!/bin/bash
# generate-index-pages.sh - Generate styled index.html pages for output directories
# Usage: ./scripts/generate-index-pages.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
DOCS_PUBLIC="$ROOT_DIR/mathhook-docs-site/public/outputs"

# Common HTML header with MathHook styling
generate_header() {
  local title="$1"
  local icon="$2"
  local color="$3"

  cat << EOF
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>$title - MathHook</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <script>
    tailwind.config = {
      theme: {
        extend: {
          colors: {
            'logic-navy': { 700: '#1e3a5f', 800: '#152a45', 900: '#0d1b2a' },
            'rust-core': '#E64524',
            'solve-cyan': '#06B6D4',
            'step-green': '#22C55E',
            'chalk': { DEFAULT: '#F5F5F5', 400: '#A3A3A3', 500: '#737373', 600: '#525252' }
          }
        }
      }
    }
  </script>
  <style>
    @keyframes fade-up {
      from { opacity: 0; transform: translateY(20px); }
      to { opacity: 1; transform: translateY(0); }
    }
    .animate-fade-up { animation: fade-up 0.6s ease-out forwards; }
    .file-card { animation: fade-up 0.4s ease-out forwards; opacity: 0; }
  </style>
</head>
<body class="min-h-screen bg-logic-navy-900 text-chalk">
  <!-- Background gradient -->
  <div class="fixed inset-0 pointer-events-none">
    <div class="absolute inset-0 bg-gradient-to-br from-logic-navy-900 via-logic-navy-800 to-logic-navy-900"></div>
    <div class="absolute top-0 right-0 w-96 h-96 bg-${color}/10 rounded-full blur-3xl"></div>
    <div class="absolute bottom-0 left-0 w-96 h-96 bg-solve-cyan/10 rounded-full blur-3xl"></div>
  </div>

  <!-- Navigation -->
  <nav class="sticky top-0 z-50 bg-logic-navy-900/95 backdrop-blur-xl border-b border-logic-navy-700/50">
    <div class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="flex items-center justify-between h-14">
        <a href="/" class="flex items-center gap-2 group">
          <svg class="w-7 h-7" viewBox="0 0 48 48" fill="none">
            <defs>
              <linearGradient id="nav-logo" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stop-color="#E64524"/>
                <stop offset="100%" stop-color="#06B6D4"/>
              </linearGradient>
            </defs>
            <path d="M12 40 C12 40 12 12 12 10 C12 6 16 4 20 8 L24 16 L28 8 C32 4 36 6 36 10 C36 12 36 40 36 40" stroke="url(#nav-logo)" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
          </svg>
          <span class="text-lg font-semibold">
            <span class="text-rust-core">Math</span><span class="text-solve-cyan">Hook</span>
          </span>
        </a>
        <div class="flex items-center gap-5 text-sm">
          <a href="/outputs" class="text-chalk-400 hover:text-chalk transition-colors">← Back to Outputs</a>
        </div>
      </div>
    </div>
  </nav>

  <!-- Main Content -->
  <main class="max-w-6xl mx-auto px-4 py-10 relative z-10">
    <div class="mb-10 animate-fade-up">
      <div class="flex items-center gap-4 mb-4">
        <div class="w-14 h-14 rounded-xl bg-${color}/10 flex items-center justify-center">
          ${icon}
        </div>
        <div>
          <h1 class="text-3xl font-bold text-chalk">$title</h1>
          <p class="text-chalk-400" id="file-count"></p>
        </div>
      </div>
    </div>

    <!-- Search/Filter -->
    <div class="mb-6 animate-fade-up" style="animation-delay: 0.1s">
      <input
        type="text"
        id="search"
        placeholder="Search files..."
        class="w-full max-w-md px-4 py-2 rounded-xl bg-logic-navy-800/50 border border-logic-navy-700/50 text-chalk placeholder-chalk-500 focus:outline-none focus:border-${color}/50 transition-colors"
      >
    </div>

    <!-- Files Grid -->
    <div id="files-grid" class="grid gap-3">
EOF
}

# Generate file cards
generate_file_cards() {
  local dir="$1"
  local ext="$2"
  local color="$3"
  local delay=0

  for file in "$dir"/*"$ext"; do
    if [ -f "$file" ]; then
      local filename=$(basename "$file")
      local name="${filename%$ext}"
      # Convert filename to title (replace - and _ with spaces, capitalize)
      local title=$(echo "$name" | sed 's/[-_]/ /g' | sed 's/\b\(.\)/\u\1/g')

      cat << EOF
      <a href="$filename"
         class="file-card block p-4 rounded-xl bg-logic-navy-800/30 border border-logic-navy-700/50 hover:bg-logic-navy-800/50 hover:border-${color}/30 transition-all duration-300 group"
         style="animation-delay: ${delay}ms"
         data-filename="$filename">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-lg bg-${color}/10 flex items-center justify-center group-hover:bg-${color}/20 transition-colors">
            <svg class="w-5 h-5 text-${color}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"/>
            </svg>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="font-medium text-chalk truncate group-hover:text-${color} transition-colors">$title</h3>
            <p class="text-sm text-chalk-500 truncate">$filename</p>
          </div>
          <svg class="w-5 h-5 text-chalk-500 group-hover:text-${color} group-hover:translate-x-1 transition-all" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
          </svg>
        </div>
      </a>
EOF
      delay=$((delay + 20))
      if [ $delay -gt 500 ]; then
        delay=500
      fi
    fi
  done
}

# Common HTML footer
generate_footer() {
  cat << 'EOF'
    </div>
  </main>

  <script>
    // Count files and update display
    const fileCards = document.querySelectorAll('.file-card');
    document.getElementById('file-count').textContent = fileCards.length + ' files available';

    // Search functionality
    document.getElementById('search').addEventListener('input', function(e) {
      const query = e.target.value.toLowerCase();
      fileCards.forEach(card => {
        const filename = card.dataset.filename.toLowerCase();
        card.style.display = filename.includes(query) ? 'block' : 'none';
      });
    });
  </script>
</body>
</html>
EOF
}

echo "📝 Generating index pages..."

# Generate LaTeX index
echo "   Creating LaTeX index..."
LATEX_ICON='<svg class="w-7 h-7 text-violet-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"/></svg>'
{
  generate_header "LaTeX Documents" "$LATEX_ICON" "violet-500"
  generate_file_cards "$DOCS_PUBLIC/latex" ".tex" "violet-500"
  generate_footer
} > "$DOCS_PUBLIC/latex/index.html"

# Generate LLM-RAG index
echo "   Creating LLM-RAG index..."
RAG_ICON='<svg class="w-7 h-7 text-solve-cyan" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg>'
{
  generate_header "LLM-RAG Optimized Chunks" "$RAG_ICON" "solve-cyan"
  generate_file_cards "$DOCS_PUBLIC/llm-rag" ".rag.md" "solve-cyan"
  generate_footer
} > "$DOCS_PUBLIC/llm-rag/index.html"

# Generate Colab index
echo "   Creating Colab index..."
COLAB_ICON='<svg class="w-7 h-7 text-amber-500" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm0 22C6.486 22 2 17.514 2 12S6.486 2 12 2s10 4.486 10 10-4.486 10-10 10z"/><path d="M10 8l6 4-6 4V8z"/></svg>'
{
  generate_header "Google Colab Notebooks" "$COLAB_ICON" "amber-500"
  generate_file_cards "$DOCS_PUBLIC/colab" ".colab.ipynb" "amber-500"
  generate_footer
} > "$DOCS_PUBLIC/colab/index.html"

# Generate Jupyter index
echo "   Creating Jupyter index..."
JUPYTER_ICON='<svg class="w-7 h-7 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/></svg>'
{
  generate_header "Jupyter Notebooks" "$JUPYTER_ICON" "amber-500"
  generate_file_cards "$DOCS_PUBLIC/jupyter" ".ipynb" "amber-500"
  generate_footer
} > "$DOCS_PUBLIC/jupyter/index.html"

echo "✅ Index pages generated!"
echo "   - latex/index.html"
echo "   - llm-rag/index.html"
echo "   - colab/index.html"
echo "   - jupyter/index.html"
