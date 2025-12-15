<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Differential Equation Solving</h1>
      <p class="description">Solve ordinary differential equations (ODEs) and partial differential equations
(PDEs) symbolically in MathHook, with support for initial conditions, boundary
conditions, and various solution methods.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">Ordinary Differential Equation (ODE):
$$F\left(x, y, \frac{dy}{dx}, \frac{d^2y}{dx^2}, \ldots\right) = 0$$

Partial Differential Equation (PDE):
$$F\left(x, y, u, \frac{\partial u}{\partial x}, \frac{\partial u}{\partial y}, \ldots\right) = 0$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>First-Order ODE</h3>
        <p>Solve dy/dx = 2x with initial condition y(0) = 1</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
let y = Function::new("y", vec![x.clone()]);

let ode = expr!(diff(y, x) - 2*x);
let solver = ODESolver::new();

let solution = solver.solve(&ode, &y, Some((&x, expr!(0), expr!(1))));
// Result: y = x^2 + 1
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = symbol('x')
y = Function('y')(x)

ode = diff(y, x) - 2*x
solution = dsolve(ode, y, ics={y.subs(x, 0): 1})
# Result: y = x**2 + 1
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');
const y = func('y', [x]);

const ode = diff(y, x).sub(mul(2, x));
const solution = ode_solve(ode, y, {x0: 0, y0: 1});
// Result: y = x^2 + 1
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Second-Order Linear ODE</h3>
        <p>Solve y'' + y = 0 (simple harmonic oscillator)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
let y = Function::new("y", vec![x.clone()]);

let ode = expr!(diff(y, x, 2) + y);
let solution = ode_solver.solve(&ode, &y, None);
// Result: y = C1*cos(x) + C2*sin(x)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = symbol('x')
y = Function('y')(x)

ode = diff(y, x, 2) + y
solution = dsolve(ode, y)
# Result: y = C1*cos(x) + C2*sin(x)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');
const y = func('y', [x]);

const ode = diff(y, x, 2).add(y);
const solution = ode_solve(ode, y);
// Result: y = C1*cos(x) + C2*sin(x)
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
const metaDescription = "Solve ordinary differential equations (ODEs) and partial differential equations
(PDEs) symbolically in MathHook, with support for initial conditions, boundary
conditions, and various solution methods.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Differential Equation Solving',
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
