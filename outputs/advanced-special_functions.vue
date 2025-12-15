<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Special Mathematical Functions</h1>
      <p class="description">Comprehensive support for special mathematical functions including the Gamma
function family (Gamma, Digamma, Polygamma), Beta function, and other
advanced special functions used in mathematics, physics, and statistics.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">Gamma function: $$\Gamma(z) = \int_0^\infty t^{z-1} e^{-t} \, dt$$

Digamma function: $$\psi(z) = \frac{d}{dz} \ln(\Gamma(z)) = \frac{\Gamma'(z)}{\Gamma(z)}$$

Polygamma function: $$\psi^{(n)}(z) = \frac{d^{n+1}}{dz^{n+1}} \ln(\Gamma(z))$$

Beta function: $$B(a, b) = \frac{\Gamma(a) \cdot \Gamma(b)}{\Gamma(a+b)}$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Gamma Function</h3>
        <p>Factorial extension: Γ(n) = (n-1)! for positive integers</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Integer factorial: Γ(5) = 4!
let result = gamma(&expr!(5));
// Result: 24

// Half-integer (exact symbolic): Γ(1/2) = √π
let result = gamma(&Expression::rational(1, 2));
// Result: sqrt(pi)

// Numerical evaluation
let result = gamma(&Expression::float(3.7));
// Result: ~4.17
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import gamma, sqrt, pi

# Integer factorial
result = gamma(5)
# Result: 24

# Half-integer
result = gamma(Rational(1, 2))
# Result: sqrt(pi)

# Numerical
result = gamma(3.7)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Integer factorial
const result1 = gamma(5);
// Result: 24

// Half-integer
const result2 = gamma(0.5);
// Result: sqrt(pi)

// Numerical
const result3 = gamma(3.7);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Digamma Function</h3>
        <p>Logarithmic derivative of Gamma: ψ(z) = Γ'(z)/Γ(z)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Special value: ψ(1) = -γ (Euler-Mascheroni constant)
let result = digamma(&expr!(1));
// Result: -EulerGamma (symbolic)

// Using recurrence: ψ(5) = ψ(1) + 1 + 1/2 + 1/3 + 1/4
let result = digamma(&expr!(5));
// Result: -EulerGamma + 25/12
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import digamma, EulerGamma

# Special value
result = digamma(1)
# Result: -EulerGamma

# Recurrence relation
result = digamma(5)
# Result: -EulerGamma + 25/12
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Special value
const result1 = digamma(1);
// Result: -EulerGamma

// Recurrence
const result2 = digamma(5);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Polygamma Function</h3>
        <p>Higher derivatives: ψ^(n)(z) = d^n/dz^n ψ(z)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Trigamma (n=1): ψ^(1)(1) = π²/6
let result = polygamma(1, &expr!(1));
// Result: pi^2/6

// Tetragamma (n=2)
let result = polygamma(2, &expr!(1));
// Result: -2*zeta(3)

// Higher orders
let result = polygamma(3, &expr!(1.5));
// Result: numerical value
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import polygamma, pi, zeta

# Trigamma
result = polygamma(1, 1)
# Result: pi**2/6

# Tetragamma
result = polygamma(2, 1)
# Result: -2*zeta(3)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Trigamma
const result1 = polygamma(1, 1);
// Result: pi^2/6

// Tetragamma
const result2 = polygamma(2, 1);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Beta Function</h3>
        <p>Beta function: B(a,b) = Γ(a)Γ(b)/Γ(a+b)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Integer values: B(2, 3) = 1/12
let result = beta(&expr!(2), &expr!(3));

// Numerical evaluation
let result = beta(&Expression::float(2.5), &Expression::float(3.7));

// Symmetry property
assert_eq!(beta(&expr!(2), &expr!(5)), beta(&expr!(5), &expr!(2)));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import beta

# Integer values
result = beta(2, 3)
# Result: 1/12

# Symmetry
assert beta(2, 5) == beta(5, 2)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Beta function
const result1 = beta(2, 3);
// Result: 1/12

// Numerical
const result2 = beta(2.5, 3.7);
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
const metaDescription = "Comprehensive support for special mathematical functions including the Gamma
function family (Gamma, Digamma, Polygamma), Beta function, and other
advanced special functions used in mathematics, physics, and statistics.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Special Mathematical Functions',
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
