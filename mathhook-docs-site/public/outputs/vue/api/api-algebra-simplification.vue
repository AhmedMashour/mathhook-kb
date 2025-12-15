<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Symbolic Simplification</h1>
      <p class="description">Comprehensive symbolic simplification for mathematical expressions, with full
support for noncommutative algebra (matrices, operators, quaternions). Implements
canonical forms and mathematical identities to reduce expressions to simplest form.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Simplification</h3>
        <p>Identity elements and constant folding</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Identity elements
let expr = expr!((x + 0) * 1);
let simplified = expr.simplify();
// Result: x

// Constant folding
let expr = expr!(2 + 3);
let simplified = expr.simplify();
// Result: 5
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

x = symbol('x')

# Identity elements
expr = (x + 0) * 1
simplified = expr.simplify()
# Result: x

# Constant folding
expr = 2 + 3
simplified = expr.simplify()
# Result: 5
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse } from 'mathhook';

const x = symbol('x');

// Identity elements
const expr = parse('(x + 0) * 1');
const simplified = expr.simplify();
// Result: x

// Constant folding
const expr2 = parse('2 + 3');
const simplified2 = expr2.simplify();
// Result: 5
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Power Rule Simplification</h3>
        <p>Combine like powers with same base</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Combine like powers
let expr = expr!((x^2) * (x^3));
let simplified = expr.simplify();
// Result: x^5

// Multiple powers
let expr = expr!((x^2) * (x^3) * (x^4));
let simplified = expr.simplify();
// Result: x^9
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

x = symbol('x')

# Combine like powers
expr = x**2 * x**3
simplified = expr.simplify()
# Result: x^5

# Multiple powers
expr = x**2 * x**3 * x**4
simplified = expr.simplify()
# Result: x^9
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse } from 'mathhook';

const x = symbol('x');

// Combine like powers
const expr = parse('x^2 * x^3');
const simplified = expr.simplify();
// Result: x^5

// Multiple powers
const expr2 = parse('x^2 * x^3 * x^4');
const simplified2 = expr2.simplify();
// Result: x^9
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Noncommutative Algebra</h3>
        <p>Preserve order for noncommutative symbols</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Scalar symbols (commutative) - factors can be sorted
let x = symbol!(x);
let y = symbol!(y);
let expr = expr!(y * x);
let simplified = expr.simplify();
// Result: x * y (sorted alphabetically)

// Matrix symbols (noncommutative) - order preserved
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let expr = expr!(B * A);
let simplified = expr.simplify();
// Result: B * A (original order preserved)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

# Scalar symbols (commutative)
x = symbol('x')
y = symbol('y')
expr = y * x
simplified = expr.simplify()
# Result: x * y (sorted)

# Matrix symbols (noncommutative)
A = symbol('A', matrix=True)
B = symbol('B', matrix=True)
expr = B * A
simplified = expr.simplify()
# Result: B * A (order preserved)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse } from 'mathhook';

// Scalar symbols (commutative)
const x = symbol('x');
const y = symbol('y');
const expr = parse('y * x');
const simplified = expr.simplify();
// Result: x * y (sorted)

// Matrix symbols (noncommutative)
const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });
const expr2 = parse('B * A');
const simplified2 = expr2.simplify();
// Result: B * A (order preserved)
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Power Distribution (Commutative Only)</h3>
        <p>Distribute powers for scalars, not for matrices</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Scalars (commutative): distributes power
let x = symbol!(x);
let y = symbol!(y);
let expr = expr!((x * y) ^ 2);
let simplified = expr.simplify();
// Result: x^2 * y^2 (distributed)

// Matrices (noncommutative): does NOT distribute
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let expr = expr!((A * B) ^ 2);
let simplified = expr.simplify();
// Result: (A*B)^2 (NOT distributed to A^2 * B^2)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol

# Scalars (commutative)
x = symbol('x')
y = symbol('y')
expr = (x * y)**2
simplified = expr.simplify()
# Result: x^2 * y^2

# Matrices (noncommutative)
A = symbol('A', matrix=True)
B = symbol('B', matrix=True)
expr = (A * B)**2
simplified = expr.simplify()
# Result: (A*B)^2
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse } from 'mathhook';

// Scalars (commutative)
const expr = parse('(x * y)^2');
const simplified = expr.simplify();
// Result: x^2 * y^2

// Matrices (noncommutative)
const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });
const expr2 = parse('(A * B)^2');
const simplified2 = expr2.simplify();
// Result: (A*B)^2
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Rational Arithmetic</h3>
        <p>Exact rational computation with arbitrary precision</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let expr = expr!(1/3 + 1/6);  // Rational arithmetic
let simplified = expr.simplify();
// Result: 1/2 (exact rational, not 0.5)

// Arbitrary precision
let expr = expr!(1/999999999 + 1/999999999);
let simplified = expr.simplify();
// Result: 2/999999999 (exact, no overflow)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr as parse_expr

expr = parse_expr('1/3 + 1/6')
simplified = expr.simplify()
# Result: 1/2 (exact rational)

# Arbitrary precision
expr = parse_expr('1/999999999 + 1/999999999')
simplified = expr.simplify()
# Result: 2/999999999 (exact)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { parse } from 'mathhook';

const expr = parse('1/3 + 1/6');
const simplified = expr.simplify();
// Result: 1/2 (exact rational)

// Arbitrary precision
const expr2 = parse('1/999999999 + 1/999999999');
const simplified2 = expr2.simplify();
// Result: 2/999999999 (exact)
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
const metaDescription = "Comprehensive symbolic simplification for mathematical expressions, with full
support for noncommutative algebra (matrices, operators, quaternions). Implements
canonical forms and mathematical identities to reduce expressions to simplest form.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Symbolic Simplification',
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
