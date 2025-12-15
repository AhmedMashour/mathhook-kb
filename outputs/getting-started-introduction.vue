<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Introduction to MathHook</h1>
      <p class="description">MathHook is a high-performance educational computer algebra system (CAS) written in Rust,
designed to combine mathematical correctness with exceptional performance.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Expression Building</h3>
        <p>Create mathematical expressions using macros</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(add: (x ^ 2), (2 * x), 1);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr

x = symbol('x')
expression = expr('x^2 + 2*x + 1')
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');

const x = symbol('x');
const expression = expr('x^2 + 2*x + 1');
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Symbolic Computation</h3>
        <p>Perform algebraic manipulations</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(add: (x ^ 2), (2 * x), 1);

let simplified = expr.simplify();
let expanded = expr.expand();
let factored = expr.factor();
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr

x = symbol('x')
expression = expr('x^2 + 2*x + 1')

simplified = expression.simplify()
expanded = expression.expand()
factored = expression.factor()
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');

const x = symbol('x');
const expression = expr('x^2 + 2*x + 1');

const simplified = expression.simplify();
const expanded = expression.expand();
const factored = expression.factor();
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Calculus Operations</h3>
        <p>Compute derivatives and integrals</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(add: (x ^ 2), (2 * x), 1);

let derivative = expr.derivative(x.clone());
let integral = expr.integrate(x, 0);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr

x = symbol('x')
expression = expr('x^2 + 2*x + 1')

derivative = expression.derivative(x)
integral = expression.integrate(x)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');

const x = symbol('x');
const expression = expr('x^2 + 2*x + 1');

const derivative = expression.derivative(x);
const integral = expression.integrate(x);
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
const metaDescription = "MathHook is a high-performance educational computer algebra system (CAS) written in Rust,
designed to combine mathematical correctness with exceptional performance.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Introduction to MathHook',
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
