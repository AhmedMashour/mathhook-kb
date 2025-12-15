<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Piecewise Functions</h1>
      <p class="description">Define functions with different formulas in different regions, essential for
modeling discontinuous behavior, conditional logic, step functions, and
threshold-based systems.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">Piecewise function:
$$f(x) = \begin{cases}
f_1(x) & \text{if } C_1(x) \\
f_2(x) & \text{if } C_2(x) \\
\vdots & \\
f_n(x) & \text{if } C_n(x) \\
f_{\text{default}} & \text{otherwise}
\end{cases}$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Absolute Value Function</h3>
        <p>|x| = { x if x ≥ 0, -x if x < 0 }</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);

let abs_x = Expression::piecewise(
    vec![
        (expr!(x), expr!(x >= 0)),
        (expr!(-x), expr!(x < 0)),
    ],
    None,
);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import symbols, Piecewise

x = symbols('x')
abs_x = Piecewise((x, x >= 0), (-x, x < 0))
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');

const abs_x = piecewise([
    [x, ge(x, 0)],
    [neg(x), lt(x, 0)]
]);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Heaviside Step Function</h3>
        <p>H(x) = { 0 if x < 0, 1 if x ≥ 0 }</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);

let heaviside = Expression::piecewise(
    vec![
        (expr!(0), expr!(x < 0)),
        (expr!(1), expr!(x >= 0)),
    ],
    None,
);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import symbols, Heaviside

x = symbols('x')
H = Heaviside(x)  # Built-in Heaviside function
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');

const H = piecewise([
    [0, lt(x, 0)],
    [1, ge(x, 0)]
]);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Tax Bracket Example</h3>
        <p>Progressive tax with income thresholds</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let income = symbol!(income);

// 10% on first $10k, 12% on next $30k, 22% on remainder
let tax = Expression::piecewise(
    vec![
        (expr!(0.10 * income), expr!(income <= 10000)),
        (expr!(1000 + 0.12 * (income - 10000)), expr!(income <= 40000)),
    ],
    Some(expr!(4600 + 0.22 * (income - 40000))),
);

// Calculate tax for $50,000
let tax_owed = tax.substitute(&income, &expr!(50000));
// Result: 4600 + 0.22 * 10000 = $6,800
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import symbols, Piecewise

income = symbols('income')

tax = Piecewise(
    (0.10 * income, income <= 10000),
    (1000 + 0.12 * (income - 10000), income <= 40000),
    (4600 + 0.22 * (income - 40000), True)
)

tax_owed = tax.subs(income, 50000)
# Result: 6800
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const income = symbol('income');

const tax = piecewise([
    [mul(0.10, income), le(income, 10000)],
    [add(1000, mul(0.12, sub(income, 10000))), le(income, 40000)],
    [add(4600, mul(0.22, sub(income, 40000))), true]
]);

const tax_owed = tax.subs(income, 50000);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Differentiation of Piecewise</h3>
        <p>Derivative computed piece-by-piece</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);

// f(x) = { x^2 if x ≥ 0, -x^2 if x < 0 }
let f = Expression::piecewise(
    vec![
        (expr!(x^2), expr!(x >= 0)),
        (expr!(-x^2), expr!(x < 0)),
    ],
    None,
);

// Derivative
let df = f.derivative(&x, 1);
// Result: { 2x if x ≥ 0, -2x if x < 0 }
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy import symbols, Piecewise, diff

x = symbols('x')
f = Piecewise((x**2, x >= 0), (-x**2, x < 0))

df = diff(f, x)
# Result: Piecewise((2*x, x > 0), (-2*x, x < 0))
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');

const f = piecewise([
    [pow(x, 2), ge(x, 0)],
    [neg(pow(x, 2)), lt(x, 0)]
]);

const df = diff(f, x);
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
const metaDescription = "Define functions with different formulas in different regions, essential for
modeling discontinuous behavior, conditional logic, step functions, and
threshold-based systems.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Piecewise Functions',
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
