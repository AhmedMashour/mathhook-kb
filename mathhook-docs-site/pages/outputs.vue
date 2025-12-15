<template>
  <div class="min-h-screen bg-logic-navy-900 text-chalk overflow-hidden relative">
    <!-- Animated Background -->
    <div class="fixed inset-0 overflow-hidden pointer-events-none">
      <!-- Base gradient -->
      <div class="absolute inset-0 bg-gradient-to-b from-logic-navy-900 via-logic-navy-900/95 to-logic-navy-900"></div>

      <!-- Animated gradient mesh -->
      <div class="absolute inset-0 opacity-40">
        <div class="absolute top-0 left-0 w-full h-full bg-gradient-to-br from-rust-core/20 via-transparent to-solve-cyan/20 animate-gradient-shift"></div>
      </div>

      <!-- Large vibrant orbs with parallax -->
      <div
        class="absolute w-[800px] h-[800px] rounded-full blur-[150px] opacity-30"
        :style="{
          background: 'radial-gradient(circle, rgba(230, 69, 36, 0.5) 0%, transparent 70%)',
          top: '-200px',
          right: '-200px',
          transform: `translate(${-mouseX * 0.02}px, ${mouseY * 0.02}px)`
        }"
      ></div>
      <div
        class="absolute w-[700px] h-[700px] rounded-full blur-[130px] opacity-25"
        :style="{
          background: 'radial-gradient(circle, rgba(6, 182, 212, 0.5) 0%, transparent 70%)',
          bottom: '-150px',
          left: '-150px',
          transform: `translate(${mouseX * 0.015}px, ${-mouseY * 0.015}px)`
        }"
      ></div>
      <div
        class="absolute w-[500px] h-[500px] rounded-full blur-[100px] opacity-20"
        :style="{
          background: 'radial-gradient(circle, rgba(139, 92, 246, 0.5) 0%, transparent 70%)',
          top: '40%',
          left: '30%',
          transform: `translate(${-mouseX * 0.01}px, ${mouseY * 0.01}px)`
        }"
      ></div>

      <!-- Floating particles -->
      <div v-for="i in 20" :key="i"
           class="absolute w-1 h-1 rounded-full bg-white/20 animate-float-particle"
           :style="{
             left: `${Math.random() * 100}%`,
             top: `${Math.random() * 100}%`,
             animationDelay: `${Math.random() * 5}s`,
             animationDuration: `${5 + Math.random() * 10}s`
           }"
      ></div>

      <!-- Grid overlay -->
      <div class="absolute inset-0 opacity-[0.03]"
           style="background-image: linear-gradient(rgba(255,255,255,.1) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,.1) 1px, transparent 1px); background-size: 50px 50px;">
      </div>
    </div>

    <!-- Navigation -->
    <nav
      class="sticky top-0 z-50 transition-all duration-500"
      :class="scrolled ? 'bg-logic-navy-900/95 backdrop-blur-xl border-b border-logic-navy-700/50 shadow-lg shadow-black/10' : 'bg-transparent'"
    >
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-14">
          <NuxtLink to="/" class="flex items-center gap-2 group">
            <svg class="w-7 h-7 transition-transform duration-300 group-hover:scale-110" viewBox="0 0 48 48" fill="none">
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
            <NuxtLink to="/docs" class="text-chalk-500 hover:text-chalk transition-colors duration-300 hover:scale-105 transform">Docs</NuxtLink>
            <NuxtLink to="/outputs" class="text-solve-cyan font-medium relative after:absolute after:bottom-0 after:left-0 after:w-full after:h-0.5 after:bg-solve-cyan after:rounded-full">Outputs</NuxtLink>
            <a href="https://github.com/AhmedMashour/mathhook" target="_blank"
               class="text-chalk-500 hover:text-chalk transition-all duration-300 flex items-center gap-1.5 hover:scale-105 transform">
              <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
              GitHub
            </a>
          </div>
        </div>
      </div>
    </nav>

    <!-- Main Content -->
    <div class="max-w-6xl mx-auto px-4 py-10 relative z-10">
      <!-- Header Section -->
      <div class="mb-14" v-scroll-animate="{ animation: 'fade-up' }">
        <div class="relative inline-block mb-4">
          <h1 class="text-4xl md:text-5xl font-bold">
            <span class="relative">
              <span v-for="(word, i) in 'Documentation'.split('')" :key="i"
                    class="inline-block animate-text-reveal"
                    :style="{ animationDelay: `${i * 0.03}s` }">
                {{ word }}
              </span>
            </span>
            <span class="relative ml-2">
              <span v-for="(word, i) in 'Outputs'.split('')" :key="i"
                    class="inline-block bg-clip-text text-transparent bg-gradient-to-r from-rust-core to-solve-cyan animate-text-reveal"
                    :style="{ animationDelay: `${0.4 + i * 0.03}s` }">
                {{ word }}
              </span>
            </span>
          </h1>
          <!-- Animated underline -->
          <div class="absolute -bottom-2 left-0 h-1 bg-gradient-to-r from-rust-core via-violet-500 to-solve-cyan rounded-full animate-expand-width"></div>
        </div>
        <p class="text-xl text-chalk-400 max-w-2xl animate-fade-in-delayed">
          Access MathHook documentation in your preferred format — from interactive notebooks to searchable books
        </p>
      </div>

      <!-- Output Cards Grid -->
      <div class="grid md:grid-cols-2 gap-6 mb-12">
        <!-- mdBook -->
        <a href="/outputs/mdbook/index.html" target="_blank"
           v-scroll-animate="{ animation: 'fade-up', delay: 100 }"
           class="group relative rounded-2xl p-6 border border-logic-navy-700/50 bg-logic-navy-800/30 backdrop-blur-sm hover:bg-logic-navy-800/50 transition-all duration-500 hover:scale-[1.02] hover:shadow-2xl hover:shadow-rust-core/10 overflow-hidden">
          <!-- Hover glow effect -->
          <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 bg-gradient-to-br from-rust-core/10 via-transparent to-transparent pointer-events-none"></div>

          <div class="relative z-10">
            <div class="flex items-center gap-4 mb-4">
              <div class="w-14 h-14 rounded-xl bg-rust-core/10 flex items-center justify-center group-hover:scale-110 group-hover:bg-rust-core/20 transition-all duration-300">
                <svg class="w-7 h-7 text-rust-core" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
                </svg>
              </div>
              <div>
                <h3 class="text-lg font-semibold text-rust-core group-hover:text-rust-core-300 transition-colors">
                  Rust mdBook
                </h3>
                <p class="text-sm text-chalk-600">Searchable Rust documentation</p>
              </div>
            </div>
            <p class="text-chalk-400 mb-5">
              Interactive book format with code examples in Rust, Python, and JavaScript.
              Perfect for Rust developers.
            </p>
            <div class="flex items-center text-rust-core font-medium">
              Open Book
              <svg class="w-5 h-5 ml-2 group-hover:translate-x-2 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
              </svg>
            </div>
          </div>
        </a>

        <!-- Jupyter Book -->
        <a href="/outputs/jupyter-book/_build/html/index.html" target="_blank"
           v-scroll-animate="{ animation: 'fade-up', delay: 200 }"
           class="group relative rounded-2xl p-6 border border-logic-navy-700/50 bg-logic-navy-800/30 backdrop-blur-sm hover:bg-logic-navy-800/50 transition-all duration-500 hover:scale-[1.02] hover:shadow-2xl hover:shadow-amber-500/10 overflow-hidden">
          <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 bg-gradient-to-br from-amber-500/10 via-transparent to-transparent pointer-events-none"></div>

          <div class="relative z-10">
            <div class="flex items-center gap-4 mb-4">
              <div class="w-14 h-14 rounded-xl bg-amber-500/10 flex items-center justify-center group-hover:scale-110 group-hover:bg-amber-500/20 transition-all duration-300">
                <svg class="w-7 h-7 text-amber-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                </svg>
              </div>
              <div>
                <h3 class="text-lg font-semibold text-amber-500 group-hover:text-amber-400 transition-colors">
                  Jupyter Book
                </h3>
                <p class="text-sm text-chalk-600">Interactive Python notebooks</p>
              </div>
            </div>
            <p class="text-chalk-400 mb-5">
              Executable notebooks with live code cells. Download and run locally or open in Colab.
              Perfect for data scientists.
            </p>
            <div class="flex items-center text-amber-500 font-medium">
              Open Notebooks
              <svg class="w-5 h-5 ml-2 group-hover:translate-x-2 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
              </svg>
            </div>
          </div>
        </a>

        <!-- API Documentation -->
        <a href="/outputs/api-docs/index.html" target="_blank"
           v-scroll-animate="{ animation: 'fade-up', delay: 300 }"
           class="group relative rounded-2xl p-6 border border-logic-navy-700/50 bg-logic-navy-800/30 backdrop-blur-sm hover:bg-logic-navy-800/50 transition-all duration-500 hover:scale-[1.02] hover:shadow-2xl hover:shadow-step-green/10 overflow-hidden">
          <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 bg-gradient-to-br from-step-green/10 via-transparent to-transparent pointer-events-none"></div>

          <div class="relative z-10">
            <div class="flex items-center gap-4 mb-4">
              <div class="w-14 h-14 rounded-xl bg-step-green/10 flex items-center justify-center group-hover:scale-110 group-hover:bg-step-green/20 transition-all duration-300">
                <svg class="w-7 h-7 text-step-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                </svg>
              </div>
              <div>
                <h3 class="text-lg font-semibold text-step-green group-hover:text-step-green-300 transition-colors">
                  API Playground
                </h3>
                <p class="text-sm text-chalk-600">Interactive OpenAPI docs</p>
              </div>
            </div>
            <p class="text-chalk-400 mb-5">
              Try MathHook APIs directly in your browser with Swagger UI.
              Test requests and see responses in real-time.
            </p>
            <div class="flex items-center text-step-green font-medium">
              Try API
              <svg class="w-5 h-5 ml-2 group-hover:translate-x-2 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
              </svg>
            </div>
          </div>
        </a>

        <!-- LaTeX PDFs -->
        <a href="/outputs/latex/" target="_blank"
           v-scroll-animate="{ animation: 'fade-up', delay: 400 }"
           class="group relative rounded-2xl p-6 border border-logic-navy-700/50 bg-logic-navy-800/30 backdrop-blur-sm hover:bg-logic-navy-800/50 transition-all duration-500 hover:scale-[1.02] hover:shadow-2xl hover:shadow-violet-500/10 overflow-hidden">
          <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 bg-gradient-to-br from-violet-500/10 via-transparent to-transparent pointer-events-none"></div>

          <div class="relative z-10">
            <div class="flex items-center gap-4 mb-4">
              <div class="w-14 h-14 rounded-xl bg-violet-500/10 flex items-center justify-center group-hover:scale-110 group-hover:bg-violet-500/20 transition-all duration-300">
                <svg class="w-7 h-7 text-violet-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"/>
                </svg>
              </div>
              <div>
                <h3 class="text-lg font-semibold text-violet-500 group-hover:text-violet-400 transition-colors">
                  LaTeX/PDF Documents
                </h3>
                <p class="text-sm text-chalk-600">Academic paper format</p>
              </div>
            </div>
            <p class="text-chalk-400 mb-5">
              Professional academic documentation in PDF format.
              Perfect for offline reading and printing.
            </p>
            <div class="flex items-center text-violet-500 font-medium">
              Browse PDFs
              <svg class="w-5 h-5 ml-2 group-hover:translate-x-2 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
              </svg>
            </div>
          </div>
        </a>

        <!-- Google Colab -->
        <div
           v-scroll-animate="{ animation: 'fade-up', delay: 500 }"
           class="group relative rounded-2xl p-6 border border-logic-navy-700/50 bg-logic-navy-800/30 backdrop-blur-sm hover:bg-logic-navy-800/50 transition-all duration-500 hover:shadow-2xl hover:shadow-amber-500/10 overflow-hidden">
          <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 bg-gradient-to-br from-amber-500/5 via-transparent to-transparent pointer-events-none"></div>

          <div class="relative z-10">
            <div class="flex items-center gap-4 mb-4">
              <div class="w-14 h-14 rounded-xl bg-amber-500/10 flex items-center justify-center group-hover:scale-110 group-hover:bg-amber-500/20 transition-all duration-300">
                <svg class="w-7 h-7 text-amber-500" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm0 22C6.486 22 2 17.514 2 12S6.486 2 12 2s10 4.486 10 10-4.486 10-10 10z"/>
                  <path d="M10 8l6 4-6 4V8z"/>
                </svg>
              </div>
              <div>
                <h3 class="text-lg font-semibold text-chalk group-hover:text-amber-400 transition-colors">
                  Google Colab Notebooks
                </h3>
                <p class="text-sm text-chalk-600">Cloud executable notebooks</p>
              </div>
            </div>
            <p class="text-chalk-400 mb-5">
              Run MathHook examples in Google Colab with zero setup.
              One-click execution in the cloud.
            </p>
            <a href="https://colab.research.google.com/github/AhmedMashour/mathhook/blob/master/outputs/colab/"
               target="_blank"
               class="inline-flex items-center px-5 py-2.5 bg-gradient-to-r from-amber-500 to-amber-600 text-logic-navy-900 rounded-xl hover:from-amber-400 hover:to-amber-500 transition-all duration-300 font-medium shadow-lg shadow-amber-500/20 hover:shadow-xl hover:shadow-amber-500/30 hover:scale-105">
              <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12 12-5.373 12-12S18.627 0 12 0zm0 22C6.486 22 2 17.514 2 12S6.486 2 12 2s10 4.486 10 10-4.486 10-10 10z"/>
              </svg>
              Open in Colab
            </a>
          </div>
        </div>

        <!-- LLM-RAG Chunks -->
        <div
           v-scroll-animate="{ animation: 'fade-up', delay: 600 }"
           class="group relative rounded-2xl p-6 border border-logic-navy-700/50 bg-logic-navy-800/30 backdrop-blur-sm hover:bg-logic-navy-800/50 transition-all duration-500 hover:shadow-2xl hover:shadow-solve-cyan/10 overflow-hidden">
          <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 bg-gradient-to-br from-solve-cyan/5 via-transparent to-transparent pointer-events-none"></div>

          <div class="relative z-10">
            <div class="flex items-center gap-4 mb-4">
              <div class="w-14 h-14 rounded-xl bg-solve-cyan/10 flex items-center justify-center group-hover:scale-110 group-hover:bg-solve-cyan/20 transition-all duration-300">
                <svg class="w-7 h-7 text-solve-cyan" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
                </svg>
              </div>
              <div>
                <h3 class="text-lg font-semibold text-chalk group-hover:text-solve-cyan transition-colors">
                  LLM-RAG Optimized
                </h3>
                <p class="text-sm text-chalk-600">For AI assistants & search</p>
              </div>
            </div>
            <p class="text-chalk-400 mb-5">
              Documentation chunks optimized for vector databases and AI retrieval.
              Perfect for building MathHook-aware AI assistants.
            </p>
            <a href="/outputs/llm-rag/"
               target="_blank"
               class="inline-flex items-center px-5 py-2.5 bg-gradient-to-r from-solve-cyan to-solve-cyan-600 text-logic-navy-900 rounded-xl hover:from-solve-cyan-400 hover:to-solve-cyan transition-all duration-300 font-medium shadow-lg shadow-solve-cyan/20 hover:shadow-xl hover:shadow-solve-cyan/30 hover:scale-105">
              Browse RAG Chunks
            </a>
          </div>
        </div>
      </div>

      <!-- Build Info Section -->
      <div
        v-scroll-animate="{ animation: 'fade-up', delay: 100 }"
        class="relative rounded-2xl p-8 border border-rust-core/30 bg-logic-navy-800/30 backdrop-blur-sm overflow-hidden">
        <!-- Animated border glow -->
        <div class="absolute inset-0 rounded-2xl opacity-50" style="background: linear-gradient(90deg, transparent, rgba(230, 69, 36, 0.1), transparent); animation: shimmer 3s infinite;"></div>

        <div class="relative z-10">
          <div class="flex items-center gap-4 mb-5">
            <div class="w-12 h-12 rounded-xl bg-rust-core/10 flex items-center justify-center animate-pulse-slow">
              <svg class="w-6 h-6 text-rust-core" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/>
              </svg>
            </div>
            <div>
              <h3 class="text-xl font-semibold text-chalk">
                Build Your Own Outputs
              </h3>
              <p class="text-sm text-chalk-600">Generate documentation in any format</p>
            </div>
          </div>

          <p class="text-chalk-400 mb-6 max-w-2xl">
            All outputs are generated from YAML schemas using the MathHook KB CLI tool.
            You can build custom formats or regenerate existing ones.
          </p>

          <div class="relative rounded-xl bg-logic-navy-900/80 border border-logic-navy-700/50 p-5 font-mono text-sm overflow-hidden group">
            <!-- Code syntax highlighting -->
            <code class="block text-chalk-300 leading-relaxed">
              <span class="text-chalk-600"># Generate all output formats from a schema</span><br/>
              <span class="text-solve-cyan">cd</span> <span class="text-chalk-400">/path/to/mathhook-kb</span><br/>
              <span class="text-solve-cyan">cargo run</span> <span class="text-rust-core">--release</span> <span class="text-rust-core">-p</span> <span class="text-amber-400">kb-cli</span> <span class="text-rust-core">--</span> build <span class="text-step-green">schemas/calculus/derivatives.yaml</span> <span class="text-rust-core">--generators</span> <span class="text-violet-400">all</span>
            </code>

            <!-- Copy button -->
            <button
              @click="copyCommand"
              class="absolute top-3 right-3 p-2 rounded-lg bg-logic-navy-700/50 hover:bg-logic-navy-700 text-chalk-500 hover:text-chalk transition-all duration-300 opacity-0 group-hover:opacity-100"
            >
              <svg v-if="!copied" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
              </svg>
              <svg v-else class="w-4 h-4 text-step-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

useHead({
  title: 'Documentation Outputs - MathHook',
  meta: [
    {
      name: 'description',
      content: 'Access MathHook documentation in multiple formats: Rust mdBook, Jupyter notebooks, API playground, LaTeX PDFs, and more. Symbolic Power. Educational Clarity. Native Speed.'
    }
  ]
})

// State
const scrolled = ref(false)
const mouseX = ref(0)
const mouseY = ref(0)
const copied = ref(false)

// Scroll handler
const handleScroll = () => {
  scrolled.value = window.scrollY > 20
}

// Mouse move handler for parallax
const handleMouseMove = (e) => {
  mouseX.value = (e.clientX - window.innerWidth / 2) / 50
  mouseY.value = (e.clientY - window.innerHeight / 2) / 50
}

// Copy command
const copyCommand = async () => {
  const command = 'cd /path/to/mathhook-kb && cargo run --release -p kb-cli -- build schemas/calculus/derivatives.yaml --generators all'
  await navigator.clipboard.writeText(command)
  copied.value = true
  setTimeout(() => {
    copied.value = false
  }, 2000)
}

// Scroll animate directive
const vScrollAnimate = {
  mounted(el, binding) {
    const options = binding.value || {}
    const animation = options.animation || 'fade-up'
    const delay = options.delay || 0

    // Initial state
    el.style.opacity = '0'
    if (animation === 'fade-up') {
      el.style.transform = 'translateY(40px)'
    } else if (animation === 'fade-left') {
      el.style.transform = 'translateX(-40px)'
    } else if (animation === 'fade-right') {
      el.style.transform = 'translateX(40px)'
    } else if (animation === 'scale') {
      el.style.transform = 'scale(0.95)'
    }

    el.style.transition = `all 0.8s cubic-bezier(0.16, 1, 0.3, 1) ${delay}ms`

    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            el.style.opacity = '1'
            el.style.transform = 'translateY(0) translateX(0) scale(1)'
            observer.unobserve(el)
          }
        })
      },
      { threshold: 0.1, rootMargin: '0px 0px -50px 0px' }
    )

    observer.observe(el)
  }
}

onMounted(() => {
  window.addEventListener('scroll', handleScroll)
  window.addEventListener('mousemove', handleMouseMove)
})

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
  window.removeEventListener('mousemove', handleMouseMove)
})
</script>

<style scoped>
/* Text reveal animation */
@keyframes text-reveal {
  0% {
    opacity: 0;
    transform: translateY(20px) rotateX(-20deg);
  }
  100% {
    opacity: 1;
    transform: translateY(0) rotateX(0);
  }
}

.animate-text-reveal {
  animation: text-reveal 0.6s ease-out forwards;
  opacity: 0;
}

/* Expanding underline */
@keyframes expand-width {
  0% {
    width: 0;
    opacity: 0;
  }
  100% {
    width: 100%;
    opacity: 1;
  }
}

.animate-expand-width {
  animation: expand-width 1s ease-out 0.8s forwards;
  width: 0;
}

/* Fade in delayed */
@keyframes fade-in-delayed {
  0% {
    opacity: 0;
    transform: translateY(10px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-fade-in-delayed {
  animation: fade-in-delayed 0.8s ease-out 0.6s forwards;
  opacity: 0;
}

/* Gradient shift animation */
@keyframes gradient-shift {
  0%, 100% {
    transform: translate(0, 0) rotate(0deg);
  }
  25% {
    transform: translate(5%, 5%) rotate(1deg);
  }
  50% {
    transform: translate(0, 10%) rotate(0deg);
  }
  75% {
    transform: translate(-5%, 5%) rotate(-1deg);
  }
}

.animate-gradient-shift {
  animation: gradient-shift 20s ease-in-out infinite;
}

/* Floating particle animation */
@keyframes float-particle {
  0%, 100% {
    transform: translateY(0) translateX(0);
    opacity: 0.2;
  }
  50% {
    transform: translateY(-30px) translateX(10px);
    opacity: 0.5;
  }
}

.animate-float-particle {
  animation: float-particle 10s ease-in-out infinite;
}

/* Slow pulse */
@keyframes pulse-slow {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.7;
  }
}

.animate-pulse-slow {
  animation: pulse-slow 3s ease-in-out infinite;
}

/* Shimmer effect */
@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}
</style>
