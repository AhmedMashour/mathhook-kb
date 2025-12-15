<template>
  <header class="relative pt-32 pb-20 px-4 min-h-screen flex items-center">
    <div class="max-w-5xl mx-auto text-center relative z-10 w-full">
      <!-- Announcement Badge -->
      <div
        class="mb-8"
        :class="mounted ? 'animate-fade-up' : ''"
        :style="{ '--delay': '0.1s' }"
      >
        <span class="inline-flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-rust-core/20 to-solve-cyan/20 border border-logic-navy-600/50 rounded-full text-xs text-chalk-300 backdrop-blur-sm shadow-lg shadow-rust-core/5">
          <span class="relative flex h-2 w-2">
            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-step-green opacity-75"></span>
            <span class="relative inline-flex rounded-full h-2 w-2 bg-step-green"></span>
          </span>
          Now with ODE &amp; PDE support
        </span>
      </div>

      <!-- Logo + Title -->
      <div class="mb-10">
        <div
          class="flex items-center justify-center gap-5 mb-6"
          :class="mounted ? 'animate-fade-up' : ''"
          :style="{ '--delay': '0.2s' }"
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

        <!-- Animated Tagline -->
        <div class="mb-4">
          <p class="text-2xl md:text-4xl font-semibold tracking-tight leading-tight">
            <span
              v-for="(word, index) in taglineWords"
              :key="index"
              class="inline-block overflow-hidden mx-0.5"
            >
              <span
                class="inline-block"
                :class="[word.color, mounted ? 'animate-word-up' : '']"
                :style="{ '--delay': `${0.4 + index * 0.1}s` }"
              >{{ word.text }}</span>
            </span>
          </p>
        </div>

        <p
          class="text-lg md:text-xl text-chalk-400 max-w-2xl mx-auto leading-relaxed"
          :class="mounted ? 'animate-fade-up' : ''"
          :style="{ '--delay': '1s' }"
        >
          A Rust computer algebra system with native Python and Node.js bindings. No GC, no compromises.
        </p>
      </div>

      <!-- Stats Row -->
      <div
        class="flex items-center justify-center gap-8 md:gap-12 mb-12"
        :class="mounted ? 'animate-fade-up' : ''"
        :style="{ '--delay': '1.2s' }"
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

      <!-- CTA Buttons -->
      <div
        class="flex flex-col sm:flex-row items-center justify-center gap-4 mb-16"
        :class="mounted ? 'animate-fade-up' : ''"
        :style="{ '--delay': '1.4s' }"
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

      <!-- Interactive Code Demo -->
      <div
        class="max-w-3xl mx-auto"
        :class="mounted ? 'animate-fade-up' : ''"
        :style="{ '--delay': '1.6s' }"
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

            <!-- Code Content -->
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
        class="absolute bottom-8 left-1/2 -translate-x-1/2 animate-bounce transition-opacity duration-500"
        :class="scrollY < 100 ? 'opacity-100' : 'opacity-0'"
      >
        <svg class="w-6 h-6 text-chalk-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"/>
        </svg>
      </div>
    </div>
  </header>
</template>

<script setup>
import { useHomeData } from '~/composables/useHomeData'

const { taglineWords } = useHomeData()

const activeLanguage = ref('rust')
const scrollY = ref(0)
const mounted = ref(false)

const handleScroll = () => {
  scrollY.value = window.scrollY
}

onMounted(() => {
  // Trigger animations after hydration
  requestAnimationFrame(() => {
    mounted.value = true
  })
  window.addEventListener('scroll', handleScroll, { passive: true })
})

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
})
</script>

<style scoped>
/* Fade up animation - triggered by adding class */
.animate-fade-up {
  animation: fadeUp 0.8s cubic-bezier(0.16, 1, 0.3, 1) both;
  animation-delay: var(--delay, 0s);
}

@keyframes fadeUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Word slide up animation */
.animate-word-up {
  animation: wordUp 0.6s cubic-bezier(0.16, 1, 0.3, 1) both;
  animation-delay: var(--delay, 0s);
}

@keyframes wordUp {
  from {
    opacity: 0;
    transform: translateY(100%);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

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
</style>
