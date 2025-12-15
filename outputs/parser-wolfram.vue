<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Wolfram Language Notation</h1>
      <p class="description">Parse and generate Mathematica/Wolfram Language syntax for compatibility with Wolfram products.
Supports bidirectional conversion between MathHook expressions and Wolfram notation.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Wolfram Parsing</h3>
        <p>Parse common Wolfram Language expressions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::parser::wolfram::parse_wolfram;

// Functions (capital letters, brackets)
let expr = parse_wolfram("Sin[x]")?;
let expr = parse_wolfram("Exp[x]")?;
let expr = parse_wolfram("Log[x]")?;

// Powers use ^ (not brackets)
let expr = parse_wolfram("x^2")?;
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.parser import parse_wolfram

# Functions (capital letters, brackets)
expr = parse_wolfram("Sin[x]")
expr = parse_wolfram("Exp[x]")
expr = parse_wolfram("Log[x]")

# Powers use ^ (not brackets)
expr = parse_wolfram("x^2")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { parseWolfram } = require('mathhook');

// Functions (capital letters, brackets)
const expr = parseWolfram("Sin[x]");
const expr = parseWolfram("Exp[x]");
const expr = parseWolfram("Log[x]");

// Powers use ^ (not brackets)
const expr = parseWolfram("x^2");
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Calculus Operations</h3>
        <p>Wolfram calculus notation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::parser::wolfram::parse_wolfram;

// Derivative: D[expr, var]
let expr = parse_wolfram("D[x^2, x]")?;  // 2x

// Integral: Integrate[expr, var]
let expr = parse_wolfram("Integrate[x^2, x]")?;  // x^3/3

// Definite integral: Integrate[expr, {var, a, b}]
let expr = parse_wolfram("Integrate[x^2, {x, 0, 1}]")?;

// Limit: Limit[expr, var -> value]
let expr = parse_wolfram("Limit[Sin[x]/x, x -> 0]")?;
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.parser import parse_wolfram

# Derivative: D[expr, var]
expr = parse_wolfram("D[x^2, x]")  # 2x

# Integral: Integrate[expr, var]
expr = parse_wolfram("Integrate[x^2, x]")  # x^3/3

# Definite integral: Integrate[expr, {var, a, b}]
expr = parse_wolfram("Integrate[x^2, {x, 0, 1}]")

# Limit: Limit[expr, var -> value]
expr = parse_wolfram("Limit[Sin[x]/x, x -> 0]")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { parseWolfram } = require('mathhook');

// Derivative: D[expr, var]
const expr = parseWolfram("D[x^2, x]");  // 2x

// Integral: Integrate[expr, var]
const expr = parseWolfram("Integrate[x^2, x]");  // x^3/3

// Definite integral: Integrate[expr, {var, a, b}]
const expr = parseWolfram("Integrate[x^2, {x, 0, 1}]");

// Limit: Limit[expr, var -> value]
const expr = parseWolfram("Limit[Sin[x]/x, x -> 0]");
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Generating Wolfram Output</h3>
        <p>Format MathHook expressions as Wolfram Language</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::formatter::wolfram::WolframFormatter;

let formatter = WolframFormatter::new();
let x = symbol!(x);

// Functions
let expr = expr!(sin(x));
println!("{}", formatter.format(&expr));  // Sin[x]

// Complex expressions
let expr = expr!((x + 1) / (x - 1));
println!("{}", formatter.format(&expr));  // (x + 1)/(x - 1)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, WolframFormatter

formatter = WolframFormatter()
x = symbol('x')

# Functions
expr_obj = expr('sin(x)')
print(formatter.format(expr_obj))  # Sin[x]

# Complex expressions
expr_obj = expr('(x + 1) / (x - 1)')
print(formatter.format(expr_obj))  # (x + 1)/(x - 1)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, WolframFormatter } = require('mathhook');

const formatter = new WolframFormatter();
const x = symbol('x');

// Functions
const exprObj = expr('sin(x)');
console.log(formatter.format(exprObj));  // Sin[x]

// Complex expressions
const exprObj = expr('(x + 1) / (x - 1)');
console.log(formatter.format(exprObj));  // (x + 1)/(x - 1)
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Cross-Platform Validation</h3>
        <p>Export to Wolfram for verification</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::formatter::wolfram::WolframFormatter;

// Compute derivative in MathHook
let x = symbol!(x);
let f = expr!(x^3 + 2*x^2 + x);
let df = f.derivative(&x, 1);

// Export to Wolfram for verification
let formatter = WolframFormatter::new();
let wolfram_code = formatter.format(&df);

println!("Verify in Wolfram Alpha:");
println!("Simplify[{}]", wolfram_code);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, WolframFormatter

# Compute derivative in MathHook
x = symbol('x')
f = expr('x^3 + 2*x^2 + x')
df = f.derivative(x, 1)

# Export to Wolfram for verification
formatter = WolframFormatter()
wolfram_code = formatter.format(df)

print("Verify in Wolfram Alpha:")
print(f"Simplify[{wolfram_code}]")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, WolframFormatter } = require('mathhook');

// Compute derivative in MathHook
const x = symbol('x');
const f = expr('x^3 + 2*x^2 + x');
const df = f.derivative(x, 1);

// Export to Wolfram for verification
const formatter = new WolframFormatter();
const wolframCode = formatter.format(df);

console.log("Verify in Wolfram Alpha:");
console.log(`Simplify[${wolframCode}]`);
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
const metaDescription = "Parse and generate Mathematica/Wolfram Language syntax for compatibility with Wolfram products.
Supports bidirectional conversion between MathHook expressions and Wolfram notation.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Wolfram Language Notation',
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
