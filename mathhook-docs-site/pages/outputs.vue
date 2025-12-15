<template>
  <div class="min-h-screen bg-logic-navy-900 text-chalk">
    <!-- Navigation -->
    <nav class="sticky top-0 z-50 bg-logic-navy-900/90 backdrop-blur-md border-b border-logic-navy-700/50">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-14">
          <NuxtLink to="/" class="flex items-center gap-2">
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
          </NuxtLink>
          <div class="flex items-center gap-5 text-sm">
            <NuxtLink to="/docs" class="text-chalk-500 hover:text-chalk transition-colors">Docs</NuxtLink>
            <NuxtLink to="/outputs" class="text-solve-cyan font-medium">Outputs</NuxtLink>
            <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors flex items-center gap-1.5">
              <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
              GitHub
            </a>
          </div>
        </div>
      </div>
    </nav>

    <!-- Main Content -->
    <div class="max-w-6xl mx-auto px-4 py-10">
      <div class="mb-10">
        <h1 class="text-3xl md:text-4xl font-semibold mb-3 bg-clip-text text-transparent bg-gradient-to-r from-rust-core to-solve-cyan">
          Documentation Outputs
        </h1>
        <p class="text-lg text-chalk-400">
          Access MathHook documentation in your preferred format
        </p>
      </div>

      <div class="grid md:grid-cols-2 gap-6">
        <!-- mdBook -->
        <a href="/outputs/mdbook/index.html" target="_blank"
           class="card card-rust-hover group">
          <div class="flex items-center gap-4 mb-4">
            <div class="w-14 h-14 rounded-xl bg-rust-core/10 flex items-center justify-center">
              <svg class="w-7 h-7 text-rust-core" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
              </svg>
            </div>
            <div>
              <h3 class="text-lg font-medium text-rust-core group-hover:text-rust-core-300 transition-colors">
                Rust mdBook
              </h3>
              <p class="text-sm text-chalk-600">Searchable Rust documentation</p>
            </div>
          </div>
          <p class="text-chalk-400 mb-4">
            Interactive book format with code examples in Rust, Python, and JavaScript.
            Perfect for Rust developers.
          </p>
          <div class="flex items-center text-rust-core font-medium">
            Open Book
            <svg class="w-5 h-5 ml-2 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
            </svg>
          </div>
        </a>

        <!-- Jupyter Book -->
        <a href="/outputs/jupyter-book/_build/html/index.html" target="_blank"
           class="card card-hover group">
          <div class="flex items-center gap-4 mb-4">
            <div class="w-14 h-14 rounded-xl bg-amber-500/10 flex items-center justify-center">
              <svg class="w-7 h-7 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
              </svg>
            </div>
            <div>
              <h3 class="text-lg font-medium text-amber-500 group-hover:text-amber-400 transition-colors">
                Jupyter Book
              </h3>
              <p class="text-sm text-chalk-600">Interactive Python notebooks</p>
            </div>
          </div>
          <p class="text-chalk-400 mb-4">
            Executable notebooks with live code cells. Download and run locally or open in Colab.
            Perfect for data scientists and Python developers.
          </p>
          <div class="flex items-center text-amber-500 font-medium">
            Open Notebooks
            <svg class="w-5 h-5 ml-2 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
            </svg>
          </div>
        </a>

        <!-- API Documentation -->
        <a href="/outputs/api-docs/index.html" target="_blank"
           class="card card-green-hover group">
          <div class="flex items-center gap-4 mb-4">
            <div class="w-14 h-14 rounded-xl bg-step-green/10 flex items-center justify-center">
              <svg class="w-7 h-7 text-step-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
              </svg>
            </div>
            <div>
              <h3 class="text-lg font-medium text-step-green group-hover:text-step-green-300 transition-colors">
                API Playground
              </h3>
              <p class="text-sm text-chalk-600">Interactive OpenAPI docs</p>
            </div>
          </div>
          <p class="text-chalk-400 mb-4">
            Try MathHook APIs directly in your browser with Swagger UI.
            Test requests and see responses in real-time.
          </p>
          <div class="flex items-center text-step-green font-medium">
            Try API
            <svg class="w-5 h-5 ml-2 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
            </svg>
          </div>
        </a>

        <!-- LaTeX PDFs -->
        <a href="/outputs/latex/" target="_blank"
           class="card card-hover group">
          <div class="flex items-center gap-4 mb-4">
            <div class="w-14 h-14 rounded-xl bg-violet-500/10 flex items-center justify-center">
              <svg class="w-7 h-7 text-violet-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"/>
              </svg>
            </div>
            <div>
              <h3 class="text-lg font-medium text-violet-500 group-hover:text-violet-400 transition-colors">
                LaTeX/PDF Documents
              </h3>
              <p class="text-sm text-chalk-600">Academic paper format</p>
            </div>
          </div>
          <p class="text-chalk-400 mb-4">
            Professional academic documentation in PDF format.
            Perfect for offline reading and printing.
          </p>
          <div class="flex items-center text-violet-500 font-medium">
            Browse PDFs
            <svg class="w-5 h-5 ml-2 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
            </svg>
          </div>
        </a>

        <!-- Google Colab -->
        <div class="card">
          <div class="flex items-center gap-4 mb-4">
            <div class="w-14 h-14 rounded-xl bg-amber-500/10 flex items-center justify-center">
              <svg class="w-7 h-7 text-amber-500" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm0 22C6.486 22 2 17.514 2 12S6.486 2 12 2s10 4.486 10 10-4.486 10-10 10z"/>
                <path d="M10 8l6 4-6 4V8z"/>
              </svg>
            </div>
            <div>
              <h3 class="text-lg font-medium text-chalk">
                Google Colab Notebooks
              </h3>
              <p class="text-sm text-chalk-600">Cloud executable notebooks</p>
            </div>
          </div>
          <p class="text-chalk-400 mb-4">
            Run MathHook examples in Google Colab with zero setup.
            One-click execution in the cloud.
          </p>
          <a href="https://colab.research.google.com/github/AhmedMashour/mathhook/blob/master/outputs/colab/"
             target="_blank"
             class="inline-flex items-center px-4 py-2 bg-amber-500 text-logic-navy-900 rounded-lg hover:bg-amber-400 transition-colors font-medium">
            <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm0 22C6.486 22 2 17.514 2 12S6.486 2 12 2s10 4.486 10 10-4.486 10-10 10z"/>
            </svg>
            Open in Colab
          </a>
        </div>

        <!-- LLM-RAG Chunks -->
        <div class="card">
          <div class="flex items-center gap-4 mb-4">
            <div class="w-14 h-14 rounded-xl bg-solve-cyan/10 flex items-center justify-center">
              <svg class="w-7 h-7 text-solve-cyan" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
              </svg>
            </div>
            <div>
              <h3 class="text-lg font-medium text-chalk">
                LLM-RAG Optimized
              </h3>
              <p class="text-sm text-chalk-600">For AI assistants & search</p>
            </div>
          </div>
          <p class="text-chalk-400 mb-4">
            Documentation chunks optimized for vector databases and AI retrieval.
            Perfect for building MathHook-aware AI assistants.
          </p>
          <a href="/outputs/llm-rag/"
             target="_blank"
             class="inline-flex items-center px-4 py-2 bg-solve-cyan text-logic-navy-900 rounded-lg hover:bg-solve-cyan-400 transition-colors font-medium">
            Browse RAG Chunks
          </a>
        </div>
      </div>

      <!-- Build Info -->
      <div class="mt-12 card border-rust-core/30">
        <div class="flex items-center gap-3 mb-4">
          <div class="w-10 h-10 rounded-lg bg-rust-core/10 flex items-center justify-center">
            <svg class="w-5 h-5 text-rust-core" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/>
            </svg>
          </div>
          <h3 class="text-base font-medium text-chalk">
            Build Your Own Outputs
          </h3>
        </div>
        <p class="text-chalk-400 mb-4">
          All outputs are generated from YAML schemas using the MathHook KB CLI tool.
          You can build custom formats or regenerate existing ones.
        </p>
        <div class="code-block">
          <code class="text-chalk-300">
            <span class="text-chalk-600"># Generate all output formats from a schema</span><br/>
            <span class="text-solve-cyan">cd</span> /path/to/mathhook-kb<br/>
            <span class="text-solve-cyan">cargo run</span> <span class="text-rust-core">--release</span> <span class="text-rust-core">-p</span> kb-cli <span class="text-rust-core">--</span> build schemas/calculus/derivatives.yaml <span class="text-rust-core">--generators</span> all
          </code>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
useHead({
  title: 'Documentation Outputs - MathHook',
  meta: [
    {
      name: 'description',
      content: 'Access MathHook documentation in multiple formats: Rust mdBook, Jupyter notebooks, API playground, LaTeX PDFs, and more. Symbolic Power. Educational Clarity. Native Speed.'
    }
  ]
})
</script>
