<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Symbolic and Numerical Equation Solving</h1>
      <p class="description">Find solutions to equations symbolically and numerically. Supports linear, quadratic,
polynomial, and transcendental equations with automatic strategy selection. Includes
symbolic solving, numerical fallback, and parametric solutions.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Linear Equations</h3>
        <p>Solve linear equations ax + b = 0</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Solve: 2x + 3 = 0
let eq1 = expr!(2 * x + 3);
let mut solver = MathSolver::new();
let sol1 = solver.solve(&eq1, &x);
// Result: x = -3/2

// Solve: 5x - 10 = 0
let eq2 = expr!(5 * x - 10);
let sol2 = solver.solve(&eq2, &x);
// Result: x = 2
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, solve

x = symbol('x')

# Solve: 2x + 3 = 0
eq1 = 2*x + 3
sol1 = solve(eq1, x)
# Result: x = -3/2

# Solve: 5x - 10 = 0
eq2 = 5*x - 10
sol2 = solve(eq2, x)
# Result: x = 2
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse, solve } from 'mathhook';

const x = symbol('x');

// Solve: 2x + 3 = 0
const eq1 = parse('2*x + 3');
const sol1 = solve(eq1, x);
// Result: x = -3/2

// Solve: 5x - 10 = 0
const eq2 = parse('5*x - 10');
const sol2 = solve(eq2, x);
// Result: x = 2
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Quadratic Equations</h3>
        <p>Solve quadratic equations using quadratic formula</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Solve: x² - 5x + 6 = 0
let eq1 = expr!(x ^ 2 - 5 * x + 6);
let mut solver = MathSolver::new();
let solutions = solver.solve(&eq1, &x);
// Result: [x = 2, x = 3]

// Solve: x² - 4 = 0 (difference of squares)
let eq2 = expr!(x ^ 2 - 4);
let sol2 = solver.solve(&eq2, &x);
// Result: [x = -2, x = 2]

// Complex roots: x² + 1 = 0
let eq3 = expr!(x ^ 2 + 1);
let sol3 = solver.solve(&eq3, &x);
// Result: [x = i, x = -i]
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, solve

x = symbol('x')

# Solve: x² - 5x + 6 = 0
eq1 = x**2 - 5*x + 6
solutions = solve(eq1, x)
# Result: [x = 2, x = 3]

# Solve: x² - 4 = 0
eq2 = x**2 - 4
sol2 = solve(eq2, x)
# Result: [x = -2, x = 2]

# Complex roots
eq3 = x**2 + 1
sol3 = solve(eq3, x)
# Result: [x = i, x = -i]
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse, solve } from 'mathhook';

const x = symbol('x');

// Solve: x² - 5x + 6 = 0
const eq1 = parse('x^2 - 5*x + 6');
const solutions = solve(eq1, x);
// Result: [x = 2, x = 3]

// Solve: x² - 4 = 0
const eq2 = parse('x^2 - 4');
const sol2 = solve(eq2, x);
// Result: [x = -2, x = 2]

// Complex roots
const eq3 = parse('x^2 + 1');
const sol3 = solve(eq3, x);
// Result: [x = i, x = -i]
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Polynomial Equations</h3>
        <p>Solve polynomial equations via factorization</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Solve: x³ - 6x² + 11x - 6 = 0
// Factors: (x - 1)(x - 2)(x - 3)
let cubic = expr!(x ^ 3 - 6 * (x ^ 2) + 11 * x - 6);
let mut solver = MathSolver::new();
let solutions = solver.solve(&cubic, &x);
// Result: [x = 1, x = 2, x = 3]

// Solve: x⁴ - 1 = 0
let quartic = expr!(x ^ 4 - 1);
let sol2 = solver.solve(&quartic, &x);
// Result: [x = 1, x = -1, x = i, x = -i]
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, solve

x = symbol('x')

# Solve: x³ - 6x² + 11x - 6 = 0
cubic = x**3 - 6*x**2 + 11*x - 6
solutions = solve(cubic, x)
# Result: [x = 1, x = 2, x = 3]

# Solve: x⁴ - 1 = 0
quartic = x**4 - 1
sol2 = solve(quartic, x)
# Result: [x = 1, x = -1, x = i, x = -i]
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse, solve } from 'mathhook';

const x = symbol('x');

// Solve: x³ - 6x² + 11x - 6 = 0
const cubic = parse('x^3 - 6*x^2 + 11*x - 6');
const solutions = solve(cubic, x);
// Result: [x = 1, x = 2, x = 3]

// Solve: x⁴ - 1 = 0
const quartic = parse('x^4 - 1');
const sol2 = solve(quartic, x);
// Result: [x = 1, x = -1, x = i, x = -i]
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Transcendental Equations</h3>
        <p>Solve equations with trigonometric, exponential functions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);

// Solve: sin(x) = 0
let eq1 = expr!(sin(x));
let mut solver = MathSolver::new();
let solutions = solver.solve(&eq1, &x);
// Result: [x = 0, x = π, x = 2π, ...] (periodic)

// Solve: e^x = 5
let eq2 = expr!(exp(x) - 5);
let sol2 = solver.solve(&eq2, &x);
// Result: x = ln(5)

// Numerical fallback: x = cos(x)
let eq3 = expr!(x - cos(x));
let mut solver = MathSolver::new()
    .with_numerical_fallback(true)
    .with_tolerance(1e-10);
let sol3 = solver.solve(&eq3, &x);
// Result: x ≈ 0.739085133...
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, solve, sin, exp, cos

x = symbol('x')

# Solve: sin(x) = 0
eq1 = sin(x)
solutions = solve(eq1, x)
# Result: [x = 0, x = π, x = 2π, ...]

# Solve: e^x = 5
eq2 = exp(x) - 5
sol2 = solve(eq2, x)
# Result: x = ln(5)

# Numerical fallback
eq3 = x - cos(x)
sol3 = solve(eq3, x, numerical=True)
# Result: x ≈ 0.739085133...
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse, solve } from 'mathhook';

const x = symbol('x');

// Solve: sin(x) = 0
const eq1 = parse('sin(x)');
const solutions = solve(eq1, x);
// Result: [x = 0, x = π, x = 2π, ...]

// Solve: e^x = 5
const eq2 = parse('exp(x) - 5');
const sol2 = solve(eq2, x);
// Result: x = ln(5)

// Numerical fallback
const eq3 = parse('x - cos(x)');
const sol3 = solve(eq3, x, { numerical: true });
// Result: x ≈ 0.739085133...
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Real-World Application: Projectile Motion</h3>
        <p>Physics application finding time to hit ground</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let t = symbol!(t);
let v0 = symbol!(v0);  // Initial velocity
let h = symbol!(h);    // Initial height

// Position: y(t) = -16t² + v₀t + h
let position = expr!(-16 * (t ^ 2) + v0 * t + h);

// Substitute values: v₀ = 64 ft/s, h = 80 ft
let position_vals = position.substitute(&v0, &expr!(64))
                             .substitute(&h, &expr!(80));

// Find time when projectile hits ground (y = 0)
let mut solver = MathSolver::new();
let times = solver.solve(&position_vals, &t);
// Result: t ≈ 5 seconds (ignoring negative solution)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, solve

t = symbol('t')

# Position: y(t) = -16t² + 64t + 80
position = -16*t**2 + 64*t + 80

# Find time when y = 0
times = solve(position, t)
# Result: t ≈ 5 seconds (ignore negative)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse, solve } from 'mathhook';

const t = symbol('t');

// Position: y(t) = -16t² + 64t + 80
const position = parse('-16*t^2 + 64*t + 80');

// Find time when y = 0
const times = solve(position, t);
// Result: t ≈ 5 seconds
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Parametric Solutions</h3>
        <p>Solve in terms of parameters</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let x = symbol!(x);
let a = symbol!(a);
let b = symbol!(b);

// Solve: ax + b = 0 (parametric in a, b)
let equation = expr!(a * x + b);
let mut solver = MathSolver::new();
let solution = solver.solve(&equation, &x);
// Result: x = -b/a (symbolic solution)

// Now substitute specific values
let specific = solution.substitute(&a, &expr!(2))
                       .substitute(&b, &expr!(6));
// Result: x = -3
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, solve

x = symbol('x')
a = symbol('a')
b = symbol('b')

# Solve: ax + b = 0
equation = a*x + b
solution = solve(equation, x)
# Result: x = -b/a

# Substitute specific values
specific = solution.subs([(a, 2), (b, 6)])
# Result: x = -3
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">import { symbol, parse, solve } from 'mathhook';

const x = symbol('x');
const a = symbol('a');
const b = symbol('b');

// Solve: ax + b = 0
const equation = parse('a*x + b');
const solution = solve(equation, x);
// Result: x = -b/a

// Substitute specific values
const specific = solution.substitute(a, 2).substitute(b, 6);
// Result: x = -3
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
const metaDescription = "Find solutions to equations symbolically and numerically. Supports linear, quadratic,
polynomial, and transcendental equations with automatic strategy selection. Includes
symbolic solving, numerical fallback, and parametric solutions.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Symbolic and Numerical Equation Solving',
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
