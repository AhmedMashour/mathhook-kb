<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Complete PDE Examples</h1>
      <p class="description">Three complete, real-world examples demonstrating MathHook's PDE solving capabilities across heat, wave, and Laplace equations.
Each example includes full problem setup, mathematical formulation, MathHook implementation, and physical interpretation.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Example 1: Heat Diffusion**
$$\frac{\partial u}{\partial t} = \alpha \frac{\partial^2 u}{\partial x^2}$$

**Example 2: Wave Propagation**
$$\frac{\partial^2 u}{\partial t^2} = c^2 \frac{\partial^2 u}{\partial x^2}$$

**Example 3: Electrostatic Potential**
$$\frac{\partial^2 u}{\partial x^2} + \frac{\partial^2 u}{\partial y^2} = 0$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Heat Diffusion in Steel Rod - Complete Implementation</h3>
        <p>1-meter steel rod cooling from 100°C with ice water at ends. Full implementation with error handling.</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

fn solve_cooling_rod() -> Result<(), Box<dyn std::error::Error>> {
    // Define Variables
    let u = symbol!(u);
    let x = symbol!(x);
    let t = symbol!(t);

    // Create PDE
    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![x.clone(), t.clone()]);

    // Material Properties
    let alpha = expr!(0.000013);  // Steel thermal diffusivity

    // Boundary Conditions
    let bc_left = BoundaryCondition::dirichlet(
        expr!(0),
        BoundaryLocation::Simple {
            variable: x.clone(),
            value: expr!(0),
        },
    );
    let bc_right = BoundaryCondition::dirichlet(
        expr!(0),
        BoundaryLocation::Simple {
            variable: x,
            value: expr!(1),
        },
    );

    // Initial Condition
    let ic = InitialCondition::value(expr!(100));

    // Solve
    let solver = HeatEquationSolver::new();
    let result = solver.solve_heat_equation_1d(
        &pde,
        &alpha,
        &[bc_left, bc_right],
        &ic,
    )?;

    // Examine Solution
    println!("Heat Equation Solution for Cooling Steel Rod");
    println!("Solution structure: {}", result.solution);
    println!("Eigenvalues: {:?}", result.eigenvalues.iter().take(5).collect::<Vec<_>>());
    println!("Fourier coefficients: {:?}", result.coefficients.iter().take(5).collect::<Vec<_>>());

    Ok(())
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, Pde, BoundaryCondition, BoundaryLocation, InitialCondition, HeatEquationSolver

def solve_cooling_rod():
    # Define Variables
    u = symbol('u')
    x = symbol('x')
    t = symbol('t')

    # Create PDE
    equation = expr(u)
    pde = Pde(equation, u, [x, t])

    # Material Properties
    alpha = expr(0.000013)

    # Boundary Conditions
    bc_left = BoundaryCondition.dirichlet(
        expr(0),
        BoundaryLocation.simple(variable=x, value=expr(0))
    )
    bc_right = BoundaryCondition.dirichlet(
        expr(0),
        BoundaryLocation.simple(variable=x, value=expr(1))
    )

    # Initial Condition
    ic = InitialCondition.value(expr(100))

    # Solve
    solver = HeatEquationSolver()
    result = solver.solve_heat_equation_1d(pde, alpha, [bc_left, bc_right], ic)

    print(f"Solution: {result.solution}")
    print(f"Eigenvalues: {result.eigenvalues[:5]}")
    print(f"Coefficients: {result.coefficients[:5]}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, Pde, BoundaryCondition, BoundaryLocation, InitialCondition, HeatEquationSolver } = require('mathhook');

function solveCoolingRod() {
    // Define Variables
    const u = symbol('u');
    const x = symbol('x');
    const t = symbol('t');

    // Create PDE
    const equation = expr(u);
    const pde = new Pde(equation, u, [x, t]);

    // Material Properties
    const alpha = expr(0.000013);

    // Boundary Conditions
    const bcLeft = BoundaryCondition.dirichlet(
        expr(0),
        BoundaryLocation.simple({ variable: x, value: expr(0) })
    );
    const bcRight = BoundaryCondition.dirichlet(
        expr(0),
        BoundaryLocation.simple({ variable: x, value: expr(1) })
    );

    // Initial Condition
    const ic = InitialCondition.value(expr(100));

    // Solve
    const solver = new HeatEquationSolver();
    const result = solver.solveHeatEquation1d(pde, alpha, [bcLeft, bcRight], ic);

    console.log(`Solution: ${result.solution}`);
    console.log(`Eigenvalues: ${result.eigenvalues.slice(0, 5)}`);
    console.log(`Coefficients: ${result.coefficients.slice(0, 5)}`);
}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Vibrating Guitar String - Musical Analysis</h3>
        <p>E4 guitar string with musical frequency analysis and standing wave nodes.</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

fn solve_vibrating_string() -> Result<(), Box<dyn std::error::Error>> {
    let u = symbol!(u);
    let x = symbol!(x);
    let t = symbol!(t);

    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![x.clone(), t.clone()]);

    let c = expr!(442);  // Wave speed

    let bc1 = BoundaryCondition::dirichlet(
        expr!(0),
        BoundaryLocation::Simple { variable: x.clone(), value: expr!(0) },
    );
    let bc2 = BoundaryCondition::dirichlet(
        expr!(0),
        BoundaryLocation::Simple { variable: x, value: expr!(0.65) },
    );

    let ic_position = InitialCondition::value(expr!(0.005));
    let ic_velocity = InitialCondition::derivative(expr!(0));

    let solver = WaveEquationSolver::new();
    let result = solver.solve_wave_equation_1d(
        &pde, &c, &[bc1, bc2], &ic_position, &ic_velocity
    )?;

    println!("Wave Equation Solution for Vibrating Guitar String");
    println!("Solution: {}", result.solution);

    // Compute musical frequencies
    let L = 0.65;
    let c_val = 442.0;
    for n in 1..=5 {
        let f_n = (n as f64) * c_val / (2.0 * L);
        println!("f_{} = {:.2} Hz (mode {})", n, f_n, n);
    }

    Ok(())
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, Pde, BoundaryCondition, BoundaryLocation, InitialCondition, WaveEquationSolver

def solve_vibrating_string():
    u = symbol('u')
    x = symbol('x')
    t = symbol('t')

    equation = expr(u)
    pde = Pde(equation, u, [x, t])

    c = expr(442)

    bc1 = BoundaryCondition.dirichlet(expr(0), BoundaryLocation.simple(variable=x, value=expr(0)))
    bc2 = BoundaryCondition.dirichlet(expr(0), BoundaryLocation.simple(variable=x, value=expr(0.65)))

    ic_position = InitialCondition.value(expr(0.005))
    ic_velocity = InitialCondition.derivative(expr(0))

    solver = WaveEquationSolver()
    result = solver.solve_wave_equation_1d(pde, c, [bc1, bc2], ic_position, ic_velocity)

    print(f"Solution: {result.solution}")

    # Musical frequencies
    L = 0.65
    c_val = 442.0
    for n in range(1, 6):
        f_n = n * c_val / (2.0 * L)
        print(f"f_{n} = {f_n:.2f} Hz (mode {n})")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, Pde, BoundaryCondition, BoundaryLocation, InitialCondition, WaveEquationSolver } = require('mathhook');

function solveVibratingString() {
    const u = symbol('u');
    const x = symbol('x');
    const t = symbol('t');

    const equation = expr(u);
    const pde = new Pde(equation, u, [x, t]);

    const c = expr(442);

    const bc1 = BoundaryCondition.dirichlet(expr(0), BoundaryLocation.simple({ variable: x, value: expr(0) }));
    const bc2 = BoundaryCondition.dirichlet(expr(0), BoundaryLocation.simple({ variable: x, value: expr(0.65) }));

    const icPosition = InitialCondition.value(expr(0.005));
    const icVelocity = InitialCondition.derivative(expr(0));

    const solver = new WaveEquationSolver();
    const result = solver.solveWaveEquation1d(pde, c, [bc1, bc2], icPosition, icVelocity);

    console.log(`Solution: ${result.solution}`);

    // Musical frequencies
    const L = 0.65;
    const cVal = 442.0;
    for (let n = 1; n <= 5; n++) {
        const fn = n * cVal / (2.0 * L);
        console.log(`f_${n} = ${fn.toFixed(2)} Hz (mode ${n})`);
    }
}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Electrostatic Potential in Rectangular Plate</h3>
        <p>10cm × 5cm plate with grounded sides and fixed potential top edge.</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

fn solve_electrostatic_potential() -> Result<(), Box<dyn std::error::Error>> {
    let u = symbol!(u);
    let x = symbol!(x);
    let y = symbol!(y);

    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![x.clone(), y.clone()]);

    let bc_left = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple { variable: x.clone(), value: expr!(0) });
    let bc_right = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple { variable: x.clone(), value: expr!(0.1) });
    let bc_bottom = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple { variable: y.clone(), value: expr!(0) });
    let bc_top = BoundaryCondition::dirichlet(expr!(100), BoundaryLocation::Simple { variable: y, value: expr!(0.05) });

    let solver = LaplaceEquationSolver::new();
    let result = solver.solve_laplace_equation_2d(&pde, &[bc_left, bc_right, bc_bottom, bc_top])?;

    println!("Laplace Equation Solution for Electrostatic Potential");
    println!("Solution: {}", result.solution);
    println!("X-eigenvalues: {:?}", result.x_eigenvalues.iter().take(5).collect::<Vec<_>>());

    Ok(())
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, Pde, BoundaryCondition, BoundaryLocation, LaplaceEquationSolver

def solve_electrostatic_potential():
    u = symbol('u')
    x = symbol('x')
    y = symbol('y')

    equation = expr(u)
    pde = Pde(equation, u, [x, y])

    bc_left = BoundaryCondition.dirichlet(expr(0), BoundaryLocation.simple(variable=x, value=expr(0)))
    bc_right = BoundaryCondition.dirichlet(expr(0), BoundaryLocation.simple(variable=x, value=expr(0.1)))
    bc_bottom = BoundaryCondition.dirichlet(expr(0), BoundaryLocation.simple(variable=y, value=expr(0)))
    bc_top = BoundaryCondition.dirichlet(expr(100), BoundaryLocation.simple(variable=y, value=expr(0.05)))

    solver = LaplaceEquationSolver()
    result = solver.solve_laplace_equation_2d(pde, [bc_left, bc_right, bc_bottom, bc_top])

    print(f"Solution: {result.solution}")
    print(f"X-eigenvalues: {result.x_eigenvalues[:5]}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, Pde, BoundaryCondition, BoundaryLocation, LaplaceEquationSolver } = require('mathhook');

function solveElectrostaticPotential() {
    const u = symbol('u');
    const x = symbol('x');
    const y = symbol('y');

    const equation = expr(u);
    const pde = new Pde(equation, u, [x, y]);

    const bcLeft = BoundaryCondition.dirichlet(expr(0), BoundaryLocation.simple({ variable: x, value: expr(0) }));
    const bcRight = BoundaryCondition.dirichlet(expr(0), BoundaryLocation.simple({ variable: x, value: expr(0.1) }));
    const bcBottom = BoundaryCondition.dirichlet(expr(0), BoundaryLocation.simple({ variable: y, value: expr(0) }));
    const bcTop = BoundaryCondition.dirichlet(expr(100), BoundaryLocation.simple({ variable: y, value: expr(0.05) }));

    const solver = new LaplaceEquationSolver();
    const result = solver.solveLaplaceEquation2d(pde, [bcLeft, bcRight, bcBottom, bcTop]);

    console.log(`Solution: ${result.solution}`);
    console.log(`X-eigenvalues: ${result.xEigenvalues.slice(0, 5)}`);
}
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
const metaDescription = "Three complete, real-world examples demonstrating MathHook's PDE solving capabilities across heat, wave, and Laplace equations.
Each example includes full problem setup, mathematical formulation, MathHook implementation, and physical interpretation.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Complete PDE Examples',
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
