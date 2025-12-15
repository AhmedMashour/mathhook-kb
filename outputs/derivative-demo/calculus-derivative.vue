<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Symbolic Differentiation</h1>
      <p class="description">Computes the derivative of an expression with respect to a variable using
symbolic differentiation rules. Supports power rule, product rule, quotient
rule, chain rule, and derivatives of elementary functions.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">$$\frac{d}{dx} f(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Power Rule</h3>
        <p>The derivative of x^n is n*x^(n-1). This is one of the fundamental
rules of calculus and applies to any real exponent n.
</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
let f = expr!(x ^ 3);
let df = f.derivative(&x, 1);
// Result: 3*x^2
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = symbol('x')
f = expr('x^3')
df = f.derivative(x)
# Result: 3*x^2
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');
const f = expr('x^3');
const df = f.derivative(x);
// Result: 3*x^2
</code></pre>
        </div>

        
        <div class="output">
          <strong>Output:</strong>
          <pre><code>3*x^2</code></pre>
        </div>
        
      </div>
      
      <div class="example-card">
        <h3>Chain Rule</h3>
        <p>When differentiating a composition of functions f(g(x)), we use the
chain rule: (f∘g)'(x) = f'(g(x)) · g'(x)
</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
let f = expr!(sin(x ^ 2));
let df = f.derivative(&x, 1);
// Result: 2*x*cos(x^2)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = symbol('x')
f = expr('sin(x^2)')
df = f.derivative(x)
# Result: 2*x*cos(x^2)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');
const f = expr('sin(x^2)');
const df = f.derivative(x);
// Result: 2*x*cos(x^2)
</code></pre>
        </div>

        
        <div class="output">
          <strong>Output:</strong>
          <pre><code>2*x*cos(x^2)</code></pre>
        </div>
        
      </div>
      
      <div class="example-card">
        <h3>Product Rule</h3>
        <p>For the product of two functions u(x)·v(x), the derivative is
u'(x)·v(x) + u(x)·v'(x)
</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
let f = expr!((x ^ 2) * sin(x));
let df = f.derivative(&x, 1);
// Result: 2*x*sin(x) + x^2*cos(x)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = symbol('x')
f = expr('x^2 * sin(x)')
df = f.derivative(x)
# Result: 2*x*sin(x) + x^2*cos(x)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');
const f = expr('x^2 * sin(x)');
const df = f.derivative(x);
// Result: 2*x*sin(x) + x^2*cos(x)
</code></pre>
        </div>

        
        <div class="output">
          <strong>Output:</strong>
          <pre><code>2*x*sin(x) + x^2*cos(x)</code></pre>
        </div>
        
      </div>
      
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const activeTab = ref('python')

// SEO metadata
const metaDescription = "Computes the derivative of an expression with respect to a variable using
symbolic differentiation rules. Supports power rule, product rule, quotient
rule, chain rule, and derivatives of elementary functions.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Symbolic Differentiation',
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
