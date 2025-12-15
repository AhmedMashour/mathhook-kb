<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Groebner Bases</h1>
      <p class="description">Groebner bases are fundamental tools in computational algebraic geometry for working with polynomial ideals,
enabling ideal membership testing, polynomial system solving, variable elimination, and geometric theorem proving.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Groebner Basis Definition**: A set $G = \{g_1, \ldots, g_m\}$ is a Groebner basis for ideal $I$
with respect to monomial order $<$ if:

$$\langle \text{LT}(g_1), \ldots, \text{LT}(g_m) \rangle = \langle \text{LT}(I) \rangle$$

where $\text{LT}$ denotes the leading term and $\langle \cdot \rangle$ denotes the ideal generated.

**S-Polynomial**: The S-polynomial of $f$ and $g$ is:

$$S(f,g) = \frac{\text{lcm}(\text{LT}(f), \text{LT}(g))}{\text{LT}(f)} \cdot f - \frac{\text{lcm}(\text{LT}(f), \text{LT}(g))}{\text{LT}(g)} \cdot g$$

**Buchberger's Criterion**: $G$ is a Groebner basis if and only if for all pairs $g_i, g_j \in G$:

$$S(g_i, g_j) \xrightarrow{G}_+ 0$$

(i.e., $S(g_i, g_j)$ reduces to 0 modulo $G$)

**Monomial Orders**:
- **Lex**: $x^\alpha > x^\beta \iff$ first non-zero entry of $\alpha - \beta$ is positive
- **Grlex**: Total degree first, then lex
- **Grevlex**: Total degree first, then reverse lex from right
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Groebner Basis Computation</h3>
        <p>Compute Groebner basis for a polynomial ideal</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::groebner::{GroebnerBasis, MonomialOrder};
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let y = symbol!(y);

// Define polynomials: f1 = x - y, f2 = y^2 - 1
let f1 = expr!(x - y);
let f2 = expr!((y ^ 2) - 1);

// Create Groebner basis
let mut gb = GroebnerBasis::new(
    vec![f1, f2],
    vec![x.clone(), y.clone()],
    MonomialOrder::Lex
);

// Compute the basis
gb.compute();

println!("Basis has {} polynomials", gb.basis.len());
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol
from mathhook.polynomial.groebner import GroebnerBasis, MonomialOrder

x = symbol('x')
y = symbol('y')

# Define polynomials: f1 = x - y, f2 = y^2 - 1
f1 = expr('x - y')
f2 = expr('y^2 - 1')

# Create Groebner basis
gb = GroebnerBasis(
    [f1, f2],
    [x, y],
    MonomialOrder.Lex
)

# Compute the basis
gb.compute()

print(f"Basis has {len(gb.basis)} polynomials")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');
const { GroebnerBasis, MonomialOrder } = require('mathhook/polynomial/groebner');

const x = symbol('x');
const y = symbol('y');

// Define polynomials: f1 = x - y, f2 = y^2 - 1
const f1 = expr('x - y');
const f2 = expr('y^2 - 1');

// Create Groebner basis
const gb = new GroebnerBasis(
    [f1, f2],
    [x, y],
    MonomialOrder.Lex
);

// Compute the basis
gb.compute();

console.log(`Basis has ${gb.basis.length} polynomials`);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Sparse Polynomial Representation</h3>
        <p>Work with sparse polynomials for efficiency</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::groebner::{Monomial, expression_to_sparse_polynomial};
use mathhook_core::{expr, symbol};

// Create a monomial x^2 * y (exponents [2, 1])
let mono = Monomial::new(vec![2, 1]);
assert_eq!(mono.degree(), 3);

// Convert expression to sparse polynomial
let x = symbol!(x);
let y = symbol!(y);
let poly = expr!((x ^ 2) + y);

let vars = vec![x, y];
let sparse = expression_to_sparse_polynomial(&poly, &vars);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol
from mathhook.polynomial.groebner import Monomial, expression_to_sparse_polynomial

# Create a monomial x^2 * y (exponents [2, 1])
mono = Monomial([2, 1])
assert mono.degree() == 3

# Convert expression to sparse polynomial
x = symbol('x')
y = symbol('y')
poly = expr('x^2 + y')

vars = [x, y]
sparse = expression_to_sparse_polynomial(poly, vars)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');
const { Monomial, expressionToSparsePolynomial } = require('mathhook/polynomial/groebner');

// Create a monomial x^2 * y (exponents [2, 1])
const mono = new Monomial([2, 1]);
assert(mono.degree() === 3);

// Convert expression to sparse polynomial
const x = symbol('x');
const y = symbol('y');
const poly = expr('x^2 + y');

const vars = [x, y];
const sparse = expressionToSparsePolynomial(poly, vars);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Polynomial Reduction</h3>
        <p>Reduce polynomial by a set of polynomials (division algorithm)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::groebner::{
    poly_reduce,
    poly_reduce_completely
};

// Single-step reduction
let reduced = poly_reduce(&poly, &basis, &order);

// Complete reduction (until no further reduction possible)
let fully_reduced = poly_reduce_completely(&poly, &basis, &order);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.polynomial.groebner import poly_reduce, poly_reduce_completely

# Single-step reduction
reduced = poly_reduce(poly, basis, order)

# Complete reduction (until no further reduction possible)
fully_reduced = poly_reduce_completely(poly, basis, order)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { polyReduce, polyReduceCompletely } = require('mathhook/polynomial/groebner');

// Single-step reduction
const reduced = polyReduce(poly, basis, order);

// Complete reduction (until no further reduction possible)
const fullyReduced = polyReduceCompletely(poly, basis, order);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Bidirectional Expression Conversion</h3>
        <p>Convert between Expression and sparse polynomial representation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::groebner::{
    expression_to_sparse_polynomial,
    sparse_polynomial_to_expression
};
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let y = symbol!(y);
let vars = vec![x.clone(), y.clone()];

// Expression to sparse
let expr = expr!((x ^ 2) + y);
let sparse = expression_to_sparse_polynomial(&expr, &vars).unwrap();

// Sparse back to expression
let back = sparse_polynomial_to_expression(&sparse, &vars);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol
from mathhook.polynomial.groebner import (
    expression_to_sparse_polynomial,
    sparse_polynomial_to_expression
)

x = symbol('x')
y = symbol('y')
vars = [x, y]

# Expression to sparse
e = expr('x^2 + y')
sparse = expression_to_sparse_polynomial(e, vars)

# Sparse back to expression
back = sparse_polynomial_to_expression(sparse, vars)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');
const {
    expressionToSparsePolynomial,
    sparsePolynomialToExpression
} = require('mathhook/polynomial/groebner');

const x = symbol('x');
const y = symbol('y');
const vars = [x, y];

// Expression to sparse
const e = expr('x^2 + y');
const sparse = expressionToSparsePolynomial(e, vars);

// Sparse back to expression
const back = sparsePolynomialToExpression(sparse, vars);
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
const metaDescription = "Groebner bases are fundamental tools in computational algebraic geometry for working with polynomial ideals,
enabling ideal membership testing, polynomial system solving, variable elimination, and geometric theorem proving.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Groebner Bases',
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
