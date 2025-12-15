<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Separable Differential Equations</h1>
      <p class="description">Solve first-order ordinary differential equations that can be separated into functions of x and y independently</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">\frac{dy}{dx} = g(x)h(y) \quad \Rightarrow \quad \int \frac{dy}{h(y)} = \int g(x)dx + C</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Simple Exponential Growth</h3>
        <p>The simplest separable ODE models exponential growth: dy/dx = y.
This can be separated as dy/y = dx, integrating gives ln|y| = x + C,
which simplifies to y = Ce^x. This models population growth, radioactive
decay (with negative constant), and compound interest.
</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::{symbol, expr};
use mathhook_core::ode::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);

// dy/dx = y (exponential growth)
let rhs = expr!(y);
let solver = SeparableODESolver::new();

// General solution: y = C*e^x
let solution = solver.solve(&rhs, &y, &x, None)?;
println!("General solution: {}", solution);

// Particular solution with y(0) = 3
let ic = (expr!(0), expr!(3));
let particular = solver.solve(&rhs, &y, &x, Some(ic))?;
println!("Particular solution: {}", particular); // y = 3*e^x
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr
from mathhook.ode import SeparableODESolver

x = symbol('x')
y = symbol('y')

# dy/dx = y (exponential growth)
rhs = y
solver = SeparableODESolver()

# General solution: y = C*e^x
solution = solver.solve(rhs, y, x)
print(f"General solution: {solution}")

# Particular solution with y(0) = 3
particular = solver.solve(rhs, y, x, initial=(0, 3))
print(f"Particular solution: {particular}")  # y = 3*e^x
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode');

const x = symbol('x');
const y = symbol('y');

// dy/dx = y (exponential growth)
const rhs = y;
const solver = new SeparableODESolver();

// General solution: y = C*e^x
const solution = solver.solve(rhs, y, x);
console.log(`General solution: ${solution}`);

// Particular solution with y(0) = 3
const particular = solver.solve(rhs, y, x, { initial: [0, 3] });
console.log(`Particular solution: ${particular}`); // y = 3*e^x
</code></pre>
        </div>

        
        <div class="output">
          <strong>Output:</strong>
          <pre><code>y = 3*e^x</code></pre>
        </div>
        
      </div>
      
      <div class="example-card">
        <h3>Logistic Growth Model</h3>
        <p>The logistic equation dy/dx = y(1-y) models population growth with carrying
capacity. Separating variables: dy/(y(1-y)) = dx. Using partial fractions:
(1/y + 1/(1-y))dy = dx. Integrating: ln|y/(1-y)| = x + C.
Solving for y: y = 1/(1 + Ce^(-x)). This S-curve is fundamental in ecology,
epidemiology, and marketing.
</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::{symbol, expr};
use mathhook_core::ode::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);

// dy/dx = y(1-y) (logistic growth)
let rhs = expr!(y * (1 - y));
let solver = SeparableODESolver::new();

// General solution: y = 1/(1 + C*e^(-x))
let solution = solver.solve(&rhs, &y, &x, None)?;

// With y(0) = 0.1 (10% initial population)
let ic = (expr!(0), expr!(0.1));
let particular = solver.solve(&rhs, &y, &x, Some(ic))?;
println!("{}", particular); // y = 1/(1 + 9*e^(-x))
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr
from mathhook.ode import SeparableODESolver

x, y = symbol('x'), symbol('y')

# dy/dx = y(1-y) (logistic growth)
rhs = y * (1 - y)
solver = SeparableODESolver()

# With y(0) = 0.1 (10% initial population)
solution = solver.solve(rhs, y, x, initial=(0, 0.1))
print(solution)  # y = 1/(1 + 9*e^(-x))
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode');

const x = symbol('x');
const y = symbol('y');

// dy/dx = y(1-y) (logistic growth)
const rhs = expr`${y} * (1 - ${y})`;
const solver = new SeparableODESolver();

// With y(0) = 0.1 (10% initial population)
const solution = solver.solve(rhs, y, x, { initial: [0, 0.1] });
console.log(solution); // y = 1/(1 + 9*e^(-x))
</code></pre>
        </div>

        
        <div class="output">
          <strong>Output:</strong>
          <pre><code>y = 1/(1 + 9*e^(-x))</code></pre>
        </div>
        
      </div>
      
      <div class="example-card">
        <h3>Newton's Law of Cooling</h3>
        <p>Temperature change follows dT/dt = k(T - T_ambient). For T_ambient = 20°C,
this becomes dT/dt = k(T - 20). Separating: dT/(T-20) = k*dt.
Integrating: ln|T-20| = kt + C, so T = 20 + Ce^(kt). If T(0) = 100°C and
k = -0.05, then T = 20 + 80*e^(-0.05t). This models coffee cooling,
forensic time-of-death estimation, and HVAC systems.
</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook_core::{symbol, expr};
use mathhook_core::ode::SeparableODESolver;

let t = symbol!(t);
let T = symbol!(T);
let k = expr!(-0.05);

// dT/dt = k(T - 20)
let rhs = expr!(k * (T - 20));
let solver = SeparableODESolver::new();

// T(0) = 100 (coffee starts at 100°C)
let ic = (expr!(0), expr!(100));
let solution = solver.solve(&rhs, &T, &t, Some(ic))?;
println!("Temperature: {}", solution); // T = 20 + 80*e^(-0.05t)

// Evaluate at t = 10 minutes
let temp_at_10 = solution.evaluate(&t, &expr!(10))?;
println!("After 10 min: {:.1}°C", temp_at_10);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr
from mathhook.ode import SeparableODESolver

t = symbol('t')
T = symbol('T')
k = -0.05

# dT/dt = k(T - 20)
rhs = k * (T - 20)
solver = SeparableODESolver()

# T(0) = 100 (coffee starts at 100°C)
solution = solver.solve(rhs, T, t, initial=(0, 100))
print(f"Temperature: {solution}")  # T = 20 + 80*e^(-0.05t)

# Evaluate at t = 10 minutes
temp_at_10 = solution.subs(t, 10)
print(f"After 10 min: {temp_at_10:.1f}°C")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode');

const t = symbol('t');
const T = symbol('T');
const k = -0.05;

// dT/dt = k(T - 20)
const rhs = expr`${k} * (${T} - 20)`;
const solver = new SeparableODESolver();

// T(0) = 100 (coffee starts at 100°C)
const solution = solver.solve(rhs, T, t, { initial: [0, 100] });
console.log(`Temperature: ${solution}`); // T = 20 + 80*e^(-0.05t)

// Evaluate at t = 10 minutes
const tempAt10 = solution.subs(t, 10);
console.log(`After 10 min: ${tempAt10.toFixed(1)}°C`);
</code></pre>
        </div>

        
        <div class="output">
          <strong>Output:</strong>
          <pre><code>T = 20 + 80*e^(-0.05t)</code></pre>
        </div>
        
      </div>
      
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const activeTab = ref('python')

// SEO metadata
const metaDescription = "Learn to solve separable differential equations symbolically with MathHook. Step-by-step guide with Newton's cooling, exponential growth, and logistic models. Fast, accurate ODE solving for Rust, Python, JavaScript."
const keywords = ["separable differential equations","ODE solver","separation of variables","first order ODE","symbolic ODE solving"]

// Define page metadata
definePageMeta({
  title: 'Separable Differential Equations',
  description: metaDescription,
  keywords: keywords.join(', '),
  ogImage: 'https://docs.mathhook.org/images/ode-separable-og.png',
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
