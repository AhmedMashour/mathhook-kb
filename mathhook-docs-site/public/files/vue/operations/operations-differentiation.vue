<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Symbolic Differentiation</h1>
      <p class="description">Symbolic differentiation in MathHook uses automatic differentiation with the chain rule, product rule, quotient rule, and function-specific derivative rules.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Power Rule:**
$$\frac{d}{dx} x^n = n x^{n-1}$$

**Product Rule:**
$$\frac{d}{dx} [f(x) \cdot g(x)] = f'(x) \cdot g(x) + f(x) \cdot g'(x)$$

**Quotient Rule:**
$$\frac{d}{dx} \frac{f(x)}{g(x)} = \frac{f'(x) \cdot g(x) - f(x) \cdot g'(x)}{[g(x)]^2}$$

**Chain Rule:**
$$\frac{d}{dx} f(g(x)) = f'(g(x)) \cdot g'(x)$$

**Trigonometric Derivatives:**
- $\frac{d}{dx}\sin(x) = \cos(x)$
- $\frac{d}{dx}\cos(x) = -\sin(x)$
- $\frac{d}{dx}\tan(x) = \sec^2(x)$

**Exponential and Logarithmic:**
- $\frac{d}{dx}e^x = e^x$
- $\frac{d}{dx}\ln(x) = \frac{1}{x}$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Power Rule</h3>
        <p>d/dx(x^n) = n*x^(n-1)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 5);
let deriv = expr.derivative(&x, 1);
// Result: 5 * x^4
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, derivative

x = symbol('x')
expr = x**5
deriv = derivative(expr, x)
# Result: 5 * x^4
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, derivative } = require('mathhook');

const x = symbol('x');
const expr = x.pow(5);
const deriv = derivative(expr, x);
// Result: 5 * x^4
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Product Rule</h3>
        <p>d/dx(f·g) = f'·g + f·g'</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let f = expr!(x ^ 2);
let g = expr!(x ^ 3);
let product = expr!(mul: f, g);  // x^2 * x^3

let deriv = product.derivative(&x, 1);
// Result: 2*x * x^3 + x^2 * 3*x^2 = 5*x^4
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, derivative

x = symbol('x')
f = x**2
g = x**3
product = f * g

deriv = derivative(product, x)
# Result: 5*x^4
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, derivative } = require('mathhook');

const x = symbol('x');
const product = x.pow(2).mul(x.pow(3));
const deriv = derivative(product, x);
// Result: 5*x^4
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Chain Rule</h3>
        <p>d/dx(f(g(x))) = f'(g(x))·g'(x)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let inner = expr!(x ^ 2);
let outer = expr!(sin(inner));  // sin(x^2)

let deriv = outer.derivative(&x, 1);
// Result: cos(x^2) * 2*x
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, derivative, sin

x = symbol('x')
inner = x**2
outer = sin(inner)  # sin(x^2)

deriv = derivative(outer, x)
# Result: cos(x^2) * 2*x
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, derivative, parse } = require('mathhook');

const x = symbol('x');
const expr = parse('sin(x^2)');
const deriv = derivative(expr, x);
// Result: cos(x^2) * 2*x
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Partial Derivatives</h3>
        <p>Multivariable differentiation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);
let expr = expr!((x ^ 2) * y);

// Partial derivative with respect to x
let df_dx = expr.derivative(&x, 1);
// Result: 2*x*y

// Partial derivative with respect to y
let df_dy = expr.derivative(&y, 1);
// Result: x^2
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, derivative

x = symbol('x')
y = symbol('y')
expr = x**2 * y

# Partial derivative with respect to x
df_dx = derivative(expr, x)
# Result: 2*x*y

# Partial derivative with respect to y
df_dy = derivative(expr, y)
# Result: x^2
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, derivative } = require('mathhook');

const x = symbol('x');
const y = symbol('y');
const expr = x.pow(2).mul(y);

// Partial derivative with respect to x
const df_dx = derivative(expr, x);
// Result: 2*x*y

// Partial derivative with respect to y
const df_dy = derivative(expr, y);
// Result: x^2
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Higher-Order Derivatives</h3>
        <p>Second, third, or nth order derivatives</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 4);

// First derivative: 4*x^3
let first = expr.derivative(&x, 1);

// Second derivative: 12*x^2
let second = expr.derivative(&x, 2);

// Third derivative: 24*x
let third = expr.derivative(&x, 3);

// Fourth derivative: 24
let fourth = expr.derivative(&x, 4);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, derivative

x = symbol('x')
expr = x**4

# First derivative: 4*x^3
first = derivative(expr, x, order=1)

# Second derivative: 12*x^2
second = derivative(expr, x, order=2)

# Third derivative: 24*x
third = derivative(expr, x, order=3)

# Fourth derivative: 24
fourth = derivative(expr, x, order=4)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, derivative } = require('mathhook');

const x = symbol('x');
const expr = x.pow(4);

// First derivative: 4*x^3
const first = derivative(expr, x, { order: 1 });

// Second derivative: 12*x^2
const second = derivative(expr, x, { order: 2 });
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
const metaDescription = "Symbolic differentiation in MathHook uses automatic differentiation with the chain rule, product rule, quotient rule, and function-specific derivative rules.
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
