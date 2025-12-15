<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Performance Comparison</h1>
      <p class="description">MathHook is designed for high performance while maintaining mathematical correctness.
This page shows how MathHook compares to other computer algebra systems, particularly SymPy.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Simple Performance Test</h3>
        <p>Benchmark derivative computation performance</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use std::time::Instant;

let x = symbol!(x);
let expr = expr!(x ^ 2);

let start = Instant::now();
for _ in 0..1000 {
    let _ = expr.derivative(&x, 1);
}
let duration = start.elapsed();
println!("1000 derivatives: {:?}", duration);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">import mathhook
import time

x = mathhook.symbol('x')
expr = mathhook.expr('x^2')

start = time.time()
for _ in range(1000):
    expr.derivative(x)
duration = time.time() - start
print(f"1000 derivatives: {duration:.3f}s")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');

const x = symbol('x');
const expression = expr('x^2');

const start = Date.now();
for (let i = 0; i < 1000; i++) {
    expression.derivative(x);
}
const duration = Date.now() - start;
console.log(`1000 derivatives: ${duration}ms`);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Compare Operations</h3>
        <p>Compare different calculus operation speeds</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use std::time::Instant;

let x = symbol!(x);
let expr = expr!((x ^ 2) * sin(x));

// Time derivative
let start = Instant::now();
let deriv = expr.derivative(&x, 1);
println!("Derivative time: {:?}", start.elapsed());

// Time simplification
let start = Instant::now();
let simplified = deriv.simplify();
println!("Simplify time: {:?}", start.elapsed());
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">import mathhook
import time

x = mathhook.symbol('x')
expr = mathhook.expr('x^2 * sin(x)')

# Time derivative
start = time.time()
deriv = expr.derivative(x)
print(f"Derivative time: {(time.time() - start)*1000:.3f}ms")

# Time simplification
start = time.time()
simplified = deriv.simplify()
print(f"Simplify time: {(time.time() - start)*1000:.3f}ms")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');

const x = symbol('x');
const expression = expr('x^2 * sin(x)');

// Time derivative
let start = Date.now();
const deriv = expression.derivative(x);
console.log(`Derivative time: ${Date.now() - start}ms`);

// Time simplification
start = Date.now();
const simplified = deriv.simplify();
console.log(`Simplify time: ${Date.now() - start}ms`);
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
const metaDescription = "MathHook is designed for high performance while maintaining mathematical correctness.
This page shows how MathHook compares to other computer algebra systems, particularly SymPy.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Performance Comparison',
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
