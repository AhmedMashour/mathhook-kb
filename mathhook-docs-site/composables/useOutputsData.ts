export interface OutputItem {
  id: string
  title: string
  subtitle: string
  description: string
  linkUrl: string
  linkText: string
  color: 'rust' | 'amber' | 'green' | 'violet' | 'cyan'
  iconType: 'book' | 'notebook' | 'terminal' | 'document' | 'play' | 'computer'
  hasCustomAction?: boolean
  externalActionUrl?: string
  externalActionText?: string
}

export function useOutputsData() {
  const outputs: OutputItem[] = [
    {
      id: 'mdbook',
      title: 'Rust mdBook',
      subtitle: 'Searchable Rust documentation',
      description: 'Interactive book format with code examples in Rust, Python, and JavaScript. Perfect for Rust developers.',
      linkUrl: '/outputs/mdbook/index.html',
      linkText: 'Open Book',
      color: 'rust',
      iconType: 'book'
    },
    {
      id: 'jupyter',
      title: 'Jupyter Book',
      subtitle: 'Interactive Python notebooks',
      description: 'Executable notebooks with live code cells. Download and run locally or open in Colab. Perfect for data scientists.',
      linkUrl: '/outputs/jupyter-book/_build/html/index.html',
      linkText: 'Open Notebooks',
      color: 'amber',
      iconType: 'notebook'
    },
    {
      id: 'api-docs',
      title: 'API Playground',
      subtitle: 'Interactive OpenAPI docs',
      description: 'Try MathHook APIs directly in your browser with Swagger UI. Test requests and see responses in real-time.',
      linkUrl: '/outputs/api-docs/index.html',
      linkText: 'Try API',
      color: 'green',
      iconType: 'terminal'
    },
    {
      id: 'latex',
      title: 'LaTeX/PDF Documents',
      subtitle: 'Academic paper format',
      description: 'Professional academic documentation in PDF format. Perfect for offline reading and printing.',
      linkUrl: '/outputs/latex/',
      linkText: 'Browse PDFs',
      color: 'violet',
      iconType: 'document'
    },
    {
      id: 'colab',
      title: 'Google Colab Notebooks',
      subtitle: 'Cloud executable notebooks',
      description: 'Run MathHook examples in Google Colab with zero setup. One-click execution in the cloud.',
      linkUrl: '',
      linkText: '',
      color: 'amber',
      iconType: 'play',
      hasCustomAction: true,
      externalActionUrl: 'https://colab.research.google.com/github/AhmedMashour/mathhook/blob/master/outputs/colab/',
      externalActionText: 'Open in Colab'
    },
    {
      id: 'llm-rag',
      title: 'LLM-RAG Optimized',
      subtitle: 'For AI assistants & search',
      description: 'Documentation chunks optimized for vector databases and AI retrieval. Perfect for building MathHook-aware AI assistants.',
      linkUrl: '',
      linkText: '',
      color: 'cyan',
      iconType: 'computer',
      hasCustomAction: true,
      externalActionUrl: '/outputs/llm-rag/',
      externalActionText: 'Browse RAG Chunks'
    }
  ]

  return {
    outputs
  }
}
