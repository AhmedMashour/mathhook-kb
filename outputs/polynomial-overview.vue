<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Polynomial Module Overview</h1>
      <p class="description">Comprehensive symbolic polynomial manipulation capabilities in MathHook. Implements a trait-based
architecture for automatic classification, property computation, arithmetic operations, and GCD algorithms.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">A polynomial in variable $x$ over a ring $R$ is an expression of the form:

$$f(x) = a_n x^n + a_{n-1} x^{n-1} + \cdots + a_1 x + a_0$$

where $a_i \in R$ are coefficients and $n \in \mathbb{N}$ is the degree.

For multivariate polynomials in variables $x_1, x_2, \ldots, x_k$:

$$f(x_1, \ldots, x_k) = \sum_{\alpha \in \mathbb{N}^k} c_\alpha x_1^{\alpha_1} \cdots x_k^{\alpha_k}$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Polynomial Usage</h3>
        <p>Create polynomials and compute properties using trait-based API</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::{
    PolynomialClassification,
    PolynomialProperties,
    PolynomialGcdOps
};
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// Create polynomials using expr! macro
let f = expr!((x ^ 2) + (2 * x) + 1);  // x^2 + 2x + 1
let g = expr!((x ^ 2) - 1);             // x^2 - 1

// Properties
assert_eq!(f.degree(&x), Some(2));
assert!(f.is_polynomial_in(&[x.clone()]));

// GCD computation
let gcd = f.polynomial_gcd(&g).unwrap();
// gcd = x + 1 (since f = (x+1)^2 and g = (x+1)(x-1))
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol
from mathhook.polynomial import PolynomialOps

x = symbol('x')

# Create polynomials
f = expr('x^2 + 2*x + 1')
g = expr('x^2 - 1')

# Properties
assert f.degree(x) == 2
assert f.is_polynomial_in([x])

# GCD computation
gcd = f.polynomial_gcd(g)
# gcd = x + 1
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');

const x = symbol('x');

// Create polynomials
const f = expr('x^2 + 2*x + 1');
const g = expr('x^2 - 1');

// Properties
assert(f.degree(x) === 2);
assert(f.isPolynomialIn([x]));

// GCD computation
const gcd = f.polynomialGcd(g);
// gcd = x + 1
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Polynomial Classification</h3>
        <p>Automatic detection of polynomial structure and variable extraction</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::PolynomialClassification;
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let y = symbol!(y);

// Automatic detection
let poly = expr!((x ^ 2) + (y * x) + 1);
assert!(poly.is_polynomial());
assert!(poly.is_polynomial_in(&[x.clone(), y.clone()]));

// Variable extraction
let vars = poly.polynomial_variables();
// vars contains x and y
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol

x = symbol('x')
y = symbol('y')

# Automatic detection
poly = expr('x^2 + y*x + 1')
assert poly.is_polynomial()
assert poly.is_polynomial_in([x, y])

# Variable extraction
vars = poly.polynomial_variables()
# vars contains x and y
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');

const x = symbol('x');
const y = symbol('y');

// Automatic detection
const poly = expr('x^2 + y*x + 1');
assert(poly.isPolynomial());
assert(poly.isPolynomialIn([x, y]));

// Variable extraction
const vars = poly.polynomialVariables();
// vars contains x and y
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Content and Primitive Part</h3>
        <p>Extract GCD of coefficients and primitive polynomial</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::PolynomialProperties;
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let poly = expr!((6 * (x ^ 2)) + (9 * x) + 3);  // 6x^2 + 9x + 3

let content = poly.content();           // 3
let primitive = poly.primitive_part();  // 2x^2 + 3x + 1
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol

x = symbol('x')
poly = expr('6*x^2 + 9*x + 3')

content = poly.content()         # 3
primitive = poly.primitive_part() # 2*x^2 + 3*x + 1
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');

const x = symbol('x');
const poly = expr('6*x^2 + 9*x + 3');

const content = poly.content();         // 3
const primitive = poly.primitivePart(); // 2*x^2 + 3*x + 1
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
const metaDescription = "Comprehensive symbolic polynomial manipulation capabilities in MathHook. Implements a trait-based
architecture for automatic classification, property computation, arithmetic operations, and GCD algorithms.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Polynomial Module Overview',
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
