---









---

# PDE Solver Registry System

> **Topic**: `advanced.pde.registry`

MathHook uses a registry-based dispatch system for PDE solvers, eliminating hardcoded match
statements and enabling O(1) solver lookup. This architecture is inspired by the ODE module
registry and provides extensible, testable, and efficient solver selection.



## Mathematical Definition

$$Registry-based dispatch uses a HashMap for O(1) lookup by PDE type:

$$\text{Registry}: \text{PdeType} \rightarrow \text{Vec}\langle\text{Arc}\langle\text{dyn PDESolver}\rangle\rangle$$

Priority-based selection from multiple solvers per type ensures optimal solver choice.
$$



# PDE Solver Registry System

## Architecture: Registry Pattern

MathHook uses a **registry-based dispatch system** for PDE solvers, eliminating hardcoded `match` statements and enabling O(1) solver lookup.

**Design inspired by**: ODE module registry (scored 9/10 for quality)

## Registry Structure

```rust
pub struct PDESolverRegistry {
    /// Solvers organized by PDE type
    solvers: HashMap<PdeType, Vec<Arc<dyn PDESolver>>>,
    /// Priority order for trying solvers
    priority_order: Vec<PdeType>,
}
```

**Key features**:
- **O(1) lookup** by PDE type (HashMap)
- **Multiple solvers per type** (priority-sorted Vec)
- **Thread-safe** (Arc for shared solver instances)
- **Extensible** (register custom solvers)

## PDESolver Trait

All solvers implement this trait:

```rust
pub trait PDESolver: Send + Sync {
    /// Attempts to solve the given PDE
    fn solve(&self, pde: &Pde) -> PDEResult;

    /// Returns true if this solver can handle the given PDE type
    fn can_solve(&self, pde_type: PdeType) -> bool;

    /// Priority for this solver (higher = try first)
    fn priority(&self) -> u8;

    /// Solver name for diagnostics
    fn name(&self) -> &'static str;

    /// Solver description
    fn description(&self) -> &'static str;
}
```

**Why Send + Sync?** Registry is shared across threads (web servers, parallel computation).

## Default Solvers

Registry auto-registers standard solvers for three main PDE types:
- Heat Equation Solver (Parabolic, priority 100)
- Wave Equation Solver (Hyperbolic, priority 100)
- Laplace Equation Solver (Elliptic, priority 100)

## Solver Dispatch Workflow

### Automatic Classification + Solving

**Workflow**:
1. **Classify**: Compute discriminant, determine PDE type
2. **Lookup**: `HashMap::get(pde_type)` → O(1)
3. **Select**: First solver in priority-sorted Vec
4. **Solve**: Call `solver.solve(&pde)`
5. **Return**: `PDESolution` with metadata

### Try All Solvers (Fallback)

If classification uncertain, the registry can try all solvers in priority order until one succeeds.

## Adding Custom Solvers

Custom solvers can be registered with specified priority levels:
- **Override**: Higher priority than standard solver
- **Fallback**: Lower priority, try if standard fails
- **Specialized**: Same priority, but more specific `can_solve()` logic

## Error Handling

The registry provides comprehensive error types:
- `NoSolverAvailable`: No solver registered for PDE type
- `ClassificationFailed`: Cannot determine PDE type
- `SolutionFailed`: Solver encountered an error
- `InvalidBoundaryConditions`: Boundary conditions incompatible
- `InvalidInitialConditions`: Initial conditions incompatible
- `NotSeparable`: PDE not separable (for separation of variables)
- `InvalidForm`: PDE structure not recognized

## Performance Characteristics

### Lookup Complexity

- **Classification**: O(1) - pattern matching
- **Registry lookup**: O(1) - HashMap
- **Solver selection**: O(1) - first element in sorted Vec
- **Overall**: O(1) for standard PDEs

### Memory Overhead

- **Arc<dyn PDESolver>**: 16 bytes per solver (fat pointer)
- **HashMap**: ~24 bytes + entries
- **Total**: ~100 bytes for default registry (3 solvers)

**Negligible** compared to solution computation.

## Comparison: Registry vs Hardcoded Match

Registry approach provides:
- ✅ Extensible (register custom solvers)
- ✅ Priority-based selection
- ✅ Testable (inject mock solvers)
- ✅ Solver reuse (Arc-wrapped, cached)

Hardcoded approach problems:
- ❌ Cannot add solvers without modifying source
- ❌ No priority system
- ❌ Hard to test (tightly coupled)
- ❌ Creates new solver instance every time (no caching)












## Examples


### Default Registry Usage

Create and use default registry with standard solvers

<details>
<summary><b>Rust</b></summary>

```rust
let registry = PDESolverRegistry::new();

// Automatically classify and solve
let solution = registry.solve(&pde)?;

```
</details>

<details>
<summary><b>Python</b></summary>

```python
registry = PDESolverRegistry()

# Automatically classify and solve
solution = registry.solve(pde)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const registry = new PDESolverRegistry();

// Automatically classify and solve
const solution = registry.solve(pde);

```
</details>




### Custom Poisson Solver

Register a custom solver for Poisson equation (non-homogeneous Laplace)

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>




### Solver Discovery

List available solvers and inspect registry contents

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>






## Performance

**Time Complexity**: O(1)


## API Reference

- **Rust**: `mathhook_core::pde::registry`
- **Python**: `mathhook.pde.registry`
- **JavaScript**: `mathhook.pde.registry`


## See Also


- [advanced.pde.sympy_validation](../advanced/pde/sympy_validation.md)

- [advanced.pde.performance](../advanced/pde/performance.md)

- [advanced.pde.technical_guide](../advanced/pde/technical_guide.md)


