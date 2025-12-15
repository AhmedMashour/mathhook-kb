<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Noncommutative Algebra</h1>
      <p class="description">Support for noncommutative algebra in MathHook with four symbol types:
Scalar (commutative), Matrix, Operator, and Quaternion (all noncommutative).
Essential for quantum mechanics, linear algebra, and 3D rotations.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">Noncommutative multiplication: $$A \times B \neq B \times A$$

Symbol types:
- Scalar: $x \cdot y = y \cdot x$ (commutative)
- Matrix: $A \cdot B \neq B \cdot A$ (noncommutative)
- Operator: $[\hat{x}, \hat{p}] = \hat{x}\hat{p} - \hat{p}\hat{x} \neq 0$
- Quaternion: $i \cdot j = k$, but $j \cdot i = -k$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Matrix Symbols</h3>
        <p>Create noncommutative matrix symbols</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Order matters!
let AB = expr!(A * B);  // A*B
let BA = expr!(B * A);  // B*A ≠ A*B
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">A = MatrixSymbol('A', n, n)
B = MatrixSymbol('B', n, n)

AB = A * B  # Noncommutative
BA = B * A  # Different!
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const A = symbol('A', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

const AB = A.mul(B);  // Noncommutative
const BA = B.mul(A);  // Different!
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Quantum Operators</h3>
        <p>Position and momentum operators (canonical commutation relation)</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x; operator);
let p = symbol!(p; operator);

// Commutator: [x, p] = xp - px
let commutator = expr!((x * p) - (p * x));
// Physical result: [x, p] = iℏ
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = Operator('x')
p = Operator('p')

# Commutator
commutator = Commutator(x, p)
# Result: I*hbar
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x', {type: 'operator'});
const p = symbol('p', {type: 'operator'});

// Commutator
const comm = x.mul(p).sub(p.mul(x));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Quaternions</h3>
        <p>3D rotation with quaternion multiplication</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);

let ij = expr!(i * j);  // i*j = k
let ji = expr!(j * i);  // j*i = -k (different!)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy.algebras.quaternion import Quaternion

i = Quaternion(0, 1, 0, 0)
j = Quaternion(0, 0, 1, 0)

ij = i * j  # k
ji = j * i  # -k
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const i = symbol('i', {type: 'quaternion'});
const j = symbol('j', {type: 'quaternion'});

const ij = i.mul(j);  // k
const ji = j.mul(i);  // -k
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>LaTeX Type Inference</h3>
        <p>Parser infers types from LaTeX notation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let parser = Parser::new(ParserConfig::default());

// Bold notation → Matrix type
let eq1 = parser.parse(r"\mathbf{A}\mathbf{X} = \mathbf{B}").unwrap();

// Hat notation → Operator type
let eq2 = parser.parse(r"\hat{H}\hat{\psi} = E\hat{\psi}").unwrap();
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from sympy.parsing.latex import parse_latex

# Bold → Matrix
eq1 = parse_latex(r'\mathbf{A}\mathbf{X} = \mathbf{B}')

# Hat → Operator
eq2 = parse_latex(r'\hat{H}\hat{\psi} = E\hat{\psi}')
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const parser = new Parser();

// Bold → Matrix
const eq1 = parser.parse('\\mathbf{A}\\mathbf{X} = \\mathbf{B}');

// Hat → Operator
const eq2 = parser.parse('\\hat{H}\\hat{\\psi} = E\\hat{\\psi}');
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
const metaDescription = "Support for noncommutative algebra in MathHook with four symbol types:
Scalar (commutative), Matrix, Operator, and Quaternion (all noncommutative).
Essential for quantum mechanics, linear algebra, and 3D rotations.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Noncommutative Algebra',
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
