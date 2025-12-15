<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Function System</h1>
      <p class="description">MathHook provides a comprehensive mathematical function system with intelligent evaluation,
symbolic manipulation, and educational explanations. Functions are first-class expressions
supporting exact symbolic computation and high-performance numerical evaluation through
a modular intelligence architecture.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">Functions in MathHook follow standard mathematical definitions:

**Trigonometric**: $$\sin(x), \cos(x), \tan(x)$$ with periodicity $$2\pi$$

**Exponential/Logarithmic**: $$e^x, \ln(x), \log_b(x) = \frac{\ln(x)}{\ln(b)}$$

**Special Functions**: $$\Gamma(n) = \int_0^{\infty} t^{n-1} e^{-t} dt$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Creating Functions with Macros</h3>
        <p>Using function! and expr! macros for ergonomic function creation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Single argument functions
let sine = function!(sin, x);
let cosine = function!(cos, x);

// Multi-argument functions
let log_base = function!(log, x, 10);

// Using expr! macro
let trig_identity = expr!(sin(x)^2 + cos(x)^2);
assert_eq!(trig_identity.simplify(), Expression::integer(1));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, function, expr

x = symbol('x')

# Single argument functions
sine = function('sin', x)
cosine = function('cos', x)

# Multi-argument functions
log_base = function('log', x, 10)

# Using expr
trig_identity = expr('sin(x)^2 + cos(x)^2')
assert trig_identity.simplify() == 1
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, func, expr } = require('mathhook-node');

const x = symbol('x');

// Single argument functions
const sine = func('sin', x);
const cosine = func('cos', x);

// Multi-argument functions
const logBase = func('log', x, 10);

// Using expr
const trigIdentity = expr('sin(x)^2 + cos(x)^2');
console.assert(trigIdentity.simplify().equals(1));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Trigonometric Functions with Exact Values</h3>
        <p>Automatic recognition of exact trigonometric values at special angles</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Exact values recognized
assert_eq!(expr!(sin(0)), expr!(0));
assert_eq!(expr!(sin(pi/6)), expr!(1/2));
assert_eq!(expr!(sin(pi/4)), expr!(sqrt(2)/2));
assert_eq!(expr!(sin(pi/2)), expr!(1));

assert_eq!(expr!(cos(0)), expr!(1));
assert_eq!(expr!(cos(pi/3)), expr!(1/2));
assert_eq!(expr!(cos(pi/2)), expr!(0));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr

# Exact values recognized
assert expr('sin(0)') == 0
assert expr('sin(pi/6)') == expr('1/2')
assert expr('sin(pi/4)') == expr('sqrt(2)/2')
assert expr('sin(pi/2)') == 1

assert expr('cos(0)') == 1
assert expr('cos(pi/3)') == expr('1/2')
assert expr('cos(pi/2)') == 0
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr } = require('mathhook-node');

// Exact values recognized
console.assert(expr('sin(0)').equals(0));
console.assert(expr('sin(pi/6)').equals(expr('1/2')));
console.assert(expr('sin(pi/4)').equals(expr('sqrt(2)/2')));
console.assert(expr('sin(pi/2)').equals(1));

console.assert(expr('cos(0)').equals(1));
console.assert(expr('cos(pi/3)').equals(expr('1/2')));
console.assert(expr('cos(pi/2)').equals(0));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Logarithm and Exponential Identities</h3>
        <p>Automatic application of logarithm laws and exponential identities</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let a = symbol!(a);
let b = symbol!(b);
let n = symbol!(n);

// Logarithm laws
assert_eq!(expr!(ln(a*b)).expand(), expr!(ln(a) + ln(b)));
assert_eq!(expr!(ln(a/b)).expand(), expr!(ln(a) - ln(b)));
assert_eq!(expr!(ln(a^n)).expand(), expr!(n*ln(a)));

// Exponential identities
assert_eq!(expr!(e^(ln(a))).simplify(), a);
assert_eq!(expr!(ln(e^a)).simplify(), a);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr

a = symbol('a')
b = symbol('b')
n = symbol('n')

# Logarithm laws
assert expr('ln(a*b)').expand() == expr('ln(a) + ln(b)')
assert expr('ln(a/b)').expand() == expr('ln(a) - ln(b)')
assert expr('ln(a^n)').expand() == expr('n*ln(a)')

# Exponential identities
assert expr('e^(ln(a))').simplify() == a
assert expr('ln(e^a)').simplify() == a
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook-node');

const a = symbol('a');
const b = symbol('b');
const n = symbol('n');

// Logarithm laws
console.assert(expr('ln(a*b)').expand().equals(expr('ln(a) + ln(b)')));
console.assert(expr('ln(a/b)').expand().equals(expr('ln(a) - ln(b)')));
console.assert(expr('ln(a^n)').expand().equals(expr('n*ln(a)')));

// Exponential identities
console.assert(expr('e^(ln(a))').simplify().equals(a));
console.assert(expr('ln(e^a)').simplify().equals(a));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Function Derivatives (Automatic Chain Rule)</h3>
        <p>Functions know their derivatives with automatic chain rule application</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Basic derivatives
assert_eq!(expr!(sin(x)).derivative(&x, 1), expr!(cos(x)));
assert_eq!(expr!(cos(x)).derivative(&x, 1), expr!(-sin(x)));
assert_eq!(expr!(exp(x)).derivative(&x, 1), expr!(exp(x)));
assert_eq!(expr!(ln(x)).derivative(&x, 1), expr!(1/x));

// Chain rule automatic
let f = expr!(sin(x^2));
assert_eq!(f.derivative(&x, 1), expr!(2*x*cos(x^2)));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr

x = symbol('x')

# Basic derivatives
assert expr('sin(x)').derivative(x) == expr('cos(x)')
assert expr('cos(x)').derivative(x) == expr('-sin(x)')
assert expr('exp(x)').derivative(x) == expr('exp(x)')
assert expr('ln(x)').derivative(x) == expr('1/x')

# Chain rule automatic
f = expr('sin(x^2)')
assert f.derivative(x) == expr('2*x*cos(x^2)')
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook-node');

const x = symbol('x');

// Basic derivatives
console.assert(expr('sin(x)').derivative(x).equals(expr('cos(x)')));
console.assert(expr('cos(x)').derivative(x).equals(expr('-sin(x)')));
console.assert(expr('exp(x)').derivative(x).equals(expr('exp(x)')));
console.assert(expr('ln(x)').derivative(x).equals(expr('1/x')));

// Chain rule automatic
const f = expr('sin(x^2)');
console.assert(f.derivative(x).equals(expr('2*x*cos(x^2)')));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Special Functions (Gamma and Bessel)</h3>
        <p>Advanced special functions for scientific and engineering applications</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Gamma function (factorial generalization)
assert_eq!(expr!(gamma(1)), expr!(1));   // Γ(1) = 0! = 1
assert_eq!(expr!(gamma(5)), expr!(24));  // Γ(5) = 4! = 24
assert_eq!(expr!(gamma(1/2)), expr!(sqrt(pi)));

// Bessel functions (wave propagation)
let x = symbol!(x);
let bessel_j0 = expr!(bessel_j(0, x));
let bessel_y0 = expr!(bessel_y(0, x));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol

# Gamma function
assert expr('gamma(1)') == 1
assert expr('gamma(5)') == 24
assert expr('gamma(1/2)') == expr('sqrt(pi)')

# Bessel functions
x = symbol('x')
bessel_j0 = expr('bessel_j(0, x)')
bessel_y0 = expr('bessel_y(0, x)')
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook-node');

// Gamma function
console.assert(expr('gamma(1)').equals(1));
console.assert(expr('gamma(5)').equals(24));
console.assert(expr('gamma(1/2)').equals(expr('sqrt(pi)')));

// Bessel functions
const x = symbol('x');
const besselJ0 = expr('bessel_j(0, x)');
const besselY0 = expr('bessel_y(0, x)');
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Real-World Physics Application</h3>
        <p>Damped harmonic oscillator using exponential and trigonometric functions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Damped harmonic motion: x(t) = A*e^(-γt)*cos(ωt + φ)
let t = symbol!(t);
let A = expr!(1);
let gamma = expr!(0.1);
let omega = expr!(2*pi);
let phi = expr!(0);

let position = expr!(A * e^(-gamma*t) * cos(omega*t + phi));
let velocity = position.derivative(&t, 1);
let acceleration = velocity.derivative(&t, 1);

// Verify: ẍ + 2γẋ + ω²x = 0
let lhs = expr!(acceleration + 2*gamma*velocity + (omega^2)*position);
assert_eq!(lhs.simplify(), expr!(0));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr

# Damped harmonic motion
t = symbol('t')
position = expr('e^(-0.1*t) * cos(2*pi*t)')
velocity = position.derivative(t)
acceleration = velocity.derivative(t)

# Differential equation verification
gamma = 0.1
omega = expr('2*pi')
lhs = expr(f'acceleration + 2*{gamma}*velocity + omega^2*position')
# Should simplify to 0
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook-node');

// Damped harmonic motion
const t = symbol('t');
const position = expr('e^(-0.1*t) * cos(2*pi*t)');
const velocity = position.derivative(t);
const acceleration = velocity.derivative(t);

// Differential equation verification
const gamma = 0.1;
const omega = expr('2*pi');
// Should satisfy: ẍ + 2γẋ + ω²x = 0
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
const metaDescription = "MathHook provides a comprehensive mathematical function system with intelligent evaluation,
symbolic manipulation, and educational explanations. Functions are first-class expressions
supporting exact symbolic computation and high-performance numerical evaluation through
a modular intelligence architecture.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Function System',
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
