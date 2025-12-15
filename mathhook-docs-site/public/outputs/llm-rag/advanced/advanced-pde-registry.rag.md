# PDE Solver Registry System

MathHook uses a registry-based dispatch system for PDE solvers, eliminating hardcoded match
statements and enabling O(1) solver lookup. This architecture is inspired by the ODE module
registry and provides extensible, testable, and efficient solver selection.


---
chunk_id: advanced_pde_registry::0
topic: advanced.pde.registry
title: Default Registry Usage
priority: medium
keywords:
  - registry
  - default registry usage
languages: [rust, python, javascript]
chunk: 1/3
---

## Default Registry Usage

Create and use default registry with standard solvers

### Rust

```rust
let registry = PDESolverRegistry::new();

// Automatically classify and solve
let solution = registry.solve(&pde)?;

```

### Python

```python
registry = PDESolverRegistry()

# Automatically classify and solve
solution = registry.solve(pde)

```

### JavaScript

```javascript
const registry = new PDESolverRegistry();

// Automatically classify and solve
const solution = registry.solve(pde);

```



---
chunk_id: advanced_pde_registry::1
topic: advanced.pde.registry
title: Custom Poisson Solver
priority: medium
keywords:
  - registry
  - custom poisson solver
languages: [rust, python, javascript]
chunk: 2/3
---

## Custom Poisson Solver

Register a custom solver for Poisson equation (non-homogeneous Laplace)

### Rust

```rust
struct PoissonEquationSolver {
    max_terms: usize,
}

impl PDESolver for PoissonEquationSolver {
    fn solve(&self, pde: &Pde) -> PDEResult {
        // Poisson solver logic here
        Ok(PDESolution::laplace(solution, eigenvalues, coefficients))
    }

    fn can_solve(&self, pde_type: PdeType) -> bool {
        matches!(pde_type, PdeType::Elliptic)
    }

    fn priority(&self) -> u8 {
        90  // Lower than Laplace solver (100)
    }

    fn name(&self) -> &'static str {
        "Poisson Equation Solver"
    }

    fn description(&self) -> &'static str {
        "Solves Poisson equation ∇²u = f with non-zero source term"
    }
}

// Register custom solver
let mut registry = PDESolverRegistry::new();
registry.register(
    PdeType::Elliptic,
    Arc::new(PoissonEquationSolver { max_terms: 10 }),
);

```

### Python

```python
class PoissonEquationSolver:
    def __init__(self, max_terms=10):
        self.max_terms = max_terms

    def solve(self, pde):
        # Poisson solver logic here
        return PDESolution.laplace(solution, eigenvalues, coefficients)

    def can_solve(self, pde_type):
        return pde_type == PdeType.Elliptic

    def priority(self):
        return 90

    def name(self):
        return "Poisson Equation Solver"

    def description(self):
        return "Solves Poisson equation ∇²u = f with non-zero source term"

# Register custom solver
registry = PDESolverRegistry()
registry.register(PdeType.Elliptic, PoissonEquationSolver(max_terms=10))

```

### JavaScript

```javascript
class PoissonEquationSolver {
    constructor(maxTerms = 10) {
        this.maxTerms = maxTerms;
    }

    solve(pde) {
        // Poisson solver logic here
        return PDESolution.laplace(solution, eigenvalues, coefficients);
    }

    canSolve(pdeType) {
        return pdeType === PdeType.Elliptic;
    }

    priority() {
        return 90;
    }

    name() {
        return "Poisson Equation Solver";
    }

    description() {
        return "Solves Poisson equation ∇²u = f with non-zero source term";
    }
}

// Register custom solver
const registry = new PDESolverRegistry();
registry.register(PdeType.Elliptic, new PoissonEquationSolver(10));

```



---
chunk_id: advanced_pde_registry::2
topic: advanced.pde.registry
title: Solver Discovery
priority: medium
keywords:
  - registry
  - solver discovery
languages: [rust, python, javascript]
chunk: 3/3
---

## Solver Discovery

List available solvers and inspect registry contents

### Rust

```rust
let registry = PDESolverRegistry::new();

println!("Registered PDE types: {:?}", registry.registered_types());
// [Parabolic, Hyperbolic, Elliptic]

println!("Total solvers: {}", registry.solver_count());
// 3

// Get solver for specific type
if let Some(solver) = registry.get_solver(&PdeType::Parabolic) {
    println!("Heat solver: {}", solver.name());
    println!("Description: {}", solver.description());
}

```

### Python

```python
registry = PDESolverRegistry()

print(f"Registered PDE types: {registry.registered_types()}")
# [Parabolic, Hyperbolic, Elliptic]

print(f"Total solvers: {registry.solver_count()}")
# 3

# Get solver for specific type
solver = registry.get_solver(PdeType.Parabolic)
if solver:
    print(f"Heat solver: {solver.name()}")
    print(f"Description: {solver.description()}")

```

### JavaScript

```javascript
const registry = new PDESolverRegistry();

console.log(`Registered PDE types: ${registry.registeredTypes()}`);
// [Parabolic, Hyperbolic, Elliptic]

console.log(`Total solvers: ${registry.solverCount()}`);
// 3

// Get solver for specific type
const solver = registry.getSolver(PdeType.Parabolic);
if (solver) {
    console.log(`Heat solver: ${solver.name()}`);
    console.log(`Description: ${solver.description()}`);
}

```



