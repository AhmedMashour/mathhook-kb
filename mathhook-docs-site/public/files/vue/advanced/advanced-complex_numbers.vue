<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Complex Number Operations</h1>
      <p class="description">Work with complex numbers in MathHook including imaginary unit i, complex
arithmetic, polar form, Euler's formula, and operations like conjugate,
magnitude, and argument.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">Complex number: $$z = a + bi$$ where $a, b \in \mathbb{R}$ and $i^2 = -1$

Polar form: $$z = r e^{i\theta} = r(\cos\theta + i\sin\theta)$$

where $r = |z| = \sqrt{a^2 + b^2}$ and $\theta = \arg(z) = \arctan(b/a)$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Complex Arithmetic</h3>
        <p></p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let i = Expression::i();
let z1 = expr!(3 + 4*i);
let z2 = expr!(1 - 2*i);

let sum = expr!(z1 + z2);      // 4 + 2i
let product = expr!(z1 * z2);  // 11 - 2i
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">i = expr('I')
z1 = expr('3 + 4*I')
z2 = expr('1 - 2*I')

sum_z = z1 + z2       # 4 + 2*I
product = z1 * z2     # 11 - 2*I
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const i = expr('I');
const z1 = expr('3 + 4*I');
const z2 = expr('1 - 2*I');

const sum = z1.add(z2);      // 4 + 2*I
const product = z1.mul(z2);  // 11 - 2*I
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Euler's Formula</h3>
        <p></p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let theta = symbol!(theta);
let euler = expr!(exp(i * theta));

// Expands to: cos(theta) + i*sin(theta)
let expanded = euler.expand();
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">theta = symbol('theta')
euler = exp(I * theta)

# Expands to: cos(theta) + I*sin(theta)
expanded = expand(euler)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const theta = symbol('theta');
const euler = exp(mul(I, theta));

// Expands to: cos(theta) + I*sin(theta)
const expanded = euler.expand();
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Polar Form Conversion</h3>
        <p></p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let z = expr!(3 + 4*i);

let magnitude = expr!(abs(z));  // 5
let angle = expr!(arg(z));      // arctan(4/3)

// Polar form: r*exp(i*theta)
let polar = expr!(magnitude * exp(i * angle));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">z = expr('3 + 4*I')

magnitude = abs(z)  # 5
angle = arg(z)      # atan(4/3)

# Polar form
polar = magnitude * exp(I * angle)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const z = expr('3 + 4*I');

const magnitude = abs(z);  // 5
const angle = arg(z);      // atan(4/3)

// Polar form
const polar = magnitude.mul(exp(I.mul(angle)));
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
const metaDescription = "Work with complex numbers in MathHook including imaginary unit i, complex
arithmetic, polar form, Euler's formula, and operations like conjugate,
magnitude, and argument.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Complex Number Operations',
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
