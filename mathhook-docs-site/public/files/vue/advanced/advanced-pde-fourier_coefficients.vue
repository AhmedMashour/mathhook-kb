<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Fourier Coefficients: Why They're Symbolic</h1>
      <p class="description">Explanation of why Fourier coefficients in PDE solutions are returned as symbolic
expressions rather than numerical values. Covers the orthogonality principle,
symbolic integration requirements, and workarounds for computing coefficients manually.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">For any PDE solution via separation of variables:
$$u(x,t) = \sum_{n=1}^{\infty} A_n X_n(x) T_n(t)$$

Coefficients from initial conditions:
$$u(x,0) = f(x) = \sum_{n=1}^{\infty} A_n X_n(x)$$

**Orthogonality** gives:
$$A_n = \frac{\langle f, X_n \rangle}{\langle X_n, X_n \rangle} = \frac{\int_0^L f(x) X_n(x) \, dx}{\int_0^L X_n^2(x) \, dx}$$

**Heat Equation (Dirichlet BCs):**
$$X_n(x) = \sin\left(\frac{n\pi x}{L}\right)$$

$$A_n = \frac{2}{L} \int_0^L f(x) \sin\left(\frac{n\pi x}{L}\right) dx$$

**Constant Initial Condition** ($f(x) = c$):
$$A_n = \frac{2c}{n\pi} [1 - (-1)^n] = \begin{cases}
\frac{4c}{n\pi} & n \text{ odd} \\
0 & n \text{ even}
\end{cases}$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Manual Coefficient Computation for Constant Initial Condition</h3>
        <p>Computing Fourier coefficients manually for heat equation with constant initial temperature</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::pde::standard::heat::HeatEquationSolver;
use mathhook_core::{symbol, expr};

// Setup PDE, BCs, IC...
let result = solver.solve_heat_equation_1d(&pde, &alpha, &bcs, &ic)?;

// Coefficients are symbolic
println!("Symbolic: {:?}", result.coefficients);  // [A_1, A_2, A_3, ...]

// Manually compute for f(x) = 100 (constant)
let mut numerical_coeffs = Vec::new();
for n in 1..=10 {
    let a_n = if n % 2 == 1 {
        // Odd n: A_n = 400/(nπ)
        expr!(400.0 / ((n as f64) * std::f64::consts::PI))
    } else {
        // Even n: A_n = 0
        expr!(0)
    };
    numerical_coeffs.push(a_n);
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.pde.heat import HeatEquationSolver
from mathhook import symbol, expr
import math

# Setup PDE, BCs, IC...
result = solver.solve_heat_equation_1d(pde, alpha, bcs, ic)

# Coefficients are symbolic
print("Symbolic:", result.coefficients)  # [A_1, A_2, A_3, ...]

# Manually compute for f(x) = 100 (constant)
numerical_coeffs = []
for n in range(1, 11):
    if n % 2 == 1:
        # Odd n: A_n = 400/(nπ)
        a_n = expr(400.0 / (n * math.pi))
    else:
        # Even n: A_n = 0
        a_n = expr(0)
    numerical_coeffs.append(a_n)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { HeatEquationSolver } = require('mathhook/pde/heat');
const { symbol, expr } = require('mathhook');

// Setup PDE, BCs, IC...
const result = solver.solveHeatEquation1d(pde, alpha, bcs, ic);

// Coefficients are symbolic
console.log("Symbolic:", result.coefficients);  // [A_1, A_2, A_3, ...]

// Manually compute for f(x) = 100 (constant)
const numericalCoeffs = [];
for (let n = 1; n <= 10; n++) {
    let aN;
    if (n % 2 === 1) {
        // Odd n: A_n = 400/(nπ)
        aN = expr(400.0 / (n * Math.PI));
    } else {
        // Even n: A_n = 0
        aN = expr(0);
    }
    numericalCoeffs.push(aN);
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
const metaDescription = "Explanation of why Fourier coefficients in PDE solutions are returned as symbolic
expressions rather than numerical values. Covers the orthogonality principle,
symbolic integration requirements, and workarounds for computing coefficients manually.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Fourier Coefficients: Why They're Symbolic',
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
