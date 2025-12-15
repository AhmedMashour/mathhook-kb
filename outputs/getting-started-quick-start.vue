<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Quick Start</h1>
      <p class="description">Get up and running with MathHook in 5 minutes. Learn basic expression creation,
parsing, differentiation, and common operations across Rust, Python, and Node.js.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>First Expression - Quadratic</h3>
        <p>Build and simplify x^2 + 2x + 1</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

fn main() {
    let x = symbol!(x);
    let expr = expr!(add: (x ^ 2), (2 * x), 1);
    let simplified = expr.simplify();

    println!("Original: {}", expr);
    println!("Simplified: {}", simplified);
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')
expr = x.pow(2).add(x.multiply(2)).add(1)
simplified = expr.simplify()

print(f"Original: {expr}")
print(f"Simplified: {simplified}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const expr = x.pow(2).add(x.multiply(2)).add(1);
const simplified = expr.simplify();

console.log(`Original: ${expr.toString()}`);
console.log(`Simplified: ${simplified.toString()}`);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Parsing LaTeX</h3>
        <p>Parse LaTeX notation into symbolic expression</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let parser = Parser::new(ParserConfig::default());
let expr = parser.parse(r"\frac{x^2 + 1}{2}").unwrap();
println!("{}", expr);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Parser, ParserConfig

parser = Parser(ParserConfig.default())
expr = parser.parse(r"\frac{x^2 + 1}{2}")
print(expr)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Parser, ParserConfig } from 'mathhook-node';

const parser = new Parser(ParserConfig.default());
const expr = parser.parse(String.raw`\frac{x^2 + 1}{2}`);
console.log(expr.toString());
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Computing Derivatives</h3>
        <p>Compute first and second derivatives of x^3</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 3);

let derivative = expr.derivative(x.clone());
let second_derivative = expr.nth_derivative(x, 2);

println!("f(x) = {}", expr);
println!("f'(x) = {}", derivative);
println!("f''(x) = {}", second_derivative);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')
expr = x.pow(3)

derivative = expr.derivative(x)
second_derivative = expr.nth_derivative(x, 2)

print(f"f(x) = {expr}")
print(f"f'(x) = {derivative}")
print(f"f''(x) = {second_derivative}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const expr = x.pow(3);

const derivative = expr.derivative(x);
const secondDerivative = expr.nthDerivative(x, 2);

console.log(`f(x) = ${expr.toString()}`);
console.log(`f'(x) = ${derivative.toString()}`);
console.log(`f''(x) = ${secondDerivative.toString()}`);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Solving Equations</h3>
        <p>Solve x^2 = 4 symbolically</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let mut solver = MathSolver::new();
let equation = Expression::equation(expr!(x ^ 2), expr!(4));
let solutions = solver.solve(&equation, &x);

println!("Solutions: {:?}", solutions);
// Output: [x = 2, x = -2]
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression, MathSolver

x = Expression.symbol('x')
solver = MathSolver()
equation = Expression.equation(x.pow(2), Expression.integer(4))
solutions = solver.solve(equation, x)

print(f"Solutions: {solutions}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression, MathSolver } from 'mathhook-node';

const x = Expression.symbol('x');
const solver = new MathSolver();
const equation = Expression.equation(x.pow(2), Expression.integer(4));
const solutions = solver.solve(equation, x);

console.log(`Solutions: ${solutions}`);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Substitution</h3>
        <p>Substitute x = 3 into x^2 + 2x + 1</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use std::collections::HashMap;

let x = symbol!(x);
let expr = expr!(add: (x ^ 2), (2 * x), 1);

let mut vars = HashMap::new();
vars.insert("x".to_string(), Expression::integer(3));
let result = expr.substitute(&vars);
println!("Result: {}", result);
// Output: 16
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')
expr = x.pow(2).add(x.multiply(2)).add(1)

vars = {'x': Expression.integer(3)}
result = expr.substitute(vars)
print(f"Result: {result}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const expr = x.pow(2).add(x.multiply(2)).add(1);

const vars = new Map([['x', Expression.integer(3)]]);
const result = expr.substitute(vars);
console.log(`Result: ${result.toString()}`);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Creating Expressions Programmatically</h3>
        <p>Use macros for compile-time values, explicit API for runtime</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Compile-time - use macros
let expr = expr!((x ^ 2) + 3);

// Runtime - use explicit API
let mut terms = Vec::new();
for i in 0..5 {
    terms.push(Expression::mul(vec![
        Expression::integer(i as i64),
        Expression::pow(x.clone().into(), Expression::integer(i as i64))
    ]));
}
let polynomial = Expression::add(terms);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')

# Direct creation
expr = x.pow(2).add(3)

# Runtime creation
terms = []
for i in range(5):
    terms.append(
        Expression.mul([
            Expression.integer(i),
            x.pow(Expression.integer(i))
        ])
    )
polynomial = Expression.add(terms)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');

// Direct creation
const expr = x.pow(2).add(3);

// Runtime creation
const terms = [];
for (let i = 0; i < 5; i++) {
    terms.push(
        Expression.mul([
            Expression.integer(i),
            x.pow(Expression.integer(i))
        ])
    );
}
const polynomial = Expression.add(terms);
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
const metaDescription = "Get up and running with MathHook in 5 minutes. Learn basic expression creation,
parsing, differentiation, and common operations across Rust, Python, and Node.js.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Quick Start',
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
