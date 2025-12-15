<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Partial Differential Equations (PDEs)</h1>
      <p class="description">Comprehensive overview of partial differential equations in MathHook CAS.
Covers mathematical foundations, classification, solution methods, and current capabilities.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">A second-order linear PDE in two independent variables has the general form:

$$A \frac{\partial^2 u}{\partial x^2} + B \frac{\partial^2 u}{\partial x \partial y} + C \frac{\partial^2 u}{\partial y^2} + D \frac{\partial u}{\partial x} + E \frac{\partial u}{\partial y} + Fu = G$$

where:
- $u(x,y)$ is the unknown function
- $A, B, C, D, E, F, G$ are coefficients (may depend on $x$, $y$, or $u$)
- $x, y$ are independent variables (typically spatial coordinates or time)
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Registry-Based Solver Dispatch</h3>
        <p>Automatic PDE classification and solver selection using O(1) registry lookup</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Create registry (auto-registers all solvers)
let registry = PDESolverRegistry::new();

// Define PDE
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);
let equation = expr!(add: x, t);  // Heat equation pattern
let pde = Pde::new(equation, u, vec![x, t]);

// Automatic classification and solving
let solution = registry.solve(&pde)?;

println!("Solution: {}", solution.solution);
println!("Eigenvalues: {:?}", solution.get_eigenvalues());
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, Pde, PDESolverRegistry

# Create registry
registry = PDESolverRegistry()

# Define PDE
u = symbol('u')
x = symbol('x')
t = symbol('t')
equation = expr(x + t)  # Heat equation pattern
pde = Pde(equation, u, [x, t])

# Automatic solving
solution = registry.solve(pde)

print(f"Solution: {solution.solution}")
print(f"Eigenvalues: {solution.get_eigenvalues()}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, Pde, PDESolverRegistry } = require('mathhook');

// Create registry
const registry = new PDESolverRegistry();

// Define PDE
const u = symbol('u');
const x = symbol('x');
const t = symbol('t');
const equation = expr(x + t);  // Heat equation pattern
const pde = new Pde(equation, u, [x, t]);

// Automatic solving
const solution = registry.solve(pde);

console.log(`Solution: ${solution.solution}`);
console.log(`Eigenvalues: ${solution.getEigenvalues()}`);
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
const metaDescription = "Comprehensive overview of partial differential equations in MathHook CAS.
Covers mathematical foundations, classification, solution methods, and current capabilities.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Partial Differential Equations (PDEs)',
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
