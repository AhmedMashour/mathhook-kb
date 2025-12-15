<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Evaluation vs Simplification</h1>
      <p class="description">Understand the critical differences between evaluation (computing numerical results
with domain checking) and simplification (algebraic transformation) in MathHook's
symbolic engine. Knowing when to use each operation is essential for correct
mathematical computation.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Evaluation** maps expressions to numerical values:
$$\text{eval}: E \times \sigma \to \mathbb{R} \cup \mathbb{C} \cup \{\text{Error}\}$$
where $E$ is the set of expressions and $\sigma$ is a variable substitution.

**Simplification** maps expressions to equivalent canonical forms:
$$\text{simplify}: E \to E \quad \text{such that } \forall e \in E: e \equiv \text{simplify}(e)$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Evaluation vs Simplification</h3>
        <p>Shows the fundamental difference between the two operations</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::{expr, symbol};

// Simplify: algebraic reduction
let x = symbol!(x);
let simplified = expr!(x + x + x).simplify();
assert_eq!(simplified, expr!(3 * x));  // Still symbolic

// Evaluate: numerical computation
let result = expr!(2 + 3).evaluate().unwrap();
assert_eq!(result, expr!(5));  // Numerical value
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr

# Simplify: algebraic reduction
x = symbol('x')
simplified = (x + x + x).simplify()
# Result: 3*x (still symbolic)

# Evaluate: numerical computation
result = expr('2 + 3').evaluate()
# Result: 5 (numerical value)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');

// Simplify: algebraic reduction
const x = symbol('x');
const simplified = expr(x.add(x).add(x)).simplify();
// Result: 3*x (still symbolic)

// Evaluate: numerical computation
const result = expr('2 + 3').evaluate();
// Result: 5 (numerical value)
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Evaluation with Variable Substitution</h3>
        <p>Using evaluate_with_context for variable substitution</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::expression::eval_numeric::EvalContext;
use mathhook_core::{expr, symbol};
use std::collections::HashMap;

let x = symbol!(x);
let y = symbol!(y);

// Create context with variable values
let mut vars = HashMap::new();
vars.insert("x".to_string(), expr!(3));
vars.insert("y".to_string(), expr!(4));
let ctx = EvalContext::numeric(vars);

// Evaluate x² + 2xy + y² at (x=3, y=4)
let formula = expr!(x^2 + 2*x*y + y^2);
let result = formula.evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(49)); // (3 + 4)² = 49
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, EvalContext

x = symbol('x')
y = symbol('y')

# Create context with variable values
ctx = EvalContext({'x': 3, 'y': 4})

# Evaluate x² + 2xy + y² at (x=3, y=4)
formula = x**2 + 2*x*y + y**2
result = formula.evaluate_with_context(ctx)
# Result: 49  (which is (3+4)²)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, EvalContext } = require('mathhook');

const x = symbol('x');
const y = symbol('y');

// Create context with variable values
const ctx = new EvalContext({x: 3, y: 4});

// Evaluate x² + 2xy + y² at (x=3, y=4)
const formula = x.pow(2).add(x.mul(y).mul(2)).add(y.pow(2));
const result = formula.evaluateWithContext(ctx);
// Result: 49  (which is (3+4)²)
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Domain Error Handling</h3>
        <p>Evaluation catches mathematical domain errors</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::{expr, MathError};

// sqrt(-1) triggers domain error
match expr!(sqrt(-1)).evaluate() {
    Ok(result) => println!("Result: {}", result),
    Err(MathError::DomainError { operation, value, reason }) => {
        eprintln!("Domain error in {}: {} ({})", operation, value, reason);
    }
    Err(e) => eprintln!("Error: {:?}", e),
}

// log(0) triggers domain error
assert!(expr!(log(0)).evaluate().is_err());

// Division by zero
assert!(expr!(1 / 0).evaluate().is_err());
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr, MathError

# sqrt(-1) triggers domain error
try:
    result = expr('sqrt(-1)').evaluate()
except MathError as e:
    print(f"Domain error: {e}")

# log(0) triggers domain error
try:
    result = expr('log(0)').evaluate()
except MathError as e:
    print(f"Domain error: {e}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr, MathError } = require('mathhook');

// sqrt(-1) triggers domain error
try {
    const result = expr('sqrt(-1)').evaluate();
} catch (e) {
    if (e instanceof MathError) {
        console.error(`Domain error: ${e.message}`);
    }
}

// log(0) triggers domain error - also throws
// Division by zero - also throws
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Simplification for Algebraic Manipulation</h3>
        <p>Simplification applies algebraic identities without domain checking</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::{expr, symbol};

let x = symbol!(x);

// Combine like terms
assert_eq!(expr!(x + x + x).simplify(), expr!(3 * x));

// Remove identity elements
assert_eq!(expr!(x * 1).simplify(), expr!(x));
assert_eq!(expr!(x + 0).simplify(), expr!(x));

// Zero propagation
assert_eq!(expr!(0 * x).simplify(), expr!(0));

// Trigonometric identities
assert_eq!(expr!(sin(x)^2 + cos(x)^2).simplify(), expr!(1));

// Simplify doesn't check domain (stays symbolic)
let result = expr!(sqrt(x)).simplify(); // OK, stays sqrt(x)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, simplify

x = symbol('x')

# Combine like terms
assert simplify(x + x + x) == 3*x

# Remove identity elements
assert simplify(x * 1) == x
assert simplify(x + 0) == x

# Zero propagation
assert simplify(0 * x) == 0

# Trigonometric identities
from mathhook import sin, cos
assert simplify(sin(x)**2 + cos(x)**2) == 1
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, simplify, sin, cos } = require('mathhook');

const x = symbol('x');

// Combine like terms
console.log(simplify(x.add(x).add(x)));  // 3*x

// Remove identity elements
console.log(simplify(x.mul(1)));  // x
console.log(simplify(x.add(0)));  // x

// Zero propagation
console.log(simplify(x.mul(0)));  // 0

// Trigonometric identities
console.log(simplify(sin(x).pow(2).add(cos(x).pow(2))));  // 1
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
const metaDescription = "Understand the critical differences between evaluation (computing numerical results
with domain checking) and simplification (algebraic transformation) in MathHook's
symbolic engine. Knowing when to use each operation is essential for correct
mathematical computation.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Evaluation vs Simplification',
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
