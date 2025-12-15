<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Node.js/TypeScript API Guide</h1>
      <p class="description">Complete guide to using MathHook from Node.js and TypeScript via NAPI bindings.
Provides comprehensive documentation for the JavaScript/TypeScript API including installation,
quick start, API reference, integration patterns with Express/Next.js, and performance best practices.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Parsing and Simplification</h3>
        <p>Parse mathematical expressions from strings and simplify them</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::{expr, symbol, simplify, expand};

let x = symbol!(x);
let expr = expr!(x + x);
let result = simplify(expr);  // 2*x

let expr2 = expr!((x + 1)^2);
let expanded = expand(expr2);  // x^2 + 2*x + 1
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import parse, simplify, expand

expr = parse('x + x')
result = simplify(expr)
print(result)  # 2*x

expr2 = parse('(x + 1)^2')
expanded = expand(expr2)
print(expanded)  # x^2 + 2*x + 1
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { parse, simplify } = require('mathhook');

const expr = parse('x + x');
const result = simplify(expr);
console.log(result.toString());  // 2*x

const expr2 = parse('(x + 1)^2');
const expanded = expand(expr2);
console.log(expanded.toString());  // x^2 + 2*x + 1
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>TypeScript Type Safety</h3>
        <p>Use TypeScript for type-safe mathematical operations</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust"></code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"></code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Symbol, Expression, parse, simplify } from 'mathhook';

// Type-safe symbol creation
const x: Symbol = new Symbol('x');
const y: Symbol = new Symbol('y');

// Type-safe expression parsing
const expr: Expression = parse('x^2 + 2*x + 1');

// Type-safe operations
const simplified: Expression = simplify(expr);
console.log(simplified.toString());  // (x + 1)^2
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Derivatives in TypeScript</h3>
        <p>Compute symbolic derivatives with TypeScript type safety</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::{expr, symbol, derivative};

let x = symbol!(x);
let expr = expr!(x^3);

// First derivative
let df = derivative(&expr, &x, 1);
// Result: 3*x^2

// Second derivative
let d2f = derivative(&expr, &x, 2);
// Result: 6*x
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import Symbol, parse, derivative

x = Symbol('x')
expr = parse('x^3')

# First derivative
df = derivative(expr, x)
print(df)  # 3*x^2

# Second derivative
d2f = derivative(expr, x, order=2)
print(d2f)  # 6*x
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { Symbol, parse, derivative } from 'mathhook';

const x = new Symbol('x');
const expr = parse('x^3');

// First derivative
const df = derivative(expr, x);
console.log(df.toString());  // 3*x^2

// Second derivative
const d2f = derivative(expr, x, { order: 2 });
console.log(d2f.toString());  // 6*x
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Express.js API Integration</h3>
        <p>Build a REST API for mathematical operations using Express.js</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust"></code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"></code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import express from 'express';
import { parse, simplify, toLatex, derivative, Symbol } from 'mathhook';

const app = express();
app.use(express.json());

// Simplify endpoint
app.post('/api/simplify', (req, res) => {
    try {
        const { expression } = req.body;
        const expr = parse(expression);
        const simplified = simplify(expr);

        res.json({
            original: expression,
            simplified: simplified.toString(),
            latex: toLatex(simplified)
        });
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

// Derivative endpoint
app.post('/api/derivative', (req, res) => {
    try {
        const { expression, variable } = req.body;
        const expr = parse(expression);
        const x = new Symbol(variable);
        const deriv = derivative(expr, x);

        res.json({
            expression: expression,
            derivative: deriv.toString(),
            latex: toLatex(deriv)
        });
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

app.listen(3000, () => {
    console.log('Math API running on port 3000');
});
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Next.js Server Actions</h3>
        <p>Use MathHook in Next.js server actions for server-side computation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust"></code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"></code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// app/actions/math.ts
'use server';

import { parse, simplify, derivative, Symbol } from 'mathhook';

export async function simplifyExpression(expression: string) {
    try {
        const expr = parse(expression);
        const simplified = simplify(expr);
        return {
            success: true,
            result: simplified.toString()
        };
    } catch (error) {
        return {
            success: false,
            error: error.message
        };
    }
}

export async function computeDerivative(expression: string, variable: string) {
    try {
        const expr = parse(expression);
        const x = new Symbol(variable);
        const deriv = derivative(expr, x);
        return {
            success: true,
            result: deriv.toString()
        };
    } catch (error) {
        return {
            success: false,
            error: error.message
        };
    }
}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>WebSocket Server</h3>
        <p>Build a WebSocket server for real-time mathematical computation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust"></code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"></code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { WebSocketServer } from 'ws';
import { parse, simplify, derivative, Symbol } from 'mathhook';

const wss = new WebSocketServer({ port: 8080 });

wss.on('connection', (ws) => {
    ws.on('message', (data) => {
        try {
            const request = JSON.parse(data.toString());

            switch (request.operation) {
                case 'simplify': {
                    const expr = parse(request.expression);
                    const result = simplify(expr);
                    ws.send(JSON.stringify({
                        operation: 'simplify',
                        result: result.toString()
                    }));
                    break;
                }
                case 'derivative': {
                    const expr = parse(request.expression);
                    const x = new Symbol(request.variable);
                    const result = derivative(expr, x);
                    ws.send(JSON.stringify({
                        operation: 'derivative',
                        result: result.toString()
                    }));
                    break;
                }
            }
        } catch (error) {
            ws.send(JSON.stringify({ error: error.message }));
        }
    });
});
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Evaluation with Context</h3>
        <p>Advanced evaluation with custom contexts and variable substitutions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust"></code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import PyExpression as Expression, EvalContext

x = Expression.symbol("x")
y = Expression.symbol("y")

# Formula: x² + 2xy + y²
expr = x.pow(Expression.integer(2)).add(
    Expression.integer(2).multiply(x).multiply(y)
).add(y.pow(Expression.integer(2)))

# Create numerical context with variable substitutions
ctx = EvalContext.numeric({
    "x": Expression.integer(3),
    "y": Expression.integer(4)
})

# Evaluate: (3)² + 2(3)(4) + (4)² = 9 + 24 + 16 = 49
result = expr.evaluate_with_context(ctx)
print(result)  # 49
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { JsExpression, EvalContext, symbols } from 'mathhook';

function symbol(name: string) {
    const [sym] = symbols(name);
    return sym;
}

function integer(n: number) {
    return JsExpression.integer(n);
}

const x = symbol('x');
const y = symbol('y');

// Formula: x² + 2xy + y²
const expr = x.pow(integer(2))
    .add(integer(2).multiply(x).multiply(y))
    .add(y.pow(integer(2)));

// Create numerical context with variable substitutions
const ctx = EvalContext.numeric([
    ['x', integer(3)],
    ['y', integer(4)]
]);

// Evaluate: (3)² + 2(3)(4) + (4)² = 9 + 24 + 16 = 49
const result = expr.evaluateWithContext(ctx);
console.log(result.toSimple());  // '49'

// Symbolic evaluation (no numerical conversion)
const ctxSymbolic = EvalContext.symbolic();
const resultSymbolic = expr.evaluateWithContext(ctxSymbolic);
console.log(resultSymbolic.toSimple());  // 'x^2 + 2*x*y + y^2' (still symbolic)
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
const metaDescription = "Complete guide to using MathHook from Node.js and TypeScript via NAPI bindings.
Provides comprehensive documentation for the JavaScript/TypeScript API including installation,
quick start, API reference, integration patterns with Express/Next.js, and performance best practices.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Node.js/TypeScript API Guide',
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
