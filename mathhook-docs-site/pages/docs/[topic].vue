<template>
  <div class="min-h-screen bg-slate-50 dark:bg-slate-900">
    <!-- Header -->
    <header class="bg-white dark:bg-slate-800 shadow-sm sticky top-0 z-50 border-b border-gray-200 dark:border-gray-700">
      <div class="container mx-auto px-4 py-4">
        <div class="flex items-center justify-between">
          <NuxtLink to="/" class="flex items-center space-x-2">
            <span class="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">MathHook</span>
          </NuxtLink>
          <nav class="flex gap-6">
            <NuxtLink to="/docs" class="text-gray-600 dark:text-gray-300 hover:text-blue-600 font-medium">
              Docs
            </NuxtLink>
            <a href="https://github.com/mathhook/mathhook" target="_blank" class="text-gray-600 dark:text-gray-300 hover:text-blue-600 font-medium">
              GitHub
            </a>
          </nav>
        </div>
      </div>
    </header>

    <!-- Loading State -->
    <div v-if="loading" class="container mx-auto px-4 py-12">
      <div class="max-w-4xl mx-auto text-center">
        <div class="animate-pulse space-y-4">
          <div class="h-10 bg-gray-200 dark:bg-gray-700 rounded w-3/4 mx-auto"></div>
          <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-full"></div>
          <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-5/6"></div>
        </div>
      </div>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="container mx-auto px-4 py-12">
      <div class="max-w-4xl mx-auto">
        <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-6">
          <h2 class="text-2xl font-semibold text-red-800 dark:text-red-200 mb-2">
            Document Not Found
          </h2>
          <p class="text-red-600 dark:text-red-300 mb-4">
            {{ error }}
          </p>
          <NuxtLink to="/docs" class="text-blue-600 hover:underline">
            ← Back to documentation
          </NuxtLink>
        </div>
      </div>
    </div>

    <!-- Content -->
    <div v-else-if="schema" class="container mx-auto px-4 py-12">
      <article class="max-w-5xl mx-auto">
        <!-- Breadcrumb -->
        <nav class="mb-6 text-sm text-gray-600 dark:text-gray-400">
          <NuxtLink to="/" class="hover:text-blue-600">Home</NuxtLink>
          <span class="mx-2">/</span>
          <NuxtLink to="/docs" class="hover:text-blue-600">Docs</NuxtLink>
          <span class="mx-2">/</span>
          <span class="text-gray-800 dark:text-gray-200">{{ schema.title }}</span>
        </nav>

        <!-- Title -->
        <header class="mb-10">
          <h1 class="text-5xl font-bold text-gray-900 dark:text-white mb-4 leading-tight">
            {{ schema.title }}
          </h1>
          <p class="text-xl text-gray-600 dark:text-gray-300 leading-relaxed">
            {{ schema.description }}
          </p>
        </header>

        <!-- Mathematical Definition -->
        <section v-if="schema.mathematical_definition" class="mb-12 bg-gradient-to-br from-blue-50 to-indigo-50 dark:from-slate-800 dark:to-slate-700 p-8 rounded-xl border border-blue-100 dark:border-slate-600">
          <h2 class="text-2xl font-bold mb-6 text-gray-800 dark:text-white flex items-center">
            <span class="mr-2">📐</span> Mathematical Definition
          </h2>
          <div class="flex justify-center">
            <div class="katex-display bg-white dark:bg-slate-900 px-8 py-6 rounded-lg shadow-sm" v-html="renderMath(schema.mathematical_definition)"></div>
          </div>
        </section>

        <!-- Introduction -->
        <section v-if="schema.article?.introduction" class="mb-12 prose prose-lg dark:prose-invert max-w-none">
          <h2 class="text-3xl font-bold mb-6 text-gray-800 dark:text-white">
            Introduction
          </h2>
          <p class="text-gray-700 dark:text-gray-300 text-lg leading-relaxed">{{ schema.article.introduction.hook }}</p>
        </section>

        <!-- Interactive Examples -->
        <section class="mb-12">
          <h2 class="text-3xl font-bold mb-8 text-gray-800 dark:text-white">
            Interactive Examples
          </h2>
          <div class="space-y-8">
            <div
              v-for="(example, idx) in schema.examples"
              :key="idx"
              class="bg-white dark:bg-slate-800 rounded-xl shadow-lg border border-gray-200 dark:border-slate-700 overflow-hidden"
            >
              <div class="p-6 border-b border-gray-200 dark:border-slate-700">
                <h3 class="text-2xl font-bold mb-3 text-gray-900 dark:text-white">
                  {{ example.title }}
                </h3>
                <p class="text-gray-600 dark:text-gray-300 text-base leading-relaxed">
                  {{ example.explanation }}
                </p>
              </div>

              <!-- Code Tabs -->
              <div class="bg-gray-50 dark:bg-slate-900 border-b border-gray-200 dark:border-slate-700">
                <div class="flex px-6">
                  <button
                    @click="activeTab[idx] = 'python'"
                    :class="[
                      'px-6 py-3 font-semibold text-sm transition-all border-b-2',
                      activeTab[idx] === 'python'
                        ? 'text-blue-600 dark:text-blue-400 border-blue-600 dark:border-blue-400 bg-white dark:bg-slate-800'
                        : 'text-gray-600 dark:text-gray-400 border-transparent hover:text-gray-800 dark:hover:text-gray-200'
                    ]"
                  >
                    <span class="mr-2">🐍</span>Python
                  </button>
                  <button
                    @click="activeTab[idx] = 'rust'"
                    :class="[
                      'px-6 py-3 font-semibold text-sm transition-all border-b-2',
                      activeTab[idx] === 'rust'
                        ? 'text-blue-600 dark:text-blue-400 border-blue-600 dark:border-blue-400 bg-white dark:bg-slate-800'
                        : 'text-gray-600 dark:text-gray-400 border-transparent hover:text-gray-800 dark:hover:text-gray-200'
                    ]"
                  >
                    <span class="mr-2">🦀</span>Rust
                  </button>
                  <button
                    @click="activeTab[idx] = 'nodejs'"
                    :class="[
                      'px-6 py-3 font-semibold text-sm transition-all border-b-2',
                      activeTab[idx] === 'nodejs'
                        ? 'text-blue-600 dark:text-blue-400 border-blue-600 dark:border-blue-400 bg-white dark:bg-slate-800'
                        : 'text-gray-600 dark:text-gray-400 border-transparent hover:text-gray-800 dark:hover:text-gray-200'
                    ]"
                  >
                    <span class="mr-2">📜</span>JavaScript
                  </button>
                </div>
              </div>

              <!-- Code Content with Proper Syntax Highlighting -->
              <div class="relative">
                <pre v-if="activeTab[idx] === 'python'" class="!m-0 !p-6 !bg-[#1e1e1e] overflow-x-auto"><code class="language-python text-sm leading-relaxed" style="font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;">{{ example.code.python }}</code></pre>
                <pre v-if="activeTab[idx] === 'rust'" class="!m-0 !p-6 !bg-[#1e1e1e] overflow-x-auto"><code class="language-rust text-sm leading-relaxed" style="font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;">{{ example.code.rust }}</code></pre>
                <pre v-if="activeTab[idx] === 'nodejs'" class="!m-0 !p-6 !bg-[#1e1e1e] overflow-x-auto"><code class="language-javascript text-sm leading-relaxed" style="font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;">{{ example.code.nodejs }}</code></pre>
              </div>

              <!-- Output -->
              <div v-if="example.expected_output" class="p-6 bg-gradient-to-br from-green-50 to-emerald-50 dark:from-slate-800 dark:to-slate-700 border-t border-gray-200 dark:border-slate-600">
                <div class="flex items-center mb-2">
                  <span class="mr-2 text-green-600 dark:text-green-400">▶</span>
                  <strong class="text-gray-700 dark:text-gray-200 font-semibold">Output:</strong>
                </div>
                <pre class="mt-2 text-gray-800 dark:text-gray-100 font-mono text-sm bg-white dark:bg-slate-900 p-4 rounded-lg border border-green-200 dark:border-slate-600">{{ example.expected_output }}</pre>
              </div>
            </div>
          </div>
        </section>

        <!-- Article Sections -->
        <section v-if="schema.article?.sections" class="mb-12">
          <div v-for="(section, idx) in schema.article.sections" :key="idx" class="mb-10">
            <h2 class="text-3xl font-bold mb-6 text-gray-800 dark:text-white">
              {{ section.title }}
            </h2>
            <div class="prose prose-lg dark:prose-invert max-w-none text-gray-700 dark:text-gray-300" v-html="section.content"></div>
          </div>
        </section>
      </article>
    </div>
  </div>
</template>

<script setup>
import { getHighlighter } from 'shiki'

const route = useRoute()
const topic = route.params.topic

const schema = ref(null)
const loading = ref(true)
const error = ref(null)
const activeTab = ref({})

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
      })
    }

    // Apply syntax highlighting after DOM updates
    nextTick(() => {
      applySyntaxHighlighting()
    })
  } catch (err) {
    error.value = err.message
  } finally {
    loading.value = false
  }
})

// Watch for tab changes to reapply syntax highlighting
watch(activeTab, () => {
  nextTick(() => {
    applySyntaxHighlighting()
  })
}, { deep: true })

// Apply syntax highlighting using Shiki
const applySyntaxHighlighting = async () => {
  if (typeof window === 'undefined') return

  const highlighter = await getHighlighter({
    themes: ['dark-plus'],
    langs: ['python', 'rust', 'javascript']
  })

  const codeBlocks = document.querySelectorAll('code.language-python, code.language-rust, code.language-javascript')
  codeBlocks.forEach(block => {
    const lang = block.className.replace('language-', '')
    const code = block.textContent
    const html = highlighter.codeToHtml(code, { lang, theme: 'dark-plus' })
    const tempDiv = document.createElement('div')
    tempDiv.innerHTML = html
    const pre = tempDiv.querySelector('pre')
    if (pre) {
      block.innerHTML = pre.innerHTML
    }
  })
}

// Render math using KaTeX
const renderMath = (latex) => {
  if (typeof window !== 'undefined' && window.katex) {
    try {
      return window.katex.renderToString(latex, {
        throwOnError: false,
        displayMode: true
      })
    } catch (e) {
      return latex
    }
  }
  return latex
}

// Set page metadata
useHead(() => ({
  title: schema.value ? `${schema.value.title} - MathHook KB` : 'Loading...',
  meta: [
    {
      name: 'description',
      content: schema.value?.description || 'Mathematical documentation'
    }
  ]
}))
</script>

<style scoped>
/* Code block styling */
pre {
  @apply rounded-lg;
}

code {
  @apply font-mono;
}

/* Ensure proper spacing for mathematical content */
.katex-display {
  font-size: 1.3em;
}

/* Prose styling for article sections */
.prose {
  @apply text-gray-700 dark:text-gray-300;
}

.prose h2 {
  @apply text-gray-900 dark:text-white;
}

.prose p {
  @apply leading-relaxed;
}
</style>
