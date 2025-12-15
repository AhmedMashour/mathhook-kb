# Separation of Variables

Separation of Variables is the fundamental technique for solving linear second-order partial differential equations (PDEs) with boundary conditions.
It transforms the PDE into multiple ordinary differential equations (ODEs) that can be solved independently.


---
chunk_id: advanced_pde_separation_of_variables::0
topic: advanced.pde.separation_of_variables
title: Heat Equation with Separation of Variables
priority: medium
keywords:
  - separation_of_variables
  - heat equation with separation of variables
languages: [rust, python, javascript]
chunk: 1/1
---

## Heat Equation with Separation of Variables

Demonstrates separation of variables for 1D heat diffusion in a rod.

### Rust

```rust
use mathhook::prelude::*;

let registry = PDESolverRegistry::new();
let solution = registry.solve(&pde).unwrap();
// Automatically uses separation of variables if applicable

// The solver internally:
// 1. Assumes u(x,t) = X(x)*T(t)
// 2. Separates into spatial and temporal ODEs
// 3. Applies BCs to find eigenvalues
// 4. Constructs superposition solution

```

### Python

```python
from mathhook import PDESolverRegistry

registry = PDESolverRegistry()
solution = registry.solve(pde)
# Automatically uses separation of variables if applicable

```

### JavaScript

```javascript
const { PDESolverRegistry } = require('mathhook');

const registry = new PDESolverRegistry();
const solution = registry.solve(pde);
// Automatically uses separation of variables if applicable

```



