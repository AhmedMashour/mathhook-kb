<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>WebAssembly Bindings</h1>
      <p class="description">WebAssembly (WASM) bindings for MathHook, enabling browser-based symbolic mathematics
without requiring Node.js. Documentation is under development.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Planned Browser Usage (Future)</h3>
        <p>Example of how WASM bindings will be used in browsers (not yet available)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Current Rust usage (direct)
use mathhook::{expr, symbol, simplify};

let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);
let simplified = simplify(expr);
// Result: (x + 1)^2
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"></code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Future WASM usage (not yet available)
import init, { Symbol, parse, simplify } from 'mathhook-wasm';

await init();

// Create symbols
const x = Symbol.create('x');

// Parse and simplify
const expr = parse('x^2 + 2*x + 1');
const simplified = simplify(expr);

console.log(simplified.toString());  // (x + 1)^2
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>React Component Example (Future)</h3>
        <p>Planned integration with React components using WASM</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust"></code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"></code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Future React integration (not yet available)
import React, { useState, useEffect } from 'react';
import init, { parse, simplify } from 'mathhook-wasm';

function Calculator() {
  const [initialized, setInitialized] = useState(false);
  const [input, setInput] = useState('');
  const [result, setResult] = useState('');

  useEffect(() => {
    init().then(() => setInitialized(true));
  }, []);

  const handleSimplify = () => {
    if (!initialized) return;

    try {
      const expr = parse(input);
      const simplified = simplify(expr);
      setResult(simplified.toString());
    } catch (error) {
      setResult(`Error: ${error.message}`);
    }
  };

  return (
    <div>
      <input
        value={input}
        onChange={(e) => setInput(e.target.value)}
        placeholder="Enter expression"
      />
      <button onClick={handleSimplify} disabled={!initialized}>
        Simplify
      </button>
      {result && <div>Result: {result}</div>}
    </div>
  );
}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Web Worker Integration (Future)</h3>
        <p>Offload heavy symbolic computation to Web Workers</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust"></code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"></code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Future Web Worker usage (not yet available)
// worker.js
import init, { parse, simplify, derivative } from 'mathhook-wasm';

await init();

self.onmessage = (e) => {
  const { operation, expression, variable } = e.data;

  try {
    const expr = parse(expression);
    let result;

    switch (operation) {
      case 'simplify':
        result = simplify(expr);
        break;
      case 'derivative':
        const x = Symbol.create(variable);
        result = derivative(expr, x);
        break;
    }

    self.postMessage({ success: true, result: result.toString() });
  } catch (error) {
    self.postMessage({ success: false, error: error.message });
  }
};

// main.js
const worker = new Worker('worker.js', { type: 'module' });

worker.postMessage({
  operation: 'simplify',
  expression: 'x^2 + 2*x + 1'
});

worker.onmessage = (e) => {
  console.log('Result:', e.data.result);
};
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
const metaDescription = "WebAssembly (WASM) bindings for MathHook, enabling browser-based symbolic mathematics
without requiring Node.js. Documentation is under development.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'WebAssembly Bindings',
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
