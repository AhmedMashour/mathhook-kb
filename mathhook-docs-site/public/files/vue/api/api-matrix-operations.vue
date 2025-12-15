<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Matrix Operations and Noncommutative Algebra</h1>
      <p class="description">Symbolic and numeric matrix operations with full support for noncommutative algebra.
Matrix symbols preserve multiplication order (AB ≠ BA), enabling correct handling of
linear algebra, quantum mechanics operators, and transformation matrices.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Creating Matrix Symbols</h3>
        <p>Matrix symbols are noncommutative - order matters</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Matrix symbols (noncommutative)
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let X = symbol!(X; matrix);

// Matrix multiplication - ORDER MATTERS!
let product_ab = expr!(A * B);  // A*B
let product_ba = expr!(B * A);  // B*A ≠ A*B in general!

// Matrix equation: A*X = B
let equation = expr!(A * X);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

# Matrix symbols (noncommutative)
A = symbol('A', matrix=True)
B = symbol('B', matrix=True)
X = symbol('X', matrix=True)

# Matrix multiplication - ORDER MATTERS!
product_ab = A * B  # A*B
product_ba = B * A  # B*A ≠ A*B

# Matrix equation
equation = A * X
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse } from 'mathhook';

// Matrix symbols (noncommutative)
const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });
const X = symbol('X', { type: 'matrix' });

// Matrix multiplication - ORDER MATTERS!
const product_ab = parse('A * B');  // A*B
const product_ba = parse('B * A');  // B*A ≠ A*B
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Creating Numeric Matrices</h3>
        <p>Build matrices from expression arrays</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// 2×2 matrix with symbolic entries
let matrix = Expression::matrix(vec![
    vec![expr!(x), expr!(1)],
    vec![expr!(0), expr!(x)],
]);

// 3×3 identity matrix
let identity = Expression::identity_matrix(3);

// Zero matrix (2 rows, 3 columns)
let zero = Expression::zero_matrix(2, 3);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Matrix, symbol

x = symbol('x')

# 2×2 matrix with symbolic entries
matrix = Matrix([
    [x, 1],
    [0, x]
])

# 3×3 identity matrix
identity = Matrix.identity(3)

# Zero matrix
zero = Matrix.zero(2, 3)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Matrix, symbol } from 'mathhook';

const x = symbol('x');

// 2×2 matrix with symbolic entries
const matrix = new Matrix([
    [x, 1],
    [0, x]
]);

// 3×3 identity matrix
const identity = Matrix.identity(3);

// Zero matrix
const zero = Matrix.zero(2, 3);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Matrix Multiplication (Critical!)</h3>
        <p>Order matters - demonstrate noncommutativity</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let C = symbol!(C; matrix);

// Order matters!
let ab = expr!(A * B);  // AB
let ba = expr!(B * A);  // BA ≠ AB in general

// Associative (but not commutative)
let abc_left = expr!((A * B) * C);   // (AB)C
let abc_right = expr!(A * (B * C));  // A(BC)
// (AB)C = A(BC) always

// Mixed scalar-matrix
let x = symbol!(x);
let xAB = expr!(x * A * B);  // x(AB) = (xA)B = A(xB)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

A = symbol('A', matrix=True)
B = symbol('B', matrix=True)
C = symbol('C', matrix=True)

# Order matters!
ab = A * B  # AB
ba = B * A  # BA ≠ AB

# Associative
abc_left = (A * B) * C   # (AB)C
abc_right = A * (B * C)  # A(BC)
# These are equal

# Mixed scalar-matrix
x = symbol('x')
xAB = x * A * B  # Scalars commute with matrices
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse } from 'mathhook';

const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });
const C = symbol('C', { type: 'matrix' });

// Order matters!
const ab = parse('A * B');  // AB
const ba = parse('B * A');  // BA ≠ AB

// Associative
const abc_left = parse('(A * B) * C');
const abc_right = parse('A * (B * C)');
// Equal
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Left Division vs Right Division</h3>
        <p>Solving matrix equations correctly</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Left division: A*X = B → X = A⁻¹*B
let left_eq = expr!(A * X - B);
let mut solver = MatrixEquationSolver::new();
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

A = symbol('A', matrix=True)
X = symbol('X', matrix=True)
B = symbol('B', matrix=True)

# Left division: A*X = B
left_eq = A*X - B
solution_left = solve(left_eq, X)
# Result: X = A⁻¹*B

# Right division: X*A = B
right_eq = X*A - B
solution_right = solve(right_eq, X)
# Result: X = B*A⁻¹
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse, solve } from 'mathhook';

const A = symbol('A', { type: 'matrix' });
const X = symbol('X', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });

// Left division: A*X = B
const left_eq = parse('A*X - B');
const solution_left = solve(left_eq, X);
// Result: X = A⁻¹*B

// Right division: X*A = B
const right_eq = parse('X*A - B');
const solution_right = solve(right_eq, X);
// Result: X = B*A⁻¹
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Matrix Operations: Inverse and Determinant</h3>
        <p>Compute matrix properties</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::Expression;

let A = symbol!(A; matrix);

// Symbolic inverse
let A_inv = expr!(A ^ (-1));  // A^(-1)

// Symbolic determinant
let det_A = expr!(det(A));

// Numeric determinant
let matrix = Expression::matrix(vec![
    vec![expr!(1), expr!(2)],
    vec![expr!(3), expr!(4)],
]);
let det = expr!(det(matrix));
// Evaluates to: 1*4 - 2*3 = -2
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, Matrix, det

A = symbol('A', matrix=True)

# Symbolic inverse
A_inv = A**(-1)

# Symbolic determinant
det_A = det(A)

# Numeric determinant
matrix = Matrix([
    [1, 2],
    [3, 4]
])
det_val = det(matrix)
# Result: -2
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, Matrix, parse } from 'mathhook';

const A = symbol('A', { type: 'matrix' });

// Symbolic inverse
const A_inv = parse('A^(-1)');

// Symbolic determinant
const det_A = parse('det(A)');

// Numeric determinant
const matrix = new Matrix([
    [1, 2],
    [3, 4]
]);
const det_val = matrix.det();
// Result: -2
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Real-World Application: Quantum Mechanics</h3>
        <p>Pauli matrices and commutation relations</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::Expression;

// Pauli matrices
let sigma_x = Expression::matrix(vec![
    vec![expr!(0), expr!(1)],
    vec![expr!(1), expr!(0)],
]);

let i = Expression::i();  // Imaginary unit

let sigma_y = Expression::matrix(vec![
    vec![expr!(0), expr!(-i)],
    vec![i, expr!(0)],
]);

let sigma_z = Expression::matrix(vec![
    vec![expr!(1), expr!(0)],
    vec![expr!(0), expr!(-1)],
]);

// Commutation relations: [σ_x, σ_y] = 2iσ_z
let comm_xy = expr!((sigma_x * sigma_y) - (sigma_y * sigma_x));
let expected = expr!(2 * i * sigma_z);

// Verify
let difference = expr!(comm_xy - expected);
let simplified = difference.simplify();
// Should equal zero matrix
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Matrix, I

# Pauli matrices
sigma_x = Matrix([[0, 1], [1, 0]])
sigma_y = Matrix([[0, -I], [I, 0]])
sigma_z = Matrix([[1, 0], [0, -1]])

# Commutation: [σ_x, σ_y] = 2iσ_z
comm_xy = sigma_x*sigma_y - sigma_y*sigma_x
expected = 2*I*sigma_z

# Verify
assert (comm_xy - expected).simplify() == Matrix.zero(2, 2)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Matrix, I } from 'mathhook';

// Pauli matrices
const sigma_x = new Matrix([[0, 1], [1, 0]]);
const sigma_y = new Matrix([[0, -I], [I, 0]]);
const sigma_z = new Matrix([[1, 0], [0, -1]]);

// Commutation: [σ_x, σ_y] = 2iσ_z
const comm_xy = sigma_x.mul(sigma_y).sub(sigma_y.mul(sigma_x));
const expected = sigma_z.mul(2*I);
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
const metaDescription = "Symbolic and numeric matrix operations with full support for noncommutative algebra.
Matrix symbols preserve multiplication order (AB ≠ BA), enabling correct handling of
linear algebra, quantum mechanics operators, and transformation matrices.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Matrix Operations and Noncommutative Algebra',
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
