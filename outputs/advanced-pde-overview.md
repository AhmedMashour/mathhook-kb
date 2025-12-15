---









---

# Partial Differential Equations (PDEs)

> **Topic**: `advanced.pde.overview`

Comprehensive overview of partial differential equations in MathHook CAS.
Covers mathematical foundations, classification, solution methods, and current capabilities.



## Mathematical Definition

$$A second-order linear PDE in two independent variables has the general form:

$$A \frac{\partial^2 u}{\partial x^2} + B \frac{\partial^2 u}{\partial x \partial y} + C \frac{\partial^2 u}{\partial y^2} + D \frac{\partial u}{\partial x} + E \frac{\partial u}{\partial y} + Fu = G$$

where:
- $u(x,y)$ is the unknown function
- $A, B, C, D, E, F, G$ are coefficients (may depend on $x$, $y$, or $u$)
- $x, y$ are independent variables (typically spatial coordinates or time)
$$



# Partial Differential Equations (PDEs)

## What Are PDEs?

Partial Differential Equations (PDEs) describe relationships involving functions of multiple variables and their partial derivatives. Unlike Ordinary Differential Equations (ODEs) which involve functions of a single variable, PDEs govern phenomena that vary in space **and** time.

### Why PDEs Matter

PDEs are the mathematical language of:
1. **Physics**: Heat conduction, wave propagation, electromagnetic fields, quantum mechanics
2. **Engineering**: Structural analysis, fluid dynamics, signal processing, control systems
3. **Finance**: Option pricing (Black-Scholes), risk modeling
4. **Biology**: Population dynamics, pattern formation, reaction-diffusion systems
5. **Computer Graphics**: Image processing, surface modeling, fluid simulation

## MathHook PDE Module Capabilities

### What MathHook Provides (Version 7.5/10)

✅ **Core Functionality**:
- PDE classification via discriminant ($B^2 - 4AC$)
- Heat equation solver (1D, Dirichlet boundary conditions)
- Wave equation solver (1D, Dirichlet boundary conditions)
- Laplace equation solver (2D rectangular domains, Dirichlet boundary conditions)
- Eigenvalue computation for standard boundary conditions
- Registry-based solver dispatch (O(1) lookup)
- Symbolic solution representation

✅ **Mathematical Correctness**:
- Verified against SymPy reference implementation
- Correct eigenvalue formulas
- Proper separation of variables structure
- Accurate boundary condition handling

### Current Limitations (Honestly Documented)

⚠️ **Symbolic Fourier Coefficients**:
- Solutions contain symbolic coefficients ($A_1, A_2, A_3, \ldots$)
- Numerical evaluation requires symbolic integration (Phase 2)
- Example: Heat equation returns $u(x,t) = \sum A_n \sin(\lambda_n x) e^{-\lambda_n \alpha t}$ with $A_n$ symbolic

⚠️ **Limited Boundary Conditions**:
- Only Dirichlet (fixed value) boundary conditions fully supported
- Neumann (derivative) and Robin (mixed) BCs planned for Phase 2

⚠️ **Standard Equations Only**:
- Supports heat, wave, and Laplace equations
- General nonlinear PDEs not yet supported

## Solution Methodology: Separation of Variables

All MathHook PDE solvers use the **separation of variables** technique.












## Examples


### Registry-Based Solver Dispatch

Automatic PDE classification and solver selection using O(1) registry lookup

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

// Create registry (auto-registers all solvers)
let registry = PDESolverRegistry::new();

// Define PDE
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);
let equation = expr!(add: x, t);  // Heat equation pattern
let pde = Pde::new(equation, u, vec![x, t]);

// Automatic classification and solving
let solution = registry.solve(&pde)?;

println!("Solution: {}", solution.solution);
println!("Eigenvalues: {:?}", solution.get_eigenvalues());

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr, Pde, PDESolverRegistry

# Create registry
registry = PDESolverRegistry()

# Define PDE
u = symbol('u')
x = symbol('x')
t = symbol('t')
equation = expr(x + t)  # Heat equation pattern
pde = Pde(equation, u, [x, t])

# Automatic solving
solution = registry.solve(pde)

print(f"Solution: {solution.solution}")
print(f"Eigenvalues: {solution.get_eigenvalues()}")

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr, Pde, PDESolverRegistry } = require('mathhook');

// Create registry
const registry = new PDESolverRegistry();

// Define PDE
const u = symbol('u');
const x = symbol('x');
const t = symbol('t');
const equation = expr(x + t);  // Heat equation pattern
const pde = new Pde(equation, u, [x, t]);

// Automatic solving
const solution = registry.solve(pde);

console.log(`Solution: ${solution.solution}`);
console.log(`Eigenvalues: ${solution.getEigenvalues()}`);

```
</details>






## Performance

**Time Complexity**: O(1) solver lookup, O(n) eigenvalue computation


## API Reference

- **Rust**: `mathhook_core::pde`
- **Python**: `mathhook.pde`
- **JavaScript**: `mathhook.pde`


## See Also


- [advanced.pde.classification](../advanced/pde/classification.md)

- [advanced.pde.heat_equation](../advanced/pde/heat_equation.md)

- [advanced.pde.wave_equation](../advanced/pde/wave_equation.md)

- [advanced.pde.laplace_equation](../advanced/pde/laplace_equation.md)

- [advanced.pde.fourier_coefficients](../advanced/pde/fourier_coefficients.md)

- [advanced.pde.boundary_conditions](../advanced/pde/boundary_conditions.md)


