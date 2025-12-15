<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Expressions</h1>
      <p class="description">The Expression type is the foundation of MathHook. Expressions are represented as an enum
with variants for different mathematical constructs including numbers, variables, operations,
functions, constants, matrices, and relations.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Expression Creation with Macros</h3>
        <p>Using expr! and symbol! macros for ergonomic expression creation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Basic arithmetic
let sum = expr!(x + y);
let product = expr!(x * y);
let power = expr!(x ^ 2);

// Complex expressions
let quadratic = expr!(a * x ^ 2 + b * x + c);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr

x = symbol('x')
y = symbol('y')

# Basic arithmetic
sum_expr = expr('x + y')
product = expr('x * y')
power = expr('x^2')

# Complex expressions
quadratic = expr('a*x^2 + b*x + c')
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook-node');

const x = symbol('x');
const y = symbol('y');

// Basic arithmetic
const sum = expr('x + y');
const product = expr('x * y');
const power = expr('x^2');

// Complex expressions
const quadratic = expr('a*x^2 + b*x + c');
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Canonical Form Normalization</h3>
        <p>Expressions are automatically normalized to canonical form</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let expr1 = expr!(x + y);
let expr2 = expr!(y + x);

// Both normalized to same form
assert_eq!(expr1, expr2);

// Rationals reduced
let frac = Expression::rational(6, 4);
assert_eq!(frac, Expression::rational(3, 2));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, Expression

expr1 = expr('x + y')
expr2 = expr('y + x')

# Both normalized to same form
assert expr1 == expr2

# Rationals reduced
frac = Expression.rational(6, 4)
assert frac == Expression.rational(3, 2)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, Expression } = require('mathhook-node');

const expr1 = expr('x + y');
const expr2 = expr('y + x');

// Both normalized to same form
console.assert(expr1.equals(expr2));

// Rationals reduced
const frac = Expression.rational(6, 4);
console.assert(frac.equals(Expression.rational(3, 2)));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Immutable Operations</h3>
        <p>All expression operations return new expressions without modifying originals</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let expr = expr!(x + 1);
let doubled = expr.mul(&expr!(2));

// Original unchanged
println!("Original: {}", expr);  // x + 1
println!("Doubled: {}", doubled); // 2*(x + 1)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr

original = expr('x + 1')
doubled = original * 2

# Original unchanged
print(f"Original: {original}")  # x + 1
print(f"Doubled: {doubled}")    # 2*(x + 1)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr } = require('mathhook-node');

const original = expr('x + 1');
const doubled = original.mul(2);

// Original unchanged
console.log(`Original: ${original}`);  // x + 1
console.log(`Doubled: ${doubled}`);    // 2*(x + 1)
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
const metaDescription = "The Expression type is the foundation of MathHook. Expressions are represented as an enum
with variants for different mathematical constructs including numbers, variables, operations,
functions, constants, matrices, and relations.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Expressions',
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
