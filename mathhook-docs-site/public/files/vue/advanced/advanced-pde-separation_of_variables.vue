<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Separation of Variables</h1>
      <p class="description">Separation of Variables is the fundamental technique for solving linear second-order partial differential equations (PDEs) with boundary conditions.
It transforms the PDE into multiple ordinary differential equations (ODEs) that can be solved independently.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Core Assumption**: For PDE with two independent variables $x$ and $t$:
$$u(x,t) = X(x) \cdot T(t)$$

For three variables (e.g., Laplace in 2D):
$$u(x,y) = X(x) \cdot Y(y)$$

**The Separation Process**:
1. Substitute product form into PDE
2. Separate variables (all $x$-terms on one side, all $t$-terms on other)
3. Both sides equal same constant (separation constant $\lambda$)
4. Solve resulting ODEs
5. Apply boundary conditions to find eigenvalues
6. Construct general solution as superposition
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Heat Equation with Separation of Variables</h3>
        <p>Demonstrates separation of variables for 1D heat diffusion in a rod.</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let registry = PDESolverRegistry::new();
let solution = registry.solve(&pde).unwrap();
// Automatically uses separation of variables if applicable

// The solver internally:
// 1. Assumes u(x,t) = X(x)*T(t)
// 2. Separates into spatial and temporal ODEs
// 3. Applies BCs to find eigenvalues
// 4. Constructs superposition solution
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import PDESolverRegistry

registry = PDESolverRegistry()
solution = registry.solve(pde)
# Automatically uses separation of variables if applicable
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { PDESolverRegistry } = require('mathhook');

const registry = new PDESolverRegistry();
const solution = registry.solve(pde);
// Automatically uses separation of variables if applicable
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
const metaDescription = "Separation of Variables is the fundamental technique for solving linear second-order partial differential equations (PDEs) with boundary conditions.
It transforms the PDE into multiple ordinary differential equations (ODEs) that can be solved independently.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Separation of Variables',
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
