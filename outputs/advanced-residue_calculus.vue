<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Residue Calculus and Pole Finding</h1>
      <p class="description">Find poles of rational and transcendental functions for applications in
control theory, signal processing, and complex analysis. SymPy-validated
pole locations for trigonometric functions.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">Pole of order $n$ at $z = z_0$:
$$\lim_{z \to z_0} (z - z_0)^n f(z) = c \neq 0$$

Residue theorem:
$$\oint_C f(z) \, dz = 2\pi i \sum_k \text{Res}(f, z_k)$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Rational Function Poles</h3>
        <p>Find poles where denominator equals zero</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let z = symbol!(z);

// Simple pole at z = 0
let f1 = expr!(1 / z);
let poles1 = f1.find_poles(&z);
// Returns: [expr!(0)]

// Pole of order 2 at z = 3
let f2 = expr!(1 / ((z - 3) ^ 2));
let poles2 = f2.find_poles(&z);
// Returns: [expr!(3)]

// Multiple simple poles
let f3 = expr!(1 / ((z - 1) * (z + 2)));
let poles3 = f3.find_poles(&z);
// Returns: [expr!(1), expr!(-2)]
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import symbols, singularities

z = symbols('z')

# Simple pole
f1 = 1/z
poles1 = singularities(f1, z)
# Returns: {0}

# Multiple poles
f3 = 1/((z-1)*(z+2))
poles3 = singularities(f3, z)
# Returns: {1, -2}
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const z = symbol('z');

// Simple pole
const f1 = div(1, z);
const poles1 = f1.findPoles(z);
// Returns: [0]

// Multiple poles
const f3 = div(1, mul(sub(z, 1), add(z, 2)));
const poles3 = f3.findPoles(z);
// Returns: [1, -2]
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Trigonometric Function Poles (SymPy Validated)</h3>
        <p>Poles of tan, cot, sec, csc functions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);

// tan(x) has poles at x = π/2 + nπ
let f = expr!(tan(x));
let poles = f.find_poles(&x);
// Returns principal pole: [expr!(pi / 2)]

// cot(x) has poles at x = nπ
let f = expr!(cot(x));
let poles = f.find_poles(&x);
// Returns principal pole: [expr!(0)]

// sec(x) has poles at x = π/2 + nπ
let f = expr!(sec(x));
let poles = f.find_poles(&x);
// Returns principal pole: [expr!(pi / 2)]

// csc(x) has poles at x = nπ
let f = expr!(csc(x));
let poles = f.find_poles(&x);
// Returns principal pole: [expr!(0)]
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import symbols, tan, cot, sec, csc, singularities, pi

x = symbols('x', real=True)

# tan(x) poles
poles_tan = singularities(tan(x), x)
# Principal: pi/2

# cot(x) poles
poles_cot = singularities(cot(x), x)
# Principal: 0

# sec(x) poles
poles_sec = singularities(sec(x), x)
# Principal: pi/2

# csc(x) poles
poles_csc = singularities(csc(x), x)
# Principal: 0
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');

// tan(x) poles
const poles_tan = tan(x).findPoles(x);
// Returns: [pi/2]

// cot(x) poles
const poles_cot = cot(x).findPoles(x);
// Returns: [0]
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Control System Stability</h3>
        <p>Transfer function pole analysis for stability</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let s = symbol!(s);
let zeta = expr!(0.7);      // Damping ratio
let omega_n = expr!(10);    // Natural frequency
let K = expr!(100);

let denominator = expr!(s^2 + 2*zeta*omega_n*s + omega_n^2);
let H = expr!(K / denominator);

let poles = H.find_poles(&s);

// For ζ = 0.7, ωₙ = 10:
// Poles ≈ -7 ± 7.14i
// - Stable (negative real part)
// - Damped oscillation at ~7.14 rad/s
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import symbols, sqrt

s = symbols('s')
zeta = 0.7
omega_n = 10
K = 100

H = K / (s**2 + 2*zeta*omega_n*s + omega_n**2)
poles = solve(denom(H), s)

# Stability: all poles have Re(pole) < 0
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const s = symbol('s');
const zeta = 0.7;
const omega_n = 10;
const K = 100;

const denom = add(pow(s, 2), mul(2, zeta, omega_n, s), pow(omega_n, 2));
const H = div(K, denom);

const poles = H.findPoles(s);
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
const metaDescription = "Find poles of rational and transcendental functions for applications in
control theory, signal processing, and complex analysis. SymPy-validated
pole locations for trigonometric functions.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Residue Calculus and Pole Finding',
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
