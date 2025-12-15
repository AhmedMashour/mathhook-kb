<template>
  <div class="min-h-screen bg-logic-navy-900 text-chalk overflow-hidden">
    <!-- Mathematical Grid Background -->
    <div class="fixed inset-0 pointer-events-none">
      <div class="absolute inset-0 bg-[linear-gradient(rgba(6,182,212,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(6,182,212,0.03)_1px,transparent_1px)] bg-[size:48px_48px]"></div>
      <div class="absolute inset-0 bg-gradient-to-b from-logic-navy-900 via-transparent to-logic-navy-900"></div>
      <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-rust-core/5 rounded-full blur-3xl animate-pulse"></div>
      <div class="absolute bottom-1/4 right-1/4 w-96 h-96 bg-solve-cyan/5 rounded-full blur-3xl animate-pulse" style="animation-delay: 1s"></div>
      <div class="absolute top-3/4 left-1/2 w-64 h-64 bg-step-green/5 rounded-full blur-3xl animate-pulse" style="animation-delay: 2s"></div>
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
    <header class="relative pt-28 pb-16 px-4">
      <div class="max-w-5xl mx-auto text-center relative z-10">
        <!-- Announcement Badge -->
        <div class="mb-6">
          <span class="inline-flex items-center gap-2 px-3 py-1.5 bg-gradient-to-r from-rust-core/20 to-solve-cyan/20 border border-logic-navy-600 rounded-full text-xs text-chalk-300">
            <span class="w-2 h-2 bg-step-green rounded-full animate-pulse"></span>
            Now with ODE & PDE support
          </span>
        </div>

        <!-- Logo + Title -->
        <div class="mb-8">
          <div class="flex items-center justify-center gap-4 mb-5">
            <svg class="w-14 h-14 md:w-16 md:h-16" viewBox="0 0 48 48" fill="none">
              <defs>
                <linearGradient id="hero-logo" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" stop-color="#E64524"/>
                  <stop offset="100%" stop-color="#06B6D4"/>
                </linearGradient>
              </defs>
              <path d="M12 40 C12 40 12 12 12 10 C12 6 16 4 20 8 L24 16 L28 8 C32 4 36 6 36 10 C36 12 36 40 36 40" stroke="url(#hero-logo)" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
            </svg>
            <h1 class="text-5xl md:text-6xl font-bold tracking-tight">
              <span class="text-rust-core">Math</span><span class="text-solve-cyan">Hook</span>
            </h1>
          </div>
          <p class="text-2xl md:text-3xl text-chalk font-medium mb-3 tracking-tight">
            Symbolic Power. Educational Clarity. Native Speed.
          </p>
          <p class="text-lg text-chalk-400 max-w-2xl mx-auto leading-relaxed">
            The high-performance computer algebra system written in Rust with first-class bindings for Python and Node.js
          </p>
        </div>

        <!-- Stats Row -->
        <div class="flex items-center justify-center gap-8 mb-10 text-sm">
          <div class="text-center">
            <div class="text-2xl font-bold text-rust-core">10-100x</div>
            <div class="text-chalk-500">Faster than SymPy</div>
          </div>
          <div class="w-px h-10 bg-logic-navy-700"></div>
          <div class="text-center">
            <div class="text-2xl font-bold text-solve-cyan">&lt;300ns</div>
            <div class="text-chalk-500">Per operation</div>
          </div>
          <div class="w-px h-10 bg-logic-navy-700"></div>
          <div class="text-center">
            <div class="text-2xl font-bold text-step-green">21+</div>
            <div class="text-chalk-500">Math topics</div>
          </div>
        </div>

        <!-- CTA Buttons -->
        <div class="flex items-center justify-center gap-4 mb-12">
          <NuxtLink to="/docs" class="group px-6 py-3 bg-gradient-to-r from-rust-core to-rust-core-400 text-white font-medium rounded-lg hover:opacity-90 transition-all hover:scale-105 flex items-center gap-2">
            Get Started
            <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"/></svg>
          </NuxtLink>
          <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="px-6 py-3 bg-logic-navy-800 border border-logic-navy-600 text-chalk font-medium rounded-lg hover:bg-logic-navy-700 transition-colors flex items-center gap-2">
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
            Star on GitHub
          </a>
        </div>

        <!-- Interactive Code Demo -->
        <div class="max-w-3xl mx-auto">
          <div class="bg-logic-navy-800/80 backdrop-blur border border-logic-navy-700 rounded-xl overflow-hidden shadow-2xl">
            <!-- Language Tabs -->
            <div class="flex border-b border-logic-navy-700">
              <button
                v-for="lang in ['rust', 'python', 'javascript']"
                :key="lang"
                @click="activeLanguage = lang"
                :class="[
                  'flex-1 px-4 py-3 text-sm font-medium transition-all relative',
                  activeLanguage === lang
                    ? lang === 'rust' ? 'text-rust-core bg-logic-navy-800' : lang === 'python' ? 'text-amber-400 bg-logic-navy-800' : 'text-step-green bg-logic-navy-800'
                    : 'text-chalk-500 hover:text-chalk hover:bg-logic-navy-800/50'
                ]"
              >
                <span class="flex items-center justify-center gap-2">
                  <span>{{ lang === 'rust' ? '🦀' : lang === 'python' ? '🐍' : '📜' }}</span>
                  <span class="capitalize">{{ lang }}</span>
                </span>
                <div v-if="activeLanguage === lang" class="absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-rust-core to-solve-cyan"></div>
              </button>
            </div>

            <!-- Code Content -->
            <div class="p-5 text-left font-mono text-sm leading-relaxed min-h-[180px]">
              <!-- Rust -->
              <div v-if="activeLanguage === 'rust'">
                <div><span class="text-rust-core">use</span> <span class="text-solve-cyan">mathhook</span>::{symbol, expr};</div>
                <div class="mt-3 text-chalk-600">// Symbolic differentiation</div>
                <div><span class="text-rust-core">let</span> x = <span class="text-solve-cyan">symbol!</span>(x);</div>
                <div><span class="text-rust-core">let</span> f = <span class="text-solve-cyan">expr!</span>(x^<span class="text-step-green">3</span> + <span class="text-step-green">2</span>*x^<span class="text-step-green">2</span> - <span class="text-step-green">5</span>*x);</div>
                <div><span class="text-rust-core">let</span> df = f.<span class="text-solve-cyan">derivative</span>(&x);</div>
                <div class="mt-3 text-chalk-600">// Result: 3x² + 4x - 5</div>
                <div class="mt-1"><span class="text-rust-core">let</span> steps = f.<span class="text-solve-cyan">derivative_steps</span>(&x);</div>
                <div class="text-chalk-600">// Get detailed derivation steps!</div>
              </div>

              <!-- Python -->
              <div v-if="activeLanguage === 'python'">
                <div><span class="text-rust-core">from</span> <span class="text-solve-cyan">mathhook</span> <span class="text-rust-core">import</span> symbol, expr</div>
                <div class="mt-3 text-chalk-600"># Symbolic differentiation</div>
                <div>x = <span class="text-solve-cyan">symbol</span>(<span class="text-amber-400">'x'</span>)</div>
                <div>f = <span class="text-solve-cyan">expr</span>(<span class="text-amber-400">'x^3 + 2*x^2 - 5*x'</span>)</div>
                <div>df = f.<span class="text-solve-cyan">derivative</span>(x)</div>
                <div class="mt-3 text-chalk-600"># Result: 3x² + 4x - 5</div>
                <div class="mt-1">steps = f.<span class="text-solve-cyan">derivative_steps</span>(x)</div>
                <div class="text-chalk-600"># Get detailed derivation steps!</div>
              </div>

              <!-- JavaScript -->
              <div v-if="activeLanguage === 'javascript'">
                <div><span class="text-rust-core">import</span> { symbol, expr } <span class="text-rust-core">from</span> <span class="text-amber-400">'mathhook'</span>;</div>
                <div class="mt-3 text-chalk-600">// Symbolic differentiation</div>
                <div><span class="text-rust-core">const</span> x = <span class="text-solve-cyan">symbol</span>(<span class="text-amber-400">'x'</span>);</div>
                <div><span class="text-rust-core">const</span> f = <span class="text-solve-cyan">expr</span>(<span class="text-amber-400">'x^3 + 2*x^2 - 5*x'</span>);</div>
                <div><span class="text-rust-core">const</span> df = f.<span class="text-solve-cyan">derivative</span>(x);</div>
                <div class="mt-3 text-chalk-600">// Result: 3x² + 4x - 5</div>
                <div class="mt-1"><span class="text-rust-core">const</span> steps = f.<span class="text-solve-cyan">derivativeSteps</span>(x);</div>
                <div class="text-chalk-600">// Get detailed derivation steps!</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </header>

    <!-- Ecosystem Badges -->
    <section class="py-8 px-4 relative z-10">
      <div class="max-w-4xl mx-auto">
        <div class="flex flex-wrap items-center justify-center gap-4">
          <a href="https://crates.io/crates/mathhook" target="_blank" class="inline-flex items-center gap-2 px-4 py-2 bg-logic-navy-800/50 border border-logic-navy-700 rounded-lg text-sm text-chalk-300 hover:border-rust-core/50 transition-colors">
            <svg class="w-5 h-5 text-rust-core" viewBox="0 0 512 512" fill="currentColor"><path d="M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256 256-114.6 256-256S397.4 0 256 0zm0 464c-114.7 0-208-93.31-208-208S141.3 48 256 48s208 93.31 208 208-93.31 208-208 208z"/><path d="M256 128c-70.69 0-128 57.31-128 128s57.31 128 128 128 128-57.31 128-128-57.31-128-128-128zm0 208c-44.11 0-80-35.89-80-80s35.89-80 80-80 80 35.89 80 80-35.89 80-80 80z"/></svg>
            Crates.io
          </a>
          <a href="https://pypi.org/project/mathhook" target="_blank" class="inline-flex items-center gap-2 px-4 py-2 bg-logic-navy-800/50 border border-logic-navy-700 rounded-lg text-sm text-chalk-300 hover:border-amber-500/50 transition-colors">
            <svg class="w-5 h-5 text-amber-400" viewBox="0 0 256 255" fill="currentColor"><path d="M126.916.072c-64.832 0-60.784 28.115-60.784 28.115l.072 29.128h61.868v8.745H41.631S.145 61.355.145 126.77c0 65.417 36.21 63.097 36.21 63.097h21.61v-30.356s-1.165-36.21 35.632-36.21h61.362s34.475.557 34.475-33.319V33.97S194.67.072 126.916.072zM92.802 19.66a11.12 11.12 0 0 1 11.13 11.13 11.12 11.12 0 0 1-11.13 11.13 11.12 11.12 0 0 1-11.13-11.13 11.12 11.12 0 0 1 11.13-11.13z"/><path d="M128.757 254.126c64.832 0 60.784-28.115 60.784-28.115l-.072-29.127H127.6v-8.745h86.441s41.486 4.705 41.486-60.712c0-65.416-36.21-63.096-36.21-63.096h-21.61v30.355s1.165 36.21-35.632 36.21h-61.362s-34.475-.557-34.475 33.32v56.013s-5.235 33.897 62.518 33.897zm34.114-19.586a11.12 11.12 0 0 1-11.13-11.13 11.12 11.12 0 0 1 11.13-11.131 11.12 11.12 0 0 1 11.13 11.13 11.12 11.12 0 0 1-11.13 11.13z"/></svg>
            PyPI
          </a>
          <a href="https://www.npmjs.com/package/mathhook" target="_blank" class="inline-flex items-center gap-2 px-4 py-2 bg-logic-navy-800/50 border border-logic-navy-700 rounded-lg text-sm text-chalk-300 hover:border-red-500/50 transition-colors">
            <svg class="w-5 h-5 text-red-400" viewBox="0 0 576 512" fill="currentColor"><path d="M288 288h-32v-64h32v64zm288-128v192H288v32H160v-32H0V160h576z"/></svg>
            npm
          </a>
          <a href="https://github.com/AhmedMashour/mathhook/blob/main/LICENSE" target="_blank" class="inline-flex items-center gap-2 px-4 py-2 bg-logic-navy-800/50 border border-logic-navy-700 rounded-lg text-sm text-chalk-300 hover:border-step-green/50 transition-colors">
            <svg class="w-5 h-5 text-step-green" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
            MIT License
          </a>
        </div>
      </div>
    </section>

    <!-- Why MathHook Section -->
    <section class="py-20 px-4 relative z-10">
      <div class="max-w-6xl mx-auto">
        <div class="text-center mb-14">
          <h2 class="text-3xl font-bold text-chalk mb-3">Why MathHook?</h2>
          <p class="text-chalk-400 max-w-2xl mx-auto">Built for developers who need mathematical computing without the performance penalty</p>
        </div>

        <div class="grid md:grid-cols-3 gap-6">
          <!-- Performance -->
          <div class="group bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-6 hover:border-rust-core/50 transition-all hover:-translate-y-1">
            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-rust-core/20 to-rust-core/5 flex items-center justify-center mb-4">
              <svg class="w-6 h-6 text-rust-core" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
              </svg>
            </div>
            <h3 class="text-lg font-semibold text-chalk mb-2">Blazing Fast</h3>
            <p class="text-chalk-400 text-sm leading-relaxed mb-4">
              32-byte optimized expressions with SIMD vectorization. No GC pauses, no interpreter overhead.
            </p>
            <div class="flex items-center gap-2 text-xs">
              <span class="px-2 py-1 bg-rust-core/10 text-rust-core rounded">Zero-copy</span>
              <span class="px-2 py-1 bg-rust-core/10 text-rust-core rounded">SIMD</span>
            </div>
          </div>

          <!-- Educational -->
          <div class="group bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-6 hover:border-step-green/50 transition-all hover:-translate-y-1">
            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-step-green/20 to-step-green/5 flex items-center justify-center mb-4">
              <svg class="w-6 h-6 text-step-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
              </svg>
            </div>
            <h3 class="text-lg font-semibold text-chalk mb-2">Step-by-Step</h3>
            <p class="text-chalk-400 text-sm leading-relaxed mb-4">
              Every computation includes detailed derivation steps. Perfect for learning and debugging.
            </p>
            <div class="flex items-center gap-2 text-xs">
              <span class="px-2 py-1 bg-step-green/10 text-step-green rounded">LaTeX output</span>
              <span class="px-2 py-1 bg-step-green/10 text-step-green rounded">Explainable</span>
            </div>
          </div>

          <!-- Multi-language -->
          <div class="group bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-6 hover:border-solve-cyan/50 transition-all hover:-translate-y-1">
            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-solve-cyan/20 to-solve-cyan/5 flex items-center justify-center mb-4">
              <svg class="w-6 h-6 text-solve-cyan" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
              </svg>
            </div>
            <h3 class="text-lg font-semibold text-chalk mb-2">Multi-Language</h3>
            <p class="text-chalk-400 text-sm leading-relaxed mb-4">
              Native Rust core with first-class Python and Node.js bindings. Same API, same performance.
            </p>
            <div class="flex items-center gap-2 text-xs">
              <span class="px-2 py-1 bg-rust-core/10 text-rust-core rounded">Rust</span>
              <span class="px-2 py-1 bg-amber-500/10 text-amber-400 rounded">Python</span>
              <span class="px-2 py-1 bg-step-green/10 text-step-green rounded">Node</span>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Performance Benchmarks -->
    <section class="py-20 px-4 relative z-10 border-t border-logic-navy-800">
      <div class="max-w-5xl mx-auto">
        <div class="text-center mb-14">
          <h2 class="text-3xl font-bold text-chalk mb-3">Performance That Matters</h2>
          <p class="text-chalk-400">Benchmarked against leading symbolic math libraries</p>
        </div>

        <div class="grid md:grid-cols-2 gap-8 items-center">
          <!-- Benchmark Bars -->
          <div class="space-y-6">
            <div>
              <div class="flex justify-between text-sm mb-2">
                <span class="text-chalk font-medium">MathHook (Rust)</span>
                <span class="text-step-green">0.3ms</span>
              </div>
              <div class="h-3 bg-logic-navy-800 rounded-full overflow-hidden">
                <div class="h-full bg-gradient-to-r from-rust-core to-solve-cyan rounded-full" style="width: 3%"></div>
              </div>
            </div>
            <div>
              <div class="flex justify-between text-sm mb-2">
                <span class="text-chalk-400">SymPy (Python)</span>
                <span class="text-chalk-500">12ms</span>
              </div>
              <div class="h-3 bg-logic-navy-800 rounded-full overflow-hidden">
                <div class="h-full bg-amber-500/50 rounded-full" style="width: 40%"></div>
              </div>
            </div>
            <div>
              <div class="flex justify-between text-sm mb-2">
                <span class="text-chalk-400">Mathjs (Node.js)</span>
                <span class="text-chalk-500">28ms</span>
              </div>
              <div class="h-3 bg-logic-navy-800 rounded-full overflow-hidden">
                <div class="h-full bg-step-green/50 rounded-full" style="width: 95%"></div>
              </div>
            </div>
            <p class="text-xs text-chalk-600 mt-4">* Benchmark: 1000x derivative of x³ + 2x² - 5x</p>
          </div>

          <!-- Performance Stats -->
          <div class="grid grid-cols-2 gap-4">
            <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-5 text-center">
              <div class="text-3xl font-bold text-rust-core mb-1">40x</div>
              <div class="text-sm text-chalk-400">Faster than SymPy</div>
            </div>
            <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-5 text-center">
              <div class="text-3xl font-bold text-solve-cyan mb-1">93x</div>
              <div class="text-sm text-chalk-400">Faster than Mathjs</div>
            </div>
            <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-5 text-center">
              <div class="text-3xl font-bold text-step-green mb-1">32B</div>
              <div class="text-sm text-chalk-400">Expression size</div>
            </div>
            <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-5 text-center">
              <div class="text-3xl font-bold text-violet-400 mb-1">0</div>
              <div class="text-sm text-chalk-400">GC pauses</div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Topics Coverage -->
    <section class="py-20 px-4 relative z-10 border-t border-logic-navy-800">
      <div class="max-w-6xl mx-auto">
        <div class="text-center mb-14">
          <h2 class="text-3xl font-bold text-chalk mb-3">Comprehensive Math Coverage</h2>
          <p class="text-chalk-400">From basic algebra to advanced differential equations</p>
        </div>

        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <NuxtLink to="/docs" class="group p-5 bg-logic-navy-800/30 border border-logic-navy-700 rounded-xl hover:border-rust-core/50 transition-all hover:-translate-y-1">
            <div class="text-2xl mb-2">∫</div>
            <div class="text-rust-core font-semibold mb-1 group-hover:text-rust-core-300">Calculus</div>
            <div class="text-xs text-chalk-500">Derivatives, Integrals, Limits, Series</div>
          </NuxtLink>
          <NuxtLink to="/docs" class="group p-5 bg-logic-navy-800/30 border border-logic-navy-700 rounded-xl hover:border-solve-cyan/50 transition-all hover:-translate-y-1">
            <div class="text-2xl mb-2">dy/dx</div>
            <div class="text-solve-cyan font-semibold mb-1 group-hover:text-solve-cyan-300">ODEs</div>
            <div class="text-xs text-chalk-500">Separable, Linear, Bernoulli, Exact</div>
          </NuxtLink>
          <NuxtLink to="/docs" class="group p-5 bg-logic-navy-800/30 border border-logic-navy-700 rounded-xl hover:border-step-green/50 transition-all hover:-translate-y-1">
            <div class="text-2xl mb-2">∂²u</div>
            <div class="text-step-green font-semibold mb-1 group-hover:text-step-green-300">PDEs</div>
            <div class="text-xs text-chalk-500">Heat, Wave, Laplace, Poisson</div>
          </NuxtLink>
          <NuxtLink to="/docs" class="group p-5 bg-logic-navy-800/30 border border-logic-navy-700 rounded-xl hover:border-amber-500/50 transition-all hover:-translate-y-1">
            <div class="text-2xl mb-2">Σ</div>
            <div class="text-amber-400 font-semibold mb-1 group-hover:text-amber-300">Algebra</div>
            <div class="text-xs text-chalk-500">Simplify, Factor, Expand, Solve</div>
          </NuxtLink>
          <NuxtLink to="/docs" class="group p-5 bg-logic-navy-800/30 border border-logic-navy-700 rounded-xl hover:border-violet-500/50 transition-all hover:-translate-y-1">
            <div class="text-2xl mb-2">[ ]</div>
            <div class="text-violet-400 font-semibold mb-1 group-hover:text-violet-300">Linear Algebra</div>
            <div class="text-xs text-chalk-500">Matrices, Eigenvalues, SVD</div>
          </NuxtLink>
          <NuxtLink to="/docs" class="group p-5 bg-logic-navy-800/30 border border-logic-navy-700 rounded-xl hover:border-pink-500/50 transition-all hover:-translate-y-1">
            <div class="text-2xl mb-2">sin</div>
            <div class="text-pink-400 font-semibold mb-1 group-hover:text-pink-300">Trigonometry</div>
            <div class="text-xs text-chalk-500">Identities, Equations, Inverses</div>
          </NuxtLink>
          <NuxtLink to="/docs" class="group p-5 bg-logic-navy-800/30 border border-logic-navy-700 rounded-xl hover:border-cyan-500/50 transition-all hover:-translate-y-1">
            <div class="text-2xl mb-2">ℤ</div>
            <div class="text-cyan-400 font-semibold mb-1 group-hover:text-cyan-300">Number Theory</div>
            <div class="text-xs text-chalk-500">Primes, GCD, Modular Arithmetic</div>
          </NuxtLink>
          <NuxtLink to="/docs" class="group p-5 bg-logic-navy-800/30 border border-logic-navy-700 rounded-xl hover:border-orange-500/50 transition-all hover:-translate-y-1">
            <div class="text-2xl mb-2">∇</div>
            <div class="text-orange-400 font-semibold mb-1 group-hover:text-orange-300">Vector Calculus</div>
            <div class="text-xs text-chalk-500">Gradient, Divergence, Curl</div>
          </NuxtLink>
        </div>

        <div class="mt-10 text-center">
          <NuxtLink to="/docs" class="inline-flex items-center gap-2 px-5 py-2.5 bg-logic-navy-800 border border-logic-navy-600 rounded-lg text-chalk hover:bg-logic-navy-700 transition-colors">
            Explore all 21+ topics
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"/></svg>
          </NuxtLink>
        </div>
      </div>
    </section>

    <!-- Use Cases -->
    <section class="py-20 px-4 relative z-10 border-t border-logic-navy-800">
      <div class="max-w-6xl mx-auto">
        <div class="text-center mb-14">
          <h2 class="text-3xl font-bold text-chalk mb-3">Built For</h2>
          <p class="text-chalk-400">From research to production</p>
        </div>

        <div class="grid md:grid-cols-4 gap-6">
          <div class="text-center p-6">
            <div class="w-14 h-14 mx-auto mb-4 rounded-xl bg-rust-core/10 flex items-center justify-center">
              <svg class="w-7 h-7 text-rust-core" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
              </svg>
            </div>
            <h3 class="font-semibold text-chalk mb-2">Machine Learning</h3>
            <p class="text-sm text-chalk-500">Symbolic gradients for neural networks and optimization</p>
          </div>
          <div class="text-center p-6">
            <div class="w-14 h-14 mx-auto mb-4 rounded-xl bg-step-green/10 flex items-center justify-center">
              <svg class="w-7 h-7 text-step-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
              </svg>
            </div>
            <h3 class="font-semibold text-chalk mb-2">Education</h3>
            <p class="text-sm text-chalk-500">Step-by-step solutions for teaching and learning</p>
          </div>
          <div class="text-center p-6">
            <div class="w-14 h-14 mx-auto mb-4 rounded-xl bg-solve-cyan/10 flex items-center justify-center">
              <svg class="w-7 h-7 text-solve-cyan" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/>
              </svg>
            </div>
            <h3 class="font-semibold text-chalk mb-2">Research</h3>
            <p class="text-sm text-chalk-500">High-performance symbolic computation for science</p>
          </div>
          <div class="text-center p-6">
            <div class="w-14 h-14 mx-auto mb-4 rounded-xl bg-violet-500/10 flex items-center justify-center">
              <svg class="w-7 h-7 text-violet-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"/>
              </svg>
            </div>
            <h3 class="font-semibold text-chalk mb-2">Development</h3>
            <p class="text-sm text-chalk-500">Embed math in your applications with native speed</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Quick Install -->
    <section class="py-20 px-4 relative z-10 border-t border-logic-navy-800">
      <div class="max-w-4xl mx-auto">
        <div class="text-center mb-12">
          <h2 class="text-3xl font-bold text-chalk mb-3">Get Started in Seconds</h2>
          <p class="text-chalk-400">Choose your language and start computing</p>
        </div>

        <div class="grid md:grid-cols-3 gap-6">
          <!-- Rust -->
          <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-6 hover:border-rust-core/50 transition-colors">
            <div class="flex items-center gap-3 mb-4">
              <span class="text-2xl">🦀</span>
              <span class="text-rust-core font-semibold">Rust</span>
            </div>
            <code class="text-sm text-chalk-300 font-mono block bg-logic-navy-900 rounded-lg p-3">
              cargo add mathhook
            </code>
          </div>

          <!-- Python -->
          <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-6 hover:border-amber-500/50 transition-colors">
            <div class="flex items-center gap-3 mb-4">
              <span class="text-2xl">🐍</span>
              <span class="text-amber-400 font-semibold">Python</span>
            </div>
            <code class="text-sm text-chalk-300 font-mono block bg-logic-navy-900 rounded-lg p-3">
              pip install mathhook
            </code>
          </div>

          <!-- Node.js -->
          <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-6 hover:border-step-green/50 transition-colors">
            <div class="flex items-center gap-3 mb-4">
              <span class="text-2xl">📜</span>
              <span class="text-step-green font-semibold">Node.js</span>
            </div>
            <code class="text-sm text-chalk-300 font-mono block bg-logic-navy-900 rounded-lg p-3">
              npm install mathhook
            </code>
          </div>
        </div>
      </div>
    </section>

    <!-- CTA Section -->
    <section class="py-20 px-4 relative z-10 border-t border-logic-navy-800">
      <div class="max-w-3xl mx-auto text-center">
        <h2 class="text-3xl font-bold text-chalk mb-4">Ready to Solve?</h2>
        <p class="text-chalk-400 mb-8 text-lg">Join developers using MathHook for high-performance symbolic math</p>
        <div class="flex items-center justify-center gap-4">
          <NuxtLink to="/docs" class="px-8 py-3 bg-gradient-to-r from-rust-core to-solve-cyan text-white font-medium rounded-lg hover:opacity-90 transition-opacity">
            Read the Docs
          </NuxtLink>
          <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="px-8 py-3 bg-logic-navy-800 border border-logic-navy-600 text-chalk font-medium rounded-lg hover:bg-logic-navy-700 transition-colors">
            View on GitHub
          </a>
        </div>
      </div>
    </section>

    <!-- Footer -->
    <footer class="py-12 px-4 border-t border-logic-navy-800 relative z-10">
      <div class="max-w-6xl mx-auto">
        <div class="flex flex-col md:flex-row items-center justify-between gap-6">
          <div class="flex items-center gap-3">
            <svg class="w-6 h-6" viewBox="0 0 48 48" fill="none">
              <defs>
                <linearGradient id="footer-logo" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" stop-color="#E64524"/>
                  <stop offset="100%" stop-color="#06B6D4"/>
                </linearGradient>
              </defs>
              <path d="M12 40 C12 40 12 12 12 10 C12 6 16 4 20 8 L24 16 L28 8 C32 4 36 6 36 10 C36 12 36 40 36 40" stroke="url(#footer-logo)" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
            </svg>
            <span class="text-chalk-400 text-sm">
              <span class="text-rust-core">Math</span><span class="text-solve-cyan">Hook</span>
              <span class="text-chalk-600 ml-2">Symbolic Power. Educational Clarity. Native Speed.</span>
            </span>
          </div>
          <div class="flex items-center gap-6 text-sm">
            <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors">GitHub</a>
            <a href="https://crates.io/crates/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors">Crates.io</a>
            <a href="https://pypi.org/project/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors">PyPI</a>
            <a href="https://www.npmjs.com/package/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors">npm</a>
          </div>
        </div>
        <div class="mt-8 pt-6 border-t border-logic-navy-800 text-center text-xs text-chalk-600">
          Built with Rust. Open source under MIT License.
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup>
const activeLanguage = ref('rust')

useHead({
  title: 'MathHook - High-Performance Computer Algebra System',
  meta: [
    { name: 'description', content: 'MathHook: High-performance CAS in Rust with Python and Node.js bindings. Symbolic Power. Educational Clarity. Native Speed.' }
  ]
})
</script>

<style scoped>
@keyframes pulse {
  0%, 100% { opacity: 0.5; }
  50% { opacity: 1; }
}
</style>
