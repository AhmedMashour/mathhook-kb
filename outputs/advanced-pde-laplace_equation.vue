<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Laplace Equation Solver</h1>
      <p class="description">Laplace's equation describes steady-state (equilibrium) distributions in physics.
Solves elliptic PDEs with boundary conditions for harmonic functions in 2D rectangular domains.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">$$\nabla^2 u = 0$$

In 2D:
$$\frac{\partial^2 u}{\partial x^2} + \frac{\partial^2 u}{\partial y^2} = 0$$

**Key property**: No time dependence → equilibrium state.

Physical applications:
- Electrostatics: $\nabla^2 \phi = 0$ (electric potential in charge-free regions)
- Steady-state heat: $\nabla^2 T = 0$ (temperature at equilibrium)
- Potential flow: $\nabla^2 \psi = 0$ (stream function)
- Gravity: $\nabla^2 U = 0$ (gravitational potential in vacuum)
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Electrostatic Potential in Rectangular Plate</h3>
        <p>10cm × 5cm conducting plate with bottom/sides grounded (0V) and top at 100V. Demonstrates equilibrium potential distribution.</p>
        
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

let equation = expr!(u);
let pde = Pde::new(equation, u, vec![x.clone(), y.clone()]);

// Boundary conditions
let bc_left = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: x.clone(), value: expr!(0) },
);
let bc_right = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: x.clone(), value: expr!(0.1) },  // a=10cm
);
let bc_bottom = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: y.clone(), value: expr!(0) },
);
let bc_top = BoundaryCondition::dirichlet(
    expr!(100),
    BoundaryLocation::Simple { variable: y, value: expr!(0.05) },  // b=5cm
);

// Solve
let solver = LaplaceEquationSolver::new();
let result = solver.solve_laplace_equation_2d(&pde, &[bc_left, bc_right, bc_bottom, bc_top])?;

// What you get:
println!("Solution: {}", result.solution);
// u(x,y) = C_1*sin(λ₁*x)*sinh(λ₁*y) + C_2*sin(λ₂*x)*sinh(λ₂*y) + ...

println!("X-eigenvalues: {:?}", result.x_eigenvalues);
// [π/a, 2π/a, 3π/a, ...] = [nπ/a for n=1,2,3,...]

println!("Coefficients: {:?}", result.coefficients);
// [C_1, C_2, C_3, ...] SYMBOLIC
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, Pde, BoundaryCondition, BoundaryLocation, LaplaceEquationSolver

u = symbol('u')
x = symbol('x')
y = symbol('y')

equation = expr(u)
pde = Pde(equation, u, [x, y])

# Boundary conditions
bc_left = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple(variable=x, value=expr(0))
)
bc_right = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple(variable=x, value=expr(0.1))
)
bc_bottom = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple(variable=y, value=expr(0))
)
bc_top = BoundaryCondition.dirichlet(
    expr(100),
    BoundaryLocation.simple(variable=y, value=expr(0.05))
)

# Solve
solver = LaplaceEquationSolver()
result = solver.solve_laplace_equation_2d(pde, [bc_left, bc_right, bc_bottom, bc_top])

print(f"Solution: {result.solution}")
print(f"X-eigenvalues: {result.x_eigenvalues}")
print(f"Coefficients: {result.coefficients}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, Pde, BoundaryCondition, BoundaryLocation, LaplaceEquationSolver } = require('mathhook');

const u = symbol('u');
const x = symbol('x');
const y = symbol('y');

const equation = expr(u);
const pde = new Pde(equation, u, [x, y]);

// Boundary conditions
const bcLeft = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple({ variable: x, value: expr(0) })
);
const bcRight = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple({ variable: x, value: expr(0.1) })
);
const bcBottom = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.simple({ variable: y, value: expr(0) })
);
const bcTop = BoundaryCondition.dirichlet(
    expr(100),
    BoundaryLocation.simple({ variable: y, value: expr(0.05) })
);

// Solve
const solver = new LaplaceEquationSolver();
const result = solver.solveLaplaceEquation2d(pde, [bcLeft, bcRight, bcBottom, bcTop]);

console.log(`Solution: ${result.solution}`);
console.log(`X-eigenvalues: ${result.xEigenvalues}`);
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
const metaDescription = "Laplace's equation describes steady-state (equilibrium) distributions in physics.
Solves elliptic PDEs with boundary conditions for harmonic functions in 2D rectangular domains.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Laplace Equation Solver',
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
