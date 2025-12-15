<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Symbols and Numbers</h1>
      <p class="description">Symbols represent mathematical variables (x, y, θ, etc.) using efficient string interning.
Numbers support integers, rationals, floats, and complex numbers with exact symbolic representation
for precise mathematical computation.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Symbol Creation and Equality</h3>
        <p>Creating symbols with string interning for O(1) equality checks</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x1 = symbol!(x);
let x2 = symbol!(x);
let y = symbol!(y);

// O(1) pointer comparison
assert_eq!(x1, x2);
assert_ne!(x1, y);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

x1 = symbol('x')
x2 = symbol('x')
y = symbol('y')

# Fast equality check
assert x1 == x2
assert x1 != y
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook-node');

const x1 = symbol('x');
const x2 = symbol('x');
const y = symbol('y');

// Fast equality check
console.assert(x1.equals(x2));
console.assert(!x1.equals(y));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Exact Rational Arithmetic</h3>
        <p>Using rationals for exact fractional computation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Exact: 1/3
let third = Expression::rational(1, 3);
let result = expr!(3 * third);
assert_eq!(result, Expression::integer(1));

// Auto-reduction: 6/4 = 3/2
let frac = Expression::rational(6, 4);
assert_eq!(frac, Expression::rational(3, 2));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression, expr

# Exact: 1/3
third = Expression.rational(1, 3)
result = expr('3 * third')
assert result == Expression.integer(1)

# Auto-reduction: 6/4 = 3/2
frac = Expression.rational(6, 4)
assert frac == Expression.rational(3, 2)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { Expression, expr } = require('mathhook-node');

// Exact: 1/3
const third = Expression.rational(1, 3);
const result = expr('3 * third');
console.assert(result.equals(Expression.integer(1)));

// Auto-reduction: 6/4 = 3/2
const frac = Expression.rational(6, 4);
console.assert(frac.equals(Expression.rational(3, 2)));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Complex Numbers</h3>
        <p>Working with complex numbers and imaginary unit</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// 3 + 4i
let z = Expression::complex(
    Expression::integer(3),
    Expression::integer(4)
);

// Magnitude: |z| = sqrt(3^2 + 4^2) = 5
let magnitude = expr!(sqrt((3^2) + (4^2)));
assert_eq!(magnitude.simplify(), Expression::integer(5));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression, expr

# 3 + 4i
z = Expression.complex(3, 4)

# Magnitude: |z| = 5
magnitude = expr('sqrt(3^2 + 4^2)')
assert magnitude.simplify() == Expression.integer(5)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { Expression, expr } = require('mathhook-node');

// 3 + 4i
const z = Expression.complex(3, 4);

// Magnitude: |z| = 5
const magnitude = expr('sqrt(3^2 + 4^2)');
console.assert(magnitude.simplify().equals(Expression.integer(5)));
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
const metaDescription = "Symbols represent mathematical variables (x, y, θ, etc.) using efficient string interning.
Numbers support integers, rationals, floats, and complex numbers with exact symbolic representation
for precise mathematical computation.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Symbols and Numbers',
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
