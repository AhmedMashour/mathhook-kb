<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Parser Design for Noncommutative Algebra</h1>
      <p class="description">Design documentation for MathHook's type-aware LaTeX parser that automatically infers symbol types.
Enables seamless support for noncommutative algebra without explicit type annotations in mathematical expressions.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Matrix Type Inference</h3>
        <p>LaTeX bold notation creates matrix symbols</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::parser::latex::parse_latex;

// Bold notation → Matrix symbols
let expr = parse_latex(r"\mathbf{A}\mathbf{B} \neq \mathbf{B}\mathbf{A}")?;

// A and B are noncommutative matrices
// A*B ≠ B*A in general
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.parser import parse_latex

# Bold notation → Matrix symbols
expr = parse_latex(r"\mathbf{A}\mathbf{B} \neq \mathbf{B}\mathbf{A}")

# A and B are noncommutative matrices
# A*B ≠ B*A in general
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { parseLatex } = require('mathhook');

// Bold notation → Matrix symbols
const expr = parseLatex(String.raw`\mathbf{A}\mathbf{B} \neq \mathbf{B}\mathbf{A}`);

// A and B are noncommutative matrices
// A*B ≠ B*A in general
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Operator Type Inference</h3>
        <p>LaTeX hat notation creates operator symbols</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::parser::latex::parse_latex;

// Hat notation → Operator symbols
let expr = parse_latex(r"[\hat{x}, \hat{p}] = i\hbar")?;

// Canonical commutation relation
// x and p are noncommutative operators
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.parser import parse_latex

# Hat notation → Operator symbols
expr = parse_latex(r"[\hat{x}, \hat{p}] = i\hbar")

# Canonical commutation relation
# x and p are noncommutative operators
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { parseLatex } = require('mathhook');

// Hat notation → Operator symbols
const expr = parseLatex(String.raw`[\hat{x}, \hat{p}] = i\hbar`);

// Canonical commutation relation
// x and p are noncommutative operators
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Mixed Type Expression</h3>
        <p>Different symbol types in same expression</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::parser::latex::parse_latex;

// Quantum mechanics: scalar + operators + matrix
let expr = parse_latex(r"\hbar \omega \hat{a}^\dagger \hat{a} + \mathbf{H}_0")?;

// ℏ and ω: scalars (commutative)
// â†, â: operators (noncommutative)
// H₀: matrix (noncommutative)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.parser import parse_latex

# Quantum mechanics: scalar + operators + matrix
expr = parse_latex(r"\hbar \omega \hat{a}^\dagger \hat{a} + \mathbf{H}_0")

# ℏ and ω: scalars (commutative)
# â†, â: operators (noncommutative)
# H₀: matrix (noncommutative)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { parseLatex } = require('mathhook');

// Quantum mechanics: scalar + operators + matrix
const expr = parseLatex(String.raw`\hbar \omega \hat{a}^\dagger \hat{a} + \mathbf{H}_0`);

// ℏ and ω: scalars (commutative)
// â†, â: operators (noncommutative)
// H₀: matrix (noncommutative)
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
const metaDescription = "Design documentation for MathHook's type-aware LaTeX parser that automatically infers symbol types.
Enables seamless support for noncommutative algebra without explicit type annotations in mathematical expressions.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Parser Design for Noncommutative Algebra',
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
