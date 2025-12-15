<template>
  <div class="min-h-screen bg-logic-navy-900 text-chalk overflow-hidden relative">
    <!-- Subtle Animated Background (optimized for reading) -->
    <div class="fixed inset-0 overflow-hidden pointer-events-none">
      <!-- Base dark layer -->
      <div class="absolute inset-0 bg-logic-navy-900"></div>

      <!-- Subtle animated mesh gradient blobs -->
      <div class="absolute inset-0">
        <!-- Primary rust blob - top right (subtle) -->
        <div
          class="absolute w-[500px] h-[500px] rounded-full animate-blob-slow-1"
          :style="{
            background: 'radial-gradient(circle at center, rgba(230, 69, 36, 0.2) 0%, rgba(230, 69, 36, 0.08) 40%, transparent 70%)',
            filter: 'blur(80px)',
            top: '-15%',
            right: '-10%',
            transform: `translate(${-mouseX * 0.015}px, ${mouseY * 0.015}px)`
          }"
        ></div>

        <!-- Secondary cyan blob - bottom left (subtle) -->
        <div
          class="absolute w-[450px] h-[450px] rounded-full animate-blob-slow-2"
          :style="{
            background: 'radial-gradient(circle at center, rgba(6, 182, 212, 0.22) 0%, rgba(6, 182, 212, 0.08) 40%, transparent 70%)',
            filter: 'blur(80px)',
            bottom: '-10%',
            left: '-8%',
            transform: `translate(${mouseX * 0.012}px, ${-mouseY * 0.012}px)`
          }"
        ></div>

        <!-- Tertiary violet blob - center right (very subtle) -->
        <div
          class="absolute w-[350px] h-[350px] rounded-full animate-blob-slow-3"
          :style="{
            background: 'radial-gradient(circle at center, rgba(139, 92, 246, 0.15) 0%, rgba(139, 92, 246, 0.05) 40%, transparent 70%)',
            filter: 'blur(100px)',
            top: '40%',
            right: '5%',
            transform: `translate(${-mouseX * 0.01}px, ${mouseY * 0.01}px)`
          }"
        ></div>
      </div>

      <!-- Very subtle floating particles -->
      <div class="absolute inset-0">
        <div v-for="i in 10" :key="'particle-'+i"
             class="absolute rounded-full animate-float-particle-subtle"
             :style="{
               width: `${1 + Math.random() * 2}px`,
               height: `${1 + Math.random() * 2}px`,
               background: ['rgba(230, 69, 36, 0.3)', 'rgba(6, 182, 212, 0.3)', 'rgba(139, 92, 246, 0.25)'][Math.floor(Math.random() * 3)],
               left: `${Math.random() * 100}%`,
               top: `${Math.random() * 100}%`,
               animationDelay: `${Math.random() * 10}s`,
               animationDuration: `${15 + Math.random() * 20}s`
             }"
        ></div>
      </div>

      <!-- Readability overlay -->
      <div class="absolute inset-0 bg-gradient-to-b from-logic-navy-900/50 via-logic-navy-900/30 to-logic-navy-900/60"></div>
    </div>

    <!-- Navigation -->
    <nav
      class="sticky top-0 z-50 transition-all duration-500"
      :class="scrolled ? 'bg-logic-navy-900/95 backdrop-blur-xl border-b border-logic-navy-700/50 shadow-lg shadow-black/10' : 'bg-logic-navy-900/90 backdrop-blur-md border-b border-logic-navy-700/50'"
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
            <NuxtLink to="/docs" class="text-solve-cyan font-medium relative after:absolute after:bottom-0 after:left-0 after:w-full after:h-0.5 after:bg-solve-cyan after:rounded-full">Docs</NuxtLink>
            <NuxtLink to="/outputs" class="text-chalk-500 hover:text-chalk transition-colors duration-300">Outputs</NuxtLink>
            <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-all duration-300 flex items-center gap-1.5">
              <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
              GitHub
            </a>
          </div>
        </div>
      </div>
    </nav>

    <!-- Loading State -->
    <div v-if="loading" class="max-w-4xl mx-auto px-4 py-12 relative z-10">
      <div class="animate-pulse space-y-4">
        <div class="h-10 bg-logic-navy-700/50 rounded-xl w-3/4"></div>
        <div class="h-4 bg-logic-navy-700/50 rounded w-full"></div>
        <div class="h-4 bg-logic-navy-700/50 rounded w-5/6"></div>
      </div>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="max-w-4xl mx-auto px-4 py-12 relative z-10">
      <div class="bg-rust-core/10 border-l-4 border-rust-core p-6 rounded-r-xl animate-fade-in">
        <div class="flex items-start">
          <span class="text-3xl mr-3">❌</span>
          <div>
            <h2 class="text-xl font-semibold text-rust-core mb-2">Document Not Found</h2>
            <p class="text-chalk-400 mb-4">{{ error }}</p>
            <NuxtLink to="/docs" class="text-solve-cyan hover:text-solve-cyan-300 font-medium transition-colors">
              ← Back to documentation
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>

    <!-- Content -->
    <div v-else-if="schema" class="max-w-7xl mx-auto px-4 py-8 relative z-10">
      <div class="flex gap-8">
        <!-- Table of Contents Sidebar -->
        <aside class="hidden lg:block w-64 sticky top-20 self-start animate-fade-in-left">
          <div class="bg-logic-navy-800/40 backdrop-blur-sm border border-logic-navy-700/50 rounded-2xl p-5 transition-all duration-300 hover:border-logic-navy-600/50">
            <h3 class="text-xs font-medium text-chalk-500 mb-4 uppercase tracking-wide">On This Page</h3>
            <nav class="space-y-1.5">
              <a href="#definition"
                 class="block text-sm text-chalk-500 hover:text-solve-cyan transition-all duration-300 py-1.5 px-2 rounded-lg hover:bg-logic-navy-700/30">
                Definition
              </a>
              <a href="#introduction"
                 class="block text-sm text-chalk-500 hover:text-solve-cyan transition-all duration-300 py-1.5 px-2 rounded-lg hover:bg-logic-navy-700/30">
                Introduction
              </a>
              <a href="#examples"
                 class="block text-sm text-chalk-500 hover:text-solve-cyan transition-all duration-300 py-1.5 px-2 rounded-lg hover:bg-logic-navy-700/30">
                Code Examples
              </a>
              <a v-for="(section, idx) in schema.article?.sections" :key="idx"
                 :href="`#section-${idx}`"
                 class="block text-sm text-chalk-500 hover:text-solve-cyan transition-all duration-300 py-1.5 px-2 rounded-lg hover:bg-logic-navy-700/30">
                {{ section.title }}
              </a>
            </nav>
          </div>

          <!-- Quick Reference Card -->
          <div class="mt-4 bg-gradient-to-br from-rust-core/10 to-solve-cyan/10 border border-logic-navy-700/50 rounded-2xl p-5 animate-fade-in-left" style="animation-delay: 0.1s;">
            <h3 class="text-xs font-medium text-chalk-500 mb-3 uppercase tracking-wide flex items-center gap-2">
              <span class="text-solve-cyan">⚡</span> Quick Reference
            </h3>
            <div class="space-y-3 text-sm">
              <div>
                <div class="font-medium text-chalk-400 mb-1">Category</div>
                <div class="text-chalk-500">{{ schema.topic }}</div>
              </div>
              <div v-if="schema.article?.complexity">
                <div class="font-medium text-chalk-400 mb-1">Complexity</div>
                <div class="flex items-center gap-1">
                  <span v-for="i in 5" :key="i"
                        :class="['transition-all duration-300', i <= (schema.article.complexity === 'beginner' ? 1 : schema.article.complexity === 'intermediate' ? 3 : 5) ? 'text-solve-cyan' : 'text-logic-navy-600']">
                    ●
                  </span>
                </div>
              </div>
            </div>
          </div>
        </aside>

        <!-- Main Content -->
        <article class="flex-1 max-w-4xl">
          <!-- Breadcrumb -->
          <nav class="mb-6 flex items-center gap-2 text-sm text-chalk-500 animate-fade-in">
            <NuxtLink to="/" class="hover:text-solve-cyan transition-colors duration-300">Home</NuxtLink>
            <span class="text-chalk-600">/</span>
            <NuxtLink to="/docs" class="hover:text-solve-cyan transition-colors duration-300">Docs</NuxtLink>
            <span class="text-chalk-600">/</span>
            <span class="text-chalk">{{ schema.title }}</span>
          </nav>

          <!-- Title Section -->
          <header class="mb-10 animate-fade-in" style="animation-delay: 0.1s;">
            <h1 class="text-3xl md:text-4xl font-semibold text-chalk mb-3 leading-tight">
              {{ schema.title }}
            </h1>
            <p class="text-base text-chalk-400 leading-relaxed">
              {{ schema.description }}
            </p>
          </header>

          <!-- Mathematical Definition Callout -->
          <div id="definition" v-if="schema.mathematical_definition" class="mb-10 scroll-mt-24 animate-fade-in" style="animation-delay: 0.15s;">
            <div class="bg-logic-navy-800/50 backdrop-blur-sm border border-logic-navy-600/50 rounded-2xl p-6 transition-all duration-300 hover:border-logic-navy-500/50">
              <div class="flex items-center gap-3 mb-4">
                <div class="w-9 h-9 bg-logic-navy-700/80 rounded-xl flex items-center justify-center text-solve-cyan text-lg">📐</div>
                <h2 class="text-lg font-semibold text-chalk">Mathematical Definition</h2>
              </div>
              <div class="math-content text-chalk-300" v-html="renderContent(schema.mathematical_definition)"></div>
            </div>
          </div>

          <!-- Introduction -->
          <section id="introduction" v-if="schema.article?.introduction" class="mb-10 scroll-mt-24 animate-fade-in" style="animation-delay: 0.2s;">
            <h2 class="text-xl font-semibold text-chalk mb-4 flex items-center gap-3">
              <span class="w-8 h-1 bg-gradient-to-r from-rust-core to-solve-cyan rounded-full"></span>
              Introduction
            </h2>
            <div class="text-chalk-300 text-base leading-relaxed rendered-content" v-html="renderContent(schema.article.introduction.hook)"></div>
          </section>

          <!-- Code Examples -->
          <section id="examples" class="mb-10 scroll-mt-24 animate-fade-in" style="animation-delay: 0.25s;">
            <h2 class="text-xl font-semibold text-chalk mb-6 flex items-center gap-3">
              <span class="w-8 h-1 bg-gradient-to-r from-rust-core to-solve-cyan rounded-full"></span>
              Code Examples
            </h2>
            <div class="space-y-6">
              <div v-for="(example, idx) in schema.examples" :key="idx"
                   class="bg-logic-navy-800/40 backdrop-blur-sm border border-logic-navy-700/50 rounded-2xl overflow-hidden transition-all duration-300 hover:border-logic-navy-600/50 hover:shadow-lg hover:shadow-black/5">
                <!-- Example Header -->
                <div class="p-5 border-b border-logic-navy-700/50">
                  <h3 class="text-lg font-medium text-chalk mb-2">{{ example.title }}</h3>
                  <div class="text-chalk-400 text-sm leading-relaxed rendered-content whitespace-pre-wrap" v-html="renderContent(example.explanation)"></div>
                </div>

                <!-- Language Tabs with Animation -->
                <div class="bg-logic-navy-900/80 border-b border-logic-navy-700/50">
                  <div class="flex">
                    <button @click="activeTab[idx] = 'python'"
                            :class="['flex-1 px-4 py-2.5 font-medium text-sm transition-all duration-300 relative', activeTab[idx] === 'python' ? 'text-solve-cyan bg-logic-navy-800' : 'text-chalk-500 hover:text-chalk hover:bg-logic-navy-800/50']">
                      <span class="flex items-center justify-center gap-2">
                        <span>🐍</span><span>Python</span>
                      </span>
                      <div :class="['absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-rust-core to-solve-cyan transition-all duration-300', activeTab[idx] === 'python' ? 'opacity-100 scale-x-100' : 'opacity-0 scale-x-0']"></div>
                    </button>
                    <button @click="activeTab[idx] = 'rust'"
                            :class="['flex-1 px-4 py-2.5 font-medium text-sm transition-all duration-300 relative', activeTab[idx] === 'rust' ? 'text-rust-core bg-logic-navy-800' : 'text-chalk-500 hover:text-chalk hover:bg-logic-navy-800/50']">
                      <span class="flex items-center justify-center gap-2">
                        <span>🦀</span><span>Rust</span>
                      </span>
                      <div :class="['absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-rust-core to-solve-cyan transition-all duration-300', activeTab[idx] === 'rust' ? 'opacity-100 scale-x-100' : 'opacity-0 scale-x-0']"></div>
                    </button>
                    <button @click="activeTab[idx] = 'nodejs'"
                            :class="['flex-1 px-4 py-2.5 font-medium text-sm transition-all duration-300 relative', activeTab[idx] === 'nodejs' ? 'text-step-green bg-logic-navy-800' : 'text-chalk-500 hover:text-chalk hover:bg-logic-navy-800/50']">
                      <span class="flex items-center justify-center gap-2">
                        <span>📜</span><span>JavaScript</span>
                      </span>
                      <div :class="['absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-rust-core to-solve-cyan transition-all duration-300', activeTab[idx] === 'nodejs' ? 'opacity-100 scale-x-100' : 'opacity-0 scale-x-0']"></div>
                    </button>
                  </div>
                </div>

                <!-- Code Content with Copy Button -->
                <div class="relative group/code">
                  <button @click="copyCode(example.code[activeTab[idx]], idx)"
                          class="absolute top-3 right-3 px-3 py-1.5 bg-logic-navy-700/80 hover:bg-logic-navy-600 text-chalk-400 text-xs font-medium rounded-lg opacity-0 group-hover/code:opacity-100 transition-all duration-300 z-10 flex items-center gap-1.5 backdrop-blur-sm">
                    <span v-if="!copied[idx]">📋 Copy</span>
                    <span v-else class="text-step-green">✓ Copied!</span>
                  </button>
                  <Transition name="fade-code" mode="out-in">
                    <pre v-if="activeTab[idx] === 'python'" :key="'python-'+idx" class="language-python !m-0 !p-5 overflow-x-auto !bg-logic-navy-900"><code v-html="highlightCode(example.code.python, 'python')"></code></pre>
                    <pre v-else-if="activeTab[idx] === 'rust'" :key="'rust-'+idx" class="language-rust !m-0 !p-5 overflow-x-auto !bg-logic-navy-900"><code v-html="highlightCode(example.code.rust, 'rust')"></code></pre>
                    <pre v-else-if="activeTab[idx] === 'nodejs'" :key="'nodejs-'+idx" class="language-javascript !m-0 !p-5 overflow-x-auto !bg-logic-navy-900"><code v-html="highlightCode(example.code.nodejs, 'javascript')"></code></pre>
                  </Transition>
                </div>

                <!-- Output Section -->
                <div v-if="example.expected_output" class="border-t border-logic-navy-700/50">
                  <div class="bg-step-green/5 p-5">
                    <div class="flex items-center gap-2 mb-3">
                      <div class="w-5 h-5 bg-step-green rounded flex items-center justify-center text-white text-xs">▶</div>
                      <strong class="text-chalk font-medium text-sm">Output</strong>
                    </div>
                    <pre class="text-chalk-200 font-mono text-sm bg-logic-navy-900/80 p-4 rounded-xl border border-step-green/20">{{ example.expected_output }}</pre>
                  </div>
                </div>
              </div>
            </div>
          </section>

          <!-- Article Sections with Callouts -->
          <section v-if="schema.article?.sections" class="mb-10 space-y-10">
            <div v-for="(section, idx) in schema.article.sections" :key="idx"
                 :id="`section-${idx}`"
                 class="scroll-mt-24 animate-fade-in"
                 :style="{ animationDelay: `${0.3 + idx * 0.05}s` }">
              <h2 class="text-xl font-semibold text-chalk mb-4 flex items-center gap-3">
                <span class="w-8 h-1 bg-gradient-to-r from-rust-core to-solve-cyan rounded-full"></span>
                {{ section.title }}
              </h2>
              <div class="prose prose-lg max-w-none rendered-content text-chalk-300" v-html="renderContentWithCallouts(section.content)"></div>
            </div>
          </section>

          <!-- Sidebars as Callouts -->
          <section v-if="schema.article?.sidebars" class="mb-10 space-y-4">
            <div v-for="(sidebar, idx) in schema.article.sidebars" :key="idx"
                 class="animate-fade-in"
                 :style="{ animationDelay: `${0.4 + idx * 0.05}s` }">
              <div :class="['border-l-4 rounded-r-2xl p-5 transition-all duration-300 hover:translate-x-1',
                           sidebar.type === 'note' ? 'bg-solve-cyan/10 border-solve-cyan' : '',
                           sidebar.type === 'warning' ? 'bg-amber-500/10 border-amber-500' : '',
                           sidebar.type === 'info' ? 'bg-violet-500/10 border-violet-500' : '',
                           sidebar.type === 'performance' ? 'bg-step-green/10 border-step-green' : '']">
                <div class="flex items-start gap-3">
                  <span class="text-xl">{{ sidebar.type === 'note' ? '📝' : sidebar.type === 'warning' ? '⚠️' : sidebar.type === 'info' ? 'ℹ️' : '⚡' }}</span>
                  <div class="flex-1">
                    <h3 class="text-base font-medium text-chalk mb-2">{{ sidebar.title }}</h3>
                    <div class="text-chalk-400 text-sm rendered-content" v-html="renderContentWithCallouts(sidebar.content)"></div>
                  </div>
                </div>
              </div>
            </div>
          </section>

          <!-- Related Topics -->
          <section class="mt-12 pt-8 border-t border-logic-navy-700/50 animate-fade-in" style="animation-delay: 0.5s;">
            <h2 class="text-lg font-semibold text-chalk mb-4 flex items-center gap-2">
              <span class="text-solve-cyan">🔗</span> Related Topics
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
              <NuxtLink to="/docs"
                        class="group p-4 bg-logic-navy-800/40 backdrop-blur-sm rounded-xl border border-logic-navy-700/50 hover:border-solve-cyan/50 transition-all duration-300 hover:shadow-lg hover:shadow-solve-cyan/5">
                <div class="font-semibold text-chalk group-hover:text-solve-cyan transition-colors duration-300">Browse All Topics</div>
                <div class="text-sm text-chalk-500 mt-1">Explore the full documentation library</div>
              </NuxtLink>
            </div>
          </section>
        </article>
      </div>
    </div>

    <!-- Back to top button -->
    <Transition name="fade">
      <button
        v-if="showBackToTop"
        @click="scrollToTop"
        class="fixed bottom-6 right-6 w-12 h-12 bg-logic-navy-800/90 backdrop-blur-sm border border-logic-navy-700/50 rounded-xl text-chalk-400 hover:text-solve-cyan hover:border-solve-cyan/50 transition-all duration-300 flex items-center justify-center shadow-lg z-50"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 10l7-7m0 0l7 7m-7-7v18"/>
        </svg>
      </button>
    </Transition>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import Prism from 'prismjs'
import 'prismjs/components/prism-python'
import 'prismjs/components/prism-rust'
import 'prismjs/components/prism-javascript'
import katex from 'katex'
import 'katex/dist/katex.min.css'

const route = useRoute()
const topic = route.params.topic

const schema = ref(null)
const loading = ref(true)
const error = ref(null)
const activeTab = ref({})
const copied = ref({})
const scrolled = ref(false)
const mouseX = ref(0)
const mouseY = ref(0)
const showBackToTop = ref(false)

// Scroll handler
const handleScroll = () => {
  scrolled.value = window.scrollY > 20
  showBackToTop.value = window.scrollY > 400
}

// Mouse move handler for parallax
const handleMouseMove = (e) => {
  mouseX.value = (e.clientX - window.innerWidth / 2) / 100
  mouseY.value = (e.clientY - window.innerHeight / 2) / 100
}

// Scroll to top
const scrollToTop = () => {
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

// Load schema data
onMounted(async () => {
  window.addEventListener('scroll', handleScroll)
  window.addEventListener('mousemove', handleMouseMove)

  try {
    const response = await fetch(`/data/${topic}.json`)
    if (!response.ok) {
      throw new Error(`Document "${topic}" not found`)
    }
    schema.value = await response.json()

    // Initialize active tabs (default to Python)
    if (schema.value.examples) {
      schema.value.examples.forEach((_, idx) => {
        activeTab.value[idx] = 'python'
        copied.value[idx] = false
      })
    }
  } catch (err) {
    error.value = err.message
  } finally {
    loading.value = false
  }
})

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
  window.removeEventListener('mousemove', handleMouseMove)
})

// Copy code to clipboard
const copyCode = async (code, idx) => {
  try {
    await navigator.clipboard.writeText(code)
    copied.value[idx] = true
    setTimeout(() => {
      copied.value[idx] = false
    }, 2000)
  } catch (err) {
    console.error('Failed to copy:', err)
  }
}

// Highlight code using Prism
const highlightCode = (code, language) => {
  if (typeof window === 'undefined') return code
  return Prism.highlight(code, Prism.languages[language], language)
}

// Render math using KaTeX
const renderMath = (latex, displayMode = true) => {
  try {
    const cleanLatex = latex.replace(/\\\\/g, '\\')
    return katex.renderToString(cleanLatex, {
      throwOnError: false,
      displayMode: displayMode
    })
  } catch (e) {
    console.error('KaTeX render error:', e)
    return latex
  }
}

// Render content with inline and display LaTeX support
const renderContent = (text) => {
  if (!text) return ''
  let rendered = text

  // Display math
  rendered = rendered.replace(/\$\$([^$]+)\$\$/g, (match, latex) => {
    try {
      const cleanLatex = latex.replace(/\\\\/g, '\\').trim()
      return '<div class="my-6 flex justify-center">' +
             katex.renderToString(cleanLatex, {
               throwOnError: false,
               displayMode: true
             }) + '</div>'
    } catch (e) {
      return match
    }
  })

  // Inline math
  rendered = rendered.replace(/\$([^$]+)\$/g, (match, latex) => {
    try {
      const cleanLatex = latex.replace(/\\\\/g, '\\')
      return katex.renderToString(cleanLatex, {
        throwOnError: false,
        displayMode: false
      })
    } catch (e) {
      return match
    }
  })

  // Headers
  rendered = rendered.replace(/###\s*([^#\n]+)\s*###/g, '<h3 class="text-lg font-bold text-chalk mt-6 mb-3">$1</h3>')

  // Bold
  rendered = rendered.replace(/\*\*([^*]+)\*\*/g, '<strong class="font-semibold text-chalk">$1</strong>')

  // Paragraphs
  rendered = rendered.replace(/\n\n/g, '</p><p class="mb-4">')
  rendered = '<p class="mb-4">' + rendered + '</p>'

  return rendered
}

// Render content with callouts and special formatting
const renderContentWithCallouts = (text) => {
  return renderContent(text)
}

// Set page metadata with enhanced SEO
useHead(() => {
  if (!schema.value) return { title: 'Loading...' }

  const url = `https://mathook.org/docs/${topic}`
  const imageUrl = `https://mathook.org/og-images/${topic}.png`

  return {
    title: `${schema.value.title} - MathHook`,
    meta: [
      { name: 'description', content: schema.value.description },
      { name: 'keywords', content: `mathhook, ${schema.value.topic}, mathematics, ${schema.value.title}, symbolic math, CAS` },
      { name: 'author', content: 'MathHook Team' },
      { property: 'og:type', content: 'article' },
      { property: 'og:url', content: url },
      { property: 'og:title', content: schema.value.title },
      { property: 'og:description', content: schema.value.description },
      { property: 'og:image', content: imageUrl },
      { name: 'twitter:card', content: 'summary_large_image' },
      { name: 'twitter:url', content: url },
      { name: 'twitter:title', content: schema.value.title },
      { name: 'twitter:description', content: schema.value.description },
      { name: 'twitter:image', content: imageUrl },
      { name: 'robots', content: 'index, follow' },
      { name: 'googlebot', content: 'index, follow' }
    ],
    link: [
      { rel: 'canonical', href: url },
      { rel: 'stylesheet', href: 'https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css' }
    ],
    script: [
      {
        type: 'application/ld+json',
        children: JSON.stringify({
          '@context': 'https://schema.org',
          '@type': 'TechArticle',
          headline: schema.value.title,
          description: schema.value.description,
          author: { '@type': 'Organization', name: 'MathHook Team' },
          publisher: {
            '@type': 'Organization',
            name: 'MathHook',
            logo: { '@type': 'ImageObject', url: 'https://mathook.org/logo.png' }
          },
          datePublished: '2024-01-01',
          dateModified: new Date().toISOString().split('T')[0],
          mainEntityOfPage: { '@type': 'WebPage', '@id': url },
          keywords: `${schema.value.topic}, mathematics, symbolic computation`,
          articleSection: schema.value.topic,
          inLanguage: 'en-US'
        })
      }
    ]
  }
})
</script>

<style scoped>
/* Fade in animation */
@keyframes fade-in {
  0% {
    opacity: 0;
    transform: translateY(15px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-fade-in {
  animation: fade-in 0.6s ease-out forwards;
  opacity: 0;
}

/* Fade in from left */
@keyframes fade-in-left {
  0% {
    opacity: 0;
    transform: translateX(-20px);
  }
  100% {
    opacity: 1;
    transform: translateX(0);
  }
}

.animate-fade-in-left {
  animation: fade-in-left 0.6s ease-out forwards;
  opacity: 0;
}

/* Code transition */
.fade-code-enter-active,
.fade-code-leave-active {
  transition: all 0.2s ease;
}

.fade-code-enter-from,
.fade-code-leave-to {
  opacity: 0;
  transform: translateY(5px);
}

/* Back to top button transition */
.fade-enter-active,
.fade-leave-active {
  transition: all 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(20px);
}

/* Code block styling */
pre {
  @apply rounded-xl;
}

code {
  @apply font-mono text-sm leading-relaxed;
  font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', 'Monaco', monospace;
}

/* Math content styling */
.math-content {
  @apply text-chalk-300 leading-relaxed;
}

.math-content :deep(.katex) {
  @apply text-chalk-200;
  font-size: 1.05em;
}

.math-content :deep(.katex-display) {
  @apply my-4 text-chalk-100;
  font-size: 1.15em;
}

.math-content :deep(strong) {
  @apply font-semibold text-chalk-200;
}

.math-content :deep(p) {
  @apply mb-3;
}

/* Rendered content */
.rendered-content :deep(.katex) {
  @apply text-chalk-200;
  font-size: 1.05em;
}

.rendered-content :deep(.katex-display) {
  @apply my-4 text-chalk-100;
  font-size: 1.15em;
}

.rendered-content :deep(h3) {
  @apply mt-6 mb-3 text-chalk font-semibold;
}

.rendered-content :deep(p) {
  @apply mb-3 leading-relaxed text-chalk-300;
}

.rendered-content :deep(strong) {
  @apply font-semibold text-chalk-200;
}

/* Prose */
.prose :deep(ul) {
  @apply list-disc list-inside space-y-2 my-4;
}

.prose :deep(ol) {
  @apply list-decimal list-inside space-y-2 my-4;
}

.prose :deep(li) {
  @apply text-chalk-300;
}

/* KaTeX color overrides for consistency */
:deep(.katex) {
  color: inherit;
}

:deep(.katex .mord),
:deep(.katex .mop),
:deep(.katex .mbin),
:deep(.katex .mrel),
:deep(.katex .mpunct),
:deep(.katex .mopen),
:deep(.katex .mclose) {
  color: inherit;
}

/* Subtle blob animations - slower and more gentle for reading */
@keyframes blob-slow-1 {
  0%, 100% { transform: translate(0, 0) scale(1); }
  33% { transform: translate(40px, -30px) scale(1.05); }
  66% { transform: translate(-30px, 40px) scale(0.98); }
}

@keyframes blob-slow-2 {
  0%, 100% { transform: translate(0, 0) scale(1); }
  33% { transform: translate(-35px, 25px) scale(1.03); }
  66% { transform: translate(25px, -30px) scale(0.97); }
}

@keyframes blob-slow-3 {
  0%, 100% { transform: translate(0, 0) scale(1); }
  50% { transform: translate(30px, 35px) scale(1.04); }
}

.animate-blob-slow-1 { animation: blob-slow-1 30s ease-in-out infinite; }
.animate-blob-slow-2 { animation: blob-slow-2 35s ease-in-out infinite; }
.animate-blob-slow-3 { animation: blob-slow-3 28s ease-in-out infinite; }

/* Subtle floating particles */
@keyframes float-particle-subtle {
  0%, 100% { transform: translateY(0) translateX(0); opacity: 0; }
  15% { opacity: 0.8; }
  85% { opacity: 0.8; }
  100% { transform: translateY(-80vh) translateX(30px); opacity: 0; }
}

.animate-float-particle-subtle {
  animation: float-particle-subtle 20s ease-in-out infinite;
}
</style>
