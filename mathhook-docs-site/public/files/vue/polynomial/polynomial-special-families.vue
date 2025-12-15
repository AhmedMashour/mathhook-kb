<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Special Polynomial Families</h1>
      <p class="description">Classical orthogonal polynomial families including Legendre, Chebyshev (1st and 2nd kind),
Hermite, and Laguerre polynomials with both symbolic expansion and numerical evaluation capabilities.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Orthogonal Polynomials**: A sequence $\{P_n(x)\}$ satisfying orthogonality relation:

$$\int_a^b P_n(x) P_m(x) w(x) \, dx = 0 \quad \text{for } n \neq m$$

where $w(x)$ is the weight function on interval $[a, b]$.

**Three-Term Recurrence**: All orthogonal polynomials satisfy:

$$P_{n+1}(x) = (a_n x + b_n) P_n(x) - c_n P_{n-1}(x)$$

**Family Definitions**:

1. **Legendre**: Interval $[-1, 1]$, $w(x) = 1$
   - Differential equation: $(1-x^2)P_n'' - 2xP_n' + n(n+1)P_n = 0$
   - Recurrence: $P_{n+1} = \frac{(2n+1)xP_n - nP_{n-1}}{n+1}$

2. **Chebyshev (1st)**: Interval $[-1, 1]$, $w(x) = \frac{1}{\sqrt{1-x^2}}$
   - Definition: $T_n(\cos\theta) = \cos(n\theta)$
   - Recurrence: $T_{n+1} = 2xT_n - T_{n-1}$

3. **Chebyshev (2nd)**: Interval $[-1, 1]$, $w(x) = \sqrt{1-x^2}$
   - Definition: $U_n(\cos\theta) = \frac{\sin((n+1)\theta)}{\sin\theta}$
   - Recurrence: $U_{n+1} = 2xU_n - U_{n-1}$

4. **Hermite**: Interval $(-\infty, \infty)$, $w(x) = e^{-x^2}$
   - Differential equation: $H_n'' - 2xH_n' + 2nH_n = 0$
   - Rodriguez formula: $H_n(x) = (-1)^n e^{x^2} \frac{d^n}{dx^n}(e^{-x^2})$
   - Recurrence: $H_{n+1} = 2xH_n - 2nH_{n-1}$

5. **Laguerre**: Interval $[0, \infty)$, $w(x) = e^{-x}$
   - Differential equation: $xL_n'' + (1-x)L_n' + nL_n = 0$
   - Recurrence: $L_{n+1} = \frac{(2n+1-x)L_n - nL_{n-1}}{n+1}$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Legendre Polynomials</h3>
        <p>Solutions to Legendre's differential equation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::special_families::Legendre;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

let x = symbol!(x);

// Symbolic expansion
let p0 = Legendre::polynomial(0, &x);  // 1
let p1 = Legendre::polynomial(1, &x);  // x
let p2 = Legendre::polynomial(2, &x);  // (3x^2 - 1)/2

// Numerical evaluation
let val = Legendre::evaluate(2, 0.5);  // P_2(0.5) = -0.125

// Recurrence: P_{n+1} = ((2n+1)x*P_n - n*P_{n-1}) / (n+1)
let (a, b, c) = Legendre::recurrence_coefficients(2);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol
from mathhook.polynomial.special_families import Legendre

x = symbol('x')

# Symbolic expansion
p0 = Legendre.polynomial(0, x)  # 1
p1 = Legendre.polynomial(1, x)  # x
p2 = Legendre.polynomial(2, x)  # (3*x^2 - 1)/2

# Numerical evaluation
val = Legendre.evaluate(2, 0.5)  # P_2(0.5) = -0.125

# Recurrence: P_{n+1} = ((2n+1)*x*P_n - n*P_{n-1}) / (n+1)
a, b, c = Legendre.recurrence_coefficients(2)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');
const { Legendre } = require('mathhook/polynomial/special_families');

const x = symbol('x');

// Symbolic expansion
const p0 = Legendre.polynomial(0, x);  // 1
const p1 = Legendre.polynomial(1, x);  // x
const p2 = Legendre.polynomial(2, x);  // (3*x^2 - 1)/2

// Numerical evaluation
const val = Legendre.evaluate(2, 0.5);  // P_2(0.5) = -0.125

// Recurrence: P_{n+1} = ((2n+1)*x*P_n - n*P_{n-1}) / (n+1)
const [a, b, c] = Legendre.recurrenceCoefficients(2);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Chebyshev Polynomials (First Kind)</h3>
        <p>Defined by T_n(cos(theta)) = cos(n*theta)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::special_families::ChebyshevT;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

let x = symbol!(x);

// Symbolic
let t0 = ChebyshevT::polynomial(0, &x);  // 1
let t1 = ChebyshevT::polynomial(1, &x);  // x
let t2 = ChebyshevT::polynomial(2, &x);  // 2x^2 - 1

// Numerical
let val = ChebyshevT::evaluate(2, 0.5);  // T_2(0.5) = -0.5

// Recurrence: T_{n+1} = 2x*T_n - T_{n-1}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol
from mathhook.polynomial.special_families import ChebyshevT

x = symbol('x')

# Symbolic
t0 = ChebyshevT.polynomial(0, x)  # 1
t1 = ChebyshevT.polynomial(1, x)  # x
t2 = ChebyshevT.polynomial(2, x)  # 2*x^2 - 1

# Numerical
val = ChebyshevT.evaluate(2, 0.5)  # T_2(0.5) = -0.5

# Recurrence: T_{n+1} = 2*x*T_n - T_{n-1}
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');
const { ChebyshevT } = require('mathhook/polynomial/special_families');

const x = symbol('x');

// Symbolic
const t0 = ChebyshevT.polynomial(0, x);  // 1
const t1 = ChebyshevT.polynomial(1, x);  // x
const t2 = ChebyshevT.polynomial(2, x);  // 2*x^2 - 1

// Numerical
const val = ChebyshevT.evaluate(2, 0.5);  // T_2(0.5) = -0.5

// Recurrence: T_{n+1} = 2*x*T_n - T_{n-1}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Hermite Polynomials</h3>
        <p>Solutions to Hermite's equation (physicist's convention)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::special_families::Hermite;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

let x = symbol!(x);

// Symbolic
let h0 = Hermite::polynomial(0, &x);  // 1
let h1 = Hermite::polynomial(1, &x);  // 2x
let h2 = Hermite::polynomial(2, &x);  // 4x^2 - 2

// Numerical
let val = Hermite::evaluate(1, 0.5);  // H_1(0.5) = 1

// Recurrence: H_{n+1} = 2x*H_n - 2n*H_{n-1}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol
from mathhook.polynomial.special_families import Hermite

x = symbol('x')

# Symbolic
h0 = Hermite.polynomial(0, x)  # 1
h1 = Hermite.polynomial(1, x)  # 2*x
h2 = Hermite.polynomial(2, x)  # 4*x^2 - 2

# Numerical
val = Hermite.evaluate(1, 0.5)  # H_1(0.5) = 1

# Recurrence: H_{n+1} = 2*x*H_n - 2*n*H_{n-1}
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');
const { Hermite } = require('mathhook/polynomial/special_families');

const x = symbol('x');

// Symbolic
const h0 = Hermite.polynomial(0, x);  // 1
const h1 = Hermite.polynomial(1, x);  // 2*x
const h2 = Hermite.polynomial(2, x);  // 4*x^2 - 2

// Numerical
const val = Hermite.evaluate(1, 0.5);  // H_1(0.5) = 1

// Recurrence: H_{n+1} = 2*x*H_n - 2*n*H_{n-1}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Laguerre Polynomials</h3>
        <p>Solutions to Laguerre's equation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::special_families::Laguerre;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

let x = symbol!(x);

// Symbolic
let l0 = Laguerre::polynomial(0, &x);  // 1
let l1 = Laguerre::polynomial(1, &x);  // 1 - x
let l2 = Laguerre::polynomial(2, &x);  // (x^2 - 4x + 2)/2

// Numerical
let val = Laguerre::evaluate(1, 0.5);  // L_1(0.5) = 0.5

// Recurrence: L_{n+1} = ((2n+1-x)*L_n - n*L_{n-1}) / (n+1)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol
from mathhook.polynomial.special_families import Laguerre

x = symbol('x')

# Symbolic
l0 = Laguerre.polynomial(0, x)  # 1
l1 = Laguerre.polynomial(1, x)  # 1 - x
l2 = Laguerre.polynomial(2, x)  # (x^2 - 4*x + 2)/2

# Numerical
val = Laguerre.evaluate(1, 0.5)  # L_1(0.5) = 0.5

# Recurrence: L_{n+1} = ((2*n+1-x)*L_n - n*L_{n-1}) / (n+1)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');
const { Laguerre } = require('mathhook/polynomial/special_families');

const x = symbol('x');

// Symbolic
const l0 = Laguerre.polynomial(0, x);  // 1
const l1 = Laguerre.polynomial(1, x);  // 1 - x
const l2 = Laguerre.polynomial(2, x);  // (x^2 - 4*x + 2)/2

// Numerical
const val = Laguerre.evaluate(1, 0.5);  // L_1(0.5) = 0.5

// Recurrence: L_{n+1} = ((2*n+1-x)*L_n - n*L_{n-1}) / (n+1)
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Variable Substitution</h3>
        <p>Use any variable symbol in polynomial generation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::special_families::Legendre;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

// Use variable t instead of x
let t = symbol!(t);
let p2_t = Legendre::polynomial(2, &t);
// Result uses t: (3t^2 - 1)/2
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol
from mathhook.polynomial.special_families import Legendre

# Use variable t instead of x
t = symbol('t')
p2_t = Legendre.polynomial(2, t)
# Result uses t: (3*t^2 - 1)/2
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');
const { Legendre } = require('mathhook/polynomial/special_families');

// Use variable t instead of x
const t = symbol('t');
const p2T = Legendre.polynomial(2, t);
// Result uses t: (3*t^2 - 1)/2
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
const metaDescription = "Classical orthogonal polynomial families including Legendre, Chebyshev (1st and 2nd kind),
Hermite, and Laguerre polynomials with both symbolic expansion and numerical evaluation capabilities.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Special Polynomial Families',
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
