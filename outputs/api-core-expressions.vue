<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Core Expression System</h1>
      <p class="description">The Expression type is the foundation of MathHook. Expressions are immutable,
32-byte cache-optimized structures representing mathematical constructs from
numbers to complex symbolic operations.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Creating Expressions with Macros</h3>
        <p>Use expr!() and symbol!() macros for ergonomic expression creation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Basic arithmetic
let sum = expr!(x + y);
let product = expr!(x * y);
let power = expr!(x ^ 2);

// Complex nested expressions
let complex = expr!(sin(x ^ 2) + cos(y ^ 2));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr

x = symbol('x')
y = symbol('y')

# Basic arithmetic
sum_expr = x + y
product = x * y
power = x**2

# Complex nested expressions
from mathhook import sin, cos
complex_expr = sin(x**2) + cos(y**2)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse } from 'mathhook';

const x = symbol('x');
const y = symbol('y');

// Parse expressions
const sum = parse('x + y');
const product = parse('x * y');
const power = parse('x^2');

// Complex nested expressions
const complex = parse('sin(x^2) + cos(y^2)');
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Immutability and Operations</h3>
        <p>All operations return new expressions, original unchanged</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let expr = expr!(x + 1);
let doubled = expr.mul(&expr!(2));  // Returns new expression
// `expr` is unchanged - still x + 1

// Safe to use in multiple threads
use std::sync::Arc;
let expr_arc = Arc::new(expr!(x ^ 2));
let clone = Arc::clone(&expr_arc);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">expr = x + 1
doubled = expr * 2  # Returns new expression
# expr is unchanged - still x + 1

# Safe for concurrent use
import threading
shared_expr = x**2
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const expr = parse('x + 1');
const doubled = expr.mul(2);  // Returns new expression
// expr is unchanged - still x + 1

// Immutable - safe for concurrent access
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Canonical Forms and Equality</h3>
        <p>Automatic normalization ensures equivalent expressions are equal</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let expr1 = expr!(x + y);
let expr2 = expr!(y + x);
assert_eq!(expr1, expr2);  // True - both normalized to x + y

// Flattening
let nested = expr!((x + y) + z);
// Automatically flattened to Add(x, y, z)

// Identity removal
let identity = expr!(x + 0);
assert_eq!(identity.simplify(), expr!(x));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">expr1 = x + y
expr2 = y + x
assert expr1 == expr2  # True - both normalized to x + y

# Flattening and identity removal
nested = (x + y) + z
identity = x + 0
assert identity.simplify() == x
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const expr1 = parse('x + y');
const expr2 = parse('y + x');
// Both normalized to x + y

// Identity removal
const identity = parse('x + 0');
const simplified = identity.simplify();
// Result: x
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Pattern Matching and Structure</h3>
        <p>Work with expression structure using pattern matching</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::Expression;

match expr {
    Expression::Add(terms) => {
        println!("Sum with {} terms", terms.len());
    }
    Expression::Mul(factors) => {
        println!("Product with {} factors", factors.len());
    }
    Expression::Pow(base, exp) => {
        println!("Power: {} ^ {}", base, exp);
    }
    Expression::Function(name, args) => {
        println!("Function {} with {} args", name, args.len());
    }
    _ => {}
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

# Python uses method introspection
if expr.is_add():
    terms = expr.get_terms()
    print(f"Sum with {len(terms)} terms")
elif expr.is_mul():
    factors = expr.get_factors()
    print(f"Product with {len(factors)} factors")
elif expr.is_pow():
    base, exp = expr.get_base_exp()
    print(f"Power: {base} ^ {exp}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook';

// Node.js uses type checking methods
if (expr.isAdd()) {
    const terms = expr.getTerms();
    console.log(`Sum with ${terms.length} terms`);
} else if (expr.isMul()) {
    const factors = expr.getFactors();
    console.log(`Product with ${factors.length} factors`);
} else if (expr.isPow()) {
    const [base, exp] = expr.getBaseExp();
    console.log(`Power: ${base} ^ ${exp}`);
}
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
const metaDescription = "The Expression type is the foundation of MathHook. Expressions are immutable,
32-byte cache-optimized structures representing mathematical constructs from
numbers to complex symbolic operations.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Core Expression System',
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
