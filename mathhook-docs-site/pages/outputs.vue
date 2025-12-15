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
        <OutputsOutputCard
          title="Rust mdBook"
          subtitle="Searchable Rust documentation"
          description="Interactive book format with code examples in Rust, Python, and JavaScript. Perfect for Rust developers."
          link-url="/outputs/mdbook/index.html"
          link-text="Open Book"
          color="rust"
          :animation-delay="100"
        >
          <template #icon>
            <OutputsIconsBookIcon class="w-7 h-7 text-rust-core" />
          </template>
        </OutputsOutputCard>

        <!-- Jupyter Book -->
        <OutputsOutputCard
          title="Jupyter Book"
          subtitle="Interactive Python notebooks"
          description="Executable notebooks with live code cells. Download and run locally or open in Colab. Perfect for data scientists."
          link-url="/outputs/jupyter-book/_build/html/index.html"
          link-text="Open Notebooks"
          color="amber"
          :animation-delay="200"
        >
          <template #icon>
            <OutputsIconsNotebookIcon class="w-7 h-7 text-amber-500" />
          </template>
        </OutputsOutputCard>

        <!-- API Documentation -->
        <OutputsOutputCard
          title="API Playground"
          subtitle="Interactive OpenAPI docs"
          description="Try MathHook APIs directly in your browser with Swagger UI. Test requests and see responses in real-time."
          link-url="/outputs/api-docs/index.html"
          link-text="Try API"
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
          link-url="/outputs/latex/"
          link-text="Browse LaTeX"
          color="violet"
          :animation-delay="400"
        >
          <template #icon>
            <OutputsIconsDocumentIcon class="w-7 h-7 text-violet-500" />
          </template>
        </OutputsOutputCard>

        <!-- Raw Jupyter Notebooks -->
        <OutputsOutputCard
          title="Jupyter Notebooks"
          subtitle="Downloadable .ipynb files"
          description="Raw Jupyter notebook files for local execution. Download and run with your own Python environment."
          link-url="/outputs/jupyter/"
          link-text="Browse Notebooks"
          color="amber"
          :animation-delay="500"
        >
          <template #icon>
            <OutputsIconsNotebookIcon class="w-7 h-7 text-amber-500" />
          </template>
        </OutputsOutputCard>

        <!-- Google Colab -->
        <OutputsOutputCard
          title="Google Colab Notebooks"
          subtitle="Cloud executable notebooks"
          description="Colab-optimized notebooks with pre-configured dependencies. Download and upload to Colab for zero-setup execution."
          link-url="/outputs/colab/"
          link-text="Browse Colab Files"
          color="amber"
          :animation-delay="600"
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
          link-url="/outputs/llm-rag/"
          link-text="Browse RAG Chunks"
          color="cyan"
          :animation-delay="700"
        >
          <template #icon>
            <OutputsIconsComputerIcon class="w-7 h-7 text-solve-cyan" />
          </template>
        </OutputsOutputCard>
      </div>

      <!-- Build Info Section -->
      <OutputsBuildInfoSection />
    </div>
  </div>
</template>

<script setup>
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
