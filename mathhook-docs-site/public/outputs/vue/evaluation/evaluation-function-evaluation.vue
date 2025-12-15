<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Function Evaluation</h1>
      <p class="description">MathHook provides a unified, intelligent function evaluation system that handles both symbolic
and numerical computation. The system uses the Universal Function Registry architecture to
dispatch function calls to specialized implementations while maintaining mathematical correctness.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Elementary Functions</h3>
        <p>Evaluating basic trigonometric and exponential functions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

let sin_x = expr!(sin(x));
let cos_x = expr!(cos(x));
let exp_x = expr!(exp(x));
let log_x = expr!(log(x));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr

x = symbol('x')

sin_x = expr('sin(x)')
cos_x = expr('cos(x)')
exp_x = expr('exp(x)')
log_x = expr('log(x)')
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');

const x = symbol('x');

const sinX = expr('sin(x)');
const cosX = expr('cos(x)');
const expX = expr('exp(x)');
const logX = expr('log(x)');
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Special Value Evaluation</h3>
        <p>Automatic simplification of known exact values</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Trigonometric special values
let sin_0 = expr!(sin(0));
assert_eq!(sin_0.simplify(), expr!(0));

let cos_0 = expr!(cos(0));
assert_eq!(cos_0.simplify(), expr!(1));

// Exponential and logarithmic
let exp_0 = expr!(exp(0));
assert_eq!(exp_0.simplify(), expr!(1));

let log_1 = expr!(log(1));
assert_eq!(log_1.simplify(), expr!(0));
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import expr

# Trigonometric special values
sin_0 = expr('sin(0)')
assert sin_0.simplify() == expr('0')

cos_0 = expr('cos(0)')
assert cos_0.simplify() == expr('1')

# Exponential and logarithmic
exp_0 = expr('exp(0)')
assert exp_0.simplify() == expr('1')

log_1 = expr('log(1)')
assert log_1.simplify() == expr('0')
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { expr } = require('mathhook');

// Trigonometric special values
const sin0 = expr('sin(0)');
console.assert(sin0.simplify().equals(expr('0')));

const cos0 = expr('cos(0)');
console.assert(cos0.simplify().equals(expr('1')));

// Exponential and logarithmic
const exp0 = expr('exp(0)');
console.assert(exp0.simplify().equals(expr('1')));

const log1 = expr('log(1)');
console.assert(log1.simplify().equals(expr('0')));
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Composite Expression Evaluation</h3>
        <p>Mixed symbolic and numeric evaluation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// sqrt(4) evaluates to 2, symbolic parts preserved
let composite = expr!((sin((x^2) + 1) * cos(y)) - sqrt(4));
let result = composite.simplify();
// Result: sin(x^2 + 1) * cos(y) - 2
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr

x = symbol('x')
y = symbol('y')

# sqrt(4) evaluates to 2, symbolic parts preserved
composite = expr('sin(x^2 + 1) * cos(y) - sqrt(4)')
result = composite.simplify()
# Result: sin(x^2 + 1) * cos(y) - 2
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');

const x = symbol('x');
const y = symbol('y');

// sqrt(4) evaluates to 2, symbolic parts preserved
const composite = expr('sin(x^2 + 1) * cos(y) - sqrt(4)');
const result = composite.simplify();
// Result: sin(x^2 + 1) * cos(y) - 2
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Function Composition</h3>
        <p>Nested and composed functions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// sin(cos(x))
let nested = expr!(sin(cos(x)));

// exp(log(x)) simplifies to x
let exp_log = expr!(exp(log(x)));
let simplified = exp_log.simplify();
// Result: x (identity simplification)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr

x = symbol('x')

# sin(cos(x))
nested = expr('sin(cos(x))')

# exp(log(x)) simplifies to x
exp_log = expr('exp(log(x))')
simplified = exp_log.simplify()
# Result: x
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');

const x = symbol('x');

// sin(cos(x))
const nested = expr('sin(cos(x))');

// exp(log(x)) simplifies to x
const expLog = expr('exp(log(x))');
const simplified = expLog.simplify();
// Result: x
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Bulk Evaluation</h3>
        <p>Efficient numerical evaluation over multiple points</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::functions::FunctionEvaluator;

let evaluator = FunctionEvaluator::new();
let points = vec![0.0, 0.5, 1.0, 1.5, 2.0];

// SIMD-optimized evaluation
if let Some(results) = evaluator.evaluate_bulk_f64("sin", &points) {
    println!("Results: {:?}", results);
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.functions import FunctionEvaluator

evaluator = FunctionEvaluator()
points = [0.0, 0.5, 1.0, 1.5, 2.0]

# SIMD-optimized evaluation
results = evaluator.evaluate_bulk('sin', points)
print(f"Results: {results}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { FunctionEvaluator } = require('mathhook/functions');

const evaluator = new FunctionEvaluator();
const points = [0.0, 0.5, 1.0, 1.5, 2.0];

// SIMD-optimized evaluation
const results = evaluator.evaluateBulk('sin', points);
console.log(`Results: ${results}`);
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
const metaDescription = "MathHook provides a unified, intelligent function evaluation system that handles both symbolic
and numerical computation. The system uses the Universal Function Registry architecture to
dispatch function calls to specialized implementations while maintaining mathematical correctness.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Function Evaluation',
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
