<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Common Patterns</h1>
      <p class="description">Common patterns and best practices when using MathHook, including macro usage
guidelines, polynomial construction, substitution patterns, function composition,
matrix operations, error handling, performance patterns, and educational features.
Includes detailed pitfalls to avoid.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Macro Usage - Correct Patterns</h3>
        <p>When to use macros vs explicit API</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// ALWAYS use macros for symbols
let x = symbol!(x);  // NOT Symbol::new("x")

// Simple expressions - use macros
let expr = expr!(x + y);
let expr = expr!(2 * x);
let expr = expr!(x ^ 2);

// Function calls - use macros
let expr = expr!(sin(x));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

# Python uses method chaining
x = Expression.symbol('x')
y = Expression.symbol('y')

expr = x.add(y)
expr = Expression.integer(2).mul(x)
expr = x.pow(2)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

// Node.js uses method chaining
const x = Expression.symbol('x');
const y = Expression.symbol('y');

const expr = x.add(y);
const expr2 = Expression.integer(2).mul(x);
const expr3 = x.pow(2);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Runtime Variables - Explicit API Required</h3>
        <p>Why macros don't work with loop variables</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// WRONG - creates Symbol("i"), not integer value
for i in 0..10 {
    let expr = expr!(i);  // BAD!
}

// CORRECT - use explicit API for runtime variables
for i in 0..10 {
    let term = Expression::integer(i);  // GOOD!
}

// CORRECT - programmatic construction
let x = symbol!(x);
let coefficients = vec![1, 2, 3];
let mut terms = Vec::new();
for i in 0..coefficients.len() {
    let coeff = Expression::integer(coefficients[i]);
    let x_expr = Expression::from(x.clone());
    let power = Expression::integer(i as i64);
    terms.push(Expression::mul(vec![coeff, Expression::pow(x_expr, power)]));
}
let polynomial = Expression::add(terms);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

# Python doesn't have compile-time macros
# Always use explicit API (which is fine)

x = Expression.symbol('x')
coefficients = [1, 2, 3]
terms = []
for i, coeff in enumerate(coefficients):
    coeff_expr = Expression.integer(coeff)
    power_expr = Expression.integer(i)
    term = coeff_expr.mul(x.pow(power_expr))
    terms.append(term)
polynomial = Expression.add(terms)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

// Node.js doesn't have compile-time macros
// Always use explicit API

const x = Expression.symbol('x');
const coefficients = [1, 2, 3];
const terms = [];
for (let i = 0; i < coefficients.length; i++) {
    const coeffExpr = Expression.integer(coefficients[i]);
    const powerExpr = Expression.integer(i);
    const term = coeffExpr.mul(x.pow(powerExpr));
    terms.push(term);
}
const polynomial = Expression.add(terms);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Building Polynomials - Dynamic Degree</h3>
        <p>Construct polynomials with runtime coefficients</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

fn build_polynomial(coefficients: &[i64], x: &Symbol) -> Expression {
    let mut terms = Vec::new();
    for (i, &coeff) in coefficients.iter().enumerate() {
        let coeff_expr = Expression::integer(coeff);
        let x_expr = Expression::from(x.clone());
        let power = Expression::integer(i as i64);
        let term = Expression::mul(vec![coeff_expr, Expression::pow(x_expr, power)]);
        terms.push(term);
    }
    Expression::add(terms)
}

let x = symbol!(x);
let poly = build_polynomial(&[1, -5, 6], &x);  // x^2 - 5x + 6
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

def build_polynomial(coefficients, x):
    terms = []
    for i, coeff in enumerate(coefficients):
        coeff_expr = Expression.integer(coeff)
        power = Expression.integer(i)
        term = coeff_expr.mul(x.pow(power))
        terms.append(term)
    return Expression.add(terms)

x = Expression.symbol('x')
poly = build_polynomial([1, -5, 6], x)  # x^2 - 5x + 6
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

function buildPolynomial(coefficients: number[], x: Expression): Expression {
    const terms = [];
    for (let i = 0; i < coefficients.length; i++) {
        const coeffExpr = Expression.integer(coefficients[i]);
        const power = Expression.integer(i);
        const term = coeffExpr.mul(x.pow(power));
        terms.push(term);
    }
    return Expression.add(terms);
}

const x = Expression.symbol('x');
const poly = buildPolynomial([1, -5, 6], x);  // x^2 - 5x + 6
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Substitution - Single and Multiple</h3>
        <p>Replace symbols with values</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use std::collections::HashMap;

let x = symbol!(x);
let y = symbol!(y);
let expr = expr!(add: (x * y), x, y);

// Single substitution
let mut vars = HashMap::new();
vars.insert("x".to_string(), Expression::integer(3));
let result = expr.substitute(&vars);

// Multiple substitutions
let mut vars = HashMap::new();
vars.insert("x".to_string(), Expression::integer(2));
vars.insert("y".to_string(), Expression::integer(3));
let result = expr.substitute(&vars);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')
y = Expression.symbol('y')
expr = x.mul(y).add(x).add(y)

# Single substitution
vars = {'x': Expression.integer(3)}
result = expr.substitute(vars)

# Multiple substitutions
vars = {'x': Expression.integer(2), 'y': Expression.integer(3)}
result = expr.substitute(vars)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const y = Expression.symbol('y');
const expr = x.mul(y).add(x).add(y);

// Single substitution
const vars1 = new Map([['x', Expression.integer(3)]]);
const result1 = expr.substitute(vars1);

// Multiple substitutions
const vars2 = new Map([
    ['x', Expression.integer(2)],
    ['y', Expression.integer(3)]
]);
const result2 = expr.substitute(vars2);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Function Composition</h3>
        <p>Compose functions by nesting</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// sin(cos(x)) - direct nesting
let composed = expr!(sin(cos(x)));

// Or build step by step
let inner = expr!(cos(x));
let composed_alt = function!(sin, inner);

println!("Composed function: {}", composed);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')

# Build step by step
inner = Expression.function('cos', [x])
composed = Expression.function('sin', [inner])

print(f"Composed function: {composed}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');

// Build step by step
const inner = Expression.function('cos', [x]);
const composed = Expression.function('sin', [inner]);

console.log(`Composed function: ${composed.toString()}`);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Performance - Bulk Operations</h3>
        <p>Efficient batch processing</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Simplify many expressions efficiently
let expressions = vec![
    expr!(x + x),
    expr!(x * 1),
    expr!(add: (x ^ 2), (-(x ^ 2))),
];

let simplified: Vec<_> = expressions
    .iter()
    .map(|e| e.simplify())
    .collect();
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')

# Simplify many expressions
expressions = [
    x.add(x),
    x.mul(Expression.integer(1)),
    x.pow(2).add(x.pow(2).neg())
]

simplified = [e.simplify() for e in expressions]
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');

// Simplify many expressions
const expressions = [
    x.add(x),
    x.mul(Expression.integer(1)),
    x.pow(2).add(x.pow(2).neg())
];

const simplified = expressions.map(e => e.simplify());
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Performance - Caching Results</h3>
        <p>Cache frequently computed expressions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use std::collections::HashMap;

let x = symbol!(x);
let mut cache: HashMap<String, Expression> = HashMap::new();

let expr = expr!(x ^ 2);
let key = format!("{}", expr);

if let Some(cached) = cache.get(&key) {
    println!("Using cached result");
} else {
    let result = expr.simplify();
    cache.insert(key, result.clone());
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')
cache = {}

expr = x.pow(2)
key = str(expr)

if key in cache:
    print("Using cached result")
else:
    result = expr.simplify()
    cache[key] = result
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const cache = new Map<string, Expression>();

const expr = x.pow(2);
const key = expr.toString();

if (cache.has(key)) {
    console.log("Using cached result");
} else {
    const result = expr.simplify();
    cache.set(key, result);
}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Common Pitfall - Float Equality</h3>
        <p>Never use == for approximate values</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// WRONG - comparing floats directly
let val1: f64 = 3.14;
let val2: f64 = 3.14000000001;
// if val1 == val2 { }  // BAD!

// CORRECT - use epsilon comparison
let tolerance: f64 = 1e-10;
if (val1 - val2).abs() < tolerance {
    println!("Values are approximately equal");
}

// OR use exact rationals for symbolic computation
let exact = Expression::rational(314, 100);  // Exact 3.14
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

# WRONG - comparing floats directly
val1 = 3.14
val2 = 3.14000000001
# if val1 == val2:  # BAD!

# CORRECT - use epsilon comparison
tolerance = 1e-10
if abs(val1 - val2) < tolerance:
    print("Values are approximately equal")

# OR use exact rationals
exact = Expression.rational(314, 100)  # Exact 3.14
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

// WRONG - comparing floats directly
const val1 = 3.14;
const val2 = 3.14000000001;
// if (val1 === val2) { }  // BAD!

// CORRECT - use epsilon comparison
const tolerance = 1e-10;
if (Math.abs(val1 - val2) < tolerance) {
    console.log("Values are approximately equal");
}

// OR use exact rationals
const exact = Expression.rational(314, 100);  // Exact 3.14
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
const metaDescription = "Common patterns and best practices when using MathHook, including macro usage
guidelines, polynomial construction, substitution patterns, function composition,
matrix operations, error handling, performance patterns, and educational features.
Includes detailed pitfalls to avoid.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Common Patterns',
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
