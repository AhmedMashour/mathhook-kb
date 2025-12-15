<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Learning Paths</h1>
      <p class="description">Choose your journey based on background and goals. Structured learning paths for
Python data scientists, Node.js developers, Rust programmers, mathematics educators,
and computational scientists with time estimates and outcomes.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Python Data Scientist - SymPy Migration</h3>
        <p>Quick comparison of SymPy vs MathHook syntax</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Not applicable for Python path
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># SymPy syntax (familiar to data scientists)
# from sympy import symbols, simplify
# x, y = symbols('x y')
# expr = (x + y)**2

# MathHook syntax (similar but faster)
from mathhook import Expression

x = Expression.symbol('x')
y = Expression.symbol('y')
expr = (x.add(y)).pow(2)
simplified = expr.simplify()
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Not applicable for Python path
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Node.js Developer - Web Form Parsing</h3>
        <p>Parse user input LaTeX from web forms</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Not applicable for Node.js path
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Not applicable for Node.js path
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Parser, ParserConfig } from 'mathhook-node';

// Parse LaTeX from web form input
const userInput = req.body.equation; // e.g., "x^2 + 2x + 1"
const parser = new Parser(ParserConfig.default());
const expr = parser.parse(userInput);

// Render LaTeX output for display
const latex = expr.toLatex();
res.json({ latex });
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Rust Programmer - Custom Extension</h3>
        <p>Extend Universal Function Registry with custom function</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Implement custom simplification rule
fn custom_simplify(expr: &Expression) -> Expression {
    // Custom logic here
    expr.clone()
}

// Register custom function
// (Actual API may vary - check documentation)
let x = symbol!(x);
let expr = expr!(x ^ 2);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Not applicable for Rust path
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Not applicable for Rust path
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Mathematics Educator - Step-by-Step</h3>
        <p>Generate educational explanations for students</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!((x + 1) * (x - 1));

let explanation = expr.explain_simplification();
for step in &explanation.steps {
    println!("{}: {}", step.title, step.description);
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')
expr = (x.add(1)).mul(x.sub(1))

explanation = expr.explain_simplification()
for step in explanation.steps:
    print(f"{step.title}: {step.description}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const expr = x.add(1).mul(x.sub(1));

const explanation = expr.explainSimplification();
for (const step of explanation.steps) {
    console.log(`${step.title}: ${step.description}`);
}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Computational Scientist - Symbolic Jacobian</h3>
        <p>Generate Jacobian matrix for nonlinear system</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// System of equations
let f1 = expr!(add: (x ^ 2), y);
let f2 = expr!(x * y);

// Compute Jacobian symbolically
let df1_dx = f1.derivative(x.clone());
let df1_dy = f1.derivative(y.clone());
let df2_dx = f2.derivative(x.clone());
let df2_dy = f2.derivative(y.clone());

let jacobian = Expression::matrix(vec![
    vec![df1_dx, df1_dy],
    vec![df2_dx, df2_dy],
]);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')
y = Expression.symbol('y')

# System of equations
f1 = x.pow(2).add(y)
f2 = x.mul(y)

# Compute Jacobian symbolically
df1_dx = f1.derivative(x)
df1_dy = f1.derivative(y)
df2_dx = f2.derivative(x)
df2_dy = f2.derivative(y)

jacobian = Expression.matrix([
    [df1_dx, df1_dy],
    [df2_dx, df2_dy]
])
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const y = Expression.symbol('y');

// System of equations
const f1 = x.pow(2).add(y);
const f2 = x.mul(y);

// Compute Jacobian symbolically
const df1_dx = f1.derivative(x);
const df1_dy = f1.derivative(y);
const df2_dx = f2.derivative(x);
const df2_dy = f2.derivative(y);

const jacobian = Expression.matrix([
    [df1_dx, df1_dy],
    [df2_dx, df2_dy]
]);
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
const metaDescription = "Choose your journey based on background and goals. Structured learning paths for
Python data scientists, Node.js developers, Rust programmers, mathematics educators,
and computational scientists with time estimates and outcomes.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Learning Paths',
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
