<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>System of Equations Solving</h1>
      <p class="description">Solve systems of equations (linear and nonlinear) with multiple unknowns
using substitution, elimination, matrix methods, and Newton's method for
nonlinear systems.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">Linear system matrix form: $$Ax = b$$
where $A$ is coefficient matrix, $x$ is unknown vector, $b$ is constant vector

Solution (unique): $$x = A^{-1}b$$ when $\det(A) \neq 0$

Least squares (overdetermined): $$x_{LS} = (A^T A)^{-1} A^T b$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Linear System (2×2)</h3>
        <p>Solve { 2x + y = 5, x - y = 1 }</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
let y = symbol!(y);

// Method 1: Equations as list
let solver = SystemSolver::new();
let equations = vec![
    expr!(2*x + y - 5),
    expr!(x - y - 1),
];
let vars = vec![x.clone(), y.clone()];

let solution = solver.solve_system(&equations, &vars);
// Result: { x = 2, y = 1 }

// Method 2: Matrix form Ax = b
let A = Expression::matrix(vec![
    vec![expr!(2), expr!(1)],
    vec![expr!(1), expr!(-1)],
]);
let b = Expression::matrix(vec![
    vec![expr!(5)],
    vec![expr!(1)],
]);

let solution_matrix = expr!(A^(-1) * b);
// Result: [[2], [1]]
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import symbols, solve, Matrix

x, y = symbols('x y')

# Method 1: Equations
equations = [2*x + y - 5, x - y - 1]
solution = solve(equations, [x, y])
# Result: {x: 2, y: 1}

# Method 2: Matrix form
A = Matrix([[2, 1], [1, -1]])
b = Matrix([[5], [1]])
solution_matrix = A.inv() * b
# Result: Matrix([[2], [1]])
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');
const y = symbol('y');

// Equations
const equations = [
    sub(add(mul(2, x), y), 5),
    sub(sub(x, y), 1)
];

const solution = solve(equations, [x, y]);
// Result: {x: 2, y: 1}

// Matrix form
const A = matrix([[2, 1], [1, -1]]);
const b = matrix([[5], [1]]);
const sol = A.inv().mul(b);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Nonlinear System</h3>
        <p>Solve { x^2 + y^2 = 25, x + y = 5 }</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
let y = symbol!(y);

// Step 1: Solve linear for y: y = 5 - x
// Step 2: Substitute into nonlinear
let substituted = expr!(x^2 + (5 - x)^2 - 25);
// Simplifies to: 2x^2 - 10x = 0 → x(x - 5) = 0

// Solutions: x = 0 or x = 5
// Corresponding y values: y = 5 or y = 0
// Two solutions: (0, 5) and (5, 0)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import symbols, solve

x, y = symbols('x y')

equations = [x**2 + y**2 - 25, x + y - 5]
solutions = solve(equations, [x, y])
# Result: [(0, 5), (5, 0)]
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');
const y = symbol('y');

const equations = [
    sub(add(pow(x, 2), pow(y, 2)), 25),
    sub(add(x, y), 5)
];

const solutions = solve(equations, [x, y]);
// Result: [[0, 5], [5, 0]]
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Three Variables</h3>
        <p>Solve { x + y + z = 6, 2x - y + z = 3, x + 2y - z = 2 }</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
let y = symbol!(y);
let z = symbol!(z);

// Matrix form
let A = Expression::matrix(vec![
    vec![expr!(1), expr!(1), expr!(1)],
    vec![expr!(2), expr!(-1), expr!(1)],
    vec![expr!(1), expr!(2), expr!(-1)],
]);

let b = Expression::matrix(vec![
    vec![expr!(6)],
    vec![expr!(3)],
    vec![expr!(2)],
]);

let solution = expr!(A^(-1) * b);
// Result: x = 1, y = 2, z = 3
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import symbols, solve, Matrix

x, y, z = symbols('x y z')

A = Matrix([[1, 1, 1], [2, -1, 1], [1, 2, -1]])
b = Matrix([[6], [3], [2]])

solution = A.inv() * b
# Result: Matrix([[1], [2], [3]])
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const [x, y, z] = symbols(['x', 'y', 'z']);

const A = matrix([[1, 1, 1], [2, -1, 1], [1, 2, -1]]);
const b = matrix([[6], [3], [2]]);

const solution = A.inv().mul(b);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Overdetermined System (Least Squares)</h3>
        <p>More equations than unknowns: find best approximate solution</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// System: { x + y = 1, 2x + 2y = 3, x - y = 0 }
// Inconsistent! Use least squares.

let A = Expression::matrix(vec![
    vec![expr!(1), expr!(1)],
    vec![expr!(2), expr!(2)],
    vec![expr!(1), expr!(-1)],
]);

let b = Expression::matrix(vec![
    vec![expr!(1)],
    vec![expr!(3)],
    vec![expr!(0)],
]);

// Least squares: (A^T A)^(-1) A^T b
let AT = expr!(transpose(A));
let ATA = expr!(AT * A);
let ATA_inv = expr!(ATA^(-1));
let ATb = expr!(AT * b);

let x_ls = expr!(ATA_inv * ATb);
// Result: Best approximate solution
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import Matrix

A = Matrix([[1, 1], [2, 2], [1, -1]])
b = Matrix([[1], [3], [0]])

# Least squares
x_ls = (A.T * A).inv() * A.T * b
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const A = matrix([[1, 1], [2, 2], [1, -1]]);
const b = matrix([[1], [3], [0]]);

// Least squares
const AT = A.transpose();
const x_ls = AT.mul(A).inv().mul(AT).mul(b);
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
const metaDescription = "Solve systems of equations (linear and nonlinear) with multiple unknowns
using substitution, elimination, matrix methods, and Newton's method for
nonlinear systems.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'System of Equations Solving',
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
