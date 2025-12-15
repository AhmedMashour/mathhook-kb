// File data composable for outputs file browser
// Updated: 2025-12-15T18:35:00Z

import { ref, computed, onMounted } from 'vue'

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

// Base paths for outputs (all served from /outputs/ in public folder)
const BASE_PATHS = {
  jupyter: '/outputs/jupyter',
  colab: '/outputs/colab',
  llmRag: '/outputs/llm-rag',
  latex: '/outputs/latex',
  mdbook: '/outputs/mdbook',
  apiDocs: '/outputs/api-docs',
  json: '/outputs/json',
  vue: '/outputs/vue'
}

// GitHub config for Colab URLs
const GITHUB_CONFIG = {
  user: 'AhmedMashour',
  repo: 'mathhook-kb',
  branch: 'master',
  colabBasePath: 'mathhook-docs-site/public/outputs/colab'
}

// File extension mappings
const EXTENSIONS = {
  jupyter: 'ipynb',
  colab: 'ipynb',
  llmRag: 'md',
  latex: 'tex',
  mdbook: 'md',
  apiDocs: 'json',
  json: 'json',
  vue: 'vue'
}

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

  // Build correct Colab URL for a file
  function buildColabUrl(category: string, filename: string): string {
    return `https://colab.research.google.com/github/${GITHUB_CONFIG.user}/${GITHUB_CONFIG.repo}/blob/${GITHUB_CONFIG.branch}/${GITHUB_CONFIG.colabBasePath}/${category}/${filename}`
  }

  // Build GitHub URL for a file
  function buildGithubUrl(basePath: string, category: string, filename: string): string {
    const repoPath = basePath.replace('/outputs/', 'mathhook-docs-site/public/outputs/')
    return `https://github.com/${GITHUB_CONFIG.user}/${GITHUB_CONFIG.repo}/blob/${GITHUB_CONFIG.branch}/${repoPath}/${category}/${filename}`
  }

  // Generic manifest fetcher
  async function fetchManifest(basePath: string, extension: string, isColab: boolean = false): Promise<FileItem[]> {
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

          // Add Colab URL for colab files
          if (isColab) {
            fileItem.colabUrl = item.colab_url || buildColabUrl(category, item.filename)
          }

          files.push(fileItem)
        }
      }

      return files
    } catch (err) {
      console.error(`Error fetching manifest from ${basePath}:`, err)
      return []
    }
  }

  // Fetch all files from all manifests
  async function fetchAllFiles(): Promise<void> {
    loading.value = true
    error.value = null

    try {
      // Fetch all manifests in parallel
      const [colab, jupyter, rag, latex, mdbook, apiDocs, json, vue] = await Promise.all([
        fetchManifest(BASE_PATHS.colab, EXTENSIONS.colab, true),
        fetchManifest(BASE_PATHS.jupyter, EXTENSIONS.jupyter),
        fetchManifest(BASE_PATHS.llmRag, EXTENSIONS.llmRag),
        fetchManifest(BASE_PATHS.latex, EXTENSIONS.latex),
        fetchManifest(BASE_PATHS.mdbook, EXTENSIONS.mdbook),
        fetchManifest(BASE_PATHS.apiDocs, EXTENSIONS.apiDocs),
        fetchManifest(BASE_PATHS.json, EXTENSIONS.json),
        fetchManifest(BASE_PATHS.vue, EXTENSIONS.vue)
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

  // Initialize on mount
  onMounted(() => {
    fetchAllFiles()
  })

  // Refresh all data
  async function refresh(): Promise<void> {
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
