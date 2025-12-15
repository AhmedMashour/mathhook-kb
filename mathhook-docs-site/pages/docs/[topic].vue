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
            <NuxtLink to="/docs" class="text-solve-cyan font-medium">Docs</NuxtLink>
            <NuxtLink to="/outputs" class="text-chalk-500 hover:text-chalk transition-colors">Outputs</NuxtLink>
            <a href="https://github.com/AhmedMashour/mathhook" target="_blank" class="text-chalk-500 hover:text-chalk transition-colors flex items-center gap-1.5">
              <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>
              GitHub
            </a>
          </div>
        </div>
      </div>
    </nav>

    <!-- Loading State -->
    <div v-if="loading" class="max-w-4xl mx-auto px-4 py-12">
      <div class="animate-pulse space-y-4">
        <div class="h-10 bg-logic-navy-700 rounded w-3/4"></div>
        <div class="h-4 bg-logic-navy-700 rounded w-full"></div>
        <div class="h-4 bg-logic-navy-700 rounded w-5/6"></div>
      </div>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="max-w-4xl mx-auto px-4 py-12">
      <div class="bg-rust-core/10 border-l-4 border-rust-core p-6 rounded-r-lg">
        <div class="flex items-start">
          <span class="text-3xl mr-3">❌</span>
          <div>
            <h2 class="text-xl font-semibold text-rust-core mb-2">Document Not Found</h2>
            <p class="text-chalk-400 mb-4">{{ error }}</p>
            <NuxtLink to="/docs" class="text-solve-cyan hover:text-solve-cyan-300 font-medium">
              ← Back to documentation
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>

    <!-- Content -->
    <div v-else-if="schema" class="max-w-7xl mx-auto px-4 py-8">
      <div class="flex gap-8">
        <!-- Table of Contents Sidebar -->
        <aside class="hidden lg:block w-64 sticky top-20 self-start">
          <div class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl p-5">
            <h3 class="text-xs font-medium text-chalk-500 mb-4 uppercase tracking-wide">On This Page</h3>
            <nav class="space-y-1.5">
              <a href="#definition" class="block text-sm text-chalk-500 hover:text-solve-cyan transition-colors py-1">Definition</a>
              <a href="#introduction" class="block text-sm text-chalk-500 hover:text-solve-cyan transition-colors py-1">Introduction</a>
              <a href="#examples" class="block text-sm text-chalk-500 hover:text-solve-cyan transition-colors py-1">Code Examples</a>
              <a v-for="(section, idx) in schema.article?.sections" :key="idx" :href="`#section-${idx}`" class="block text-sm text-chalk-500 hover:text-solve-cyan transition-colors py-1">
                {{ section.title }}
              </a>
            </nav>
          </div>

          <!-- Quick Reference Card -->
          <div class="mt-4 bg-gradient-to-br from-rust-core/10 to-solve-cyan/10 border border-logic-navy-700 rounded-xl p-5">
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
                  <span v-for="i in 5" :key="i" :class="i <= (schema.article.complexity === 'beginner' ? 1 : schema.article.complexity === 'intermediate' ? 3 : 5) ? 'text-solve-cyan' : 'text-logic-navy-600'">●</span>
                </div>
              </div>
            </div>
          </div>
        </aside>

        <!-- Main Content -->
        <article class="flex-1 max-w-4xl">
          <!-- Breadcrumb -->
          <nav class="mb-6 flex items-center gap-2 text-sm text-chalk-500">
            <NuxtLink to="/" class="hover:text-solve-cyan transition-colors">Home</NuxtLink>
            <span class="text-chalk-600">/</span>
            <NuxtLink to="/docs" class="hover:text-solve-cyan transition-colors">Docs</NuxtLink>
            <span class="text-chalk-600">/</span>
            <span class="text-chalk">{{ schema.title }}</span>
          </nav>

          <!-- Title Section -->
          <header class="mb-10">
            <h1 class="text-3xl md:text-4xl font-semibold text-chalk mb-3 leading-tight">
              {{ schema.title }}
            </h1>
            <p class="text-base text-chalk-400 leading-relaxed">
              {{ schema.description }}
            </p>
          </header>

          <!-- Mathematical Definition Callout -->
          <div id="definition" v-if="schema.mathematical_definition" class="mb-10 scroll-mt-24">
            <div class="bg-gradient-to-r from-rust-core to-solve-cyan p-[1px] rounded-xl">
              <div class="bg-logic-navy-900 rounded-xl p-6">
                <div class="flex items-center gap-3 mb-4">
                  <div class="w-9 h-9 bg-gradient-to-br from-rust-core to-solve-cyan rounded-lg flex items-center justify-center text-white text-lg">📐</div>
                  <h2 class="text-lg font-semibold text-chalk">Mathematical Definition</h2>
                </div>
                <div class="flex justify-center">
                  <div class="math-display" v-html="renderMath(schema.mathematical_definition, true)"></div>
                </div>
              </div>
            </div>
          </div>

          <!-- Introduction -->
          <section id="introduction" v-if="schema.article?.introduction" class="mb-10 scroll-mt-24">
            <h2 class="text-xl font-semibold text-chalk mb-4 flex items-center gap-3">
              <span class="w-8 h-1 bg-gradient-to-r from-rust-core to-solve-cyan rounded-full"></span>
              Introduction
            </h2>
            <div class="text-chalk-300 text-base leading-relaxed rendered-content" v-html="renderContent(schema.article.introduction.hook)"></div>
          </section>

          <!-- Code Examples -->
          <section id="examples" class="mb-10 scroll-mt-24">
            <h2 class="text-xl font-semibold text-chalk mb-6 flex items-center gap-3">
              <span class="w-8 h-1 bg-gradient-to-r from-rust-core to-solve-cyan rounded-full"></span>
              Code Examples
            </h2>
            <div class="space-y-6">
              <div v-for="(example, idx) in schema.examples" :key="idx" class="bg-logic-navy-800/50 border border-logic-navy-700 rounded-xl overflow-hidden">
                <!-- Example Header -->
                <div class="p-5 border-b border-logic-navy-700">
                  <h3 class="text-lg font-medium text-chalk mb-2">{{ example.title }}</h3>
                  <div class="text-chalk-400 text-sm leading-relaxed rendered-content whitespace-pre-wrap" v-html="renderContent(example.explanation)"></div>
                </div>

                <!-- Language Tabs -->
                <div class="bg-logic-navy-900 border-b border-logic-navy-700">
                  <div class="flex">
                    <button @click="activeTab[idx] = 'python'" :class="['flex-1 px-4 py-2.5 font-medium text-sm transition-all relative', activeTab[idx] === 'python' ? 'text-solve-cyan bg-logic-navy-800' : 'text-chalk-500 hover:text-chalk hover:bg-logic-navy-800/50']">
                      <span class="flex items-center justify-center gap-2">
                        <span>🐍</span><span>Python</span>
                      </span>
                      <div v-if="activeTab[idx] === 'python'" class="absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-rust-core to-solve-cyan"></div>
                    </button>
                    <button @click="activeTab[idx] = 'rust'" :class="['flex-1 px-4 py-2.5 font-medium text-sm transition-all relative', activeTab[idx] === 'rust' ? 'text-rust-core bg-logic-navy-800' : 'text-chalk-500 hover:text-chalk hover:bg-logic-navy-800/50']">
                      <span class="flex items-center justify-center gap-2">
                        <span>🦀</span><span>Rust</span>
                      </span>
                      <div v-if="activeTab[idx] === 'rust'" class="absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-rust-core to-solve-cyan"></div>
                    </button>
                    <button @click="activeTab[idx] = 'nodejs'" :class="['flex-1 px-4 py-2.5 font-medium text-sm transition-all relative', activeTab[idx] === 'nodejs' ? 'text-step-green bg-logic-navy-800' : 'text-chalk-500 hover:text-chalk hover:bg-logic-navy-800/50']">
                      <span class="flex items-center justify-center gap-2">
                        <span>📜</span><span>JavaScript</span>
                      </span>
                      <div v-if="activeTab[idx] === 'nodejs'" class="absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-rust-core to-solve-cyan"></div>
                    </button>
                  </div>
                </div>

                <!-- Code Content with Copy Button -->
                <div class="relative group/code">
                  <button @click="copyCode(example.code[activeTab[idx]], idx)" class="absolute top-3 right-3 px-2.5 py-1 bg-logic-navy-700 hover:bg-logic-navy-600 text-chalk-400 text-xs font-medium rounded opacity-0 group-hover/code:opacity-100 transition-opacity z-10 flex items-center gap-1.5">
                    <span v-if="!copied[idx]">📋 Copy</span>
                    <span v-else class="text-step-green">✓ Copied!</span>
                  </button>
                  <pre v-if="activeTab[idx] === 'python'" class="language-python !m-0 !p-5 overflow-x-auto !bg-logic-navy-900"><code v-html="highlightCode(example.code.python, 'python')"></code></pre>
                  <pre v-if="activeTab[idx] === 'rust'" class="language-rust !m-0 !p-5 overflow-x-auto !bg-logic-navy-900"><code v-html="highlightCode(example.code.rust, 'rust')"></code></pre>
                  <pre v-if="activeTab[idx] === 'nodejs'" class="language-javascript !m-0 !p-5 overflow-x-auto !bg-logic-navy-900"><code v-html="highlightCode(example.code.nodejs, 'javascript')"></code></pre>
                </div>

                <!-- Output Section -->
                <div v-if="example.expected_output" class="border-t border-logic-navy-700">
                  <div class="bg-step-green/5 p-5">
                    <div class="flex items-center gap-2 mb-3">
                      <div class="w-5 h-5 bg-step-green rounded flex items-center justify-center text-white text-xs">▶</div>
                      <strong class="text-chalk font-medium text-sm">Output</strong>
                    </div>
                    <pre class="text-chalk-200 font-mono text-sm bg-logic-navy-900 p-4 rounded-lg border border-step-green/20">{{ example.expected_output }}</pre>
                  </div>
                </div>
              </div>
            </div>
          </section>

          <!-- Article Sections with Callouts -->
          <section v-if="schema.article?.sections" class="mb-10 space-y-10">
            <div v-for="(section, idx) in schema.article.sections" :key="idx" :id="`section-${idx}`" class="scroll-mt-24">
              <h2 class="text-xl font-semibold text-chalk mb-4 flex items-center gap-3">
                <span class="w-8 h-1 bg-gradient-to-r from-rust-core to-solve-cyan rounded-full"></span>
                {{ section.title }}
              </h2>
              <div class="prose prose-lg max-w-none rendered-content text-chalk-300" v-html="renderContentWithCallouts(section.content)"></div>
            </div>
          </section>

          <!-- Sidebars as Callouts -->
          <section v-if="schema.article?.sidebars" class="mb-10 space-y-4">
            <div v-for="(sidebar, idx) in schema.article.sidebars" :key="idx">
              <div :class="['border-l-4 rounded-r-xl p-5', sidebar.type === 'note' ? 'bg-solve-cyan/10 border-solve-cyan' : '', sidebar.type === 'warning' ? 'bg-amber-500/10 border-amber-500' : '', sidebar.type === 'info' ? 'bg-violet-500/10 border-violet-500' : '', sidebar.type === 'performance' ? 'bg-step-green/10 border-step-green' : '']">
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
          <section class="mt-12 pt-8 border-t border-logic-navy-700">
            <h2 class="text-lg font-semibold text-chalk mb-4 flex items-center gap-2">
              <span class="text-solve-cyan">🔗</span> Related Topics
            </h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
              <NuxtLink to="/docs" class="group p-4 bg-logic-navy-800/50 rounded-lg border border-logic-navy-700 hover:border-solve-cyan/50 transition-all">
                <div class="font-semibold text-chalk group-hover:text-solve-cyan transition-colors">Browse All Topics</div>
                <div class="text-sm text-chalk-500 mt-1">Explore the full documentation library</div>
              </NuxtLink>
            </div>
          </section>
        </article>
      </div>
    </div>
  </div>
</template>

<script setup>
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

// Load schema data
onMounted(async () => {
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

  const url = `https://mathhook.dev/docs/${topic}`
  const imageUrl = `https://mathhook.dev/og-images/${topic}.png`

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
            logo: { '@type': 'ImageObject', url: 'https://mathhook.dev/logo.png' }
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
/* Code block styling */
pre {
  @apply rounded-lg;
}

code {
  @apply font-mono text-sm leading-relaxed;
  font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', 'Monaco', monospace;
}

/* Math display */
.math-display {
  font-size: 1.4em;
}

/* Rendered content */
.rendered-content :deep(.katex) {
  font-size: 1.1em;
}

.rendered-content :deep(.katex-display) {
  margin: 1.5em 0;
  font-size: 1.2em;
}

.rendered-content :deep(h3) {
  margin-top: 1.5rem;
  margin-bottom: 0.75rem;
}

.rendered-content :deep(p) {
  margin-bottom: 0.75rem;
  line-height: 1.7;
}

.rendered-content :deep(strong) {
  font-weight: 600;
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
</style>
