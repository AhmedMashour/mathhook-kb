<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Solving Equations</h1>
      <p class="description">Find solutions to equations symbolically and numerically.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Linear Equation:**
$$ax + b = 0 \implies x = -\frac{b}{a}$$

**Quadratic Formula:**
$$ax^2 + bx + c = 0 \implies x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$$

**Discriminant ($\Delta$):**
$$\Delta = b^2 - 4ac$$
- $\Delta > 0$: Two distinct real roots
- $\Delta = 0$: One repeated real root
- $\Delta < 0$: Two complex conjugate roots

**Matrix Equations (Noncommutative):**
- Left division: $AX = B \implies X = A^{-1}B$
- Right division: $XA = B \implies X = BA^{-1}$
- Note: $A^{-1}B \neq BA^{-1}$ for matrices!
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Linear Equations</h3>
        <p>Solve ax + b = 0</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Solve: 2x + 3 = 0
let eq1 = expr!(2 * x + 3);
let mut solver = MathSolver::new();
let sol1 = solver.solve(&eq1, &x);
// Result: x = -3/2

// Solve: 5x - 10 = 0
let eq2 = expr!(5 * x - 10);
let sol2 = solver.solve(&eq2, &x);
// Result: x = 2
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, solve

x = symbol('x')

# Solve: 2x + 3 = 0
eq1 = 2*x + 3
sol1 = solve(eq1, x)
# Result: x = -3/2

# Solve: 5x - 10 = 0
eq2 = 5*x - 10
sol2 = solve(eq2, x)
# Result: x = 2
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, solve } = require('mathhook');

const x = symbol('x');

// Solve: 2x + 3 = 0
const eq1 = x.mul(2).add(3);
const sol1 = solve(eq1, x);
// Result: x = -3/2
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Quadratic Equations</h3>
        <p>Solve ax² + bx + c = 0</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Solve: x² - 5x + 6 = 0
let eq1 = expr!(x ^ 2 - 5 * x + 6);
let mut solver = MathSolver::new();
let solutions = solver.solve(&eq1, &x);
// Result: [x = 2, x = 3]

// Solve: x² - 4 = 0 (difference of squares)
let eq2 = expr!(x ^ 2 - 4);
let sol2 = solver.solve(&eq2, &x);
// Result: [x = -2, x = 2]
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, solve

x = symbol('x')

# Solve: x² - 5x + 6 = 0
eq1 = x**2 - 5*x + 6
solutions = solve(eq1, x)
# Result: [2, 3]

# Solve: x² - 4 = 0
eq2 = x**2 - 4
sol2 = solve(eq2, x)
# Result: [-2, 2]
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, solve } = require('mathhook');

const x = symbol('x');

// Solve: x² - 5x + 6 = 0
const eq1 = x.pow(2).sub(x.mul(5)).add(6);
const solutions = solve(eq1, x);
// Result: [2, 3]
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Complex Roots</h3>
        <p>When discriminant is negative</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Solve: x² + 1 = 0 (complex roots)
let equation = expr!(x ^ 2 + 1);
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x);
// Result: [x = i, x = -i]

// Solve: x² - 2x + 5 = 0
// Discriminant: 4 - 20 = -16 < 0 (complex roots)
let eq2 = expr!(x ^ 2 - 2 * x + 5);
let sol2 = solver.solve(&eq2, &x);
// Result: [x = 1 + 2i, x = 1 - 2i]
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, solve, I

x = symbol('x')

# Solve: x² + 1 = 0
equation = x**2 + 1
solutions = solve(equation, x)
# Result: [I, -I]

# Solve: x² - 2x + 5 = 0
eq2 = x**2 - 2*x + 5
sol2 = solve(eq2, x)
# Result: [1 + 2*I, 1 - 2*I]
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, solve } = require('mathhook');

const x = symbol('x');

// Solve: x² + 1 = 0
const equation = x.pow(2).add(1);
const solutions = solve(equation, x);
// Result: [i, -i]
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Transcendental Equations</h3>
        <p>Trigonometric, exponential, logarithmic</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Solve: sin(x) = 0
let eq1 = expr!(sin(x));
let mut solver = MathSolver::new();
let solutions = solver.solve(&eq1, &x);
// Result: [x = 0, x = π, x = 2π, ...] (infinitely many)

// Solve: e^x = 5
let eq2 = expr!(exp(x) - 5);
let sol2 = solver.solve(&eq2, &x);
// Result: x = ln(5)

// Solve: log(x) = 2
let eq3 = expr!(log(x) - 2);
let sol3 = solver.solve(&eq3, &x);
// Result: x = e² (if natural log)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, solve, sin, exp, log

x = symbol('x')

# Solve: sin(x) = 0
eq1 = sin(x)
solutions = solve(eq1, x)
# Result: [0, π, 2π, ...]

# Solve: e^x = 5
eq2 = exp(x) - 5
sol2 = solve(eq2, x)
# Result: log(5)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, solve, parse } = require('mathhook');

const x = symbol('x');

// Solve: sin(x) = 0
const eq1 = parse('sin(x)');
const solutions = solve(eq1, x);
// Result: [0, π, 2π, ...]
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Matrix Equations (Noncommutative)</h3>
        <p>Left and right division for matrices</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Matrix symbols
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Left division: A*X = B → X = A⁻¹*B
let left_eq = expr!(A * X - B);
let mut solver = MathSolver::new();
let solution_left = solver.solve(&left_eq, &X);
// Result: X = A⁻¹*B

// Right division: X*A = B → X = B*A⁻¹
let right_eq = expr!(X * A - B);
let solution_right = solver.solve(&right_eq, &X);
// Result: X = B*A⁻¹

// Note: A⁻¹*B ≠ B*A⁻¹ for matrices!
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, solve

# Matrix symbols
A = symbol('A', matrix=True)
X = symbol('X', matrix=True)
B = symbol('B', matrix=True)

# Left division: A*X = B → X = A⁻¹*B
left_eq = A*X - B
solution_left = solve(left_eq, X)
# Result: X = A^(-1)*B
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, solve } = require('mathhook');

// Matrix symbols
const A = symbol('A', { type: 'matrix' });
const X = symbol('X', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });

// Left division: A*X = B → X = A⁻¹*B
const leftEq = A.mul(X).sub(B);
const solutionLeft = solve(leftEq, X);
// Result: X = A^(-1)*B
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
const metaDescription = "Find solutions to equations symbolically and numerically.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Solving Equations',
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
