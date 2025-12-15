<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Limits</h1>
      <p class="description">Compute limits of expressions as variables approach values, infinity, or points of discontinuity.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Epsilon-Delta Definition (ε-δ):**
$$\lim_{x \to a} f(x) = L$$
means: For every $\varepsilon > 0$, there exists $\delta > 0$ such that:
$$0 < |x - a| < \delta \implies |f(x) - L| < \varepsilon$$

**Limit Laws:**
1. **Sum/Difference:** $\lim_{x \to a} [f(x) \pm g(x)] = \lim_{x \to a} f(x) \pm \lim_{x \to a} g(x)$
2. **Product:** $\lim_{x \to a} [f(x) \cdot g(x)] = \lim_{x \to a} f(x) \cdot \lim_{x \to a} g(x)$
3. **Quotient:** $\lim_{x \to a} \frac{f(x)}{g(x)} = \frac{\lim_{x \to a} f(x)}{\lim_{x \to a} g(x)}$ (if denominator $\neq 0$)

**L'Hôpital's Rule (0/0 or ∞/∞):**
$$\lim_{x \to a} \frac{f(x)}{g(x)} = \lim_{x \to a} \frac{f'(x)}{g'(x)}$$
(if the limit on the right exists)
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Direct Substitution</h3>
        <p>For continuous functions, substitute directly</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Limit: lim(x→2) x² = 4
let expr1 = expr!(x ^ 2);
let limit1 = expr1.limit(&x, &expr!(2));
// Result: 4

// Limit: lim(x→π) sin(x) = 0
let expr2 = expr!(sin(x));
let limit2 = expr2.limit(&x, &Expression::pi());
// Result: 0
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, limit, pi

x = symbol('x')

# Limit: lim(x→2) x² = 4
expr1 = x**2
limit1 = limit(expr1, x, 2)
# Result: 4

# Limit: lim(x→π) sin(x) = 0
expr2 = sin(x)
limit2 = limit(expr2, x, pi)
# Result: 0
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, limit } = require('mathhook');

const x = symbol('x');

// Limit: lim(x→2) x² = 4
const expr1 = x.pow(2);
const limit1 = limit(expr1, x, 2);
// Result: 4
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>L'Hôpital's Rule (0/0 Form)</h3>
        <p>Use derivatives to resolve indeterminate forms</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Limit: lim(x→0) sin(x)/x = 1 (0/0 form)
// Apply L'Hôpital: lim(x→0) cos(x)/1 = 1
let expr = expr!(sin(x) / x);
let limit = expr.limit(&x, &expr!(0));
// Result: 1

// Limit: lim(x→0) (1 - cos(x))/x² = 1/2 (0/0 form)
let expr2 = expr!((1 - cos(x)) / (x ^ 2));
let limit2 = expr2.limit(&x, &expr!(0));
// Result: 1/2
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, limit, sin, cos

x = symbol('x')

# Limit: lim(x→0) sin(x)/x = 1
expr = sin(x)/x
result = limit(expr, x, 0)
# Result: 1

# Limit: lim(x→0) (1 - cos(x))/x²
expr2 = (1 - cos(x))/x**2
result2 = limit(expr2, x, 0)
# Result: 1/2
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, limit, parse } = require('mathhook');

const x = symbol('x');

// Limit: lim(x→0) sin(x)/x
const expr = parse('sin(x)/x');
const result = limit(expr, x, 0);
// Result: 1
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Limits at Infinity</h3>
        <p>Behavior as x approaches ±∞</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::core::Expression;

let x = symbol!(x);

// Limit: lim(x→∞) (2x² + 1)/(x² + 3) = 2
let expr1 = expr!((2 * (x ^ 2) + 1) / ((x ^ 2) + 3));
let limit1 = expr1.limit(&x, &Expression::infinity());
// Result: 2

// Limit: lim(x→∞) (x + 1)/(x² + 1) = 0
let expr2 = expr!((x + 1) / ((x ^ 2) + 1));
let limit2 = expr2.limit(&x, &Expression::infinity());
// Result: 0
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, limit, oo

x = symbol('x')

# Limit: lim(x→∞) (2x² + 1)/(x² + 3)
expr1 = (2*x**2 + 1)/(x**2 + 3)
limit1 = limit(expr1, x, oo)
# Result: 2

# Limit: lim(x→∞) (x + 1)/(x² + 1)
expr2 = (x + 1)/(x**2 + 1)
limit2 = limit(expr2, x, oo)
# Result: 0
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, limit, Infinity } = require('mathhook');

const x = symbol('x');

// Limit: lim(x→∞) (2x² + 1)/(x² + 3)
const expr1 = parse('(2*x^2 + 1)/(x^2 + 3)');
const limit1 = limit(expr1, x, Infinity);
// Result: 2
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
const metaDescription = "Compute limits of expressions as variables approach values, infinity, or points of discontinuity.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Limits',
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
