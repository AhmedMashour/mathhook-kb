// File data composable for outputs file browser
// Updated: 2025-12-15T20:15:00Z
// Optimized: Global caching, lazy loading, lightweight summary

import { ref, computed } from 'vue'

export interface FileItem {
  name: string
  path: string
  url: string
  category: string
  title?: string
  extension: string
  size?: number
  colabUrl?: string
  githubUrl?: string
}

export interface OutputManifest {
  generated_at: string
  output_type?: string
  total_files: number
  categories: Record<string, Array<{
    filename: string
    title: string
    category: string
    topic?: string
    colab_url?: string
    github_url?: string
  }>>
}

export interface OutputSummary {
  generated_at: string
  outputs: Record<string, { total_files: number; categories: number }>
}

// Output type key mapping
type OutputTypeKey = 'colab' | 'jupyter' | 'llm-rag' | 'latex' | 'mdbook' | 'api-docs' | 'json' | 'vue'

// Base paths for outputs (all served from /files/ in public folder)
const BASE_PATHS: Record<OutputTypeKey, string> = {
  jupyter: '/files/jupyter',
  colab: '/files/colab',
  'llm-rag': '/files/llm-rag',
  latex: '/files/latex',
  mdbook: '/files/mdbook',
  'api-docs': '/files/api-docs',
  json: '/files/json',
  vue: '/files/vue'
}

// GitHub config for Colab URLs
const GITHUB_CONFIG = {
  user: 'AhmedMashour',
  repo: 'mathhook-kb',
  branch: 'master',
  colabBasePath: 'mathhook-docs-site/public/files/colab'
}

// File extension mappings
const EXTENSIONS: Record<OutputTypeKey, string> = {
  jupyter: 'ipynb',
  colab: 'ipynb',
  'llm-rag': 'md',
  latex: 'tex',
  mdbook: 'md',
  'api-docs': 'json',
  json: 'json',
  vue: 'vue'
}

// ============================================
// GLOBAL CACHE (singleton, persists across navigations)
// ============================================
const globalCache = {
  summary: null as OutputSummary | null,
  manifests: new Map<OutputTypeKey, FileItem[]>(),
  summaryPromise: null as Promise<OutputSummary | null> | null,
  manifestPromises: new Map<OutputTypeKey, Promise<FileItem[]>>()
}

// Build correct Colab URL for a file
function buildColabUrl(category: string, filename: string): string {
  return `https://colab.research.google.com/github/${GITHUB_CONFIG.user}/${GITHUB_CONFIG.repo}/blob/${GITHUB_CONFIG.branch}/${GITHUB_CONFIG.colabBasePath}/${category}/${filename}`
}

// Build GitHub URL for a file
function buildGithubUrl(basePath: string, category: string, filename: string): string {
  const repoPath = basePath.replace('/files/', 'mathhook-docs-site/public/files/')
  return `https://github.com/${GITHUB_CONFIG.user}/${GITHUB_CONFIG.repo}/blob/${GITHUB_CONFIG.branch}/${repoPath}/${category}/${filename}`
}

// Fetch lightweight summary (only counts, ~500 bytes)
async function fetchSummary(): Promise<OutputSummary | null> {
  // Return cached
  if (globalCache.summary) return globalCache.summary

  // Return in-flight promise
  if (globalCache.summaryPromise) return globalCache.summaryPromise

  // Start fetch
  globalCache.summaryPromise = (async () => {
    try {
      const response = await fetch('/files/summary.json')
      if (!response.ok) return null
      const data = await response.json()
      globalCache.summary = data
      return data
    } catch (err) {
      console.error('Error fetching summary:', err)
      return null
    } finally {
      globalCache.summaryPromise = null
    }
  })()

  return globalCache.summaryPromise
}

// Fetch single manifest (lazy, cached)
async function fetchManifest(outputType: OutputTypeKey): Promise<FileItem[]> {
  // Return cached
  if (globalCache.manifests.has(outputType)) {
    return globalCache.manifests.get(outputType)!
  }

  // Return in-flight promise
  if (globalCache.manifestPromises.has(outputType)) {
    return globalCache.manifestPromises.get(outputType)!
  }

  const basePath = BASE_PATHS[outputType]
  const extension = EXTENSIONS[outputType]
  const isColab = outputType === 'colab'

  // Start fetch
  const promise = (async () => {
    try {
      const manifestUrl = `${basePath}/manifest.json`
      const response = await fetch(manifestUrl)

      if (!response.ok) {
        console.warn(`Failed to fetch manifest from ${manifestUrl}: ${response.status}`)
        return []
      }

      const manifest: OutputManifest = await response.json()
      const files: FileItem[] = []

      for (const [category, items] of Object.entries(manifest.categories)) {
        for (const item of items) {
          const fileItem: FileItem = {
            name: item.filename,
            path: `${category}/${item.filename}`,
            url: `${basePath}/${category}/${item.filename}`,
            category: category,
            title: item.title,
            extension: extension,
            githubUrl: buildGithubUrl(basePath, category, item.filename)
          }

          if (isColab) {
            fileItem.colabUrl = item.colab_url || buildColabUrl(category, item.filename)
          }

          files.push(fileItem)
        }
      }

      // Cache result
      globalCache.manifests.set(outputType, files)
      return files
    } catch (err) {
      console.error(`Error fetching manifest for ${outputType}:`, err)
      return []
    } finally {
      globalCache.manifestPromises.delete(outputType)
    }
  })()

  globalCache.manifestPromises.set(outputType, promise)
  return promise
}

// ============================================
// COMPOSABLE: For index page (counts only)
// ============================================
export function useOutputSummary() {
  const summary = ref<OutputSummary | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function load() {
    if (summary.value) return // Already loaded

    loading.value = true
    error.value = null

    try {
      summary.value = await fetchSummary()
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Unknown error'
    } finally {
      loading.value = false
    }
  }

  const fileCounts = computed(() => {
    if (!summary.value) return {}
    return Object.fromEntries(
      Object.entries(summary.value.outputs).map(([key, val]) => [key, val.total_files])
    )
  })

  return {
    summary,
    loading,
    error,
    load,
    fileCounts
  }
}

// ============================================
// COMPOSABLE: For single output type page
// ============================================
export function useOutputFiles(outputType: OutputTypeKey) {
  const files = ref<FileItem[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function load() {
    // Check cache first
    if (globalCache.manifests.has(outputType)) {
      files.value = globalCache.manifests.get(outputType)!
      return
    }

    loading.value = true
    error.value = null

    try {
      files.value = await fetchManifest(outputType)
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Unknown error'
    } finally {
      loading.value = false
    }
  }

  const categories = computed(() => {
    const uniqueCategories = new Set(files.value.map(f => f.category))
    return Array.from(uniqueCategories).sort()
  })

  async function refresh() {
    globalCache.manifests.delete(outputType)
    await load()
  }

  return {
    files,
    loading,
    error,
    load,
    refresh,
    categories
  }
}

// ============================================
// LEGACY COMPOSABLE: Full compatibility
// (only use if you need ALL files at once)
// ============================================
export function useFileData() {
  // File lists for each output type
  const colabFiles = ref<FileItem[]>([])
  const jupyterFiles = ref<FileItem[]>([])
  const ragFiles = ref<FileItem[]>([])
  const latexFiles = ref<FileItem[]>([])
  const mdbookFiles = ref<FileItem[]>([])
  const apiDocsFiles = ref<FileItem[]>([])
  const jsonFiles = ref<FileItem[]>([])
  const vueFiles = ref<FileItem[]>([])

  // Loading and error states
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Fetch all files from all manifests
  async function fetchAllFiles(): Promise<void> {
    loading.value = true
    error.value = null

    try {
      // Fetch all manifests in parallel (but uses cache if available)
      const [colab, jupyter, rag, latex, mdbook, apiDocs, json, vue] = await Promise.all([
        fetchManifest('colab'),
        fetchManifest('jupyter'),
        fetchManifest('llm-rag'),
        fetchManifest('latex'),
        fetchManifest('mdbook'),
        fetchManifest('api-docs'),
        fetchManifest('json'),
        fetchManifest('vue')
      ])

      colabFiles.value = colab
      jupyterFiles.value = jupyter
      ragFiles.value = rag
      latexFiles.value = latex
      mdbookFiles.value = mdbook
      apiDocsFiles.value = apiDocs
      jsonFiles.value = json
      vueFiles.value = vue

    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Unknown error'
      console.error('Error fetching files:', err)
    } finally {
      loading.value = false
    }
  }

  // Get files by output type
  function getFilesByType(outputType: string): FileItem[] {
    const typeMap: Record<string, FileItem[]> = {
      'colab': colabFiles.value,
      'jupyter': jupyterFiles.value,
      'llm-rag': ragFiles.value,
      'latex': latexFiles.value,
      'mdbook': mdbookFiles.value,
      'api-docs': apiDocsFiles.value,
      'json': jsonFiles.value,
      'vue': vueFiles.value
    }
    return typeMap[outputType] || []
  }

  // Get files by category across all types
  function getFilesByCategory(category: string): Record<string, FileItem[]> {
    return {
      colab: colabFiles.value.filter(f => f.category === category),
      jupyter: jupyterFiles.value.filter(f => f.category === category),
      'llm-rag': ragFiles.value.filter(f => f.category === category),
      latex: latexFiles.value.filter(f => f.category === category),
      mdbook: mdbookFiles.value.filter(f => f.category === category),
      'api-docs': apiDocsFiles.value.filter(f => f.category === category),
      json: jsonFiles.value.filter(f => f.category === category),
      vue: vueFiles.value.filter(f => f.category === category)
    }
  }

  // Get all unique categories
  const categories = computed(() => {
    const allCategories = new Set<string>()
    colabFiles.value.forEach(f => allCategories.add(f.category))
    jupyterFiles.value.forEach(f => allCategories.add(f.category))
    ragFiles.value.forEach(f => allCategories.add(f.category))
    latexFiles.value.forEach(f => allCategories.add(f.category))
    mdbookFiles.value.forEach(f => allCategories.add(f.category))
    apiDocsFiles.value.forEach(f => allCategories.add(f.category))
    jsonFiles.value.forEach(f => allCategories.add(f.category))
    vueFiles.value.forEach(f => allCategories.add(f.category))
    return Array.from(allCategories).sort()
  })

  // Refresh all data
  async function refresh(): Promise<void> {
    // Clear cache
    globalCache.manifests.clear()
    await fetchAllFiles()
  }

  return {
    // File lists by type
    colabFiles,
    jupyterFiles,
    ragFiles,
    latexFiles,
    mdbookFiles,
    apiDocsFiles,
    jsonFiles,
    vueFiles,

    // Loading states
    loading,
    error,

    // Actions
    fetchAllFiles,
    refresh,
    getFilesByType,
    getFilesByCategory,

    // Computed
    categories,

    // Stats
    totalFiles: computed(() =>
      colabFiles.value.length +
      jupyterFiles.value.length +
      ragFiles.value.length +
      latexFiles.value.length +
      mdbookFiles.value.length +
      apiDocsFiles.value.length +
      jsonFiles.value.length +
      vueFiles.value.length
    ),

    // Output type stats
    fileCountByType: computed(() => ({
      colab: colabFiles.value.length,
      jupyter: jupyterFiles.value.length,
      'llm-rag': ragFiles.value.length,
      latex: latexFiles.value.length,
      mdbook: mdbookFiles.value.length,
      'api-docs': apiDocsFiles.value.length,
      json: jsonFiles.value.length,
      vue: vueFiles.value.length
    })),

    // Base paths (for external use)
    basePaths: BASE_PATHS,

    // GitHub config (for building URLs)
    githubConfig: GITHUB_CONFIG,

    // Build URL helpers
    buildColabUrl,
    buildGithubUrl
  }
}
