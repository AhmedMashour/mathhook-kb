<template>
  <div class="min-h-screen bg-logic-navy-900 text-chalk overflow-hidden relative">
    <!-- Reusable animated background -->
    <HomeAnimatedBackground />

    <!-- Reusable navigation -->
    <HomeAppNavbar />

    <!-- Main Content -->
    <div class="max-w-6xl mx-auto px-4 py-10 pt-24 relative z-10">
      <!-- Header Section -->
      <OutputsOutputsHeader />

      <!-- Output Cards Grid -->
      <div class="grid md:grid-cols-2 gap-6 mb-12">
        <!-- mdBook -->
        <OutputsExpandableOutputCard
          title="Rust mdBook"
          subtitle="Searchable Rust documentation"
          description="Interactive book format with code examples in Rust, Python, and JavaScript. Perfect for Rust developers."
          link-url="/outputs/mdbook/"
          link-text="Open Book"
          color="rust"
          :animation-delay="100"
          :files="mdbookFiles"
        >
          <template #icon>
            <OutputsIconsBookIcon class="w-7 h-7 text-rust-core" />
          </template>
        </OutputsExpandableOutputCard>

        <!-- Jupyter Book -->
        <OutputsExpandableOutputCard
          title="Jupyter Book"
          subtitle="Interactive Python notebooks"
          description="Executable notebooks with live code cells. Download and run locally or open in Colab. Perfect for data scientists."
          link-url="/outputs/jupyter/"
          link-text="Browse Notebooks"
          color="amber"
          :animation-delay="200"
          :files="jupyterFiles"
        >
          <template #icon>
            <OutputsIconsNotebookIcon class="w-7 h-7 text-amber-500" />
          </template>
        </OutputsExpandableOutputCard>

        <!-- API Documentation -->
        <OutputsOutputCard
          title="API Playground"
          subtitle="Interactive OpenAPI docs"
          description="Try MathHook APIs directly in your browser with Swagger UI. Test requests and see responses in real-time."
          link-url="/outputs/api-docs/"
          link-text="Try API"
          color="green"
          :animation-delay="300"
        >
          <template #icon>
            <OutputsIconsTerminalIcon class="w-7 h-7 text-step-green" />
          </template>
        </OutputsOutputCard>

        <!-- LaTeX Documents -->
        <OutputsExpandableOutputCard
          title="LaTeX Documents"
          subtitle="Academic paper format"
          description="Professional LaTeX source files ready for compilation. Perfect for academic papers, offline reading, and printing."
          link-url="/outputs/latex/"
          link-text="Browse Files"
          color="violet"
          :animation-delay="400"
          :files="latexFiles"
        >
          <template #icon>
            <OutputsIconsDocumentIcon class="w-7 h-7 text-violet-500" />
          </template>
        </OutputsExpandableOutputCard>

        <!-- Google Colab -->
        <OutputsExpandableOutputCard
          title="Google Colab Notebooks"
          subtitle="Cloud executable notebooks"
          description="Colab-optimized notebooks with pre-configured dependencies. Open directly in Google Colab for zero-setup execution."
          link-url="/outputs/colab/"
          link-text="Browse Files"
          external-action-url="https://colab.research.google.com/github/AhmedMashour/mathhook-kb/tree/master/mathhook-docs-site/public/outputs/colab/"
          external-action-text="Open in Colab"
          color="amber"
          :animation-delay="500"
          :files="colabFiles"
        >
          <template #icon>
            <OutputsIconsPlayIcon class="w-7 h-7 text-amber-500" />
          </template>
        </OutputsExpandableOutputCard>

        <!-- LLM-RAG Chunks -->
        <OutputsExpandableOutputCard
          title="LLM-RAG Optimized"
          subtitle="For AI assistants & search"
          description="Documentation chunks optimized for vector databases and AI retrieval. Perfect for building MathHook-aware AI assistants."
          link-url="/outputs/llm-rag/"
          link-text="Browse Files"
          color="cyan"
          :animation-delay="700"
          :files="ragFiles"
        >
          <template #icon>
            <OutputsIconsComputerIcon class="w-7 h-7 text-solve-cyan" />
          </template>
        </OutputsExpandableOutputCard>
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
  colabLoading,
  colabError
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
