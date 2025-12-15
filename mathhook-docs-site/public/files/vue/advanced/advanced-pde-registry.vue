<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>PDE Solver Registry System</h1>
      <p class="description">MathHook uses a registry-based dispatch system for PDE solvers, eliminating hardcoded match
statements and enabling O(1) solver lookup. This architecture is inspired by the ODE module
registry and provides extensible, testable, and efficient solver selection.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">Registry-based dispatch uses a HashMap for O(1) lookup by PDE type:

$$\text{Registry}: \text{PdeType} \rightarrow \text{Vec}\langle\text{Arc}\langle\text{dyn PDESolver}\rangle\rangle$$

Priority-based selection from multiple solvers per type ensures optimal solver choice.
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Default Registry Usage</h3>
        <p>Create and use default registry with standard solvers</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let registry = PDESolverRegistry::new();

// Automatically classify and solve
let solution = registry.solve(&pde)?;
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">registry = PDESolverRegistry()

# Automatically classify and solve
solution = registry.solve(pde)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const registry = new PDESolverRegistry();

// Automatically classify and solve
const solution = registry.solve(pde);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Custom Poisson Solver</h3>
        <p>Register a custom solver for Poisson equation (non-homogeneous Laplace)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">struct PoissonEquationSolver {
    max_terms: usize,
}

impl PDESolver for PoissonEquationSolver {
    fn solve(&self, pde: &Pde) -> PDEResult {
        // Poisson solver logic here
        Ok(PDESolution::laplace(solution, eigenvalues, coefficients))
    }

    fn can_solve(&self, pde_type: PdeType) -> bool {
        matches!(pde_type, PdeType::Elliptic)
    }

    fn priority(&self) -> u8 {
        90  // Lower than Laplace solver (100)
    }

    fn name(&self) -> &'static str {
        "Poisson Equation Solver"
    }

    fn description(&self) -> &'static str {
        "Solves Poisson equation ∇²u = f with non-zero source term"
    }
}

// Register custom solver
let mut registry = PDESolverRegistry::new();
registry.register(
    PdeType::Elliptic,
    Arc::new(PoissonEquationSolver { max_terms: 10 }),
);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">class PoissonEquationSolver:
    def __init__(self, max_terms=10):
        self.max_terms = max_terms

    def solve(self, pde):
        # Poisson solver logic here
        return PDESolution.laplace(solution, eigenvalues, coefficients)

    def can_solve(self, pde_type):
        return pde_type == PdeType.Elliptic

    def priority(self):
        return 90

    def name(self):
        return "Poisson Equation Solver"

    def description(self):
        return "Solves Poisson equation ∇²u = f with non-zero source term"

# Register custom solver
registry = PDESolverRegistry()
registry.register(PdeType.Elliptic, PoissonEquationSolver(max_terms=10))
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">class PoissonEquationSolver {
    constructor(maxTerms = 10) {
        this.maxTerms = maxTerms;
    }

    solve(pde) {
        // Poisson solver logic here
        return PDESolution.laplace(solution, eigenvalues, coefficients);
    }

    canSolve(pdeType) {
        return pdeType === PdeType.Elliptic;
    }

    priority() {
        return 90;
    }

    name() {
        return "Poisson Equation Solver";
    }

    description() {
        return "Solves Poisson equation ∇²u = f with non-zero source term";
    }
}

// Register custom solver
const registry = new PDESolverRegistry();
registry.register(PdeType.Elliptic, new PoissonEquationSolver(10));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Solver Discovery</h3>
        <p>List available solvers and inspect registry contents</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let registry = PDESolverRegistry::new();

println!("Registered PDE types: {:?}", registry.registered_types());
// [Parabolic, Hyperbolic, Elliptic]

println!("Total solvers: {}", registry.solver_count());
// 3

// Get solver for specific type
if let Some(solver) = registry.get_solver(&PdeType::Parabolic) {
    println!("Heat solver: {}", solver.name());
    println!("Description: {}", solver.description());
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">registry = PDESolverRegistry()

print(f"Registered PDE types: {registry.registered_types()}")
# [Parabolic, Hyperbolic, Elliptic]

print(f"Total solvers: {registry.solver_count()}")
# 3

# Get solver for specific type
solver = registry.get_solver(PdeType.Parabolic)
if solver:
    print(f"Heat solver: {solver.name()}")
    print(f"Description: {solver.description()}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const registry = new PDESolverRegistry();

console.log(`Registered PDE types: ${registry.registeredTypes()}`);
// [Parabolic, Hyperbolic, Elliptic]

console.log(`Total solvers: ${registry.solverCount()}`);
// 3

// Get solver for specific type
const solver = registry.getSolver(PdeType.Parabolic);
if (solver) {
    console.log(`Heat solver: ${solver.name()}`);
    console.log(`Description: ${solver.description()}`);
}
</code></pre>
        </div>

        
      </div>
      
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const activeTab = ref('python')

// SEO metadata
const metaDescription = "MathHook uses a registry-based dispatch system for PDE solvers, eliminating hardcoded match
statements and enabling O(1) solver lookup. This architecture is inspired by the ODE module
registry and provides extensible, testable, and efficient solver selection.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'PDE Solver Registry System',
  description: metaDescription,
  keywords: keywords.join(', '),
  
})
</script>

<style scoped>
.doc-page {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}

.doc-header h1 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.description {
  font-size: 1.2rem;
  color: #666;
}

.math-definition {
  background: #f5f5f5;
  padding: 1.5rem;
  border-radius: 8px;
  margin: 2rem 0;
}

.example-card {
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 1.5rem;
  margin-bottom: 2rem;
}

.code-tabs {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.code-tabs button {
  padding: 0.5rem 1rem;
  border: none;
  background: #eee;
  cursor: pointer;
  border-radius: 4px;
}

.code-tabs button:hover {
  background: #ddd;
}

.code-block {
  background: #1e1e1e;
  color: #d4d4d4;
  padding: 1rem;
  border-radius: 4px;
  overflow-x: auto;
}

.output {
  margin-top: 1rem;
  padding: 1rem;
  background: #f9f9f9;
  border-left: 4px solid #42b883;
}
</style>
