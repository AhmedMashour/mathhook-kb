<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Symbolic Simplification</h1>
      <p class="description">MathHook provides comprehensive symbolic simplification for mathematical expressions, with full support for noncommutative algebra (matrices, operators, quaternions). The simplification system implements canonical forms and mathematical identities to reduce expressions to their simplest equivalent representation.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Power Rule:**
$$x^a \cdot x^b \rightarrow x^{a+b}$$

**Noncommutative Algebra:**
For noncommutative symbols (matrices, operators):
- $AB \neq BA$ in general
- $(A + B)^2 = A^2 + AB + BA + B^2$ (4 terms, not 3)

**Rational Arithmetic:**
- Exact representation: $\frac{1}{3}$ stays as rational, not float
- Automatic simplification: Reduces fractions to lowest terms
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Simplification</h3>
        <p>Identity elements and constant folding</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Identity elements
let expr = expr!((x + 0) * 1);
let simplified = expr.simplify();
// Result: x

// Constant folding
let expr = expr!(2 + 3);
let simplified = expr.simplify();
// Result: 5
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

x = symbol('x')

# Identity elements
expr = (x + 0) * 1
simplified = expr.simplify()
# Result: x

# Constant folding
expr = 2 + 3
# Result: 5
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');

const x = symbol('x');

// Identity elements
let expr = x.add(0).mul(1);
const simplified = expr.simplify();
// Result: x
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Power Rule</h3>
        <p>Combine like powers with same base</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Combine like powers
let expr = expr!((x^2) * (x^3));
let simplified = expr.simplify();
// Result: x^5

// Multiple powers
let expr = expr!((x^2) * (x^3) * (x^4));
let simplified = expr.simplify();
// Result: x^9
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

x = symbol('x')

# Combine like powers
expr = x**2 * x**3
simplified = expr.simplify()
# Result: x^5
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');

const x = symbol('x');

// Combine like powers
const expr = x.pow(2).mul(x.pow(3));
const simplified = expr.simplify();
// Result: x^5
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Noncommutative Matrices</h3>
        <p>Matrix multiplication does NOT commute</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Matrix multiplication does NOT commute
let expr = expr!(A * B);
// Simplification preserves order: A*B ≠ B*A
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

A = symbol('A', matrix=True)
B = symbol('B', matrix=True)

# Matrix multiplication does NOT commute
expr = A * B
# Simplification preserves order: A*B ≠ B*A
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');

const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });

// Matrix multiplication does NOT commute
const expr = A.mul(B);
// Simplification preserves order: A*B ≠ B*A
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
const metaDescription = "MathHook provides comprehensive symbolic simplification for mathematical expressions, with full support for noncommutative algebra (matrices, operators, quaternions). The simplification system implements canonical forms and mathematical identities to reduce expressions to their simplest equivalent representation.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Symbolic Simplification',
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
