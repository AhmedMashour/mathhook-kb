<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Polynomial Division and Factorization</h1>
      <p class="description">Polynomial division algorithms including long division, exact division, and factorization capabilities
such as square-free factorization, resultant, and discriminant computation.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Polynomial Long Division**: For polynomials $f(x), g(x)$ with $\deg(g) \leq \deg(f)$:

$$f(x) = g(x) \cdot q(x) + r(x)$$

where $q(x)$ is the quotient and $r(x)$ is the remainder with $\deg(r) < \deg(g)$.

**Resultant**: The resultant $\text{Res}(f, g)$ of polynomials $f, g$ of degrees $m, n$ is:

$$\text{Res}(f, g) = a_n^m \cdot b_m^n \cdot \prod_{i,j} (\alpha_i - \beta_j)$$

where $\alpha_i, \beta_j$ are roots of $f, g$ respectively. Properties:
- $\text{Res}(f, g) = 0 \iff f$ and $g$ share a common root
- Computed as determinant of Sylvester matrix

**Discriminant**: For polynomial $f(x)$ of degree $n$ with leading coefficient $a_n$:

$$\text{disc}(f) = \frac{(-1)^{n(n-1)/2}}{a_n} \cdot \text{Res}(f, f')$$

Properties:
- $\text{disc}(f) = 0 \iff f$ has a repeated root
- For quadratic $ax^2 + bx + c$: $\text{disc} = b^2 - 4ac$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Polynomial Long Division</h3>
        <p>Compute quotient and remainder using standard division algorithm</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::algorithms::polynomial_long_division;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// Divide (x^2 - 1) by (x - 1)
let dividend = expr!((x ^ 2) - 1);
let divisor = expr!(x - 1);

let (quotient, remainder) = polynomial_long_division(&dividend, &divisor, &x).unwrap();

// quotient = x + 1
// remainder = 0
// Verify: dividend = divisor * quotient + remainder
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol
from mathhook.polynomial.algorithms import polynomial_long_division

x = symbol('x')

# Divide (x^2 - 1) by (x - 1)
dividend = expr('x^2 - 1')
divisor = expr('x - 1')

quotient, remainder = polynomial_long_division(dividend, divisor, x)

# quotient = x + 1
# remainder = 0
# Verify: dividend = divisor * quotient + remainder
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');
const { polynomialLongDivision } = require('mathhook/polynomial/algorithms');

const x = symbol('x');

// Divide (x^2 - 1) by (x - 1)
const dividend = expr('x^2 - 1');
const divisor = expr('x - 1');

const [quotient, remainder] = polynomialLongDivision(dividend, divisor, x);

// quotient = x + 1
// remainder = 0
// Verify: dividend = divisor * quotient + remainder
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Exact Division</h3>
        <p>Division that errors if remainder is non-zero</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::algorithms::exact_division;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// x^2 / x = x (exact)
let dividend = expr!(x ^ 2);
let divisor = expr!(x);

match exact_division(&dividend, &divisor, &x) {
    Ok(quotient) => println!("Exact quotient: {}", quotient),
    Err(e) => println!("Division not exact: {:?}", e),
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol
from mathhook.polynomial.algorithms import exact_division

x = symbol('x')

# x^2 / x = x (exact)
dividend = expr('x^2')
divisor = expr('x')

try:
    quotient = exact_division(dividend, divisor, x)
    print(f"Exact quotient: {quotient}")
except Exception as e:
    print(f"Division not exact: {e}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');
const { exactDivision } = require('mathhook/polynomial/algorithms');

const x = symbol('x');

// x^2 / x = x (exact)
const dividend = expr('x^2');
const divisor = expr('x');

try {
    const quotient = exactDivision(dividend, divisor, x);
    console.log(`Exact quotient: ${quotient}`);
} catch (e) {
    console.log(`Division not exact: ${e}`);
}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Trait-Based Division</h3>
        <p>Use PolynomialArithmetic trait for ergonomic API</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::PolynomialArithmetic;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

let f = expr!((x ^ 3) - 1);
let g = expr!(x - 1);

// Returns (quotient, remainder)
let (q, r) = f.poly_div(&g, &x).unwrap();
// q = x^2 + x + 1
// r = 0
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol

x = symbol('x')

f = expr('x^3 - 1')
g = expr('x - 1')

# Returns (quotient, remainder)
q, r = f.poly_div(g, x)
# q = x^2 + x + 1
# r = 0
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');

const x = symbol('x');

const f = expr('x^3 - 1');
const g = expr('x - 1');

// Returns (quotient, remainder)
const [q, r] = f.polyDiv(g, x);
// q = x^2 + x + 1
// r = 0
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Polynomial Resultant</h3>
        <p>Test for common roots using resultant</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::algorithms::polynomial_resultant;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// p1 = x - 1
let p1 = expr!(x - 1);
// p2 = x - 2
let p2 = expr!(x - 2);

let res = polynomial_resultant(&p1, &p2, &x).unwrap();
// Resultant is non-zero (distinct roots)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol
from mathhook.polynomial.algorithms import polynomial_resultant

x = symbol('x')

# p1 = x - 1
p1 = expr('x - 1')
# p2 = x - 2
p2 = expr('x - 2')

res = polynomial_resultant(p1, p2, x)
# Resultant is non-zero (distinct roots)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');
const { polynomialResultant } = require('mathhook/polynomial/algorithms');

const x = symbol('x');

// p1 = x - 1
const p1 = expr('x - 1');
// p2 = x - 2
const p2 = expr('x - 2');

const res = polynomialResultant(p1, p2, x);
// Resultant is non-zero (distinct roots)
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Polynomial Discriminant</h3>
        <p>Detect repeated roots using discriminant</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::algorithms::polynomial_discriminant;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// (x - 1)^2 = x^2 - 2x + 1 (repeated root at 1)
let poly = expr!((x ^ 2) - (2 * x) + 1);

let disc = polynomial_discriminant(&poly, &x).unwrap();
// Discriminant = 0 (repeated root)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol
from mathhook.polynomial.algorithms import polynomial_discriminant

x = symbol('x')

# (x - 1)^2 = x^2 - 2x + 1 (repeated root at 1)
poly = expr('x^2 - 2*x + 1')

disc = polynomial_discriminant(poly, x)
# Discriminant = 0 (repeated root)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');
const { polynomialDiscriminant } = require('mathhook/polynomial/algorithms');

const x = symbol('x');

// (x - 1)^2 = x^2 - 2x + 1 (repeated root at 1)
const poly = expr('x^2 - 2*x + 1');

const disc = polynomialDiscriminant(poly, x);
// Discriminant = 0 (repeated root)
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
const metaDescription = "Polynomial division algorithms including long division, exact division, and factorization capabilities
such as square-free factorization, resultant, and discriminant computation.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Polynomial Division and Factorization',
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
