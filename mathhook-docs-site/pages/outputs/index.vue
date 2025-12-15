<template>
  <div class="min-h-screen bg-logic-navy-900 text-chalk overflow-hidden relative">
    <!-- Reusable animated background -->
    <HomeAnimatedBackground />

    <!-- Reusable navigation -->
    <HomeAppNavbar />

    <!-- Main Content -->
    <div class="max-w-6xl mx-auto px-4 py-10 pt-24 relative z-10">
      <!-- Header Section -->
      <OutputsHeader />

      <!-- Output Cards Grid -->
      <div class="grid md:grid-cols-2 gap-6 mb-12">
        <!-- mdBook -->
        <OutputsOutputCard
          title="Rust mdBook"
          subtitle="Searchable Rust documentation"
          description="Interactive book format with code examples in Rust, Python, and JavaScript. Perfect for Rust developers."
          link-url="/outputs/mdbook"
          link-text="Browse Files"
          :file-count="mdbookFiles.length"
          color="rust"
          :animation-delay="100"
        >
          <template #icon>
            <OutputsIconsBookIcon class="w-7 h-7 text-rust-core" />
          </template>
        </OutputsOutputCard>

        <!-- Jupyter Book -->
        <OutputsOutputCard
          title="Jupyter Notebooks"
          subtitle="Interactive Python notebooks"
          description="Executable notebooks with live code cells. Download and run locally or open in Colab. Perfect for data scientists."
          link-url="/outputs/jupyter"
          link-text="Browse Notebooks"
          :file-count="jupyterFiles.length"
          color="amber"
          :animation-delay="200"
        >
          <template #icon>
            <OutputsIconsNotebookIcon class="w-7 h-7 text-amber-500" />
          </template>
        </OutputsOutputCard>

        <!-- API Documentation -->
        <OutputsOutputCard
          title="API Documentation"
          subtitle="OpenAPI specifications"
          description="Machine-readable API documentation in OpenAPI format. Perfect for integration and tooling."
          link-url="/outputs/api-docs"
          link-text="Browse API Docs"
          :file-count="apiDocsFiles.length"
          color="green"
          :animation-delay="300"
        >
          <template #icon>
            <OutputsIconsTerminalIcon class="w-7 h-7 text-step-green" />
          </template>
        </OutputsOutputCard>

        <!-- LaTeX Documents -->
        <OutputsOutputCard
          title="LaTeX Documents"
          subtitle="Academic paper format"
          description="Professional LaTeX source files ready for compilation. Perfect for academic papers, offline reading, and printing."
          link-url="/outputs/latex"
          link-text="Browse Files"
          :file-count="latexFiles.length"
          color="violet"
          :animation-delay="400"
        >
          <template #icon>
            <OutputsIconsDocumentIcon class="w-7 h-7 text-violet-500" />
          </template>
        </OutputsOutputCard>

        <!-- Google Colab -->
        <OutputsOutputCard
          title="Google Colab Notebooks"
          subtitle="Cloud executable notebooks"
          description="Colab-optimized notebooks with pre-configured dependencies. Open directly in Google Colab for zero-setup execution."
          link-url="/outputs/colab"
          link-text="Browse Files"
          :file-count="colabFiles.length"
          color="amber"
          :animation-delay="500"
        >
          <template #icon>
            <OutputsIconsPlayIcon class="w-7 h-7 text-amber-500" />
          </template>
        </OutputsOutputCard>

        <!-- LLM-RAG Chunks -->
        <OutputsOutputCard
          title="LLM-RAG Optimized"
          subtitle="For AI assistants & search"
          description="Documentation chunks optimized for vector databases and AI retrieval. Perfect for building MathHook-aware AI assistants."
          link-url="/outputs/llm-rag"
          link-text="Browse Files"
          :file-count="ragFiles.length"
          color="cyan"
          :animation-delay="600"
        >
          <template #icon>
            <OutputsIconsComputerIcon class="w-7 h-7 text-solve-cyan" />
          </template>
        </OutputsOutputCard>

        <!-- JSON Data -->
        <OutputsOutputCard
          title="JSON Data"
          subtitle="Structured data format"
          description="Raw JSON documentation data for programmatic access and custom integrations."
          link-url="/outputs/json"
          link-text="Browse Files"
          :file-count="jsonFiles.length"
          color="cyan"
          :animation-delay="700"
        >
          <template #icon>
            <OutputsIconsTerminalIcon class="w-7 h-7 text-solve-cyan" />
          </template>
        </OutputsOutputCard>

        <!-- Vue Components -->
        <OutputsOutputCard
          title="Vue Components"
          subtitle="Interactive UI components"
          description="Vue.js components for building interactive documentation interfaces and dashboards."
          link-url="/outputs/vue"
          link-text="Browse Files"
          :file-count="vueFiles.length"
          color="green"
          :animation-delay="800"
        >
          <template #icon>
            <OutputsIconsComputerIcon class="w-7 h-7 text-step-green" />
          </template>
        </OutputsOutputCard>
      </div>

      <!-- Build Info Section -->
      <OutputsBuildInfoSection />
    </div>
  </div>
</template>

<script setup>
import { useFileData } from '~/composables/useFileData'

// Get file data from composable
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
  error
} = useFileData()

useHead({
  title: 'Documentation Outputs - MathHook',
  meta: [
    {
      name: 'description',
      content: 'Access MathHook documentation in multiple formats: Rust mdBook, Jupyter notebooks, API playground, LaTeX PDFs, and more. Symbolic Power. Educational Clarity. Native Speed.'
    }
  ]
})
</script>
