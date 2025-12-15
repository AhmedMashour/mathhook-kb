# Partial Differential Equations (PDEs)

Comprehensive overview of partial differential equations in MathHook CAS.
Covers mathematical foundations, classification, solution methods, and current capabilities.


---
chunk_id: advanced_pde_overview::0
topic: advanced.pde.overview
title: Registry-Based Solver Dispatch
priority: medium
keywords:
  - overview
  - registry-based solver dispatch
languages: [rust, python, javascript]
chunk: 1/1
---

## Registry-Based Solver Dispatch

Automatic PDE classification and solver selection using O(1) registry lookup

### Rust

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

### Python

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

### JavaScript

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



