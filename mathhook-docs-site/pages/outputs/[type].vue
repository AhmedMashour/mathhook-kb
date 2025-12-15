<template>
  <div class="min-h-screen bg-logic-navy-900 text-chalk overflow-hidden relative">
    <!-- Animated background -->
    <HomeAnimatedBackground />

    <!-- Navigation -->
    <HomeAppNavbar />

    <!-- Main Content -->
    <div class="max-w-6xl mx-auto px-4 py-10 pt-24 relative z-10">
      <!-- Header -->
      <div class="mb-8">
        <div class="flex items-center gap-4 mb-4">
          <NuxtLink
            to="/outputs"
            class="flex items-center gap-2 text-chalk-400 hover:text-chalk transition-colors"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
            </svg>
            Back to Outputs
          </NuxtLink>
        </div>

        <div class="flex items-center gap-4 mb-4">
          <div
            class="w-16 h-16 rounded-xl flex items-center justify-center"
            :class="platformConfig.iconBg"
          >
            <component :is="platformConfig.icon" class="w-8 h-8" :class="platformConfig.iconColor" />
          </div>
          <div>
            <h1 class="text-3xl font-bold" :class="platformConfig.titleColor">
              {{ platformConfig.title }}
            </h1>
            <p class="text-chalk-400">{{ platformConfig.subtitle }}</p>
          </div>
        </div>

        <p class="text-chalk-400 max-w-2xl mb-6">{{ platformConfig.description }}</p>

        <!-- Stats and Actions -->
        <div class="flex items-center justify-between flex-wrap gap-4">
          <div class="flex items-center gap-6 text-sm">
            <div class="flex items-center gap-2">
              <span class="px-3 py-1 rounded-full" :class="platformConfig.badge">
                {{ files.length }} files
              </span>
            </div>
            <div class="flex items-center gap-2">
              <span class="text-chalk-500">{{ categories.length }} categories</span>
            </div>
          </div>

          <!-- View Book button for mdBook -->
          <a
            v-if="platformType === 'mdbook'"
            href="/files/mdbook/index.html"
            target="_blank"
            class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-rust-core/10 text-rust-core hover:bg-rust-core/20 border border-rust-core/30 transition-all duration-200 font-medium"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
            </svg>
            Read the Book
          </a>
        </div>
      </div>

      <!-- Loading State -->
      <div v-if="loading" class="flex items-center justify-center py-20">
        <svg class="animate-spin h-8 w-8" :class="platformConfig.iconColor" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <span class="ml-3 text-chalk-400">Loading files...</span>
      </div>

      <!-- Error State -->
      <div v-else-if="error" class="text-center py-20">
        <svg class="w-16 h-16 mx-auto text-red-500/50 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
        </svg>
        <p class="text-red-400">{{ error }}</p>
        <button
          @click="refresh"
          class="mt-4 px-4 py-2 rounded-lg bg-logic-navy-700 hover:bg-logic-navy-600 text-chalk transition-colors"
        >
          Try Again
        </button>
      </div>

      <!-- File Browser -->
      <div v-else class="bg-logic-navy-800/30 backdrop-blur-sm rounded-2xl border border-logic-navy-700/50 p-6">
        <OutputsFileBrowser
          :files="files"
          :color="platformConfig.color"
          @download="handleDownload"
          @download-multiple="handleMultipleDownload"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useFileData } from '~/composables/useFileData'

const route = useRoute()
const {
  colabFiles,
  jupyterFiles,
  ragFiles,
  latexFiles,
  mdbookFiles,
  apiDocsFiles,
  jsonFiles,
  vueFiles,
  loading,
  error,
  fetchAllFiles,
  refresh,
  categories: allCategories
} = useFileData()

interface PlatformConfig {
  title: string
  subtitle: string
  description: string
  color: string
  icon: string
  iconBg: string
  iconColor: string
  titleColor: string
  badge: string
}

// Platform configurations
const platformConfigs: Record<string, PlatformConfig> = {
  jupyter: {
    title: 'Jupyter Notebooks',
    subtitle: 'Interactive Python notebooks',
    description: 'Executable notebooks with live code cells. Download and run locally with Jupyter or JupyterLab.',
    color: 'amber',
    icon: 'OutputsIconsNotebookIcon',
    iconBg: 'bg-amber-500/10',
    iconColor: 'text-amber-500',
    titleColor: 'text-amber-500',
    badge: 'bg-amber-500/10 text-amber-500'
  },
  colab: {
    title: 'Google Colab Notebooks',
    subtitle: 'Cloud executable notebooks',
    description: 'Colab-optimized notebooks with pre-configured dependencies. Open directly in Google Colab for zero-setup execution.',
    color: 'amber',
    icon: 'OutputsIconsPlayIcon',
    iconBg: 'bg-amber-500/10',
    iconColor: 'text-amber-500',
    titleColor: 'text-amber-500',
    badge: 'bg-amber-500/10 text-amber-500'
  },
  'llm-rag': {
    title: 'LLM-RAG Optimized',
    subtitle: 'For AI assistants & search',
    description: 'Documentation chunks optimized for vector databases and AI retrieval. Perfect for building MathHook-aware AI assistants.',
    color: 'cyan',
    icon: 'OutputsIconsComputerIcon',
    iconBg: 'bg-solve-cyan/10',
    iconColor: 'text-solve-cyan',
    titleColor: 'text-solve-cyan',
    badge: 'bg-solve-cyan/10 text-solve-cyan'
  },
  latex: {
    title: 'LaTeX Documents',
    subtitle: 'Academic paper format',
    description: 'Professional LaTeX source files ready for compilation. Perfect for academic papers, offline reading, and printing.',
    color: 'violet',
    icon: 'OutputsIconsDocumentIcon',
    iconBg: 'bg-violet-500/10',
    iconColor: 'text-violet-500',
    titleColor: 'text-violet-500',
    badge: 'bg-violet-500/10 text-violet-500'
  },
  mdbook: {
    title: 'Rust mdBook',
    subtitle: 'Searchable Rust documentation',
    description: 'Interactive book format with code examples in Rust, Python, and JavaScript. Perfect for Rust developers.',
    color: 'rust',
    icon: 'OutputsIconsBookIcon',
    iconBg: 'bg-rust-core/10',
    iconColor: 'text-rust-core',
    titleColor: 'text-rust-core',
    badge: 'bg-rust-core/10 text-rust-core'
  },
  'api-docs': {
    title: 'API Documentation',
    subtitle: 'OpenAPI specifications',
    description: 'Machine-readable API documentation in OpenAPI format. Perfect for integration and tooling.',
    color: 'green',
    icon: 'OutputsIconsTerminalIcon',
    iconBg: 'bg-step-green/10',
    iconColor: 'text-step-green',
    titleColor: 'text-step-green',
    badge: 'bg-step-green/10 text-step-green'
  },
  json: {
    title: 'JSON Data',
    subtitle: 'Structured data format',
    description: 'Raw JSON documentation data for programmatic access and custom integrations.',
    color: 'cyan',
    icon: 'OutputsIconsTerminalIcon',
    iconBg: 'bg-solve-cyan/10',
    iconColor: 'text-solve-cyan',
    titleColor: 'text-solve-cyan',
    badge: 'bg-solve-cyan/10 text-solve-cyan'
  },
  vue: {
    title: 'Vue Components',
    subtitle: 'Interactive UI components',
    description: 'Vue.js components for building interactive documentation interfaces and dashboards.',
    color: 'green',
    icon: 'OutputsIconsComputerIcon',
    iconBg: 'bg-step-green/10',
    iconColor: 'text-step-green',
    titleColor: 'text-step-green',
    badge: 'bg-step-green/10 text-step-green'
  }
}

// Get current platform type from route
const platformType = computed(() => String(route.params.type || 'jupyter'))

// Get platform config
const platformConfig = computed(() => {
  return platformConfigs[platformType.value] || platformConfigs.jupyter
})

// Get files for current platform
const files = computed(() => {
  const typeMap = {
    'colab': colabFiles.value,
    'jupyter': jupyterFiles.value,
    'llm-rag': ragFiles.value,
    'latex': latexFiles.value,
    'mdbook': mdbookFiles.value,
    'api-docs': apiDocsFiles.value,
    'json': jsonFiles.value,
    'vue': vueFiles.value
  }
  return typeMap[platformType.value] || []
})

// Get categories for current platform
const categories = computed(() => {
  const uniqueCategories = new Set(files.value.map(f => f.category))
  return Array.from(uniqueCategories).sort()
})

// Fetch files on mount
onMounted(() => {
  if (files.value.length === 0) {
    fetchAllFiles()
  }
})

// Handle downloads
function handleDownload(file) {
  console.log('Download:', file.name)
}

function handleMultipleDownload(files) {
  console.log('Download multiple:', files.length, 'files')
}

// SEO
useHead({
  title: () => `${platformConfig.value.title} - MathHook Outputs`,
  meta: [
    {
      name: 'description',
      content: () => platformConfig.value.description
    }
  ]
})
</script>
