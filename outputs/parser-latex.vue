<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>LaTeX Notation</h1>
      <p class="description">Parse and generate beautiful LaTeX notation for mathematical expressions.
MathHook provides full bidirectional support: Parse LaTeX → Expression, Expression → LaTeX.
Includes automatic type inference, implicit multiplication, and comprehensive coverage of 150+ LaTeX commands.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic LaTeX Parsing</h3>
        <p>Parse common LaTeX expressions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::parser::{Parser, ParserConfig};

let parser = Parser::new(ParserConfig::default());

// Fractions
let expr = parser.parse(r"\frac{x^2 + 1}{x - 1}")?;

// Functions
let expr = parser.parse(r"\sin(x) + \cos(y)")?;

// Square roots
let expr = parser.parse(r"\sqrt{x^2 + y^2}")?;
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Parser, ParserConfig

parser = Parser(ParserConfig.default())

# Fractions
expr = parser.parse(r"\frac{x^2 + 1}{x - 1}")

# Functions
expr = parser.parse(r"\sin(x) + \cos(y)")

# Square roots
expr = parser.parse(r"\sqrt{x^2 + y^2}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { Parser, ParserConfig } = require('mathhook');

const parser = new Parser(ParserConfig.default());

// Fractions
const expr = parser.parse(String.raw`\frac{x^2 + 1}{x - 1}`);

// Functions
const expr = parser.parse(String.raw`\sin(x) + \cos(y)`);

// Square roots
const expr = parser.parse(String.raw`\sqrt{x^2 + y^2}`);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Expression to LaTeX</h3>
        <p>Format expressions as LaTeX</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::formatter::latex::LaTeXFormatter;

let x = symbol!(x);
let expr = expr!(x^2 / 2);
let latex = expr.to_latex(None)?;
// Returns: \frac{x^{2}}{2}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, LaTeXFormatter

x = symbol('x')
expr = expr('x^2 / 2')
latex = expr.to_latex(None)
# Returns: \frac{x^{2}}{2}
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, LaTeXFormatter } = require('mathhook');

const x = symbol('x');
const expr = expr('x^2 / 2');
const latex = expr.toLatex(null);
// Returns: \frac{x^{2}}{2}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Noncommutative Type Inference</h3>
        <p>Automatic symbol type inference from LaTeX notation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::parser::latex::parse_latex;

// Matrix symbols (noncommutative): \mathbf{A}
let expr = parse_latex(r"\mathbf{A}\mathbf{X} = \mathbf{B}")?;
// Creates matrix symbols A, X, B where A*X ≠ X*A

// Operator symbols (noncommutative): \hat{p}
let expr = parse_latex(r"[\hat{x}, \hat{p}] = i\hbar")?;
// Creates operator symbols (quantum mechanics commutator)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.parser import parse_latex

# Matrix symbols (noncommutative): \mathbf{A}
expr = parse_latex(r"\mathbf{A}\mathbf{X} = \mathbf{B}")
# Creates matrix symbols A, X, B where A*X ≠ X*A

# Operator symbols (noncommutative): \hat{p}
expr = parse_latex(r"[\hat{x}, \hat{p}] = i\hbar")
# Creates operator symbols (quantum mechanics commutator)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { parseLatex } = require('mathhook');

// Matrix symbols (noncommutative): \mathbf{A}
const expr = parseLatex(String.raw`\mathbf{A}\mathbf{X} = \mathbf{B}`);
// Creates matrix symbols A, X, B where A*X ≠ X*A

// Operator symbols (noncommutative): \hat{p}
const expr = parseLatex(String.raw`[\hat{x}, \hat{p}] = i\hbar`);
// Creates operator symbols (quantum mechanics commutator)
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Calculus Notation</h3>
        <p>Parse calculus operations in LaTeX</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::parser::latex::parse_latex;

// Indefinite integral
let expr = parse_latex(r"\int x^2 \, dx")?;

// Definite integral
let expr = parse_latex(r"\int_0^{\infty} e^{-x} \, dx")?;

// Summations
let expr = parse_latex(r"\sum_{i=1}^{n} i^2")?;

// Limits
let expr = parse_latex(r"\lim_{x \to 0} \frac{\sin(x)}{x}")?;
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.parser import parse_latex

# Indefinite integral
expr = parse_latex(r"\int x^2 \, dx")

# Definite integral
expr = parse_latex(r"\int_0^{\infty} e^{-x} \, dx")

# Summations
expr = parse_latex(r"\sum_{i=1}^{n} i^2")

# Limits
expr = parse_latex(r"\lim_{x \to 0} \frac{\sin(x)}{x}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { parseLatex } = require('mathhook');

// Indefinite integral
const expr = parseLatex(String.raw`\int x^2 \, dx`);

// Definite integral
const expr = parseLatex(String.raw`\int_0^{\infty} e^{-x} \, dx`);

// Summations
const expr = parseLatex(String.raw`\sum_{i=1}^{n} i^2`);

// Limits
const expr = parseLatex(String.raw`\lim_{x \to 0} \frac{\sin(x)}{x}`);
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
const metaDescription = "Parse and generate beautiful LaTeX notation for mathematical expressions.
MathHook provides full bidirectional support: Parse LaTeX → Expression, Expression → LaTeX.
Includes automatic type inference, implicit multiplication, and comprehensive coverage of 150+ LaTeX commands.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'LaTeX Notation',
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
