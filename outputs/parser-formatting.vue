<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Expression Formatting</h1>
      <p class="description">Format mathematical expressions for display in multiple notations.
Supports LaTeX, Unicode, Wolfram, and custom formatters for different output targets.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Formatting</h3>
        <p>Format expressions in different notations</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::formatter::{LatexFormatter, UnicodeFormatter, WolframFormatter};

let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);

// LaTeX
let latex = LatexFormatter::new().format(&expr);
println!("{}", latex);   // x^{2} + 2 \cdot x + 1

// Unicode (pretty-print)
let unicode = UnicodeFormatter::new().format(&expr);
println!("{}", unicode); // x² + 2·x + 1

// Wolfram
let wolfram = WolframFormatter::new().format(&expr);
println!("{}", wolfram); // x^2 + 2*x + 1
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr
from mathhook.formatter import LatexFormatter, UnicodeFormatter, WolframFormatter

x = symbol('x')
expr_obj = expr('x^2 + 2*x + 1')

# LaTeX
latex = LatexFormatter().format(expr_obj)
print(latex)   # x^{2} + 2 \cdot x + 1

# Unicode (pretty-print)
unicode = UnicodeFormatter().format(expr_obj)
print(unicode) # x² + 2·x + 1

# Wolfram
wolfram = WolframFormatter().format(expr_obj)
print(wolfram) # x^2 + 2*x + 1
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');
const { LatexFormatter, UnicodeFormatter, WolframFormatter } = require('mathhook');

const x = symbol('x');
const exprObj = expr('x^2 + 2*x + 1');

// LaTeX
const latex = new LatexFormatter().format(exprObj);
console.log(latex);   // x^{2} + 2 \cdot x + 1

// Unicode (pretty-print)
const unicode = new UnicodeFormatter().format(exprObj);
console.log(unicode); // x² + 2·x + 1

// Wolfram
const wolfram = new WolframFormatter().format(exprObj);
console.log(wolfram); // x^2 + 2*x + 1
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Type-Aware Formatting</h3>
        <p>Noncommutative symbols formatted correctly</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::formatter::latex::LatexFormatter;

// Matrix symbols (bold)
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let matrix_expr = expr!(A * B);

let formatter = LatexFormatter::new();
println!("{}", formatter.format(&matrix_expr));
// Output: \mathbf{A}\mathbf{B}

// Operator symbols (hat)
let p = symbol!(p; operator);
let x = symbol!(x; operator);
let op_expr = expr!(p * x);

println!("{}", formatter.format(&op_expr));
// Output: \hat{p}\hat{x}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr
from mathhook.formatter import LatexFormatter

# Matrix symbols (bold)
A = symbol('A', type='matrix')
B = symbol('B', type='matrix')
matrix_expr = expr('A * B')

formatter = LatexFormatter()
print(formatter.format(matrix_expr))
# Output: \mathbf{A}\mathbf{B}

# Operator symbols (hat)
p = symbol('p', type='operator')
x = symbol('x', type='operator')
op_expr = expr('p * x')

print(formatter.format(op_expr))
# Output: \hat{p}\hat{x}
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');
const { LatexFormatter } = require('mathhook');

// Matrix symbols (bold)
const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });
const matrixExpr = expr('A * B');

const formatter = new LatexFormatter();
console.log(formatter.format(matrixExpr));
// Output: \mathbf{A}\mathbf{B}

// Operator symbols (hat)
const p = symbol('p', { type: 'operator' });
const x = symbol('x', { type: 'operator' });
const opExpr = expr('p * x');

console.log(formatter.format(opExpr));
// Output: \hat{p}\hat{x}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Customized LaTeX Output</h3>
        <p>Configure formatter behavior</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::formatter::latex::LatexFormatter;

// Configure formatter
let formatter = LatexFormatter::new()
    .with_precision(6)           // Float precision
    .with_explicit_multiplication(true)  // Show all * as \cdot
    .with_compact_fractions(false);      // Use \frac always

let expr = expr!(2*x / 3);
println!("{}", formatter.format(&expr));
// Output: \frac{2 \cdot x}{3}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr
from mathhook.formatter import LatexFormatter

# Configure formatter
formatter = LatexFormatter(
    precision=6,
    explicit_multiplication=True,
    compact_fractions=False
)

expr_obj = expr('2*x / 3')
print(formatter.format(expr_obj))
# Output: \frac{2 \cdot x}{3}
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');
const { LatexFormatter } = require('mathhook');

// Configure formatter
const formatter = new LatexFormatter({
    precision: 6,
    explicitMultiplication: true,
    compactFractions: false
});

const exprObj = expr('2*x / 3');
console.log(formatter.format(exprObj));
// Output: \frac{2 \cdot x}{3}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Educational Step Formatting</h3>
        <p>Format step-by-step explanations</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::formatter::latex::LatexFormatter;

let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);

// Generate step-by-step LaTeX
let formatter = LatexFormatter::new();

println!("Step 1: Start with {}", formatter.format(&expr));
let factored = expr.factor();  // (x+1)^2
println!("Step 2: Factor as {}", formatter.format(&factored));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr
from mathhook.formatter import LatexFormatter

x = symbol('x')
expr_obj = expr('x^2 + 2*x + 1')

# Generate step-by-step LaTeX
formatter = LatexFormatter()

print(f"Step 1: Start with {formatter.format(expr_obj)}")
factored = expr_obj.factor()  # (x+1)^2
print(f"Step 2: Factor as {formatter.format(factored)}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');
const { LatexFormatter } = require('mathhook');

const x = symbol('x');
const exprObj = expr('x^2 + 2*x + 1');

// Generate step-by-step LaTeX
const formatter = new LatexFormatter();

console.log(`Step 1: Start with ${formatter.format(exprObj)}`);
const factored = exprObj.factor();  // (x+1)^2
console.log(`Step 2: Factor as ${formatter.format(factored)}`);
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
const metaDescription = "Format mathematical expressions for display in multiple notations.
Supports LaTeX, Unicode, Wolfram, and custom formatters for different output targets.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Expression Formatting',
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
