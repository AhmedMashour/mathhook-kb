<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Expansion and Factoring</h1>
      <p class="description">Transform expressions between expanded and factored forms for easier manipulation and analysis.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Distributive Law:**
$$a(b + c) = ab + ac$$

**Binomial Expansion:**
$$(x + y)^n = \sum_{k=0}^{n} \binom{n}{k} x^{n-k} y^k$$

For small powers:
- $$(x + y)^2 = x^2 + 2xy + y^2$$
- $$(x + y)^3 = x^3 + 3x^2y + 3xy^2 + y^3$$
- $$(x - y)^2 = x^2 - 2xy + y^2$$

**Special Products:**
- **Difference of Squares:** $(x + y)(x - y) = x^2 - y^2$
- **Perfect Square Trinomial:** $(x + y)^2 = x^2 + 2xy + y^2$

**Noncommutative Expansion:**
For matrices (noncommutative):
$$(A + B)^2 = A^2 + AB + BA + B^2 \quad \text{(4 terms, cannot combine } AB \text{ and } BA\text{)}$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Simple Products</h3>
        <p>Expand products of sums</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Expand 2(x + 3)
let expr1 = expr!(2 * (x + 3));
let expanded1 = expr1.expand();
// Result: 2x + 6

// Expand (x + 1)(x + 2)
let expr2 = expr!((x + 1) * (x + 2));
let expanded2 = expr2.expand();
// Result: x² + 3x + 2

// Expand (x + y)(x - y) - difference of squares
let y = symbol!(y);
let expr3 = expr!((x + y) * (x - y));
let expanded3 = expr3.expand();
// Result: x² - y²
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

x = symbol('x')

# Expand 2(x + 3)
expr1 = 2 * (x + 3)
expanded1 = expr1.expand()
# Result: 2*x + 6

# Expand (x + 1)(x + 2)
expr2 = (x + 1) * (x + 2)
expanded2 = expr2.expand()
# Result: x**2 + 3*x + 2
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');

const x = symbol('x');

// Expand 2(x + 3)
const expr1 = x.add(3).mul(2);
const expanded1 = expr1.expand();
// Result: 2*x + 6
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Power Expansion</h3>
        <p>Expand expressions raised to integer powers</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Expand (x + 1)^2
let expr1 = expr!((x + 1) ^ 2);
let expanded1 = expr1.expand();
// Result: x² + 2x + 1

// Expand (x + y)^3
let expr2 = expr!((x + y) ^ 3);
let expanded2 = expr2.expand();
// Result: x³ + 3x²y + 3xy² + y³

// Expand (x - 2)^4
let expr3 = expr!((x - 2) ^ 4);
let expanded3 = expr3.expand();
// Result: x⁴ - 8x³ + 24x² - 32x + 16
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

x = symbol('x')
y = symbol('y')

# Expand (x + 1)^2
expr1 = (x + 1)**2
expanded1 = expr1.expand()
# Result: x**2 + 2*x + 1

# Expand (x + y)^3
expr2 = (x + y)**3
expanded2 = expr2.expand()
# Result: x**3 + 3*x**2*y + 3*x*y**2 + y**3
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');

const x = symbol('x');
const y = symbol('y');

// Expand (x + 1)^2
const expr1 = x.add(1).pow(2);
const expanded1 = expr1.expand();
// Result: x^2 + 2*x + 1
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Noncommutative Matrix Expansion</h3>
        <p>For matrices, order matters - (A+B)² has 4 terms</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Create matrix symbols
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let C = symbol!(C; matrix);

// Expand (A + B)^2 - preserves noncommutativity
let expr = expr!((A + B) ^ 2);
let expanded = expr.expand();
// Result: A² + AB + BA + B²   (4 terms, NOT A² + 2AB + B²)

// Expand (A + B)(C)
let expr2 = expr!((A + B) * C);
let expanded2 = expr2.expand();
// Result: AC + BC   (order preserved: A*C first, then B*C)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

# Create matrix symbols
A = symbol('A', matrix=True)
B = symbol('B', matrix=True)
C = symbol('C', matrix=True)

# Expand (A + B)^2 - preserves noncommutativity
expr = (A + B)**2
expanded = expr.expand()
# Result: A**2 + A*B + B*A + B**2   (4 terms, NOT A**2 + 2*A*B + B**2)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');

// Create matrix symbols
const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });

// Expand (A + B)^2 - preserves noncommutativity
const expr = A.add(B).pow(2);
const expanded = expr.expand();
// Result: A^2 + AB + BA + B^2   (4 terms)
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
const metaDescription = "Transform expressions between expanded and factored forms for easier manipulation and analysis.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Expansion and Factoring',
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
