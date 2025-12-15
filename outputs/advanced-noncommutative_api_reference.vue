<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Noncommutative Algebra API Reference</h1>
      <p class="description">Complete API reference for MathHook's noncommutative algebra support,
including symbol creation macros, type queries, and equation solving.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Symbol Creation Macros</h3>
        <p>Create symbols with different types</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Scalar (default)
let x = symbol!(x);

// Matrix (noncommutative)
let A = symbol!(A; matrix);

// Operator (noncommutative)
let p = symbol!(p; operator);

// Quaternion (noncommutative)
let i = symbol!(i; quaternion);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Scalar
x = symbols('x')

# Matrix
A = MatrixSymbol('A', n, n)

# Operator (quantum mechanics)
p = Operator('p')

# Quaternion
from sympy.algebras.quaternion import Quaternion
i = Quaternion(0, 1, 0, 0)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Scalar
const x = symbol('x');

// Matrix
const A = symbol('A', {type: 'matrix'});

// Operator
const p = symbol('p', {type: 'operator'});

// Quaternion
const i = symbol('i', {type: 'quaternion'});
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Bulk Symbol Creation</h3>
        <p>Create multiple symbols at once</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Multiple scalars
let scalars = symbols![x, y, z];

// Multiple matrices
let matrices = symbols![A, B, C => matrix];

// Multiple operators
let operators = symbols![p, x, H => operator];
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Multiple scalars
x, y, z = symbols('x y z')

# Multiple matrices
A, B, C = symbols('A B C', cls=MatrixSymbol)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Multiple symbols
const [x, y, z] = symbols(['x', 'y', 'z']);

// Multiple matrices
const [A, B, C] = symbols(['A', 'B', 'C'], {type: 'matrix'});
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Type Queries</h3>
        <p>Check symbol type and commutativity</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
let A = symbol!(A; matrix);

// Type check
assert_eq!(x.symbol_type(), SymbolType::Scalar);
assert_eq!(A.symbol_type(), SymbolType::Matrix);

// Commutativity check
assert_eq!(x.commutativity(), Commutativity::Commutative);
assert_eq!(A.commutativity(), Commutativity::Noncommutative);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = symbols('x')
A = MatrixSymbol('A', n, n)

# Type check
print(type(x))  # Symbol
print(type(A))  # MatrixSymbol

# Commutativity (implicit in type)
print(A.is_commutative)  # False
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');
const A = symbol('A', {type: 'matrix'});

// Type check
console.log(x.type);  // 'scalar'
console.log(A.type);  // 'matrix'

// Commutativity
console.log(x.is_commutative);  // true
console.log(A.is_commutative);  // false
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
const metaDescription = "Complete API reference for MathHook's noncommutative algebra support,
including symbol creation macros, type queries, and equation solving.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Noncommutative Algebra API Reference',
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
