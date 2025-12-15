<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>GCD Algorithms</h1>
      <p class="description">Multiple GCD (Greatest Common Divisor) algorithms for polynomials, optimized for different use cases
including univariate, multivariate, and modular GCD using Zippel's algorithm.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">For polynomials $f, g \in R[x]$ over a ring $R$, the greatest common divisor $\gcd(f, g)$ is the
monic polynomial $d$ of maximum degree such that:

$$d \mid f \quad \text{and} \quad d \mid g$$

and for any other polynomial $h$ where $h \mid f$ and $h \mid g$, we have $h \mid d$.

**Euclidean Algorithm**: For univariate polynomials over a field:

$$\gcd(f, g) = \begin{cases}
f & \text{if } g = 0 \\
\gcd(g, f \bmod g) & \text{otherwise}
\end{cases}$$

**Zippel's Modular Algorithm**:
1. Extract content: $f = c_f \cdot f_p$, $g = c_g \cdot g_p$
2. Compute $\gcd(f_p, g_p)$ in $\mathbb{Z}_p[x]$ for prime $p$
3. Use CRT to reconstruct $\gcd$ in $\mathbb{Z}[x]$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>General-Purpose GCD</h3>
        <p>Use PolynomialGcdOps trait for automatic algorithm selection</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::PolynomialGcdOps;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// f = x^2 - 1 = (x-1)(x+1)
let f = expr!((x ^ 2) - 1);
// g = x^2 - 2x + 1 = (x-1)^2
let g = expr!((x ^ 2) - (2 * x) + 1);

// Compute GCD
let gcd = f.polynomial_gcd(&g).unwrap();
// gcd = x - 1

// Compute LCM
let lcm = f.polynomial_lcm(&g).unwrap();
// lcm = (x-1)^2(x+1)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol
from mathhook.polynomial import PolynomialOps

x = symbol('x')

# f = x^2 - 1 = (x-1)(x+1)
f = expr('x^2 - 1')
# g = x^2 - 2x + 1 = (x-1)^2
g = expr('x^2 - 2*x + 1')

# Compute GCD
gcd = f.polynomial_gcd(g)
# gcd = x - 1

# Compute LCM
lcm = f.polynomial_lcm(g)
# lcm = (x-1)^2(x+1)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');

const x = symbol('x');

// f = x^2 - 1 = (x-1)(x+1)
const f = expr('x^2 - 1');
// g = x^2 - 2x + 1 = (x-1)^2
const g = expr('x^2 - 2*x + 1');

// Compute GCD
const gcd = f.polynomialGcd(g);
// gcd = x - 1

// Compute LCM
const lcm = f.polynomialLcm(g);
// lcm = (x-1)^2(x+1)
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Univariate Modular GCD with Cofactors</h3>
        <p>Returns GCD and cofactors for Bezout identity verification</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::algorithms::zippel_gcd::modular_gcd_univariate;
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let f = expr!((x ^ 2) - 1);
let g = expr!(x - 1);

// Returns (gcd, cofactor_f, cofactor_g)
let (gcd, cof_f, cof_g) = modular_gcd_univariate(&f, &g, &x).unwrap();

// Verify: f = gcd * cof_f, g = gcd * cof_g
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol
from mathhook.polynomial.algorithms import modular_gcd_univariate

x = symbol('x')
f = expr('x^2 - 1')
g = expr('x - 1')

# Returns (gcd, cofactor_f, cofactor_g)
gcd, cof_f, cof_g = modular_gcd_univariate(f, g, x)

# Verify: f = gcd * cof_f, g = gcd * cof_g
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');
const { modularGcdUnivariate } = require('mathhook/polynomial/algorithms');

const x = symbol('x');
const f = expr('x^2 - 1');
const g = expr('x - 1');

// Returns (gcd, cofactor_f, cofactor_g)
const [gcd, cofF, cofG] = modularGcdUnivariate(f, g, x);

// Verify: f = gcd * cofF, g = gcd * cofG
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Multivariate GCD with Zippel Algorithm</h3>
        <p>Compute GCD for polynomials in multiple variables</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::algorithms::zippel_gcd::{
    multivariate_gcd_zippel,
    MultivariateGcdConfig
};
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let y = symbol!(y);

// f = x*y, g = x*y + x
let f = expr!(x * y);
let g = expr!((x * y) + x);

let config = MultivariateGcdConfig::default();
let (gcd, _, _) = multivariate_gcd_zippel(&f, &g, &[x, y], config).unwrap();
// gcd = x
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, symbol
from mathhook.polynomial.algorithms import multivariate_gcd_zippel, MultivariateGcdConfig

x = symbol('x')
y = symbol('y')

# f = x*y, g = x*y + x
f = expr('x*y')
g = expr('x*y + x')

config = MultivariateGcdConfig()
gcd, _, _ = multivariate_gcd_zippel(f, g, [x, y], config)
# gcd = x
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, symbol } = require('mathhook');
const { multivariateGcdZippel, MultivariateGcdConfig } = require('mathhook/polynomial/algorithms');

const x = symbol('x');
const y = symbol('y');

// f = x*y, g = x*y + x
const f = expr('x*y');
const g = expr('x*y + x');

const config = new MultivariateGcdConfig();
const [gcd, _, __] = multivariateGcdZippel(f, g, [x, y], config);
// gcd = x
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Content and Primitive Part Decomposition</h3>
        <p>Fundamental operation for GCD computation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::polynomial::algorithms::zippel_gcd::{
    extract_content,
    primitive_part
};

let coeffs = vec![6, 12, 18];  // 6 + 12x + 18x^2

// Extract content (GCD of coefficients)
let content = extract_content(&coeffs);  // 6

// Get primitive part
let (cont, pp) = primitive_part(&coeffs);  // (6, [1, 2, 3])
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.polynomial.algorithms import extract_content, primitive_part

coeffs = [6, 12, 18]  # 6 + 12x + 18x^2

# Extract content (GCD of coefficients)
content = extract_content(coeffs)  # 6

# Get primitive part
cont, pp = primitive_part(coeffs)  # (6, [1, 2, 3])
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { extractContent, primitivePart } = require('mathhook/polynomial/algorithms');

const coeffs = [6, 12, 18];  // 6 + 12x + 18x^2

// Extract content (GCD of coefficients)
const content = extractContent(coeffs);  // 6

// Get primitive part
const [cont, pp] = primitivePart(coeffs);  // (6, [1, 2, 3])
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
const metaDescription = "Multiple GCD (Greatest Common Divisor) algorithms for polynomials, optimized for different use cases
including univariate, multivariate, and modular GCD using Zippel's algorithm.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'GCD Algorithms',
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
