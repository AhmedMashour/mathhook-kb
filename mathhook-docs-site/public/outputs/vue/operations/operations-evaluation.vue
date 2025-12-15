<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Expression Evaluation</h1>
      <p class="description">MathHook provides two fundamental operations for working with expressions:

1. **Evaluation** - Compute numerical values with domain checking
2. **Simplification** - Algebraic reduction while staying symbolic

Understanding when to use each operation is critical for correct mathematical computation.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Function Evaluation:**
$$f(a) = f(x)|_{x=a}$$

**Evaluation with Context:**
For expression $f(x_1, \ldots, x_n)$ and substitutions $\{x_i \mapsto v_i\}$:
$$\text{evaluate}(f, \{x_i \mapsto v_i\}) = f(v_1, \ldots, v_n)$$

**Domain Constraints:**
- $\sqrt{x}$ requires $x \geq 0$ in $\mathbb{R}$
- $\log(x)$ requires $x > 0$ (pole at 0)
- $\tan(x)$ has poles at $\frac{\pi}{2} + n\pi$
- $\arcsin(x), \arccos(x)$ require $|x| \leq 1$ in $\mathbb{R}$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Constants Evaluate to Numbers</h3>
        <p>Direct evaluation of constant expressions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let sum = expr!(2 + 3);
assert_eq!(sum.evaluate().unwrap(), expr!(5));

// Domain checking catches errors
let sqrt_neg = expr!(sqrt(-1));
assert!(matches!(sqrt_neg.evaluate(), Err(MathError::DomainError { .. })));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr

sum_expr = expr('2 + 3')
assert sum_expr.evaluate() == 5

# Domain checking catches errors
sqrt_neg = expr('sqrt(-1)')
try:
    sqrt_neg.evaluate()
    assert False, "Should raise domain error"
except MathError:
    pass
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr } = require('mathhook');

const sum = expr('2 + 3');
assert(sum.evaluate() === 5);

// Domain checking catches errors
const sqrtNeg = expr('sqrt(-1)');
try {
    sqrtNeg.evaluate();
    throw new Error("Should raise domain error");
} catch (e) {
    // Expected MathError
}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Evaluation with Context (Substitution)</h3>
        <p>Substitute variable values and evaluate</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::core::expression::eval_numeric::EvalContext;
use mathhook::prelude::*;
use std::collections::HashMap;

let x = symbol!(x);

// Substitute x = 3 and evaluate
let mut vars = HashMap::new();
vars.insert("x".to_string(), Expression::integer(3));
let ctx = EvalContext::numeric(vars);

let expr = Expression::pow(x.clone(), Expression::integer(2));
assert_eq!(expr.evaluate_with_context(&ctx).unwrap(), Expression::integer(9));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, Expression

x = symbol('x')
expr = x ** 2

# Substitute x = 3 and evaluate
ctx = {'x': 3}
result = expr.evaluate_with_context(ctx)
assert result == 9
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');

const x = symbol('x');
const expr = x.pow(2);

// Substitute x = 3 and evaluate
const ctx = { x: 3 };
const result = expr.evaluateWithContext(ctx);
assert(result === 9);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Simplification Without Domain Checking</h3>
        <p>Simplify operates purely symbolically without domain validation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook_core::simplify::Simplify;

let x = symbol!(x);

// Combine like terms
let sum = expr!(x + x);
assert_eq!(sum.simplify(), expr!(2 * x));

// Apply identities
assert_eq!(expr!(x * 1).simplify(), expr!(x));
assert_eq!(expr!(0 * x).simplify(), expr!(0));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

x = symbol('x')

# Combine like terms
sum_expr = x + x
assert sum_expr.simplify() == 2 * x

# Apply identities
assert (x * 1).simplify() == x
assert (0 * x).simplify() == 0
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol } = require('mathhook');

const x = symbol('x');

// Combine like terms
const sumExpr = x.add(x);
assert(sumExpr.simplify().equals(x.mul(2)));

// Apply identities
assert(x.mul(1).simplify().equals(x));
assert(x.mul(0).simplify().equals(0));
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
const metaDescription = "MathHook provides two fundamental operations for working with expressions:

1. **Evaluation** - Compute numerical values with domain checking
2. **Simplification** - Algebraic reduction while staying symbolic

Understanding when to use each operation is critical for correct mathematical computation.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Expression Evaluation',
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
