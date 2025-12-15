<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Python API Guide</h1>
      <p class="description">Complete guide to using MathHook from Python via PyO3 bindings.
Provides comprehensive documentation for the Python API including installation,
quick start, API reference, performance comparisons with SymPy, and integration patterns.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Symbol Creation and Expression Building</h3>
        <p>Create symbols and build expressions using operator overloading</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::symbol;
use mathhook::expr;

let x = symbol!(x);
let y = symbol!(y);

// Build expressions
let expr = expr!(x^2 + 2*x + 1);
let expr2 = expr!((x + 1) * (x - 1));
let expr3 = expr!(x / (x + 1));
let expr4 = expr!(-x);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Symbol

x = Symbol('x')
y = Symbol('y')

# Arithmetic operators
expr = x**2 + 2*x + 1
expr2 = (x + 1) * (x - 1)
expr3 = x / (x + 1)
expr4 = -x
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { Symbol, parse } = require('mathhook');

const x = new Symbol('x');
const y = new Symbol('y');

// Parse expressions
const expr = parse('x^2 + 2*x + 1');
const expr2 = parse('(x + 1) * (x - 1)');
const expr3 = parse('x / (x + 1)');
const expr4 = parse('-x');
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Expression Simplification</h3>
        <p>Simplify algebraic expressions using MathHook</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::{expr, symbol, simplify};

let x = symbol!(x);
let expr = expr!(x + x);
let result = simplify(expr);  // 2*x

let expr2 = expr!((x + 1) * (x - 1));
let result2 = simplify(expr2);  // x^2 - 1
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import parse, simplify

expr = parse("x + x")
result = simplify(expr)  # 2*x

expr = parse("(x + 1) * (x - 1)")
result = simplify(expr)  # x^2 - 1
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { parse, simplify } = require('mathhook');

const expr = parse('x + x');
const result = simplify(expr);  // 2*x

const expr2 = parse('(x + 1) * (x - 1)');
const result2 = simplify(expr2);  // x^2 - 1
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Symbolic Differentiation</h3>
        <p>Compute derivatives symbolically</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::{expr, symbol, derivative};

let x = symbol!(x);
let expr = expr!(x^3);

// First derivative
let df = derivative(&expr, &x, 1);
// Result: 3*x^2

// Second derivative
let d2f = derivative(&expr, &x, 2);
// Result: 6*x

// Partial derivatives
let y = symbol!(y);
let expr2 = expr!(x^2 * y);
let df_dx = derivative(&expr2, &x, 1);  // 2*x*y
let df_dy = derivative(&expr2, &y, 1);  // x^2
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Symbol, derivative

x = Symbol('x')
expr = x**3

# First derivative
df = derivative(expr, x)
print(df)  # 3*x^2

# Second derivative
d2f = derivative(expr, x, order=2)
print(d2f)  # 6*x

# Partial derivatives
y = Symbol('y')
expr = x**2 * y
df_dx = derivative(expr, x)  # 2*x*y
df_dy = derivative(expr, y)  # x^2
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { Symbol, parse, derivative } = require('mathhook');

const x = new Symbol('x');
const expr = parse('x^3');

// First derivative
const df = derivative(expr, x);
console.log(df.toString());  // 3*x^2

// Second derivative
const d2f = derivative(expr, x, { order: 2 });
console.log(d2f.toString());  // 6*x
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Equation Solving</h3>
        <p>Solve algebraic equations symbolically</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::{expr, symbol, solve};

let x = symbol!(x);

// Linear equation: 2*x + 3 = 7
let solutions = solve(expr!(2*x + 3), expr!(7), &x);
// Result: [x = 2]

// Quadratic equation: x^2 - 5*x + 6 = 0
let solutions = solve(expr!(x^2 - 5*x + 6), expr!(0), &x);
// Result: [x = 2, x = 3]
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Symbol, solve

x = Symbol('x')

# Linear equation: 2*x + 3 = 7
solutions = solve(2*x + 3, 7, x)
print(solutions)  # [x = 2]

# Quadratic equation: x^2 - 5*x + 6 = 0
solutions = solve(x**2 - 5*x + 6, 0, x)
print(solutions)  # [x = 2, x = 3]

# Multiple variables
y = Symbol('y')
solutions = solve([x + y - 5, x - y - 1], [x, y])
print(solutions)  # {x: 3, y: 2}
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { Symbol, parse, solve } = require('mathhook');

const x = new Symbol('x');

// Quadratic equation: x^2 - 5*x + 6 = 0
const expr = parse('x^2 - 5*x + 6');
const solutions = solve(expr, x);

solutions.forEach(sol => {
    console.log(sol.toString());
});
// Output: x = 2, x = 3
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Integration with NumPy</h3>
        <p>Convert symbolic expressions to NumPy functions for numerical evaluation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust"></code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">import numpy as np
from mathhook import Symbol, lambdify

x = Symbol('x')
expr = x**2 + 2*x + 1

# Convert to NumPy-compatible function
f = lambdify(expr, [x], 'numpy')

# Evaluate on NumPy array
x_values = np.linspace(-5, 5, 100)
y_values = f(x_values)

# Use with NumPy operations
mean = np.mean(y_values)
std = np.std(y_values)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript"></code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Evaluation with Context</h3>
        <p>Advanced evaluation with custom contexts and variable substitutions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust"></code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import PyExpression as Expression, EvalContext

x = Expression.symbol("x")
y = Expression.symbol("y")

# Formula: x² + 2xy + y²
expr = x.pow(Expression.integer(2)).add(
    Expression.integer(2).multiply(x).multiply(y)
).add(y.pow(Expression.integer(2)))

# Create numerical context with variable substitutions
ctx = EvalContext.numeric({
    "x": Expression.integer(3),
    "y": Expression.integer(4)
})

# Evaluate: (3)² + 2(3)(4) + (4)² = 9 + 24 + 16 = 49
result = expr.evaluate_with_context(ctx)
print(result)  # 49

# Symbolic evaluation (no numerical conversion)
ctx_symbolic = EvalContext.symbolic()
result_symbolic = expr.evaluate_with_context(ctx_symbolic)
print(result_symbolic)  # x^2 + 2*x*y + y^2 (still symbolic)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { JsExpression, EvalContext, symbols } = require('mathhook');

function symbol(name) {
    const [sym] = symbols(name);
    return sym;
}

const x = symbol('x');
const y = symbol('y');

// Formula: x² + 2xy + y²
const expr = x.pow(JsExpression.integer(2))
    .add(JsExpression.integer(2).multiply(x).multiply(y))
    .add(y.pow(JsExpression.integer(2)));

// Create numerical context with variable substitutions
const ctx = EvalContext.numeric([
    ['x', JsExpression.integer(3)],
    ['y', JsExpression.integer(4)]
]);

// Evaluate: (3)² + 2(3)(4) + (4)² = 9 + 24 + 16 = 49
const result = expr.evaluateWithContext(ctx);
console.log(result.toSimple());  // '49'
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
const metaDescription = "Complete guide to using MathHook from Python via PyO3 bindings.
Provides comprehensive documentation for the Python API including installation,
quick start, API reference, performance comparisons with SymPy, and integration patterns.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Python API Guide',
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
