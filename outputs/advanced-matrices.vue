<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Matrix Operations</h1>
      <p class="description">Work with matrices symbolically and numerically in MathHook, with full support
for noncommutative algebra where order matters. Create matrices, perform
operations, and solve matrix equations.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">Matrix multiplication: For $A_{m \times n}$ and $B_{n \times p}$:
$$C_{ij} = \sum_{k=1}^{n} A_{ik} B_{kj}$$

Matrix inverse: $A \times A^{-1} = A^{-1} \times A = I$

Determinant (2×2): $$\det\begin{pmatrix} a & b \\ c & d \end{pmatrix} = ad - bc$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Creating Matrices</h3>
        <p>Create matrix symbols and numeric matrices</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Matrix symbols (noncommutative)
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Numeric 2×2 matrix
let M = Expression::matrix(vec![
    vec![expr!(1), expr!(2)],
    vec![expr!(3), expr!(4)],
]);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Matrix symbols
A = MatrixSymbol('A', n, m)
B = MatrixSymbol('B', m, p)

# Numeric matrix
M = Matrix([[1, 2], [3, 4]])
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Matrix symbols
const A = symbol('A', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

// Numeric matrix
const M = matrix([[1, 2], [3, 4]]);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Matrix Multiplication (Noncommutative)</h3>
        <p>A*B ≠ B*A in general</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

let AB = expr!(A * B);  // A*B
let BA = expr!(B * A);  // B*A ≠ A*B
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">A = MatrixSymbol('A', n, n)
B = MatrixSymbol('B', n, n)

AB = A * B  # Matrix product
BA = B * A  # Different result!
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const A = symbol('A', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

const AB = A.mul(B);  // A*B
const BA = B.mul(A);  // B*A ≠ A*B
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Solving Linear System Ax=b</h3>
        <p>Solve matrix equation using inverse</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let A = Expression::matrix(vec![
    vec![expr!(2), expr!(1)],
    vec![expr!(1), expr!(-1)],
]);
let b = Expression::matrix(vec![
    vec![expr!(5)],
    vec![expr!(1)],
]);

// Solution: x = A^(-1)*b
let x = expr!(A^(-1) * b);
// Result: [[2], [1]]
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">A = Matrix([[2, 1], [1, -1]])
b = Matrix([[5], [1]])

# Solution
x = A.inv() * b
# Result: Matrix([[2], [1]])
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const A = matrix([[2, 1], [1, -1]]);
const b = matrix([[5], [1]]);

// Solution
const x = A.inv().mul(b);
// Result: [[2], [1]]
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Matrix Equation A*X=B (Left Division)</h3>
        <p>Solve for matrix unknown X</p>
        
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

let equation = expr!((A * X) - B);
let solution = solver.solve(&equation, &X);
// Returns: X = A^(-1)*B (left division)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">A, X, B = symbols('A X B', matrix=True)
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
      
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const activeTab = ref('python')

// SEO metadata
const metaDescription = "Work with matrices symbolically and numerically in MathHook, with full support
for noncommutative algebra where order matters. Create matrices, perform
operations, and solve matrix equations.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Matrix Operations',
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
