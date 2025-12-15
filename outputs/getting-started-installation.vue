<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Installation</h1>
      <p class="description">Complete installation guide for MathHook across Rust, Python, and Node.js platforms,
including platform-specific requirements, troubleshooting, and optional features.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Rust Installation Verification</h3>
        <p>Verify Rust installation with a simple test program</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

fn main() {
    let x = symbol!(x);
    let expr = expr!(x ^ 2);
    println!("Expression: {}", expr);
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')
expr = x.pow(2)
print(f"Expression: {expr}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const expr = x.pow(2);
console.log(`Expression: ${expr.toString()}`);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Python Virtual Environment Setup</h3>
        <p>Best practice for Python installation using virtual environments</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust"># Not applicable for Rust
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Create and activate virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install mathhook
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Not applicable for Node.js
// Use npm or yarn directly
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Building from Source</h3>
        <p>Complete source build for all platforms</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust"># Clone and build Rust core
git clone https://github.com/AhmedMashour/mathhook.git
cd mathhook
cargo build --release
cargo test
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Build Python bindings from source
cd crates/mathhook-python
pip install maturin
maturin develop --release
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Build Node.js bindings from source
cd crates/mathhook-node
npm install
npm run build
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
const metaDescription = "Complete installation guide for MathHook across Rust, Python, and Node.js platforms,
including platform-specific requirements, troubleshooting, and optional features.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Installation',
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
