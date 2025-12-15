<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Basic Usage</h1>
      <p class="description">Comprehensive guide to using MathHook including expression creation with macros
and constructors, simplification, pattern matching, symbol manipulation, number
types, constants, and function expressions.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Expression Creation - Macros</h3>
        <p>Create expressions using ergonomic macros</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Simple arithmetic
let expr1 = expr!(x + y);
let expr2 = expr!(2 * x);
let expr3 = expr!(x ^ 2);

// Complex expressions with explicit grouping
let expr4 = expr!((x + 1) * (x - 1));

// Multi-term expressions
let expr5 = expr!(add: (2*x), (3*y), (-5));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')
y = Expression.symbol('y')

# Method chaining for expressions
expr1 = x.add(y)
expr2 = Expression.integer(2).mul(x)
expr3 = x.pow(2)

# Complex expressions
expr4 = x.add(1).mul(x.sub(1))
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const y = Expression.symbol('y');

// Method chaining for expressions
const expr1 = x.add(y);
const expr2 = Expression.integer(2).mul(x);
const expr3 = x.pow(2);

// Complex expressions
const expr4 = x.add(1).mul(x.sub(1));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Expression Creation - Constructors</h3>
        <p>Programmatic construction with explicit API</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::Expression;

// Numbers
let int = Expression::integer(42);
let float = Expression::float(3.14);
let rational = Expression::rational(3, 4);  // 3/4

// Operations
let sum = expr!(1 + 2);
let product = expr!(2 * x);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

# Numbers
int_val = Expression.integer(42)
float_val = Expression.float(3.14)
rational_val = Expression.rational(3, 4)  # 3/4

# Operations
sum_val = Expression.integer(1).add(Expression.integer(2))
product_val = Expression.integer(2).mul(x)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

// Numbers
const intVal = Expression.integer(42);
const floatVal = Expression.float(3.14);
const rationalVal = Expression.rational(3, 4);  // 3/4

// Operations
const sumVal = Expression.integer(1).add(Expression.integer(2));
const productVal = Expression.integer(2).mul(x);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Simplification</h3>
        <p>Transform expressions to canonical form</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);

// Combine like terms
let expr = expr!(x + x);
let simplified = expr.simplify();
// Result: 2*x

// Apply identities
let expr = expr!(x * 1);
let simplified = expr.simplify();
// Result: x

// Evaluate constants
let expr = expr!(2 + 3);
let simplified = expr.simplify();
// Result: 5
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')

# Combine like terms
expr = x.add(x)
simplified = expr.simplify()
# Result: 2*x

# Apply identities
expr = x.mul(Expression.integer(1))
simplified = expr.simplify()
# Result: x

# Evaluate constants
expr = Expression.integer(2).add(Expression.integer(3))
simplified = expr.simplify()
# Result: 5
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');

// Combine like terms
const expr = x.add(x);
const simplified = expr.simplify();
// Result: 2*x

// Apply identities
const expr2 = x.mul(Expression.integer(1));
const simplified2 = expr2.simplify();
// Result: x

// Evaluate constants
const expr3 = Expression.integer(2).add(Expression.integer(3));
const simplified3 = expr3.simplify();
// Result: 5
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Pattern Matching (Rust)</h3>
        <p>Inspect expression structure with pattern matching</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::Expression;

let x = symbol!(x);
let y = symbol!(y);
let test_expr = expr!(x + y);

match test_expr {
    Expression::Add(terms) => {
        println!("Addition with {} terms", terms.len());
    }
    Expression::Mul(factors) => {
        println!("Multiplication with {} factors", factors.len());
    }
    Expression::Pow(base, exp) => {
        println!("Power: base={}, exp={}", base, exp);
    }
    _ => {}
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Python doesn't have Rust-style pattern matching
# Use type checking instead
from mathhook import Expression

x = Expression.symbol('x')
y = Expression.symbol('y')
test_expr = x.add(y)

# Check expression type
if test_expr.is_add():
    print("Addition expression")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// TypeScript/JavaScript doesn't have pattern matching
// Use type checking instead
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const y = Expression.symbol('y');
const testExpr = x.add(y);

// Check expression type
if (testExpr.isAdd()) {
    console.log("Addition expression");
}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Number Types</h3>
        <p>Different number representations in MathHook</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Integers (exact, arbitrary precision)
let int = Expression::integer(123456789);

// Rationals (exact fractions)
let frac = Expression::rational(22, 7);  // 22/7 ≈ π

// Floats (approximate)
let float = Expression::float(3.14159265359);

// Complex numbers
let complex = Expression::complex(
    Expression::integer(3),
    Expression::integer(4)
);  // 3 + 4i
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

# Integers (exact)
int_val = Expression.integer(123456789)

# Rationals (exact fractions)
frac = Expression.rational(22, 7)  # 22/7 ≈ π

# Floats (approximate)
float_val = Expression.float(3.14159265359)

# Complex numbers
complex_val = Expression.complex(
    Expression.integer(3),
    Expression.integer(4)
)  # 3 + 4i
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

// Integers (exact)
const intVal = Expression.integer(123456789);

// Rationals (exact fractions)
const frac = Expression.rational(22, 7);  // 22/7 ≈ π

// Floats (approximate)
const floatVal = Expression.float(3.14159265359);

// Complex numbers
const complexVal = Expression.complex(
    Expression.integer(3),
    Expression.integer(4)
);  // 3 + 4i
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Mathematical Constants</h3>
        <p>Built-in mathematical constants</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let pi = Expression::pi();
let e = Expression::e();
let i = Expression::i();              // imaginary unit
let phi = Expression::golden_ratio();
let gamma = Expression::euler_gamma();
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

pi = Expression.pi()
e = Expression.e()
i = Expression.i()              # imaginary unit
phi = Expression.golden_ratio()
gamma = Expression.euler_gamma()
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const pi = Expression.pi();
const e = Expression.e();
const i = Expression.i();              // imaginary unit
const phi = Expression.goldenRatio();
const gamma = Expression.eulerGamma();
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Function Expressions</h3>
        <p>Create elementary function calls</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Elementary functions using expr! macro
let sin_x = expr!(sin(x));
let cos_x = expr!(cos(x));
let log_x = expr!(log(x));

// Or using function! macro
let tan_x = function!(tan, x);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Expression

x = Expression.symbol('x')

# Elementary functions
sin_x = Expression.function('sin', [x])
cos_x = Expression.function('cos', [x])
log_x = Expression.function('log', [x])
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');

// Elementary functions
const sinX = Expression.function('sin', [x]);
const cosX = Expression.function('cos', [x]);
const logX = Expression.function('log', [x]);
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
const metaDescription = "Comprehensive guide to using MathHook including expression creation with macros
and constructors, simplification, pattern matching, symbol manipulation, number
types, constants, and function expressions.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Basic Usage',
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
