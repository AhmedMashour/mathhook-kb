<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Symbolic Calculus: Differentiation and Integration</h1>
      <p class="description">Symbolic differentiation and integration using automatic differentiation rules,
integration strategies, and the Risch algorithm. Supports chain rule, product rule,
quotient rule, and comprehensive integration techniques from table lookup to complete
Risch algorithm for elementary functions.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Differentiation</h3>
        <p>Compute derivatives using power rule and chain rule</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 3);

// First derivative: 3x^2
let derivative = expr.derivative(&x, 1);

// Second derivative: 6x
let second_derivative = expr.derivative(&x, 2);

// Complex function with chain rule
let expr = expr!(sin(x ^ 2));
let deriv = expr.derivative(&x, 1);
// Result: cos(x^2) * 2x
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, derivative

x = symbol('x')
expr = x**3

# First derivative: 3x^2
df = derivative(expr, x)

# Second derivative: 6x
d2f = derivative(expr, x, order=2)

# Complex function with chain rule
from mathhook import sin
expr = sin(x**2)
deriv = derivative(expr, x)
# Result: cos(x^2) * 2x
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse, derivative } from 'mathhook';

const x = symbol('x');
const expr = parse('x^3');

// First derivative: 3x^2
const df = derivative(expr, x);

// Second derivative: 6x
const d2f = derivative(expr, x, { order: 2 });

// Complex function with chain rule
const expr2 = parse('sin(x^2)');
const deriv = derivative(expr2, x);
// Result: cos(x^2) * 2x
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Product and Quotient Rules</h3>
        <p>Differentiate products and quotients</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Product rule: d/dx(x^2 * x^3) = 2x * x^3 + x^2 * 3x^2 = 5x^4
let f = expr!(x ^ 2);
let g = expr!(x ^ 3);
let product = expr!(mul: f, g);
let deriv = product.derivative(&x, 1);
// Result: 5*x^4

// Quotient rule: d/dx(x^2 / (x+1))
let numerator = expr!(x ^ 2);
let denominator = expr!(x + 1);
let quotient = expr!(div: numerator, denominator);
let deriv = quotient.derivative(&x, 1);
// Result: (2*x*(x+1) - x^2*1) / (x+1)^2
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, derivative

x = symbol('x')

# Product rule
f = x**2
g = x**3
product = f * g
deriv = derivative(product, x)
# Result: 5*x^4

# Quotient rule
numerator = x**2
denominator = x + 1
quotient = numerator / denominator
deriv = derivative(quotient, x)
# Result: (2*x*(x+1) - x^2) / (x+1)^2
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse, derivative } from 'mathhook';

const x = symbol('x');

// Product rule
const product = parse('x^2 * x^3');
const deriv1 = derivative(product, x);
// Result: 5*x^4

// Quotient rule
const quotient = parse('x^2 / (x + 1)');
const deriv2 = derivative(quotient, x);
// Result: (2*x*(x+1) - x^2) / (x+1)^2
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Partial Derivatives (Multivariable)</h3>
        <p>Compute partial derivatives with respect to each variable</p>
        
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
          <pre><code class="language-javascript">import { symbol, parse, derivative } from 'mathhook';

const x = symbol('x');
const y = symbol('y');
const expr = parse('x^2 * y');

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
        <h3>Basic Integration</h3>
        <p>Symbolic integration using layered strategy</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::calculus::integrals::Integration;

let x = symbol!(x);

// Simple polynomial (Layer 1: Table Lookup)
let expr = expr!(x ^ 2);
let result = expr.integrate(x.clone());
// Result: x^3/3 + C

// Rational function (Layer 2: Partial fractions)
let expr = expr!(1 / (x + 1));
let result = expr.integrate(x.clone());
// Result: ln|x+1| + C

// Trigonometric (Layer 3: Function registry)
let expr = expr!(sin(x));
let result = expr.integrate(x.clone());
// Result: -cos(x) + C
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, integrate

x = symbol('x')

# Simple polynomial
expr = x**2
result = integrate(expr, x)
# Result: x^3/3 + C

# Rational function
expr = 1 / (x + 1)
result = integrate(expr, x)
# Result: ln|x+1| + C

# Trigonometric
from mathhook import sin
expr = sin(x)
result = integrate(expr, x)
# Result: -cos(x) + C
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse, integrate } from 'mathhook';

const x = symbol('x');

// Simple polynomial
const expr = parse('x^2');
const result = integrate(expr, x);
// Result: x^3/3 + C

// Rational function
const expr2 = parse('1 / (x + 1)');
const result2 = integrate(expr2, x);
// Result: ln|x+1| + C

// Trigonometric
const expr3 = parse('sin(x)');
const result3 = integrate(expr3, x);
// Result: -cos(x) + C
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Integration by Parts and Substitution</h3>
        <p>Advanced integration techniques</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::calculus::integrals::Integration;

let x = symbol!(x);

// Integration by parts: ∫x*e^x dx
let expr = expr!(x * exp(x));
let result = expr.integrate(x.clone());
// Result: e^x(x-1) + C

// U-substitution: ∫2x*sin(x^2) dx
let expr = expr!(2 * x * sin(x ^ 2));
let result = expr.integrate(x.clone());
// Result: -cos(x^2) + C
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, integrate, exp, sin

x = symbol('x')

# Integration by parts
expr = x * exp(x)
result = integrate(expr, x)
# Result: e^x(x-1) + C

# U-substitution
expr = 2 * x * sin(x**2)
result = integrate(expr, x)
# Result: -cos(x^2) + C
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse, integrate } from 'mathhook';

const x = symbol('x');

// Integration by parts
const expr = parse('x * exp(x)');
const result = integrate(expr, x);
// Result: e^x(x-1) + C

// U-substitution
const expr2 = parse('2 * x * sin(x^2)');
const result2 = integrate(expr2, x);
// Result: -cos(x^2) + C
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Real-World Application: Velocity and Acceleration</h3>
        <p>Physics application of derivatives</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let t = symbol!(t);
let position = expr!((t ^ 3) - (6 * (t ^ 2)) + (9 * t));

let velocity = position.derivative(&t, 1);
// v(t) = 3t^2 - 12t + 9

let acceleration = position.derivative(&t, 2);
// a(t) = 6t - 12

// Find when velocity is zero (critical points)
// Solve: 3t^2 - 12t + 9 = 0
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, derivative

t = symbol('t')
position = t**3 - 6*t**2 + 9*t

velocity = derivative(position, t)
# v(t) = 3t^2 - 12t + 9

acceleration = derivative(position, t, order=2)
# a(t) = 6t - 12
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse, derivative } from 'mathhook';

const t = symbol('t');
const position = parse('t^3 - 6*t^2 + 9*t');

const velocity = derivative(position, t);
// v(t) = 3t^2 - 12t + 9

const acceleration = derivative(position, t, { order: 2 });
// a(t) = 6t - 12
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
const metaDescription = "Symbolic differentiation and integration using automatic differentiation rules,
integration strategies, and the Risch algorithm. Supports chain rule, product rule,
quotient rule, and comprehensive integration techniques from table lookup to complete
Risch algorithm for elementary functions.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Symbolic Calculus: Differentiation and Integration',
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
