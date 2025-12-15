<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>PDE Quick Reference Card</h1>
      <p class="description">One-page cheat sheet for Method of Characteristics covering standard forms,
solution templates, common patterns, shock formation, and troubleshooting guide.
Includes code templates, decision trees, and physical applications.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**General Quasi-Linear Form:**
$$a(x,y,u) \frac{\partial u}{\partial x} + b(x,y,u) \frac{\partial u}{\partial y} = c(x,y,u)$$

**Characteristic equations:**
$$\frac{dx}{ds} = a, \quad \frac{dy}{ds} = b, \quad \frac{du}{ds} = c$$

**Transport Equation:**
$$\frac{\partial u}{\partial t} + c \frac{\partial u}{\partial x} = 0$$
**Solution:** $u(x,t) = f(x - ct)$

**Burgers' Equation:**
$$\frac{\partial u}{\partial t} + u \frac{\partial u}{\partial x} = 0$$
**Solution:** $u = f(\xi)$ where $x = \xi + f(\xi)t$ (implicit)

**Shock speed (Rankine-Hugoniot):**
$$\frac{dx_{shock}}{dt} = \frac{[f(u)]}{[u]} = \frac{f(u_R) - f(u_L)}{u_R - u_L}$$

**Entropy condition (Lax):**
$$f'(u_L) > s > f'(u_R)$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Quick Code Template</h3>
        <p>Standard template for solving PDEs with method of characteristics</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Define symbols
let u = symbol!(u);
let t = symbol!(t);
let x = symbol!(x);

// Build PDE
let equation = expr!(u);
let pde = Pde::new(equation, u, vec![t, x]);

// Solve
let result = method_of_characteristics(&pde).unwrap();

// Apply IC and verify
let solution = expr!(f(x - c*t));  // Example for transport
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Define symbols
u = symbol('u')
t = symbol('t')
x = symbol('x')

# Build PDE
equation = expr(u)
pde = Pde.new(equation, u, [t, x])

# Solve
result = method_of_characteristics(pde)

# Apply IC and verify
solution = expr(f(x - c*t))  # Example for transport
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Define symbols
const u = symbol('u');
const t = symbol('t');
const x = symbol('x');

// Build PDE
const equation = expr(u);
const pde = Pde.new(equation, u, [t, x]);

// Solve
const result = methodOfCharacteristics(pde);

// Apply IC and verify
const solution = expr(f(x - c*t));  // Example for transport
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
const metaDescription = "One-page cheat sheet for Method of Characteristics covering standard forms,
solution templates, common patterns, shock formation, and troubleshooting guide.
Includes code templates, decision trees, and physical applications.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'PDE Quick Reference Card',
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
