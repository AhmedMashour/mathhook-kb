<template>
  <div class="min-h-screen bg-logic-navy-900 text-chalk overflow-hidden">
    <!-- Vibrant Animated Mesh Gradient Background -->
    <div class="fixed inset-0 pointer-events-none overflow-hidden" ref="backgroundRef">
      <!-- Base dark layer -->
      <div class="absolute inset-0 bg-logic-navy-900"></div>

      <!-- Animated mesh gradient blobs -->
      <div class="absolute inset-0">
        <!-- Primary rust/orange blob - top left -->
        <div
          class="absolute w-[800px] h-[800px] rounded-full animate-blob-1"
          :style="{
            background: 'radial-gradient(circle at center, rgba(230, 69, 36, 0.45) 0%, rgba(230, 69, 36, 0.2) 40%, transparent 70%)',
            filter: 'blur(60px)',
            top: '-15%',
            left: '-10%',
            transform: `translate(${parallax.x * 0.03}px, ${parallax.y * 0.03}px)`
          }"
        ></div>

        <!-- Secondary cyan blob - bottom right -->
        <div
          class="absolute w-[700px] h-[700px] rounded-full animate-blob-2"
          :style="{
            background: 'radial-gradient(circle at center, rgba(6, 182, 212, 0.5) 0%, rgba(6, 182, 212, 0.2) 40%, transparent 70%)',
            filter: 'blur(60px)',
            bottom: '-10%',
            right: '-5%',
            transform: `translate(${parallax.x * -0.025}px, ${parallax.y * -0.025}px)`
          }"
        ></div>

        <!-- Tertiary purple/violet blob - center -->
        <div
          class="absolute w-[600px] h-[600px] rounded-full animate-blob-3"
          :style="{
            background: 'radial-gradient(circle at center, rgba(139, 92, 246, 0.4) 0%, rgba(139, 92, 246, 0.15) 40%, transparent 70%)',
            filter: 'blur(80px)',
            top: '30%',
            left: '40%',
            transform: `translate(${parallax.x * 0.02}px, ${parallax.y * -0.02}px)`
          }"
        ></div>

        <!-- Accent pink blob - top right -->
        <div
          class="absolute w-[500px] h-[500px] rounded-full animate-blob-4"
          :style="{
            background: 'radial-gradient(circle at center, rgba(236, 72, 153, 0.35) 0%, rgba(236, 72, 153, 0.1) 40%, transparent 70%)',
            filter: 'blur(70px)',
            top: '5%',
            right: '15%',
            transform: `translate(${parallax.x * -0.015}px, ${parallax.y * 0.015}px)`
          }"
        ></div>

        <!-- Accent green blob - bottom left -->
        <div
          class="absolute w-[450px] h-[450px] rounded-full animate-blob-5"
          :style="{
            background: 'radial-gradient(circle at center, rgba(34, 197, 94, 0.3) 0%, rgba(34, 197, 94, 0.1) 40%, transparent 70%)',
            filter: 'blur(60px)',
            bottom: '20%',
            left: '10%',
            transform: `translate(${parallax.x * 0.02}px, ${parallax.y * 0.02}px)`
          }"
        ></div>
      </div>

      <!-- Floating particles (client-only to avoid hydration mismatch) -->
      <ClientOnly>
        <div class="absolute inset-0">
          <div v-for="i in 30" :key="'particle-'+i"
               class="absolute rounded-full animate-float-particle"
               :style="{
                 width: `${2 + Math.random() * 4}px`,
                 height: `${2 + Math.random() * 4}px`,
                 background: ['rgba(230, 69, 36, 0.6)', 'rgba(6, 182, 212, 0.6)', 'rgba(139, 92, 246, 0.5)', 'rgba(255, 255, 255, 0.4)'][Math.floor(Math.random() * 4)],
                 left: `${Math.random() * 100}%`,
                 top: `${Math.random() * 100}%`,
                 animationDelay: `${Math.random() * 8}s`,
                 animationDuration: `${8 + Math.random() * 12}s`
               }"
          ></div>
        </div>
      </ClientOnly>

      <!-- Subtle grid overlay -->
      <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:60px_60px]"></div>

      <!-- Top fade for readability -->
      <div class="absolute inset-0 bg-gradient-to-b from-logic-navy-900/50 via-transparent to-logic-navy-900/70"></div>
    </div>

    <!-- Navigation with blur on scroll -->
    <nav
      class="fixed top-0 left-0 right-0 z-50 transition-all duration-500"
      :class="scrollY > 50 ? 'bg-logic-navy-900/95 backdrop-blur-xl border-b border-logic-navy-700/50 shadow-lg shadow-black/10' : 'bg-transparent'"
    >
      <div class="max-w-6xl mx-auto px-4 sm:px-6">
        <div class="flex items-center justify-between h-16">
          <NuxtLink to="/" class="flex items-center gap-2 group">
            <svg class="w-8 h-8 transition-transform duration-300 group-hover:scale-110" viewBox="0 0 48 48" fill="none">
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
          <div class="flex items-center gap-6 text-sm">
            <NuxtLink to="/docs" class="text-chalk-400 hover:text-chalk transition-all duration-300 hover:translate-y-[-2px]">Docs</NuxtLink>
            <NuxtLink to="/outputs" class="text-chalk-400 hover:text-chalk transition-all duration-300 hover:translate-y-[-2px]">Outputs</NuxtLink>
            <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="text-chalk-400 hover:text-chalk transition-all duration-300 flex items-center gap-1.5 hover:translate-y-[-2px]">
              <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
              GitHub
            </a>
          </div>
        </div>
      </div>
    </nav>

    <!-- Hero Section with Reveal Animations -->
    <header class="relative pt-32 pb-20 px-4 min-h-screen flex items-center">
      <div class="max-w-5xl mx-auto text-center relative z-10 w-full">
        <!-- Announcement Badge with Glow -->
        <div
          class="mb-8 animate-fade-in-up"
          :class="{ 'opacity-100 translate-y-0': mounted, 'opacity-0 translate-y-4': !mounted }"
          style="transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1); transition-delay: 0.1s"
        >
          <span class="inline-flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-rust-core/20 to-solve-cyan/20 border border-logic-navy-600/50 rounded-full text-xs text-chalk-300 backdrop-blur-sm shadow-lg shadow-rust-core/5">
            <span class="relative flex h-2 w-2">
              <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-step-green opacity-75"></span>
              <span class="relative inline-flex rounded-full h-2 w-2 bg-step-green"></span>
            </span>
            Now with ODE & PDE support
          </span>
        </div>

        <!-- Logo + Title with Text Reveal -->
        <div class="mb-10">
          <div
            class="flex items-center justify-center gap-5 mb-6"
            :class="{ 'opacity-100 translate-y-0': mounted, 'opacity-0 translate-y-6': !mounted }"
            style="transition: all 1s cubic-bezier(0.16, 1, 0.3, 1); transition-delay: 0.2s"
          >
            <svg class="w-16 h-16 md:w-20 md:h-20 animate-float" viewBox="0 0 48 48" fill="none">
              <defs>
                <linearGradient id="hero-logo" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" stop-color="#E64524"/>
                  <stop offset="100%" stop-color="#06B6D4"/>
                </linearGradient>
              </defs>
              <path d="M12 40 C12 40 12 12 12 10 C12 6 16 4 20 8 L24 16 L28 8 C32 4 36 6 36 10 C36 12 36 40 36 40" stroke="url(#hero-logo)" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
            </svg>
            <h1 class="text-6xl md:text-7xl font-bold tracking-tight">
              <span class="bg-gradient-to-r from-rust-core to-rust-core-400 bg-clip-text text-transparent">Math</span><span class="bg-gradient-to-r from-solve-cyan to-cyan-300 bg-clip-text text-transparent">Hook</span>
            </h1>
          </div>

          <!-- Animated Tagline with Word Reveal -->
          <div class="mb-4 overflow-hidden">
            <p class="text-2xl md:text-4xl font-semibold tracking-tight leading-tight">
              <span
                v-for="(word, index) in taglineWords"
                :key="index"
                class="inline-block overflow-hidden mx-0.5"
              >
                <span
                  class="inline-block transition-transform duration-500 ease-out"
                  :class="[word.color, mounted ? 'translate-y-0' : 'translate-y-full']"
                  :style="{ transitionDelay: `${600 + index * 100}ms` }"
                >{{ word.text }}</span>
              </span>
            </p>
          </div>

          <p
            class="text-lg md:text-xl text-chalk-400 max-w-2xl mx-auto leading-relaxed"
            :class="{ 'opacity-100 translate-y-0': mounted, 'opacity-0 translate-y-4': !mounted }"
            style="transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1); transition-delay: 0.9s"
          >
            A Rust computer algebra system with native Python and Node.js bindings. No GC, no compromises.
          </p>
        </div>

        <!-- Stats Row with Count Animation -->
        <div
          class="flex items-center justify-center gap-8 md:gap-12 mb-12"
          :class="{ 'opacity-100 translate-y-0': mounted, 'opacity-0 translate-y-4': !mounted }"
          style="transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1); transition-delay: 1.1s"
        >
          <div class="text-center group cursor-default">
            <div class="text-3xl md:text-4xl font-bold text-rust-core transition-transform duration-300 group-hover:scale-110">&lt;300ns</div>
            <div class="text-chalk-500 text-sm mt-1">Integration</div>
          </div>
          <div class="w-px h-12 bg-gradient-to-b from-transparent via-logic-navy-600 to-transparent"></div>
          <div class="text-center group cursor-default">
            <div class="text-3xl md:text-4xl font-bold text-solve-cyan transition-transform duration-300 group-hover:scale-110">&lt;2μs</div>
            <div class="text-chalk-500 text-sm mt-1">Derivatives</div>
          </div>
          <div class="w-px h-12 bg-gradient-to-b from-transparent via-logic-navy-600 to-transparent"></div>
          <div class="text-center group cursor-default">
            <div class="text-3xl md:text-4xl font-bold text-step-green transition-transform duration-300 group-hover:scale-110">32B</div>
            <div class="text-chalk-500 text-sm mt-1">Expression size</div>
          </div>
        </div>

        <!-- CTA Buttons with Glow Effect -->
        <div
          class="flex flex-col sm:flex-row items-center justify-center gap-4 mb-16"
          :class="{ 'opacity-100 translate-y-0': mounted, 'opacity-0 translate-y-4': !mounted }"
          style="transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1); transition-delay: 1.3s"
        >
          <NuxtLink to="/docs" class="group relative px-8 py-4 bg-gradient-to-r from-rust-core to-rust-core-400 text-white font-semibold rounded-xl transition-all duration-300 hover:scale-105 hover:shadow-xl hover:shadow-rust-core/25 flex items-center gap-2 overflow-hidden">
            <span class="relative z-10">Get Started</span>
            <svg class="w-5 h-5 relative z-10 transition-transform duration-300 group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"/></svg>
            <div class="absolute inset-0 bg-gradient-to-r from-rust-core-400 to-rust-core opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
          </NuxtLink>
          <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="group px-8 py-4 bg-logic-navy-800/80 border border-logic-navy-600 text-chalk font-semibold rounded-xl hover:bg-logic-navy-700 hover:border-logic-navy-500 transition-all duration-300 hover:scale-105 flex items-center gap-2 backdrop-blur-sm">
            <svg class="w-5 h-5 transition-transform duration-300 group-hover:rotate-12" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
            Star on GitHub
          </a>
        </div>

        <!-- Interactive Code Demo with Enhanced Animation -->
        <div
          class="max-w-3xl mx-auto"
          :class="{ 'opacity-100 translate-y-0': mounted, 'opacity-0 translate-y-8': !mounted }"
          style="transition: all 1s cubic-bezier(0.16, 1, 0.3, 1); transition-delay: 1.5s"
        >
          <div class="relative group">
            <!-- Glow effect behind card -->
            <div class="absolute -inset-1 bg-gradient-to-r from-rust-core/20 via-solve-cyan/20 to-step-green/20 rounded-2xl blur-xl opacity-50 group-hover:opacity-75 transition-opacity duration-500"></div>

            <div class="relative bg-logic-navy-800/90 backdrop-blur-xl border border-logic-navy-700/50 rounded-2xl overflow-hidden shadow-2xl">
              <!-- Language Tabs -->
              <div class="flex border-b border-logic-navy-700/50">
                <button
                  v-for="lang in ['rust', 'python', 'javascript']"
                  :key="lang"
                  @click="activeLanguage = lang"
                  :class="[
                    'flex-1 px-4 py-4 text-sm font-medium transition-all duration-300 relative overflow-hidden',
                    activeLanguage === lang
                      ? lang === 'rust' ? 'text-rust-core bg-logic-navy-800/50' : lang === 'python' ? 'text-amber-400 bg-logic-navy-800/50' : 'text-step-green bg-logic-navy-800/50'
                      : 'text-chalk-500 hover:text-chalk hover:bg-logic-navy-800/30'
                  ]"
                >
                  <span class="flex items-center justify-center gap-2 relative z-10">
                    <span class="text-lg">{{ lang === 'rust' ? '🦀' : lang === 'python' ? '🐍' : '📜' }}</span>
                    <span class="capitalize">{{ lang }}</span>
                  </span>
                  <div
                    v-if="activeLanguage === lang"
                    class="absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-rust-core via-solve-cyan to-step-green animate-gradient-x"
                  ></div>
                </button>
              </div>

              <!-- Code Content with Transition -->
              <div class="p-6 text-left font-mono text-sm leading-relaxed min-h-[240px]">
                <Transition name="code-fade" mode="out-in">
                  <!-- Rust -->
                  <div v-if="activeLanguage === 'rust'" key="rust">
                    <div><span class="text-rust-core">use</span> <span class="text-solve-cyan">mathhook_core</span>::prelude::*;</div>
                    <div class="mt-4 text-chalk-600">// Create expressions using ergonomic macros</div>
                    <div><span class="text-rust-core">let</span> x = <span class="text-solve-cyan">symbol!</span>(x);</div>
                    <div><span class="text-rust-core">let</span> expr = <span class="text-solve-cyan">expr!</span>((x ^ <span class="text-step-green">2</span>) + (<span class="text-step-green">2</span> * x) + <span class="text-step-green">1</span>);</div>
                    <div class="mt-4 text-chalk-600">// Simplify and solve</div>
                    <div><span class="text-rust-core">let</span> simplified = expr.<span class="text-solve-cyan">simplify</span>();</div>
                    <div><span class="text-rust-core">let</span> solutions = solver.<span class="text-solve-cyan">solve</span>(&equation, &x);</div>
                    <div class="mt-4 text-chalk-600">// Parse LaTeX: \frac{'{'}x{'}'}{'{'}2{'}'} + y²</div>
                    <div><span class="text-rust-core">let</span> parsed = parser.<span class="text-solve-cyan">parse</span>(r<span class="text-amber-400">"\frac{'{'}x{'}'}{'{'}2{'}'}"</span>);</div>
                  </div>

                  <!-- Python -->
                  <div v-else-if="activeLanguage === 'python'" key="python">
                    <div><span class="text-rust-core">from</span> <span class="text-solve-cyan">mathhook</span> <span class="text-rust-core">import</span> Expression, MathSolver</div>
                    <div class="mt-4 text-chalk-600"># Create expressions</div>
                    <div>x = Expression.<span class="text-solve-cyan">symbol</span>(<span class="text-amber-400">'x'</span>)</div>
                    <div>expr = x.<span class="text-solve-cyan">pow</span>(<span class="text-step-green">2</span>).<span class="text-solve-cyan">add</span>(x.<span class="text-solve-cyan">multiply</span>(<span class="text-step-green">2</span>)).<span class="text-solve-cyan">add</span>(<span class="text-step-green">1</span>)</div>
                    <div class="mt-4 text-chalk-600"># Simplify and solve equations</div>
                    <div>simplified = expr.<span class="text-solve-cyan">simplify</span>()</div>
                    <div>solutions = solver.<span class="text-solve-cyan">solve</span>(equation, <span class="text-amber-400">'x'</span>)</div>
                    <div class="mt-4 text-chalk-600"># Parse LaTeX and get LaTeX output</div>
                    <div>parsed = Expression.<span class="text-solve-cyan">parse</span>(r<span class="text-amber-400">"\frac{'{'}x{'}'}{'{'}2{'}'}"</span>)</div>
                  </div>

                  <!-- JavaScript -->
                  <div v-else key="javascript">
                    <div><span class="text-rust-core">import</span> { JsExpression, JsMathSolver } <span class="text-rust-core">from</span> <span class="text-amber-400">'mathhook-node'</span>;</div>
                    <div class="mt-4 text-chalk-600">// Create expressions</div>
                    <div><span class="text-rust-core">const</span> x = JsExpression.<span class="text-solve-cyan">symbol</span>(<span class="text-amber-400">'x'</span>);</div>
                    <div><span class="text-rust-core">const</span> expr = x.<span class="text-solve-cyan">pow</span>(<span class="text-step-green">2</span>).<span class="text-solve-cyan">add</span>(x.<span class="text-solve-cyan">multiply</span>(<span class="text-step-green">2</span>)).<span class="text-solve-cyan">add</span>(<span class="text-step-green">1</span>);</div>
                    <div class="mt-4 text-chalk-600">// Simplify and solve equations</div>
                    <div><span class="text-rust-core">const</span> simplified = expr.<span class="text-solve-cyan">simplify</span>();</div>
                    <div><span class="text-rust-core">const</span> solutions = solver.<span class="text-solve-cyan">solve</span>(equation, <span class="text-amber-400">'x'</span>);</div>
                    <div class="mt-4 text-chalk-600">// Parse LaTeX</div>
                    <div><span class="text-rust-core">const</span> parsed = JsExpression.<span class="text-solve-cyan">parse</span>(String.raw<span class="text-amber-400">`\frac{'{'}x{'}'}{'{'}2{'}'}`</span>);</div>
                  </div>
                </Transition>
              </div>
            </div>
          </div>
        </div>

        <!-- Scroll indicator -->
        <div
          class="absolute bottom-8 left-1/2 -translate-x-1/2 animate-bounce"
          :class="{ 'opacity-100': mounted && scrollY < 100, 'opacity-0': !mounted || scrollY >= 100 }"
          style="transition: opacity 0.5s"
        >
          <svg class="w-6 h-6 text-chalk-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"/>
          </svg>
        </div>
      </div>
    </header>

    <!-- Ecosystem Badges with Scroll Animation -->
    <section
      class="py-12 px-4 relative z-10"
      v-scroll-animate="{ animation: 'fade-up', delay: 0 }"
    >
      <div class="max-w-4xl mx-auto">
        <div class="flex flex-wrap items-center justify-center gap-4">
          <a
            v-for="(badge, index) in ecosystemBadges"
            :key="badge.name"
            :href="badge.url"
            target="_blank"
            class="inline-flex items-center gap-2 px-5 py-2.5 bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl text-sm text-chalk-300 hover:border-opacity-100 transition-all duration-300 hover:scale-105 hover:-translate-y-1 backdrop-blur-sm"
            :class="`hover:border-${badge.color}/50 hover:shadow-lg hover:shadow-${badge.color}/10`"
            :style="{ transitionDelay: `${index * 50}ms` }"
          >
            <span class="text-lg">{{ badge.emoji }}</span>
            {{ badge.name }}
          </a>
        </div>
      </div>
    </section>

    <!-- Key Features Section with Staggered Cards -->
    <section class="py-24 px-4 relative z-10" ref="featuresSection">
      <div class="max-w-6xl mx-auto">
        <div
          class="text-center mb-16"
          v-scroll-animate="{ animation: 'fade-up' }"
        >
          <span class="inline-block px-4 py-1.5 bg-rust-core/10 border border-rust-core/20 rounded-full text-xs text-rust-core font-medium mb-4">FEATURES</span>
          <h2 class="text-4xl md:text-5xl font-bold text-chalk mb-4">Key Features</h2>
          <p class="text-chalk-400 max-w-2xl mx-auto text-lg">A high-performance educational computer algebra system</p>
        </div>

        <div class="grid md:grid-cols-4 gap-5">
          <div
            v-for="(feature, index) in features"
            :key="feature.title"
            class="group bg-logic-navy-800/30 border border-logic-navy-700/50 rounded-2xl p-6 transition-all duration-500 hover:bg-logic-navy-800/50 hover:-translate-y-2 hover:shadow-xl cursor-default"
            :class="`hover:border-${feature.color}/50 hover:shadow-${feature.color}/10`"
            v-scroll-animate="{ animation: 'fade-up', delay: index * 100 }"
          >
            <div
              class="w-12 h-12 rounded-xl flex items-center justify-center mb-4 transition-transform duration-300 group-hover:scale-110 group-hover:rotate-3"
              :class="`bg-gradient-to-br from-${feature.color}/20 to-${feature.color}/5`"
            >
              <span class="text-xl">{{ feature.icon }}</span>
            </div>
            <h3 class="text-lg font-semibold text-chalk mb-2 group-hover:text-white transition-colors">{{ feature.title }}</h3>
            <p class="text-chalk-500 text-sm leading-relaxed group-hover:text-chalk-400 transition-colors">
              {{ feature.description }}
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- Performance Benchmarks with Animated Bars -->
    <section class="py-24 px-4 relative z-10">
      <div class="absolute inset-0 bg-gradient-to-b from-logic-navy-900 via-logic-navy-800/50 to-logic-navy-900 pointer-events-none"></div>

      <div class="max-w-5xl mx-auto relative">
        <div
          class="text-center mb-16"
          v-scroll-animate="{ animation: 'fade-up' }"
        >
          <span class="inline-block px-4 py-1.5 bg-solve-cyan/10 border border-solve-cyan/20 rounded-full text-xs text-solve-cyan font-medium mb-4">PERFORMANCE</span>
          <h2 class="text-4xl md:text-5xl font-bold text-chalk mb-4">Blazing Fast</h2>
          <p class="text-chalk-400 text-lg">Nanosecond-to-microsecond performance for symbolic operations</p>
        </div>

        <div class="grid md:grid-cols-2 gap-10 items-start">
          <!-- Benchmark Table with Animated Entries -->
          <div
            class="bg-logic-navy-800/50 border border-logic-navy-700/50 rounded-2xl overflow-hidden backdrop-blur-sm"
            v-scroll-animate="{ animation: 'fade-right' }"
          >
            <div class="px-6 py-4 border-b border-logic-navy-700/50 bg-logic-navy-800/30">
              <h3 class="font-semibold text-chalk text-lg">Operation Benchmarks</h3>
            </div>
            <div class="divide-y divide-logic-navy-700/30">
              <div
                v-for="(benchmark, index) in benchmarks"
                :key="benchmark.name"
                class="flex justify-between items-center px-6 py-4 hover:bg-logic-navy-700/20 transition-colors"
                v-scroll-animate="{ animation: 'fade-up', delay: index * 100 }"
              >
                <span class="text-chalk-400 text-sm">{{ benchmark.name }}</span>
                <span :class="`text-${benchmark.color} font-mono font-bold`">{{ benchmark.time }}</span>
              </div>
            </div>
            <div class="px-6 py-3 bg-logic-navy-900/50 text-xs text-chalk-600 flex items-center gap-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>
              Benchmarked with Criterion.rs
            </div>
          </div>

          <!-- Why So Fast -->
          <div
            class="space-y-5"
            v-scroll-animate="{ animation: 'fade-left' }"
          >
            <h3 class="font-semibold text-chalk text-xl mb-6">Why So Fast?</h3>
            <div
              v-for="(reason, index) in performanceReasons"
              :key="reason.title"
              class="flex items-start gap-4 group"
              v-scroll-animate="{ animation: 'fade-up', delay: index * 100 }"
            >
              <div
                class="w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0 transition-all duration-300 group-hover:scale-110"
                :class="`bg-${reason.color}/10`"
              >
                <span :class="`text-${reason.color} text-sm font-bold`">{{ reason.icon }}</span>
              </div>
              <div>
                <div class="text-chalk font-medium group-hover:text-white transition-colors">{{ reason.title }}</div>
                <div class="text-chalk-500 text-sm group-hover:text-chalk-400 transition-colors">{{ reason.description }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Topics Coverage with Interactive Cards -->
    <section class="py-24 px-4 relative z-10">
      <div class="max-w-6xl mx-auto">
        <div
          class="text-center mb-16"
          v-scroll-animate="{ animation: 'fade-up' }"
        >
          <span class="inline-block px-4 py-1.5 bg-step-green/10 border border-step-green/20 rounded-full text-xs text-step-green font-medium mb-4">DOCUMENTATION</span>
          <h2 class="text-4xl md:text-5xl font-bold text-chalk mb-4">Comprehensive Math Coverage</h2>
          <p class="text-chalk-400 text-lg">From basic algebra to advanced differential equations</p>
          <p v-if="categoryTopics.length > 0" class="text-xs text-chalk-600 mt-3">
            {{ totalTopicCount }} topics across {{ categoryTopics.length }} categories
          </p>
        </div>

        <div class="grid grid-cols-2 md:grid-cols-4 gap-5">
          <NuxtLink
            v-for="(cat, index) in categoryTopics"
            :key="cat.id"
            :to="cat.firstTopic ? `/docs/${cat.firstTopic}` : '/docs'"
            class="group relative p-6 bg-logic-navy-800/30 border border-logic-navy-700/50 rounded-2xl transition-all duration-500 hover:-translate-y-2 overflow-hidden"
            :class="`hover:border-${cat.color}/50`"
            v-scroll-animate="{ animation: 'fade-up', delay: index * 75 }"
          >
            <!-- Gradient overlay on hover -->
            <div
              class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none"
              :class="`bg-gradient-to-br from-${cat.color}/10 to-transparent`"
            ></div>

            <div class="relative z-10">
              <div class="text-3xl mb-3 transition-transform duration-300 group-hover:scale-110 group-hover:-rotate-6">{{ cat.icon }}</div>
              <div class="font-semibold mb-1 transition-colors" :class="`text-${cat.color} group-hover:text-white`">{{ cat.title }}</div>
              <div class="text-xs text-chalk-500 group-hover:text-chalk-400 transition-colors">{{ cat.description }}</div>
              <div class="text-xs text-chalk-600 mt-3 flex items-center gap-1">
                <span>{{ cat.count }} topics</span>
                <svg class="w-3 h-3 transition-transform duration-300 group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                </svg>
              </div>
            </div>
          </NuxtLink>
        </div>

        <div
          class="mt-12 text-center"
          v-scroll-animate="{ animation: 'fade-up', delay: 400 }"
        >
          <NuxtLink to="/docs" class="group inline-flex items-center gap-2 px-6 py-3 bg-logic-navy-800/50 border border-logic-navy-600 rounded-xl text-chalk hover:bg-logic-navy-700 hover:border-logic-navy-500 transition-all duration-300 hover:scale-105">
            Explore all topics
            <svg class="w-4 h-4 transition-transform duration-300 group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"/></svg>
          </NuxtLink>
        </div>
      </div>
    </section>

    <!-- Use Cases with Hover Effects -->
    <section class="py-24 px-4 relative z-10">
      <div class="absolute inset-0 bg-gradient-to-b from-logic-navy-900 via-logic-navy-800/30 to-logic-navy-900 pointer-events-none"></div>

      <div class="max-w-6xl mx-auto relative">
        <div
          class="text-center mb-16"
          v-scroll-animate="{ animation: 'fade-up' }"
        >
          <span class="inline-block px-4 py-1.5 bg-violet-500/10 border border-violet-500/20 rounded-full text-xs text-violet-400 font-medium mb-4">USE CASES</span>
          <h2 class="text-4xl md:text-5xl font-bold text-chalk mb-4">Built For</h2>
          <p class="text-chalk-400 text-lg">From research to production</p>
        </div>

        <div class="grid md:grid-cols-4 gap-8">
          <div
            v-for="(useCase, index) in useCases"
            :key="useCase.title"
            class="text-center group cursor-default"
            v-scroll-animate="{ animation: 'fade-up', delay: index * 100 }"
          >
            <div
              class="w-16 h-16 mx-auto mb-5 rounded-2xl flex items-center justify-center transition-all duration-500 group-hover:scale-110 group-hover:rotate-6 group-hover:shadow-xl"
              :class="`bg-${useCase.color}/10 group-hover:bg-${useCase.color}/20 group-hover:shadow-${useCase.color}/20`"
            >
              <svg :class="`w-8 h-8 text-${useCase.color}`" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" :d="useCase.iconPath"/>
              </svg>
            </div>
            <h3 class="font-semibold text-chalk mb-2 group-hover:text-white transition-colors">{{ useCase.title }}</h3>
            <p class="text-sm text-chalk-500 group-hover:text-chalk-400 transition-colors">{{ useCase.description }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Quick Install with Copy Animation -->
    <section class="py-24 px-4 relative z-10">
      <div class="max-w-4xl mx-auto">
        <div
          class="text-center mb-14"
          v-scroll-animate="{ animation: 'fade-up' }"
        >
          <span class="inline-block px-4 py-1.5 bg-amber-500/10 border border-amber-500/20 rounded-full text-xs text-amber-400 font-medium mb-4">INSTALLATION</span>
          <h2 class="text-4xl md:text-5xl font-bold text-chalk mb-4">Get Started in Seconds</h2>
          <p class="text-chalk-400 text-lg">Choose your language and start computing</p>
        </div>

        <div class="grid md:grid-cols-3 gap-6">
          <div
            v-for="(install, index) in installCommands"
            :key="install.lang"
            class="group bg-logic-navy-800/50 border border-logic-navy-700/50 rounded-2xl p-6 transition-all duration-500 hover:-translate-y-2 hover:shadow-xl backdrop-blur-sm"
            :class="`hover:border-${install.color}/50 hover:shadow-${install.color}/10`"
            v-scroll-animate="{ animation: 'fade-up', delay: index * 100 }"
          >
            <div class="flex items-center gap-3 mb-5">
              <span class="text-3xl transition-transform duration-300 group-hover:scale-110">{{ install.emoji }}</span>
              <span :class="`text-${install.color} font-semibold text-lg`">{{ install.lang }}</span>
            </div>
            <div
              class="relative bg-logic-navy-900/80 rounded-xl p-4 font-mono text-sm cursor-pointer group/code"
              @click="copyCommand(install.command)"
            >
              <code class="text-chalk-300">{{ install.command }}</code>
              <div class="absolute right-3 top-1/2 -translate-y-1/2 opacity-0 group-hover/code:opacity-100 transition-opacity">
                <svg v-if="!copiedCommand || copiedCommand !== install.command" class="w-5 h-5 text-chalk-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                </svg>
                <svg v-else class="w-5 h-5 text-step-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                </svg>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- CTA Section with Gradient Animation -->
    <section class="py-32 px-4 relative z-10 overflow-hidden">
      <!-- Animated background -->
      <div class="absolute inset-0 pointer-events-none">
        <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] bg-gradient-conic from-rust-core/20 via-solve-cyan/20 to-step-green/20 rounded-full blur-[100px] animate-spin-slow"></div>
      </div>

      <div
        class="max-w-3xl mx-auto text-center relative"
        v-scroll-animate="{ animation: 'fade-up' }"
      >
        <h2 class="text-4xl md:text-6xl font-bold text-chalk mb-6">Ready to Solve?</h2>
        <p class="text-chalk-400 mb-10 text-xl">Join developers using MathHook for high-performance symbolic math</p>
        <div class="flex flex-col sm:flex-row items-center justify-center gap-4">
          <NuxtLink to="/docs" class="group relative px-10 py-4 bg-gradient-to-r from-rust-core via-solve-cyan to-step-green text-white font-semibold rounded-xl transition-all duration-300 hover:scale-105 hover:shadow-xl hover:shadow-solve-cyan/25 overflow-hidden">
            <span class="relative z-10">Read the Docs</span>
            <div class="absolute inset-0 bg-gradient-to-r from-step-green via-solve-cyan to-rust-core opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
          </NuxtLink>
          <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="group px-10 py-4 bg-logic-navy-800/80 border border-logic-navy-600 text-chalk font-semibold rounded-xl hover:bg-logic-navy-700 hover:border-logic-navy-500 transition-all duration-300 hover:scale-105 backdrop-blur-sm">
            View on GitHub
          </a>
        </div>
      </div>
    </section>

    <!-- Footer -->
    <footer class="py-16 px-4 border-t border-logic-navy-800/50 relative z-10">
      <div class="max-w-6xl mx-auto">
        <div class="flex flex-col md:flex-row items-center justify-between gap-8">
          <div class="flex items-center gap-3">
            <svg class="w-8 h-8" viewBox="0 0 48 48" fill="none">
              <defs>
                <linearGradient id="footer-logo" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" stop-color="#E64524"/>
                  <stop offset="100%" stop-color="#06B6D4"/>
                </linearGradient>
              </defs>
              <path d="M12 40 C12 40 12 12 12 10 C12 6 16 4 20 8 L24 16 L28 8 C32 4 36 6 36 10 C36 12 36 40 36 40" stroke="url(#footer-logo)" stroke-width="4" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
            </svg>
            <div>
              <span class="text-chalk-400 text-sm">
                <span class="text-rust-core font-semibold">Math</span><span class="text-solve-cyan font-semibold">Hook</span>
              </span>
              <p class="text-chalk-600 text-xs mt-0.5">Symbolic Power. Educational Clarity. Native Speed.</p>
            </div>
          </div>
          <div class="flex items-center gap-8 text-sm">
            <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors hover:-translate-y-0.5 duration-300">GitHub</a>
            <a href="https://crates.io/crates/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors hover:-translate-y-0.5 duration-300">Crates.io</a>
            <a href="https://pypi.org/project/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors hover:-translate-y-0.5 duration-300">PyPI</a>
            <a href="https://www.npmjs.com/package/mathhook-node" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors hover:-translate-y-0.5 duration-300">npm</a>
          </div>
        </div>
        <div class="mt-10 pt-8 border-t border-logic-navy-800/50 text-center text-xs text-chalk-600">
          Built with Rust. Open source under MIT OR Apache-2.0.
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup>
const activeLanguage = ref('rust')
const categoryTopics = ref([])
const totalTopicCount = ref(0)
const mounted = ref(false)
const scrollY = ref(0)
const parallax = ref({ x: 0, y: 0 })
const copiedCommand = ref(null)

// Tagline words for staggered reveal animation
const taglineWords = [
  { text: 'Symbolic', color: 'text-rust-core' },
  { text: 'Power.', color: 'text-white' },
  { text: 'Educational', color: 'text-solve-cyan' },
  { text: 'Clarity.', color: 'text-white' },
  { text: 'Native', color: 'text-step-green' },
  { text: 'Speed.', color: 'text-white' }
]

// Features data
const features = [
  { title: 'Symbolic Mathematics', description: 'Expressions, algebra, calculus, and matrix operations', icon: '∫', color: 'rust-core' },
  { title: 'Multiple Input Formats', description: 'Parse LaTeX, Wolfram Language, and standard notation', icon: '📝', color: 'solve-cyan' },
  { title: 'Equation Solving', description: 'Linear, quadratic, polynomial, and systems of equations', icon: '⚖️', color: 'amber-400' },
  { title: 'Educational Focus', description: 'Step-by-step explanations for all operations', icon: '🎓', color: 'step-green' },
  { title: 'High Performance', description: 'Rust-based core with SIMD optimizations', icon: '⚡', color: 'rust-core' },
  { title: 'Language Bindings', description: 'Native support for Python and Node.js', icon: '🔗', color: 'violet-400' },
  { title: 'Memory Efficient', description: '32-byte expressions for optimal cache performance', icon: '💾', color: 'solve-cyan' },
  { title: 'Production Ready', description: 'Zero-copy parsing, arena allocation, thread-safe', icon: '🚀', color: 'step-green' }
]

// Benchmarks data
const benchmarks = [
  { name: 'Elementary integration (cos, exp)', time: '< 300 ns', color: 'step-green' },
  { name: 'Simple derivatives', time: '< 2 μs', color: 'solve-cyan' },
  { name: 'Polynomial simplification (deg 50)', time: '< 10 μs', color: 'amber-400' },
  { name: 'Complex calculus (chain + product)', time: '< 500 μs', color: 'rust-core' }
]

// Performance reasons
const performanceReasons = [
  { title: '32-byte expressions', description: 'Two fit per CPU cache line', icon: '32B', color: 'rust-core' },
  { title: 'Zero-copy parsing', description: 'Direct AST construction without allocations', icon: '0', color: 'solve-cyan' },
  { title: 'SIMD operations', description: 'Vectorized arithmetic for bulk operations', icon: '⚡', color: 'step-green' },
  { title: 'No interpreter overhead', description: 'Native Rust, no garbage collector', icon: '🦀', color: 'amber-400' },
  { title: 'Thread-safe', description: 'Immutable expressions, lock-free operations', icon: '🔒', color: 'violet-400' }
]

// Use cases
const useCases = [
  { title: 'Machine Learning', description: 'Symbolic gradients for neural networks and optimization', color: 'rust-core', iconPath: 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z' },
  { title: 'Education', description: 'Step-by-step solutions for teaching and learning', color: 'step-green', iconPath: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253' },
  { title: 'Research', description: 'High-performance symbolic computation for science', color: 'solve-cyan', iconPath: 'M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z' },
  { title: 'Development', description: 'Embed math in your applications with native speed', color: 'violet-400', iconPath: 'M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4' }
]

// Install commands
const installCommands = [
  { lang: 'Rust', emoji: '🦀', command: 'cargo add mathhook', color: 'rust-core' },
  { lang: 'Python', emoji: '🐍', command: 'pip install mathhook', color: 'amber-400' },
  { lang: 'Node.js', emoji: '📜', command: 'npm install mathhook-node', color: 'step-green' }
]

// Ecosystem badges
const ecosystemBadges = [
  { name: 'Crates.io', url: 'https://crates.io/crates/mathhook', color: 'rust-core', emoji: '🦀' },
  { name: 'PyPI', url: 'https://pypi.org/project/mathhook', color: 'amber-400', emoji: '🐍' },
  { name: 'npm', url: 'https://www.npmjs.com/package/mathhook-node', color: 'step-green', emoji: '📦' },
  { name: 'GitHub', url: 'https://github.com/AhmedMashour/mathhook', color: 'chalk', emoji: '⭐' },
  { name: 'Docs', url: '/docs', color: 'solve-cyan', emoji: '📚' }
]

// Category configuration
const categoryConfig = {
  'calculus': { title: 'Calculus', icon: '∫', color: 'rust-core', description: 'Derivatives, Integrals, Limits, Series', order: 1, prefixes: ['operations-differentiation', 'operations-integration', 'operations-limits', 'operations-series'] },
  'ode': { title: 'ODEs', icon: 'dy/dx', color: 'solve-cyan', description: 'Separable, Linear, Bernoulli, Exact', order: 2, prefixes: ['ode-'] },
  'pde': { title: 'PDEs', icon: '∂²u', color: 'step-green', description: 'Heat, Wave, Laplace, Poisson', order: 3, prefixes: ['pde-', 'advanced-pde-'] },
  'algebra': { title: 'Algebra', icon: 'Σ', color: 'amber-400', description: 'Simplify, Factor, Expand, Solve', order: 4, prefixes: ['operations-simplification', 'operations-expansion', 'operations-solving', 'operations-substitution'] },
  'polynomial': { title: 'Polynomials', icon: 'xⁿ', color: 'violet-400', description: 'GCD, Division, Gröbner Bases', order: 5, prefixes: ['polynomial-'] },
  'advanced': { title: 'Advanced Topics', icon: '🔬', color: 'pink-400', description: 'Complex Numbers, Matrices, Special Functions', order: 6, prefixes: ['advanced-'] },
  'educational': { title: 'Educational', icon: '🎓', color: 'cyan-400', description: 'Step-by-Step, Messages, API', order: 7, prefixes: ['educational-'] },
  'getting-started': { title: 'Getting Started', icon: '🚀', color: 'orange-400', description: 'Installation, Quick Start, Patterns', order: 0, prefixes: ['getting-started-'] }
}

// Copy command to clipboard
const copyCommand = async (command) => {
  await navigator.clipboard.writeText(command)
  copiedCommand.value = command
  setTimeout(() => {
    copiedCommand.value = null
  }, 2000)
}

// Scroll handler for parallax and nav
const handleScroll = () => {
  scrollY.value = window.scrollY
}

// Mouse move for parallax
const handleMouseMove = (e) => {
  parallax.value = {
    x: (e.clientX - window.innerWidth / 2) / 50,
    y: (e.clientY - window.innerHeight / 2) / 50
  }
}

// Scroll animation directive
const vScrollAnimate = {
  mounted(el, binding) {
    const options = binding.value || {}
    const animation = options.animation || 'fade-up'
    const delay = options.delay || 0

    el.style.opacity = '0'
    el.style.transform = animation === 'fade-up' ? 'translateY(30px)' :
                         animation === 'fade-left' ? 'translateX(30px)' :
                         animation === 'fade-right' ? 'translateX(-30px)' : 'translateY(30px)'
    el.style.transition = `all 0.8s cubic-bezier(0.16, 1, 0.3, 1) ${delay}ms`

    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          setTimeout(() => {
            el.style.opacity = '1'
            el.style.transform = 'translateY(0) translateX(0)'
          }, 50)
          observer.unobserve(el)
        }
      })
    }, { threshold: 0.1, rootMargin: '50px' })

    observer.observe(el)
  }
}

// Load topics from schemas
onMounted(async () => {
  mounted.value = true

  window.addEventListener('scroll', handleScroll, { passive: true })
  window.addEventListener('mousemove', handleMouseMove, { passive: true })

  try {
    const response = await fetch('/data/_topics.json')
    if (!response.ok) return

    const topics = await response.json()
    totalTopicCount.value = topics.length

    const grouped = {}

    for (const topic of topics) {
      for (const [catId, catConfig] of Object.entries(categoryConfig)) {
        const matches = catConfig.prefixes.some(prefix => topic.startsWith(prefix))
        if (matches) {
          if (!grouped[catId]) {
            grouped[catId] = { id: catId, ...catConfig, topics: [], firstTopic: null, count: 0 }
          }
          grouped[catId].topics.push(topic)
          grouped[catId].count++
          if (!grouped[catId].firstTopic) {
            grouped[catId].firstTopic = topic
          }
          break
        }
      }
    }

    categoryTopics.value = Object.values(grouped).sort((a, b) => a.order - b.order).slice(0, 8)
  } catch (e) {
    console.error('Failed to load topics:', e)
  }
})

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
  window.removeEventListener('mousemove', handleMouseMove)
})

useHead({
  title: 'MathHook - High-Performance Computer Algebra System',
  meta: [
    { name: 'description', content: 'MathHook: High-performance CAS in Rust with Python and Node.js bindings. Symbolic Power. Educational Clarity. Native Speed.' }
  ]
})
</script>

<style scoped>
/* Floating animation */
@keyframes float {
  0%, 100% { transform: translateY(0px); }
  50% { transform: translateY(-10px); }
}

.animate-float {
  animation: float 4s ease-in-out infinite;
}

/* Gradient animation */
@keyframes gradient-x {
  0%, 100% { background-position: 0% 50%; }
  50% { background-position: 100% 50%; }
}

.animate-gradient-x {
  background-size: 200% 200%;
  animation: gradient-x 3s ease infinite;
}

/* Slow spin for CTA background */
@keyframes spin-slow {
  from { transform: translate(-50%, -50%) rotate(0deg); }
  to { transform: translate(-50%, -50%) rotate(360deg); }
}

.animate-spin-slow {
  animation: spin-slow 30s linear infinite;
}

/* Code transition */
.code-fade-enter-active,
.code-fade-leave-active {
  transition: all 0.3s ease;
}

.code-fade-enter-from {
  opacity: 0;
  transform: translateX(10px);
}

.code-fade-leave-to {
  opacity: 0;
  transform: translateX(-10px);
}

/* Conic gradient */
.bg-gradient-conic {
  background: conic-gradient(from 0deg, var(--tw-gradient-stops));
}

/* Blob animations - organic flowing movement */
@keyframes blob-1 {
  0%, 100% {
    transform: translate(0, 0) scale(1);
  }
  25% {
    transform: translate(80px, -60px) scale(1.1);
  }
  50% {
    transform: translate(40px, 80px) scale(0.95);
  }
  75% {
    transform: translate(-60px, 40px) scale(1.05);
  }
}

@keyframes blob-2 {
  0%, 100% {
    transform: translate(0, 0) scale(1);
  }
  25% {
    transform: translate(-70px, 50px) scale(1.05);
  }
  50% {
    transform: translate(60px, -40px) scale(1.1);
  }
  75% {
    transform: translate(30px, 70px) scale(0.95);
  }
}

@keyframes blob-3 {
  0%, 100% {
    transform: translate(0, 0) scale(1) rotate(0deg);
  }
  33% {
    transform: translate(50px, 60px) scale(1.08) rotate(10deg);
  }
  66% {
    transform: translate(-40px, -50px) scale(0.92) rotate(-10deg);
  }
}

@keyframes blob-4 {
  0%, 100% {
    transform: translate(0, 0) scale(1);
  }
  20% {
    transform: translate(-50px, -30px) scale(1.12);
  }
  40% {
    transform: translate(40px, 50px) scale(0.9);
  }
  60% {
    transform: translate(70px, -20px) scale(1.05);
  }
  80% {
    transform: translate(-30px, 60px) scale(0.95);
  }
}

@keyframes blob-5 {
  0%, 100% {
    transform: translate(0, 0) scale(1) rotate(0deg);
  }
  25% {
    transform: translate(60px, 40px) scale(1.1) rotate(5deg);
  }
  50% {
    transform: translate(-50px, 70px) scale(0.95) rotate(-5deg);
  }
  75% {
    transform: translate(40px, -60px) scale(1.05) rotate(8deg);
  }
}

.animate-blob-1 {
  animation: blob-1 20s ease-in-out infinite;
}

.animate-blob-2 {
  animation: blob-2 25s ease-in-out infinite;
}

.animate-blob-3 {
  animation: blob-3 18s ease-in-out infinite;
}

.animate-blob-4 {
  animation: blob-4 22s ease-in-out infinite;
}

.animate-blob-5 {
  animation: blob-5 24s ease-in-out infinite;
}

/* Floating particles animation */
@keyframes float-particle {
  0%, 100% {
    transform: translateY(0) translateX(0);
    opacity: 0;
  }
  10% {
    opacity: 1;
  }
  90% {
    opacity: 1;
  }
  100% {
    transform: translateY(-100vh) translateX(50px);
    opacity: 0;
  }
}

.animate-float-particle {
  animation: float-particle 15s ease-in-out infinite;
}

/* Pulsing glow effect for extra vibrancy */
@keyframes pulse-glow {
  0%, 100% {
    opacity: 0.4;
    filter: blur(60px);
  }
  50% {
    opacity: 0.6;
    filter: blur(80px);
  }
}

/* Blinking cursor animation */
@keyframes blink {
  0%, 50% {
    opacity: 1;
  }
  51%, 100% {
    opacity: 0;
  }
}

.animate-blink {
  animation: blink 1s step-end infinite;
}
</style>
