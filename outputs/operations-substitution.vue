<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Substitution</h1>
      <p class="description">Replace variables with values or expressions to evaluate, simplify, or transform expressions.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Function Evaluation:**
$$f(a) = f(x)|_{x=a}$$
Substitute $x = a$ into function $f(x)$.

**Composition:**
$$f(g(x)) = f(u)|_{u=g(x)}$$
Substitute $u = g(x)$ into function $f(u)$.

**U-Substitution (Integration):**
$$\int f(g(x)) \cdot g'(x) \, dx = \int f(u) \, du$$
where $u = g(x)$ and $du = g'(x) \, dx$.

**Change of Variables (Multivariable):**
$$\frac{\partial f}{\partial x} = \frac{\partial f}{\partial u} \cdot \frac{\partial u}{\partial x}$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Single Variable Substitution</h3>
        <p>Replace variable with number</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Substitute x = 2 into x² + 3x
let expr1 = expr!(x ^ 2 + 3 * x);
let result1 = expr1.substitute(&x, &expr!(2));
// Result: 4 + 6 = 10

// Substitute x = -1 into x³ - 2x + 1
let expr2 = expr!(x ^ 3 - 2 * x + 1);
let result2 = expr2.substitute(&x, &expr!(-1));
// Result: -1 + 2 + 1 = 2
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

x = symbol('x')

# Substitute x = 2 into x² + 3x
expr1 = x**2 + 3*x
result1 = expr1.subs(x, 2)
# Result: 10
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');

const x = symbol('x');

// Substitute x = 2 into x² + 3x
const expr1 = x.pow(2).add(x.mul(3));
const result1 = expr1.substitute(x, 2);
// Result: 10
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Expression Substitution</h3>
        <p>Replace with another expression</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Substitute x = y + 1 into x² + 3x
let expression = expr!(x ^ 2 + 3 * x);
let substituted = expression.substitute(&x, &expr!(y + 1));
// Result: (y+1)² + 3(y+1)

// Expand for cleaner form
let expanded = substituted.expand();
// Result: y² + 2y + 1 + 3y + 3 = y² + 5y + 4
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

x = symbol('x')
y = symbol('y')

# Substitute x = y + 1 into x² + 3x
expression = x**2 + 3*x
substituted = expression.subs(x, y + 1)
# Result: (y+1)^2 + 3(y+1)

# Expand for cleaner form
expanded = substituted.expand()
# Result: y^2 + 5*y + 4
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');

const x = symbol('x');
const y = symbol('y');

// Substitute x = y + 1 into x² + 3x
const expression = x.pow(2).add(x.mul(3));
const substituted = expression.substitute(x, y.add(1));
// Result: (y+1)^2 + 3(y+1)
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>U-Substitution for Integration</h3>
        <p>Transform difficult integrals</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);
let u = symbol!(u);

// Integrate: ∫ 2x·e^(x²) dx
// Let u = x², then du = 2x dx
let integrand = expr!(2 * x * exp(x ^ 2));

// Manual substitution
let u_expr = expr!(x ^ 2);  // u = x²
let integrand_u = integrand.substitute(&expr!(x ^ 2), &u);
// Result: ∫ e^u du = e^u + C

// Back-substitute: e^(x²) + C
let result = expr!(exp(u)).substitute(&u, &u_expr);
// Result: e^(x²) + C
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, integrate

x = symbol('x')
u = symbol('u')

# Integrate: ∫ 2x·e^(x²) dx
integrand = 2*x*exp(x**2)

# Manual substitution
u_expr = x**2  # u = x²
integrand_u = integrand.subs(x**2, u)
# Result: ∫ e^u du = e^u + C

# Back-substitute: e^(x²) + C
result = exp(u).subs(u, u_expr)
# Result: e^(x²) + C
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');

const x = symbol('x');
const u = symbol('u');

// Integrate: ∫ 2x·e^(x²) dx
const integrand = x.mul(2).mul(expr('exp(x^2)'));

// Manual substitution
const uExpr = x.pow(2);  // u = x²
const integrandU = integrand.substitute(x.pow(2), u);
// Result: ∫ e^u du = e^u + C
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
const metaDescription = "Replace variables with values or expressions to evaluate, simplify, or transform expressions.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Substitution',
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
