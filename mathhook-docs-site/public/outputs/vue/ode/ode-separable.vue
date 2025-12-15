<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Separable ODEs</h1>
      <p class="description">Separable ODEs are the most important and frequently encountered class of first-order differential
equations. MathHook provides a robust solver that handles both general and particular solutions
with automatic variable separation and symbolic integration.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">A first-order ODE $$\frac{dy}{dx} = f(x,y)$$ is **separable** if it can be written as:

$$\frac{dy}{dx} = g(x) \cdot h(y)$$

where $g(x)$ is a function of **only** $x$ and $h(y)$ is a function of **only** $y$.
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Simple Linear ODE</h3>
        <p>Solve dy/dx = x</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let solution = solver.solve(&expr!(x), &y, &x, None)?;
// Result: y = x²/2 + C
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

solution = solver.solve('x', y, x, None)
# Result: y = x²/2 + C
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const solution = solver.solve('x', y, x, null);
// Result: y = x²/2 + C
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Exponential Growth</h3>
        <p>Solve dy/dx = y (exponential growth/decay model)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let solution = solver.solve(&expr!(y), &y, &x, None)?;
// Result: y = Ce^x
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

solution = solver.solve('y', y, x, None)
# Result: y = Ce^x
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const solution = solver.solve('y', y, x, null);
// Result: y = Ce^x
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Product Form</h3>
        <p>Solve dy/dx = xy (nonlinear growth model)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let solution = solver.solve(&expr!(x * y), &y, &x, None)?;
// Result: y = Ce^(x²/2)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

solution = solver.solve('x*y', y, x, None)
# Result: y = Ce^(x²/2)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const solution = solver.solve('x*y', y, x, null);
// Result: y = Ce^(x²/2)
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Initial Value Problem</h3>
        <p>Solve dy/dx = x with y(0) = 1</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let ic = Some((expr!(0), expr!(1))); // y(0) = 1
let solution = solver.solve(&expr!(x), &y, &x, ic)?;
// Result: y = x²/2 + 1
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

ic = (expr('0'), expr('1'))  # y(0) = 1
solution = solver.solve('x', y, x, ic)
# Result: y = x²/2 + 1
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const ic = [expr('0'), expr('1')];  // y(0) = 1
const solution = solver.solve('x', y, x, ic);
// Result: y = x²/2 + 1
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Rational Function</h3>
        <p>Solve dy/dx = x/y</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let rhs = expr!(x / y);
let solution = solver.solve(&rhs, &y, &x, None)?;
// Result: y² - x² = C (implicit) or y = ±√(x² + C) (explicit)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

solution = solver.solve('x/y', y, x, None)
# Result: y² - x² = C
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const solution = solver.solve('x/y', y, x, null);
// Result: y² - x² = C
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
const metaDescription = "Separable ODEs are the most important and frequently encountered class of first-order differential
equations. MathHook provides a robust solver that handles both general and particular solutions
with automatic variable separation and symbolic integration.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Separable ODEs',
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
