<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Mathematical Constants</h1>
      <p class="description">MathHook provides built-in mathematical constants with exact symbolic representation
and high-precision numerical evaluation. Constants include π, e, i (imaginary unit),
golden ratio (φ), Euler-Mascheroni constant (γ), infinity, and undefined values.
All constants preserve mathematical exactness throughout symbolic computations.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Fundamental Constants:**

- **Pi**: $$\pi = 3.14159265358979323846\ldots$$ (ratio of circle's circumference to diameter)
- **Euler's Number**: $$e = 2.71828182845904523536\ldots = \lim_{n \to \infty} \left(1 + \frac{1}{n}\right)^n$$
- **Imaginary Unit**: $$i^2 = -1$$
- **Golden Ratio**: $$\varphi = \frac{1 + \sqrt{5}}{2} = 1.618033988749895\ldots$$
- **Euler-Mascheroni**: $$\gamma = \lim_{n \to \infty} \left(\sum_{k=1}^{n} \frac{1}{k} - \ln(n)\right) = 0.5772156649015329\ldots$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Using Pi in Expressions</h3>
        <p>Pi constant for exact trigonometric and geometric calculations</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let pi = Expression::pi();
let r = symbol!(r);

// Circle area: A = πr²
let area = expr!(pi * r^2);

// Circumference: C = 2πr
let circumference = expr!(2*pi*r);

// Exact trigonometric values
assert_eq!(expr!(sin(pi)), expr!(0));
assert_eq!(expr!(cos(pi)), expr!(-1));
assert_eq!(expr!(sin(pi/2)), expr!(1));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression, symbol, expr

pi = Expression.pi()
r = symbol('r')

# Circle area
area = expr('pi * r^2')

# Circumference
circumference = expr('2*pi*r')

# Exact trig values
assert expr('sin(pi)') == 0
assert expr('cos(pi)') == -1
assert expr('sin(pi/2)') == 1
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { Expression, symbol, expr } = require('mathhook-node');

const pi = Expression.pi();
const r = symbol('r');

// Circle area
const area = expr('pi * r^2');

// Circumference
const circumference = expr('2*pi*r');

// Exact trig values
console.assert(expr('sin(pi)').equals(0));
console.assert(expr('cos(pi)').equals(-1));
console.assert(expr('sin(pi/2)').equals(1));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Euler's Number (e) and Natural Logarithm</h3>
        <p>Using e for exponential growth and logarithmic identities</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let e = Expression::e();
let x = symbol!(x);

// Exponential growth
let growth = expr!(e^(r*t));

// Logarithm identities
assert_eq!(expr!(ln(e)), expr!(1));
assert_eq!(expr!(ln(e^x)).simplify(), x);
assert_eq!(expr!(e^(ln(x))).simplify(), x);

// Derivative property: d/dx(e^x) = e^x
let exp_x = expr!(e^x);
assert_eq!(exp_x.derivative(&x, 1), exp_x);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression, symbol, expr

e = Expression.e()
x = symbol('x')

# Exponential growth
growth = expr('e^(r*t)')

# Logarithm identities
assert expr('ln(e)') == 1
assert expr('ln(e^x)').simplify() == x
assert expr('e^(ln(x))').simplify() == x

# Derivative property
exp_x = expr('e^x')
assert exp_x.derivative(x) == exp_x
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { Expression, symbol, expr } = require('mathhook-node');

const e = Expression.e();
const x = symbol('x');

// Exponential growth
const growth = expr('e^(r*t)');

// Logarithm identities
console.assert(expr('ln(e)').equals(1));
console.assert(expr('ln(e^x)').simplify().equals(x));
console.assert(expr('e^(ln(x))').simplify().equals(x));

// Derivative property
const expX = expr('e^x');
console.assert(expX.derivative(x).equals(expX));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Imaginary Unit (i) and Euler's Identity</h3>
        <p>Complex numbers and the most beautiful equation in mathematics</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let i = Expression::i();

// Complex numbers
let z = expr!(3 + 4*i);
let magnitude = expr!(sqrt((3^2) + (4^2)));
assert_eq!(magnitude.simplify(), expr!(5));

// Euler's identity: e^(iπ) + 1 = 0
let euler_identity = expr!(e^(i*pi) + 1);
assert_eq!(euler_identity.simplify(), expr!(0));

// Powers of i
assert_eq!(expr!(i^2).simplify(), expr!(-1));
assert_eq!(expr!(i^3).simplify(), expr!(-i));
assert_eq!(expr!(i^4).simplify(), expr!(1));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression, expr

i = Expression.i()

# Complex numbers
z = expr('3 + 4*i')
magnitude = expr('sqrt(3^2 + 4^2)')
assert magnitude.simplify() == 5

# Euler's identity
euler = expr('e^(i*pi) + 1')
assert euler.simplify() == 0

# Powers of i
assert expr('i^2').simplify() == -1
assert expr('i^3').simplify() == expr('-i')
assert expr('i^4').simplify() == 1
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { Expression, expr } = require('mathhook-node');

const i = Expression.i();

// Complex numbers
const z = expr('3 + 4*i');
const magnitude = expr('sqrt(3^2 + 4^2)');
console.assert(magnitude.simplify().equals(5));

// Euler's identity
const euler = expr('e^(i*pi) + 1');
console.assert(euler.simplify().equals(0));

// Powers of i
console.assert(expr('i^2').simplify().equals(-1));
console.assert(expr('i^3').simplify().equals(expr('-i')));
console.assert(expr('i^4').simplify().equals(1));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Golden Ratio and Fibonacci Connection</h3>
        <p>Golden ratio in geometry and its relationship to Fibonacci sequence</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let phi = Expression::golden_ratio();

// Exact form: φ = (1 + √5) / 2
let phi_exact = expr!((1 + sqrt(5)) / 2);
assert_eq!(phi.simplify(), phi_exact.simplify());

// Golden ratio property: φ² = φ + 1
assert_eq!(expr!(phi^2).simplify(), expr!(phi + 1));

// Fibonacci limit: lim F(n+1)/F(n) = φ
// φ ≈ 1.618033988749895
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression, expr

phi = Expression.golden_ratio()

# Exact form
phi_exact = expr('(1 + sqrt(5)) / 2')
assert phi.simplify() == phi_exact.simplify()

# Golden ratio property
assert expr('phi^2').simplify() == expr('phi + 1')

# Numerical value
# φ ≈ 1.618033988749895
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { Expression, expr } = require('mathhook-node');

const phi = Expression.goldenRatio();

// Exact form
const phiExact = expr('(1 + sqrt(5)) / 2');
console.assert(phi.simplify().equals(phiExact.simplify()));

// Golden ratio property
console.assert(expr('phi^2').simplify().equals(expr('phi + 1')));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Infinity and Undefined Values</h3>
        <p>Working with unbounded limits and indeterminate forms</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let inf = Expression::infinity();
let neg_inf = Expression::negative_infinity();

// Defined operations
assert_eq!(expr!(infinity + 1), expr!(infinity));
assert_eq!(expr!(infinity * 2), expr!(infinity));
assert_eq!(expr!(1 / infinity).simplify(), expr!(0));

// Limits
let x = symbol!(x);
let limit = expr!(lim(1/x, x, infinity));
assert_eq!(limit.simplify(), expr!(0));

// Undefined forms (indeterminate)
// infinity - infinity → Undefined
// infinity / infinity → Undefined
// 0 * infinity → Undefined
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression, symbol, expr

inf = Expression.infinity()

# Defined operations
assert expr('infinity + 1') == inf
assert expr('infinity * 2') == inf
assert expr('1 / infinity').simplify() == 0

# Limits
x = symbol('x')
limit = expr('lim(1/x, x, infinity)')
assert limit.simplify() == 0

# Indeterminate forms return Undefined
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { Expression, symbol, expr } = require('mathhook-node');

const inf = Expression.infinity();

// Defined operations
console.assert(expr('infinity + 1').equals(inf));
console.assert(expr('infinity * 2').equals(inf));
console.assert(expr('1 / infinity').simplify().equals(0));

// Limits
const x = symbol('x');
const limit = expr('lim(1/x, x, infinity)');
console.assert(limit.simplify().equals(0));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Real-World Physics: Harmonic Motion</h3>
        <p>Using π and e in simple harmonic oscillator equations</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Simple harmonic oscillator: x(t) = A*cos(ω*t + φ)
let t = symbol!(t);
let A = expr!(2);
let omega = expr!(pi);
let phi = expr!(pi/4);

let position = expr!(A * cos(omega*t + phi));
let velocity = position.derivative(&t, 1);
let acceleration = velocity.derivative(&t, 1);

// Verify: a = -ω²x
assert_eq!(acceleration, expr!(-(omega^2) * position));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr

# Harmonic oscillator
t = symbol('t')
position = expr('2 * cos(pi*t + pi/4)')
velocity = position.derivative(t)
acceleration = velocity.derivative(t)

# Verify differential equation
omega = expr('pi')
# a = -ω²x
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook-node');

// Harmonic oscillator
const t = symbol('t');
const position = expr('2 * cos(pi*t + pi/4)');
const velocity = position.derivative(t);
const acceleration = velocity.derivative(t);

// Verify: a = -ω²x
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
const metaDescription = "MathHook provides built-in mathematical constants with exact symbolic representation
and high-precision numerical evaluation. Constants include π, e, i (imaginary unit),
golden ratio (φ), Euler-Mascheroni constant (γ), infinity, and undefined values.
All constants preserve mathematical exactness throughout symbolic computations.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Mathematical Constants',
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
