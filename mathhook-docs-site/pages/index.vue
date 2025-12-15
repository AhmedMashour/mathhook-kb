<template>
  <div class="min-h-screen bg-logic-navy-900 text-chalk overflow-hidden">
    <!-- Mathematical Grid Background -->
    <div class="fixed inset-0 pointer-events-none">
      <!-- Grid pattern -->
      <div class="absolute inset-0 bg-[linear-gradient(rgba(6,182,212,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(6,182,212,0.03)_1px,transparent_1px)] bg-[size:48px_48px]"></div>
      <!-- Gradient overlay -->
      <div class="absolute inset-0 bg-gradient-to-b from-logic-navy-900 via-transparent to-logic-navy-900"></div>
      <!-- Subtle glow spots -->
      <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-rust-core/5 rounded-full blur-3xl"></div>
      <div class="absolute bottom-1/4 right-1/4 w-96 h-96 bg-solve-cyan/5 rounded-full blur-3xl"></div>
    </div>

    <!-- Navigation -->
    <nav class="fixed top-0 left-0 right-0 z-50 bg-logic-navy-900/90 backdrop-blur-md border-b border-logic-navy-700/50">
      <div class="max-w-6xl mx-auto px-4 sm:px-6">
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
            <NuxtLink to="/outputs" class="text-chalk-500 hover:text-chalk transition-colors">Outputs</NuxtLink>
            <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors flex items-center gap-1.5">
              <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
              GitHub
            </a>
          </div>
        </div>
      </div>
    </nav>

    <!-- Hero Section -->
    <header class="relative pt-32 pb-20 px-4">
      <div class="max-w-4xl mx-auto text-center relative z-10">
        <!-- Logo + Title -->
        <div class="mb-6">
          <div class="flex items-center justify-center gap-3 mb-4">
            <svg class="w-12 h-12" viewBox="0 0 48 48" fill="none">
              <defs>
                <linearGradient id="hero-logo" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" stop-color="#E64524"/>
                  <stop offset="100%" stop-color="#06B6D4"/>
                </linearGradient>
              </defs>
              <path d="M12 40 C12 40 12 12 12 10 C12 6 16 4 20 8 L24 16 L28 8 C32 4 36 6 36 10 C36 12 36 40 36 40" stroke="url(#hero-logo)" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
            </svg>
            <h1 class="text-4xl md:text-5xl font-bold">
              <span class="text-rust-core">Math</span><span class="text-solve-cyan">Hook</span>
            </h1>
          </div>
          <p class="text-xl md:text-2xl text-chalk font-medium mb-2">
            Symbolic Power. Educational Clarity. Native Speed.
          </p>
          <p class="text-chalk-500 max-w-2xl mx-auto">
            High-performance computer algebra system written in Rust with bindings for Python and Node.js
          </p>
        </div>

        <!-- Ecosystem Badges -->
        <div class="flex items-center justify-center gap-3 mb-10">
          <a href="https://crates.io/crates/mathhook" target="_blank" class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-rust-core/10 border border-rust-core/20 rounded-full text-sm text-rust-core hover:bg-rust-core/20 transition-colors">
            <svg class="w-4 h-4" viewBox="0 0 512 512" fill="currentColor"><path d="M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256 256-114.6 256-256S397.4 0 256 0zm0 464c-114.7 0-208-93.31-208-208S141.3 48 256 48s208 93.31 208 208-93.31 208-208 208z"/><path d="M256 128c-70.69 0-128 57.31-128 128s57.31 128 128 128 128-57.31 128-128-57.31-128-128-128zm0 208c-44.11 0-80-35.89-80-80s35.89-80 80-80 80 35.89 80 80-35.89 80-80 80z"/></svg>
            Crates.io
          </a>
          <a href="https://www.npmjs.com/package/mathhook" target="_blank" class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-red-500/10 border border-red-500/20 rounded-full text-sm text-red-400 hover:bg-red-500/20 transition-colors">
            <svg class="w-4 h-4" viewBox="0 0 576 512" fill="currentColor"><path d="M288 288h-32v-64h32v64zm288-128v192H288v32H160v-32H0V160h576z"/></svg>
            npm
          </a>
          <a href="https://github.com/AhmedMashour/mathhook/blob/main/LICENSE" target="_blank" class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-step-green/10 border border-step-green/20 rounded-full text-sm text-step-green hover:bg-step-green/20 transition-colors">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
            MIT License
          </a>
        </div>

        <!-- CTA Buttons -->
        <div class="flex items-center justify-center gap-3 mb-16">
          <NuxtLink to="/docs" class="px-5 py-2.5 bg-gradient-to-r from-rust-core to-rust-core-400 text-white font-medium rounded-lg hover:opacity-90 transition-opacity">
            Get Started
          </NuxtLink>
          <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="px-5 py-2.5 bg-logic-navy-700 border border-logic-navy-600 text-chalk font-medium rounded-lg hover:bg-logic-navy-600 transition-colors flex items-center gap-2">
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
            View Source
          </a>
        </div>

        <!-- Code Example Card -->
        <div class="bg-logic-navy-800/80 backdrop-blur border border-logic-navy-700 rounded-xl overflow-hidden max-w-2xl mx-auto">
          <div class="flex items-center gap-2 px-4 py-2.5 border-b border-logic-navy-700 bg-logic-navy-800">
            <div class="flex gap-1.5">
              <div class="w-3 h-3 rounded-full bg-red-500/80"></div>
              <div class="w-3 h-3 rounded-full bg-yellow-500/80"></div>
              <div class="w-3 h-3 rounded-full bg-green-500/80"></div>
            </div>
            <span class="text-xs text-chalk-600 font-mono ml-2">main.rs</span>
          </div>
          <div class="p-5 text-left font-mono text-sm leading-relaxed">
            <div><span class="text-rust-core">use</span> <span class="text-solve-cyan">mathhook</span>::{symbol, expr};</div>
            <div class="mt-3 text-chalk-600">// Symbolic differentiation in &lt;300ns</div>
            <div><span class="text-rust-core">let</span> x = <span class="text-solve-cyan">symbol!</span>(x);</div>
            <div><span class="text-rust-core">let</span> f = <span class="text-solve-cyan">expr!</span>(x^<span class="text-step-green">3</span> + <span class="text-step-green">2</span>*x^<span class="text-step-green">2</span> - <span class="text-step-green">5</span>*x);</div>
            <div><span class="text-rust-core">let</span> df = f.<span class="text-solve-cyan">derivative</span>(&x);</div>
            <div class="mt-3 text-chalk-600">// Result: 3x² + 4x - 5</div>
          </div>
        </div>
      </div>
    </header>

    <!-- Key Features Section -->
    <section class="py-20 px-4 relative z-10">
      <div class="max-w-5xl mx-auto">
        <h2 class="text-2xl font-bold mb-10 text-chalk">Key Features</h2>

        <div class="grid md:grid-cols-2 gap-6">
          <!-- High-Performance -->
          <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-6">
            <div class="flex items-start gap-4">
              <div class="w-10 h-10 rounded-lg bg-rust-core/10 flex items-center justify-center flex-shrink-0">
                <svg class="w-5 h-5 text-rust-core" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                </svg>
              </div>
              <div>
                <h3 class="text-lg font-semibold text-rust-core mb-2">High-Performance</h3>
                <p class="text-chalk-400 text-sm leading-relaxed">
                  32-byte expression optimization with SIMD vectorization. Benchmarked at 10-100x faster than Python alternatives.
                </p>
                <div class="mt-3 inline-flex items-center gap-1.5 px-2 py-1 bg-solve-cyan/10 rounded text-xs text-solve-cyan font-mono">
                  <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"/></svg>
                  &lt; 300ns typical
                </div>
              </div>
            </div>
          </div>

          <!-- Educational -->
          <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-6">
            <div class="flex items-start gap-4">
              <div class="w-10 h-10 rounded-lg bg-step-green/10 flex items-center justify-center flex-shrink-0">
                <svg class="w-5 h-5 text-step-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                </svg>
              </div>
              <div>
                <h3 class="text-lg font-semibold text-step-green mb-2">Step-by-Step Solutions</h3>
                <p class="text-chalk-400 text-sm leading-relaxed">
                  Every computation includes detailed derivation steps. Perfect for learning, teaching, and debugging mathematical operations.
                </p>
                <div class="mt-3 inline-flex items-center gap-1.5 px-2 py-1 bg-step-green/10 rounded text-xs text-step-green">
                  21+ documented topics
                </div>
              </div>
            </div>
          </div>

          <!-- Multi-Language -->
          <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-6">
            <div class="flex items-start gap-4">
              <div class="w-10 h-10 rounded-lg bg-solve-cyan/10 flex items-center justify-center flex-shrink-0">
                <svg class="w-5 h-5 text-solve-cyan" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                </svg>
              </div>
              <div>
                <h3 class="text-lg font-semibold text-solve-cyan mb-2">Rust + Python + Node.js</h3>
                <p class="text-chalk-400 text-sm leading-relaxed">
                  Native Rust core with first-class bindings. Same API across all languages with zero performance compromise.
                </p>
                <div class="mt-3 flex gap-2">
                  <span class="px-2 py-1 bg-rust-core/10 rounded text-xs text-rust-core">Rust</span>
                  <span class="px-2 py-1 bg-amber-500/10 rounded text-xs text-amber-400">Python</span>
                  <span class="px-2 py-1 bg-step-green/10 rounded text-xs text-step-green">Node.js</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Mathematical Rigor -->
          <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-6">
            <div class="flex items-start gap-4">
              <div class="w-10 h-10 rounded-lg bg-violet-500/10 flex items-center justify-center flex-shrink-0">
                <svg class="w-5 h-5 text-violet-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"/>
                </svg>
              </div>
              <div>
                <h3 class="text-lg font-semibold text-violet-400 mb-2">Mathematically Rigorous</h3>
                <p class="text-chalk-400 text-sm leading-relaxed">
                  Exact rational arithmetic with symbolic precision. Validated against SymPy with comprehensive test coverage.
                </p>
                <div class="mt-3 inline-flex items-center gap-1.5 px-2 py-1 bg-violet-500/10 rounded text-xs text-violet-400">
                  SymPy-validated
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Topics Coverage -->
    <section class="py-20 px-4 relative z-10 border-t border-logic-navy-800">
      <div class="max-w-5xl mx-auto">
        <h2 class="text-2xl font-bold mb-10 text-chalk">What You Can Solve</h2>

        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <NuxtLink to="/docs" class="group p-4 bg-logic-navy-800/30 border border-logic-navy-700 rounded-lg hover:border-rust-core/50 transition-colors">
            <div class="text-rust-core font-semibold mb-1 group-hover:text-rust-core-300">Calculus</div>
            <div class="text-xs text-chalk-500">Derivatives, Integrals, Limits</div>
          </NuxtLink>
          <NuxtLink to="/docs" class="group p-4 bg-logic-navy-800/30 border border-logic-navy-700 rounded-lg hover:border-solve-cyan/50 transition-colors">
            <div class="text-solve-cyan font-semibold mb-1 group-hover:text-solve-cyan-300">ODEs</div>
            <div class="text-xs text-chalk-500">Separable, Linear, Bernoulli</div>
          </NuxtLink>
          <NuxtLink to="/docs" class="group p-4 bg-logic-navy-800/30 border border-logic-navy-700 rounded-lg hover:border-step-green/50 transition-colors">
            <div class="text-step-green font-semibold mb-1 group-hover:text-step-green-300">PDEs</div>
            <div class="text-xs text-chalk-500">Heat, Wave, Laplace</div>
          </NuxtLink>
          <NuxtLink to="/docs" class="group p-4 bg-logic-navy-800/30 border border-logic-navy-700 rounded-lg hover:border-amber-500/50 transition-colors">
            <div class="text-amber-500 font-semibold mb-1 group-hover:text-amber-400">Algebra</div>
            <div class="text-xs text-chalk-500">Simplification, Factoring</div>
          </NuxtLink>
        </div>

        <div class="mt-8 text-center">
          <NuxtLink to="/docs" class="text-solve-cyan hover:text-solve-cyan-300 text-sm font-medium inline-flex items-center gap-1">
            View all 21+ topics
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"/></svg>
          </NuxtLink>
        </div>
      </div>
    </section>

    <!-- Quick Install -->
    <section class="py-20 px-4 relative z-10 border-t border-logic-navy-800">
      <div class="max-w-3xl mx-auto">
        <h2 class="text-2xl font-bold mb-8 text-chalk text-center">Quick Install</h2>

        <div class="grid md:grid-cols-3 gap-4">
          <!-- Rust -->
          <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-lg p-4">
            <div class="flex items-center gap-2 mb-3">
              <span class="text-rust-core font-semibold text-sm">Rust</span>
            </div>
            <code class="text-xs text-chalk-300 font-mono block bg-logic-navy-900 rounded p-2">
              cargo add mathhook
            </code>
          </div>

          <!-- Python -->
          <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-lg p-4">
            <div class="flex items-center gap-2 mb-3">
              <span class="text-amber-400 font-semibold text-sm">Python</span>
            </div>
            <code class="text-xs text-chalk-300 font-mono block bg-logic-navy-900 rounded p-2">
              pip install mathhook
            </code>
          </div>

          <!-- Node.js -->
          <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-lg p-4">
            <div class="flex items-center gap-2 mb-3">
              <span class="text-step-green font-semibold text-sm">Node.js</span>
            </div>
            <code class="text-xs text-chalk-300 font-mono block bg-logic-navy-900 rounded p-2">
              npm install mathhook
            </code>
          </div>
        </div>
      </div>
    </section>

    <!-- Footer -->
    <footer class="py-10 px-4 border-t border-logic-navy-800 relative z-10">
      <div class="max-w-5xl mx-auto flex flex-col md:flex-row items-center justify-between gap-4">
        <div class="flex items-center gap-2">
          <svg class="w-5 h-5" viewBox="0 0 48 48" fill="none">
            <defs>
              <linearGradient id="footer-logo" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stop-color="#E64524"/>
                <stop offset="100%" stop-color="#06B6D4"/>
              </linearGradient>
            </defs>
            <path d="M12 40 C12 40 12 12 12 10 C12 6 16 4 20 8 L24 16 L28 8 C32 4 36 6 36 10 C36 12 36 40 36 40" stroke="url(#footer-logo)" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
          </svg>
          <span class="text-chalk-500 text-sm">
            <span class="text-rust-core">Math</span><span class="text-solve-cyan">Hook</span>
          </span>
        </div>
        <p class="text-chalk-600 text-sm">Symbolic Power. Educational Clarity. Native Speed.</p>
        <div class="flex items-center gap-4">
          <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk text-sm">GitHub</a>
          <a href="https://crates.io/crates/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk text-sm">Crates.io</a>
          <a href="https://www.npmjs.com/package/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk text-sm">npm</a>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup>
useHead({
  title: 'MathHook - High-Performance Computer Algebra System',
  meta: [
    { name: 'description', content: 'MathHook: High-performance CAS in Rust with Python and Node.js bindings. Symbolic Power. Educational Clarity. Native Speed.' }
  ]
})
</script>
