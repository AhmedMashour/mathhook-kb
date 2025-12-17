// Home page data composable
export const useHomeData = () => {
  // Tagline words for staggered reveal animation
  const taglineWords = [
    { text: 'Symbolic', color: 'text-rust-core' },
    { text: 'Power.', color: 'text-white' },
    { text: 'Educational', color: 'text-solve-cyan' },
    { text: 'Clarity.', color: 'text-white' },
    { text: 'Native', color: 'text-step-green' },
    { text: 'Speed.', color: 'text-white' }
  ]

  // Features data
  const features = [
    { title: 'Step-by-Step Learning', description: 'Educational explanations showing every rule applied', icon: '🎓', color: 'step-green' },
    { title: 'Symbolic Calculus', description: 'Derivatives, integrals, limits with product/chain rules', icon: '∫', color: 'rust-core' },
    { title: 'PDEs & ODEs', description: 'Heat, wave, Laplace equations with Fourier series', icon: '∂', color: 'solve-cyan' },
    { title: 'Multiple Input Formats', description: 'Parse LaTeX, Wolfram Language, and standard notation', icon: '📝', color: 'amber-400' },
    { title: 'Equation Solving', description: 'Linear, quadratic, polynomial, and systems of equations', icon: '⚖️', color: 'violet-400' },
    { title: 'High Performance', description: 'Rust-based core with SIMD optimizations', icon: '⚡', color: 'rust-core' },
    { title: 'Language Bindings', description: 'Native support for Python and Node.js', icon: '🔗', color: 'solve-cyan' },
    { title: 'Production Ready', description: 'Zero-copy parsing, 32B expressions, thread-safe', icon: '🚀', color: 'step-green' }
  ]

  // Benchmarks data
  const benchmarks = [
    { name: 'Elementary integration (cos, exp)', time: '< 300 ns', color: 'step-green' },
    { name: 'Simple derivatives', time: '< 2 μs', color: 'solve-cyan' },
    { name: 'Polynomial simplification (deg 50)', time: '< 10 μs', color: 'amber-400' },
    { name: 'Complex calculus (chain + product)', time: '< 500 μs', color: 'rust-core' }
  ]

  // Performance reasons
  const performanceReasons = [
    { title: '32-byte expressions', description: 'Two fit per CPU cache line', icon: '32B', color: 'rust-core' },
    { title: 'Zero-copy parsing', description: 'Direct AST construction without allocations', icon: '0', color: 'solve-cyan' },
    { title: 'SIMD operations', description: 'Vectorized arithmetic for bulk operations', icon: '⚡', color: 'step-green' },
    { title: 'No interpreter overhead', description: 'Native Rust, no garbage collector', icon: '🦀', color: 'amber-400' },
    { title: 'Thread-safe', description: 'Immutable expressions, lock-free operations', icon: '🔒', color: 'violet-400' }
  ]

  // Use cases
  const useCases = [
    { title: 'Machine Learning', description: 'Symbolic gradients for neural networks and optimization', color: 'rust-core', iconPath: 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z' },
    { title: 'Education', description: 'Step-by-step solutions for teaching and learning', color: 'step-green', iconPath: 'M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253' },
    { title: 'Research', description: 'High-performance symbolic computation for science', color: 'solve-cyan', iconPath: 'M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z' },
    { title: 'Development', description: 'Embed math in your applications with native speed', color: 'violet-400', iconPath: 'M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4' }
  ]

  // Install commands
  const installCommands = [
    { lang: 'Rust', emoji: '🦀', command: 'cargo add mathhook', color: 'rust-core' },
    { lang: 'Python', emoji: '🐍', command: 'pip install mathhook', color: 'amber-400' },
    { lang: 'Node.js', emoji: '📜', command: 'npm install mathhook-node', color: 'step-green' }
  ]

  // Ecosystem badges
  const ecosystemBadges = [
    { name: 'Crates.io', url: 'https://crates.io/crates/mathhook', color: 'rust-core', emoji: '🦀' },
    { name: 'PyPI', url: 'https://pypi.org/project/mathhook', color: 'amber-400', emoji: '🐍' },
    { name: 'npm', url: 'https://www.npmjs.com/package/mathhook-node', color: 'step-green', emoji: '📦' },
    { name: 'GitHub', url: 'https://github.com/AhmedMashour/mathhook', color: 'chalk', emoji: '⭐' },
    { name: 'Docs', url: '/docs', color: 'solve-cyan', emoji: '📚' }
  ]

  // Category configuration
  const categoryConfig: Record<string, {
    title: string
    icon: string
    color: string
    description: string
    order: number
    prefixes: string[]
  }> = {
    'calculus': { title: 'Calculus', icon: '∫', color: 'rust-core', description: 'Derivatives, Integrals, Limits, Series', order: 1, prefixes: ['operations-differentiation', 'operations-integration', 'operations-limits', 'operations-series'] },
    'ode': { title: 'ODEs', icon: 'dy/dx', color: 'solve-cyan', description: 'Separable, Linear, Bernoulli, Exact', order: 2, prefixes: ['ode-'] },
    'pde': { title: 'PDEs', icon: '∂²u', color: 'step-green', description: 'Heat, Wave, Laplace, Poisson', order: 3, prefixes: ['pde-', 'advanced-pde-'] },
    'algebra': { title: 'Algebra', icon: 'Σ', color: 'amber-400', description: 'Simplify, Factor, Expand, Solve', order: 4, prefixes: ['operations-simplification', 'operations-expansion', 'operations-solving', 'operations-substitution'] },
    'polynomial': { title: 'Polynomials', icon: 'xⁿ', color: 'violet-400', description: 'GCD, Division, Gröbner Bases', order: 5, prefixes: ['polynomial-'] },
    'advanced': { title: 'Advanced Topics', icon: '🔬', color: 'pink-400', description: 'Complex Numbers, Matrices, Special Functions', order: 6, prefixes: ['advanced-'] },
    'educational': { title: 'Educational', icon: '🎓', color: 'cyan-400', description: 'Step-by-Step, Messages, API', order: 7, prefixes: ['educational-'] },
    'getting-started': { title: 'Getting Started', icon: '🚀', color: 'orange-400', description: 'Installation, Quick Start, Patterns', order: 0, prefixes: ['getting-started-'] }
  }

  return {
    taglineWords,
    features,
    benchmarks,
    performanceReasons,
    useCases,
    installCommands,
    ecosystemBadges,
    categoryConfig
  }
}
