<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Heat Equation Solver</h1>
      <p class="description">The heat equation (also called diffusion equation) governs how temperature distributes through materials over time.
Solves parabolic PDEs with boundary and initial conditions using separation of variables and Fourier series.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">$$\frac{\partial u}{\partial t} = \alpha \nabla^2 u$$

where:
- $u(x,t)$ is temperature at position $x$ and time $t$
- $\alpha$ is thermal diffusivity (m²/s): $\alpha = \frac{k}{\rho c_p}$
- $\nabla^2 u = \frac{\partial^2 u}{\partial x^2}$ (1D) or $\frac{\partial^2 u}{\partial x^2} + \frac{\partial^2 u}{\partial y^2}$ (2D)

**Fourier's Law of Heat Conduction**: $\mathbf{q} = -k \nabla u$

**Conservation of Energy**: $\rho c_p \frac{\partial u}{\partial t} = \nabla \cdot (k \nabla u)$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Cooling Steel Rod</h3>
        <p>A 1-meter steel rod initially at 100°C with both ends plunged into ice water (0°C). Demonstrates heat diffusion with Dirichlet boundary conditions.</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Define variables
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

// PDE: u_t = α u_xx
let equation = expr!(u);  // Placeholder (solver knows structure)
let pde = Pde::new(equation, u, vec![x.clone(), t.clone()]);

// Thermal diffusivity for steel
let alpha = expr!(0.000013);  // 1.3 × 10^-5 m²/s

// Boundary conditions: u(0,t) = 0, u(1,t) = 0
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
        value: expr!(1),  // L = 1 meter
    },
);

// Initial condition: u(x,0) = 100°C
let ic = InitialCondition::value(expr!(100));

// Solve
let solver = HeatEquationSolver::new();
let result = solver.solve_heat_equation_1d(&pde, &alpha, &[bc1, bc2], &ic)?;

// What you get:
println!("Solution structure: {}", result.solution);
// u(x,t) = A_1*sin(π*x)*exp(-π²*α*t) + A_2*sin(2π*x)*exp(-4π²*α*t) + ...

println!("Eigenvalues: {:?}", result.eigenvalues);
// [π², 4π², 9π², ...] = [(nπ/L)² for n=1,2,3,...]

println!("Coefficients (symbolic): {:?}", result.coefficients);
// [A_1, A_2, A_3, ...] - SYMBOLIC, not numerical values
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, Pde, BoundaryCondition, BoundaryLocation, InitialCondition, HeatEquationSolver

# Define variables
u = symbol('u')
x = symbol('x')
t = symbol('t')

# PDE: u_t = α u_xx
equation = expr(u)
pde = Pde(equation, u, [x, t])

# Thermal diffusivity for steel
alpha = expr(0.000013)

# Boundary conditions: u(0,t) = 0, u(1,t) = 0
bc1 = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple(variable=x, value=expr(0))
)
bc2 = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple(variable=x, value=expr(1))
)

# Initial condition: u(x,0) = 100°C
ic = InitialCondition.value(expr(100))

# Solve
solver = HeatEquationSolver()
result = solver.solve_heat_equation_1d(pde, alpha, [bc1, bc2], ic)

print(f"Solution: {result.solution}")
print(f"Eigenvalues: {result.eigenvalues}")
print(f"Coefficients: {result.coefficients}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, Pde, BoundaryCondition, BoundaryLocation, InitialCondition, HeatEquationSolver } = require('mathhook');

// Define variables
const u = symbol('u');
const x = symbol('x');
const t = symbol('t');

// PDE: u_t = α u_xx
const equation = expr(u);
const pde = new Pde(equation, u, [x, t]);

// Thermal diffusivity for steel
const alpha = expr(0.000013);

// Boundary conditions: u(0,t) = 0, u(1,t) = 0
const bc1 = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple({ variable: x, value: expr(0) })
);
const bc2 = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple({ variable: x, value: expr(1) })
);

// Initial condition: u(x,0) = 100°C
const ic = InitialCondition.value(expr(100));

// Solve
const solver = new HeatEquationSolver();
const result = solver.solveHeatEquation1d(pde, alpha, [bc1, bc2], ic);

console.log(`Solution: ${result.solution}`);
console.log(`Eigenvalues: ${result.eigenvalues}`);
console.log(`Coefficients: ${result.coefficients}`);
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
const metaDescription = "The heat equation (also called diffusion equation) governs how temperature distributes through materials over time.
Solves parabolic PDEs with boundary and initial conditions using separation of variables and Fourier series.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Heat Equation Solver',
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
