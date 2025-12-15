<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Wave Equation Solver</h1>
      <p class="description">The wave equation governs oscillatory phenomena and wave propagation in physical systems.
Solves hyperbolic PDEs with boundary conditions and two initial conditions (position and velocity).
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">$$\frac{\partial^2 u}{\partial t^2} = c^2 \nabla^2 u$$

where:
- $u(x,t)$ is displacement at position $x$ and time $t$
- $c$ is wave speed (m/s)
- $\nabla^2 u = \frac{\partial^2 u}{\partial x^2}$ (1D)

**Newton's Second Law** for string element:
$$\rho \frac{\partial^2 u}{\partial t^2} = T \frac{\partial^2 u}{\partial x^2}$$

**Wave speed**: $c = \sqrt{T/\rho}$ where $\rho$ = linear mass density, $T$ = tension
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Vibrating Guitar String</h3>
        <p>A 0.65m guitar string plucked at center with 5mm displacement, demonstrating wave propagation and standing waves.</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Variables
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

// PDE
let equation = expr!(u);
let pde = Pde::new(equation, u, vec![x.clone(), t.clone()]);

// Wave speed
let c = expr!(442);  // m/s for steel E string

// Boundary conditions
let bc1 = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple {
        variable: x.clone(),
        value: expr!(0),
    },
);
let bc2 = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple {
        variable: x,
        value: expr!(0.65),  // L = 0.65 m
    },
);

// Initial conditions
let ic_position = InitialCondition::value(
    // Triangular function (symbolic - not yet computable)
    expr!(0.005)  // Placeholder for triangular shape
);
let ic_velocity = InitialCondition::derivative(expr!(0));  // Released from rest

// Solve
let solver = WaveEquationSolver::new();
let result = solver.solve_wave_equation_1d(
    &pde,
    &c,
    &[bc1, bc2],
    &ic_position,
    &ic_velocity
)?;

// What you get:
println!("Solution: {}", result.solution);
// u(x,t) = [A_1*cos(ω₁*t) + B_1*sin(ω₁*t)]*sin(π*x/L) + ...

println!("Eigenvalues: {:?}", result.eigenvalues);
// [λ₁, λ₂, λ₃, ...] where λₙ = (nπ/L)²

println!("Position coefficients (A_n): {:?}", result.position_coefficients);
println!("Velocity coefficients (B_n): {:?}", result.velocity_coefficients);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, Pde, BoundaryCondition, BoundaryLocation, InitialCondition, WaveEquationSolver

# Variables
u = symbol('u')
x = symbol('x')
t = symbol('t')

# PDE
equation = expr(u)
pde = Pde(equation, u, [x, t])

# Wave speed
c = expr(442)

# Boundary conditions
bc1 = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple(variable=x, value=expr(0))
)
bc2 = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple(variable=x, value=expr(0.65))
)

# Initial conditions
ic_position = InitialCondition.value(expr(0.005))
ic_velocity = InitialCondition.derivative(expr(0))

# Solve
solver = WaveEquationSolver()
result = solver.solve_wave_equation_1d(pde, c, [bc1, bc2], ic_position, ic_velocity)

print(f"Solution: {result.solution}")
print(f"Eigenvalues: {result.eigenvalues}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, Pde, BoundaryCondition, BoundaryLocation, InitialCondition, WaveEquationSolver } = require('mathhook');

// Variables
const u = symbol('u');
const x = symbol('x');
const t = symbol('t');

// PDE
const equation = expr(u);
const pde = new Pde(equation, u, [x, t]);

// Wave speed
const c = expr(442);

// Boundary conditions
const bc1 = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple({ variable: x, value: expr(0) })
);
const bc2 = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple({ variable: x, value: expr(0.65) })
);

// Initial conditions
const icPosition = InitialCondition.value(expr(0.005));
const icVelocity = InitialCondition.derivative(expr(0));

// Solve
const solver = new WaveEquationSolver();
const result = solver.solveWaveEquation1d(pde, c, [bc1, bc2], icPosition, icVelocity);

console.log(`Solution: ${result.solution}`);
console.log(`Eigenvalues: ${result.eigenvalues}`);
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
const metaDescription = "The wave equation governs oscillatory phenomena and wave propagation in physical systems.
Solves hyperbolic PDEs with boundary conditions and two initial conditions (position and velocity).
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Wave Equation Solver',
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
