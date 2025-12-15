<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Separation of Variables for PDEs</h1>
      <p class="description">Separation of variables is the fundamental technique for solving linear partial differential
equations (PDEs) with boundary conditions. This method transforms a PDE into a system of
ordinary differential equations (ODEs) that can be solved independently, then combines the
solutions into an infinite series.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">For a PDE with two independent variables ($x$ and $t$), the **product ansatz** assumes:

$$u(x,t) = X(x) \cdot T(t)$$

where $X(x)$ depends **only** on spatial variable $x$ and $T(t)$ depends **only** on
temporal variable $t$.
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Heat Equation with Dirichlet BCs</h3>
        <p>Solve 1D heat equation with fixed boundary conditions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);
let alpha = symbol!(alpha);

let equation = expr!(u);
let pde = Pde::new(equation, u, vec![x.clone(), t.clone()]);

// Boundary conditions: u(0,t) = 0, u(π,t) = 0
let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
let bc_right = BoundaryCondition::dirichlet_at(x.clone(), expr!(pi), expr!(0));
let bcs = vec![bc_left, bc_right];

// Initial condition: u(x,0) = sin(x)
let ic = InitialCondition::value(expr!(sin(x)));
let ics = vec![ic];

let solution = separate_variables(&pde, &bcs, &ics)?;
// Result: eigenvalues [1, 4, 9, 16, ...], eigenfunctions [sin(x), sin(2x), ...]
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr
from mathhook.pde import Pde, BoundaryCondition, InitialCondition, separate_variables

u = symbol('u')
x = symbol('x')
t = symbol('t')

pde = Pde(u, u, [x, t])

# Boundary conditions
bc_left = BoundaryCondition.dirichlet_at(x, expr('0'), expr('0'))
bc_right = BoundaryCondition.dirichlet_at(x, expr('pi'), expr('0'))
bcs = [bc_left, bc_right]

# Initial condition
ic = InitialCondition.value(expr('sin(x)'))
ics = [ic]

solution = separate_variables(pde, bcs, ics)
# Result: eigenvalues [1, 4, 9, 16, ...], eigenfunctions [sin(x), sin(2x), ...]
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');
const { Pde, BoundaryCondition, InitialCondition, separateVariables } = require('mathhook/pde');

const u = symbol('u');
const x = symbol('x');
const t = symbol('t');

const pde = new Pde(u, u, [x, t]);

// Boundary conditions
const bcLeft = BoundaryCondition.dirichletAt(x, expr('0'), expr('0'));
const bcRight = BoundaryCondition.dirichletAt(x, expr('pi'), expr('0'));
const bcs = [bcLeft, bcRight];

// Initial condition
const ic = InitialCondition.value(expr('sin(x)'));
const ics = [ic];

const solution = separateVariables(pde, bcs, ics);
// Result: eigenvalues [1, 4, 9, 16, ...], eigenfunctions [sin(x), sin(2x), ...]
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Wave Equation</h3>
        <p>Solve 1D wave equation with Dirichlet boundary conditions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);
let L = symbol!(L);

let pde = Pde::new(expr!(u), u, vec![x.clone(), t.clone()]);

let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
let bc_right = BoundaryCondition::dirichlet_at(x.clone(), expr!(L), expr!(0));
let bcs = vec![bc_left, bc_right];

// Initial displacement and velocity
let ic_displacement = InitialCondition::value(expr!(sin(pi * x / L)));
let ic_velocity = InitialCondition::derivative(expr!(0));
let ics = vec![ic_displacement, ic_velocity];

let solution = separate_variables(&pde, &bcs, &ics)?;
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr
from mathhook.pde import Pde, BoundaryCondition, InitialCondition, separate_variables

u = symbol('u')
x = symbol('x')
t = symbol('t')
L = symbol('L')

pde = Pde(u, u, [x, t])

bc_left = BoundaryCondition.dirichlet_at(x, expr('0'), expr('0'))
bc_right = BoundaryCondition.dirichlet_at(x, L, expr('0'))
bcs = [bc_left, bc_right]

ic_displacement = InitialCondition.value(expr('sin(pi*x/L)'))
ic_velocity = InitialCondition.derivative(expr('0'))
ics = [ic_displacement, ic_velocity]

solution = separate_variables(pde, bcs, ics)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');
const { Pde, BoundaryCondition, InitialCondition, separateVariables } = require('mathhook/pde');

const u = symbol('u');
const x = symbol('x');
const t = symbol('t');
const L = symbol('L');

const pde = new Pde(u, u, [x, t]);

const bcLeft = BoundaryCondition.dirichletAt(x, expr('0'), expr('0'));
const bcRight = BoundaryCondition.dirichletAt(x, L, expr('0'));
const bcs = [bcLeft, bcRight];

const icDisplacement = InitialCondition.value(expr('sin(pi*x/L)'));
const icVelocity = InitialCondition.derivative(expr('0'));
const ics = [icDisplacement, icVelocity];

const solution = separateVariables(pde, bcs, ics);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Laplace Equation on Rectangle</h3>
        <p>Solve Laplace's equation on rectangular domain</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

let u = symbol!(u);
let x = symbol!(x);
let y = symbol!(y);
let a = symbol!(a);

let pde = Pde::new(expr!(u), u, vec![x.clone(), y.clone()]);

let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
let bc_right = BoundaryCondition::dirichlet_at(x.clone(), expr!(a), expr!(0));
let bcs = vec![bc_left, bc_right];

let ics = vec![];  // Laplace is elliptic, not time-dependent

let solution = separate_variables(&pde, &bcs, &ics)?;
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr
from mathhook.pde import Pde, BoundaryCondition, separate_variables

u = symbol('u')
x = symbol('x')
y = symbol('y')
a = symbol('a')

pde = Pde(u, u, [x, y])

bc_left = BoundaryCondition.dirichlet_at(x, expr('0'), expr('0'))
bc_right = BoundaryCondition.dirichlet_at(x, a, expr('0'))
bcs = [bc_left, bc_right]

ics = []  # Laplace is elliptic

solution = separate_variables(pde, bcs, ics)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr } = require('mathhook');
const { Pde, BoundaryCondition, separateVariables } = require('mathhook/pde');

const u = symbol('u');
const x = symbol('x');
const y = symbol('y');
const a = symbol('a');

const pde = new Pde(u, u, [x, y]);

const bcLeft = BoundaryCondition.dirichletAt(x, expr('0'), expr('0'));
const bcRight = BoundaryCondition.dirichletAt(x, a, expr('0'));
const bcs = [bcLeft, bcRight];

const ics = [];  // Laplace is elliptic

const solution = separateVariables(pde, bcs, ics);
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
const metaDescription = "Separation of variables is the fundamental technique for solving linear partial differential
equations (PDEs) with boundary conditions. This method transforms a PDE into a system of
ordinary differential equations (ODEs) that can be solved independently, then combines the
solutions into an infinite series.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Separation of Variables for PDEs',
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
