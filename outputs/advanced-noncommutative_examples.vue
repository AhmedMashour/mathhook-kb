<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Noncommutative Algebra Examples</h1>
      <p class="description">Comprehensive examples of noncommutative algebra in MathHook covering
quantum mechanics, matrix algebra, and quaternion rotations.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Quantum Commutator</h3>
        <p>Position-momentum canonical commutation relation [x,p] = iℏ</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x; operator);
let p = symbol!(p; operator);

// Commutator: [x, p] = xp - px
let xp = expr!(x * p);
let px = expr!(p * x);
let commutator = expr!(xp - px);

// LaTeX output
let latex = commutator.to_latex(None).unwrap();
// Output: \hat{x}\hat{p} - \hat{p}\hat{x}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy.physics.quantum import Operator, Commutator

x = Operator('x')
p = Operator('p')

# Commutator
comm = Commutator(x, p)
# Result: I*hbar (in quantum mechanics)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x', {type: 'operator'});
const p = symbol('p', {type: 'operator'});

// Commutator
const xp = x.mul(p);
const px = p.mul(x);
const comm = xp.sub(px);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Matrix Equation Left Division</h3>
        <p>Solve A*X = B using left division X = A^(-1)*B</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let solver = MatrixEquationSolver::new();
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Equation: A*X - B = 0
let equation = expr!((A * X) - B);

let result = solver.solve(&equation, &X);
// Returns: X = A^(-1)*B (left division)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import symbols, MatrixSymbol, solve, Eq

A = MatrixSymbol('A', n, n)
X = MatrixSymbol('X', n, n)
B = MatrixSymbol('B', n, n)

# Solve A*X = B
equation = Eq(A*X, B)
solution = solve(equation, X)
# Returns: X = A^(-1)*B
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const A = symbol('A', {type: 'matrix'});
const X = symbol('X', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

const equation = A.mul(X).sub(B);
const solution = solve(equation, X);
// Returns: X = A.inv().mul(B)
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Quaternion Multiplication</h3>
        <p>Noncommutative quaternion basis multiplication i*j = k, j*i = -k</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);

// i*j = k
let ij = expr!(i * j);

// j*i = -k (different!)
let ji = expr!(j * i);

// Order matters
assert_ne!(ij.to_string(), ji.to_string());
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy.algebras.quaternion import Quaternion

i = Quaternion(0, 1, 0, 0)
j = Quaternion(0, 0, 1, 0)
k = Quaternion(0, 0, 0, 1)

# Verify: i*j = k
assert i * j == k

# Verify: j*i = -k (noncommutative!)
assert j * i == -k
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const i = symbol('i', {type: 'quaternion'});
const j = symbol('j', {type: 'quaternion'});

const ij = i.mul(j);  // k
const ji = j.mul(i);  // -k
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
const metaDescription = "Comprehensive examples of noncommutative algebra in MathHook covering
quantum mechanics, matrix algebra, and quaternion rotations.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Noncommutative Algebra Examples',
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
