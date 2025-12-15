<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Symbolic Integration</h1>
      <p class="description">MathHook's integration system provides symbolic integration capabilities with an 8-layer strategy architecture from fast heuristics to complete Risch algorithm. Coverage: 93-95% of elementary integrals.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Fundamental Theorem of Calculus:**
$$\int_a^b f(x) \, dx = F(b) - F(a)$$
where $F'(x) = f(x)$.

**Integration by Parts:**
$$\int u \, dv = uv - \int v \, du$$

**U-Substitution:**
$$\int f(g(x)) \cdot g'(x) \, dx = \int f(u) \, du$$
where $u = g(x)$ and $du = g'(x) \, dx$.

**Power Rule:**
$$\int x^n \, dx = \frac{x^{n+1}}{n+1} + C \quad (n \neq -1)$$

**Logarithm Special Case:**
$$\int \frac{1}{x} \, dx = \ln|x| + C$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Integration (Layer 1: Table Lookup)</h3>
        <p>Direct table hits for common patterns</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);

// Polynomial: ∫x^3 dx = x^4/4 + C
let poly = expr!(x ^ 3);
let result = poly.integrate(x.clone());
// Result: x^4/4 + C

// Rational: ∫1/(x+1) dx = ln|x+1| + C
let rational = expr!(1 / (x + 1));
let result = rational.integrate(x.clone());
// Result: ln|x+1| + C

// Trigonometric: ∫sin(x) dx = -cos(x) + C
let trig = expr!(sin(x));
let result = trig.integrate(x.clone());
// Result: -cos(x) + C
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, integrate

x = symbol('x')

# Polynomial
poly = x**3
result = integrate(poly, x)
# Result: x**4/4

# Rational
rational = 1/(x+1)
result = integrate(rational, x)
# Result: log(x+1)

# Trigonometric
trig = sin(x)
result = integrate(trig, x)
# Result: -cos(x)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, integrate } = require('mathhook');

const x = symbol('x');

// Polynomial
const poly = x.pow(3);
const result = integrate(poly, x);
// Result: x^4/4
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Integration by Parts (Layer 4: LIATE)</h3>
        <p>∫u dv = uv - ∫v du using LIATE rule</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);

// ∫x*e^x dx: u = x (algebraic), dv = e^x (exponential)
let expr = expr!(x * exp(x));
let result = expr.integrate(x.clone());
// Result: x*e^x - e^x + C = e^x(x-1) + C

// ∫x*sin(x) dx: u = x (algebraic), dv = sin(x) (trig)
let expr2 = expr!(x * sin(x));
let result2 = expr2.integrate(x.clone());
// Result: -x*cos(x) + sin(x) + C
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, integrate, exp, sin

x = symbol('x')

# ∫x*e^x dx
expr = x * exp(x)
result = integrate(expr, x)
# Result: x*exp(x) - exp(x)

# ∫x*sin(x) dx
expr2 = x * sin(x)
result2 = integrate(expr2, x)
# Result: -x*cos(x) + sin(x)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, integrate, parse } = require('mathhook');

const x = symbol('x');

// ∫x*e^x dx
const expr = parse('x*exp(x)');
const result = integrate(expr, x);
// Result: x*exp(x) - exp(x)
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>U-Substitution (Layer 5)</h3>
        <p>∫f(g(x))*g'(x) dx = ∫f(u) du</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);

// ∫2x*sin(x^2) dx: u = x^2, du = 2x dx
let expr = expr!(2 * x * sin(x ^ 2));
let result = expr.integrate(x.clone());
// Result: -cos(x^2) + C

// ∫2x*e^(x^2) dx: u = x^2, du = 2x dx
let expr2 = expr!(2 * x * exp(x ^ 2));
let result2 = expr2.integrate(x.clone());
// Result: e^(x^2) + C
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, integrate, sin, exp

x = symbol('x')

# ∫2x*sin(x^2) dx
expr = 2*x*sin(x**2)
result = integrate(expr, x)
# Result: -cos(x^2)

# ∫2x*e^(x^2) dx
expr2 = 2*x*exp(x**2)
result2 = integrate(expr2, x)
# Result: exp(x^2)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, integrate, parse } = require('mathhook');

const x = symbol('x');

// ∫2x*sin(x^2) dx
const expr = parse('2*x*sin(x^2)');
const result = integrate(expr, x);
// Result: -cos(x^2)
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
const metaDescription = "MathHook's integration system provides symbolic integration capabilities with an 8-layer strategy architecture from fast heuristics to complete Risch algorithm. Coverage: 93-95% of elementary integrals.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Symbolic Integration',
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
