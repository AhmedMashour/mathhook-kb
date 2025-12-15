# PDE User Guide - MathHook CAS

Comprehensive user guide for solving partial differential equations with MathHook. Covers
fundamental PDE concepts, classification systems, the method of characteristics for first-order
PDEs, and practical examples including transport equations, Burgers' equation, and heat equations.
Includes educational features, troubleshooting, and performance considerations.


---
chunk_id: advanced_pde_user_guide::0
topic: advanced.pde.user_guide
title: Transport Equation (Step-by-Step)
priority: medium
keywords:
  - user_guide
  - transport equation (step-by-step)
languages: [rust, python, javascript]
chunk: 1/4
---

## Transport Equation (Step-by-Step)

Solve transport equation ∂u/∂t + 2·∂u/∂x = 0 with initial condition u(x,0) = sin(x)

### Rust

```rust
let u = symbol!(u);
let t = symbol!(t);
let x = symbol!(x);

// Build the PDE
let equation = expr!(u);
let pde = Pde::new(equation, u.clone(), vec![t.clone(), x.clone()]);

// Solve using method of characteristics
let result = method_of_characteristics(&pde)?;

println!("Characteristic equations:");
for (i, char_eq) in result.characteristic_equations.iter().enumerate() {
    println!("  Equation {}: {}", i + 1, char_eq);
}

println!("\nGeneral solution: {}", result.solution);

// Apply initial condition: u(x,0) = sin(x)
// Solution: u(x,t) = sin(x - 2t)
let solution_with_ic = expr!(sin(x + (-2) * t));
println!("\nSpecific solution: u(x,t) = {}", solution_with_ic);

```

### Python

```python
u = symbol('u')
t = symbol('t')
x = symbol('x')

# Build the PDE
equation = expr(u)
pde = Pde(equation, u, [t, x])

# Solve using method of characteristics
result = method_of_characteristics(pde)

print("Characteristic equations:")
for i, char_eq in enumerate(result.characteristic_equations):
    print(f"  Equation {i + 1}: {char_eq}")

print(f"\nGeneral solution: {result.solution}")

# Apply initial condition: u(x,0) = sin(x)
# Solution: u(x,t) = sin(x - 2t)
solution_with_ic = expr('sin(x + (-2) * t)')
print(f"\nSpecific solution: u(x,t) = {solution_with_ic}")

```

### JavaScript

```javascript
const u = symbol('u');
const t = symbol('t');
const x = symbol('x');

// Build the PDE
const equation = expr(u);
const pde = new Pde(equation, u, [t, x]);

// Solve using method of characteristics
const result = methodOfCharacteristics(pde);

console.log("Characteristic equations:");
result.characteristicEquations.forEach((charEq, i) => {
    console.log(`  Equation ${i + 1}: ${charEq}`);
});

console.log(`\nGeneral solution: ${result.solution}`);

// Apply initial condition: u(x,0) = sin(x)
// Solution: u(x,t) = sin(x - 2t)
const solutionWithIc = expr('sin(x + (-2) * t)');
console.log(`\nSpecific solution: u(x,t) = ${solutionWithIc}`);

```



---
chunk_id: advanced_pde_user_guide::1
topic: advanced.pde.user_guide
title: Verifying PDE Solution
priority: medium
keywords:
  - user_guide
  - verifying pde solution
languages: [rust, python, javascript]
chunk: 2/4
---

## Verifying PDE Solution

Check that solution satisfies the PDE and initial condition

### Rust

```rust
use derivatives::Derivative;
use mathhook::simplify::Simplify;

// Solution: u(x,t) = sin(x - 2*t)
let solution = expr!(sin(x + (-2) * t));

// Verify PDE: ∂u/∂t + 2·∂u/∂x = 0
let du_dt = solution.derivative(t.clone());
let du_dx = solution.derivative(x.clone());

println!("∂u/∂t = {}", du_dt);
// Output: -2*cos(x - 2*t)

println!("∂u/∂x = {}", du_dx);
// Output: cos(x - 2*t)

// Check PDE
let lhs = expr!(du_dt + 2 * du_dx);
println!("PDE LHS = {}", lhs.simplify());
// Output: 0 ✓

```

### Python

```python
from mathhook.derivatives import derivative
from mathhook.simplify import simplify

# Solution: u(x,t) = sin(x - 2*t)
solution = expr('sin(x + (-2) * t)')

# Verify PDE: ∂u/∂t + 2·∂u/∂x = 0
du_dt = derivative(solution, t)
du_dx = derivative(solution, x)

print(f"∂u/∂t = {du_dt}")
# Output: -2*cos(x - 2*t)

print(f"∂u/∂x = {du_dx}")
# Output: cos(x - 2*t)

# Check PDE
lhs = expr(f"{du_dt} + 2 * {du_dx}")
print(f"PDE LHS = {simplify(lhs)}")
# Output: 0 ✓

```

### JavaScript

```javascript
const { derivative } = require('mathhook/derivatives');
const { simplify } = require('mathhook/simplify');

// Solution: u(x,t) = sin(x - 2*t)
const solution = expr('sin(x + (-2) * t)');

// Verify PDE: ∂u/∂t + 2·∂u/∂x = 0
const duDt = derivative(solution, t);
const duDx = derivative(solution, x);

console.log(`∂u/∂t = ${duDt}`);
// Output: -2*cos(x - 2*t)

console.log(`∂u/∂x = ${duDx}`);
// Output: cos(x - 2*t)

// Check PDE
const lhs = expr(`${duDt} + 2 * ${duDx}`);
console.log(`PDE LHS = ${simplify(lhs)}`);
// Output: 0 ✓

```



---
chunk_id: advanced_pde_user_guide::2
topic: advanced.pde.user_guide
title: Burgers' Equation (Nonlinear)
priority: medium
keywords:
  - user_guide
  - burgers' equation (nonlinear)
languages: [rust, python, javascript]
chunk: 3/4
---

## Burgers' Equation (Nonlinear)

Analyze Burgers' equation ∂u/∂t + u·∂u/∂x = 0 showing nonlinear characteristics

### Rust

```rust
// Burgers' equation coefficients
let u_sym = symbol!(u);
let coefficients = PdeCoefficients {
    a: expr!(1),                         // Coefficient of ∂u/∂t
    b: expr!(u_sym),                     // Coefficient of ∂u/∂x (nonlinear!)
    c: expr!(0),                         // RHS
};

println!("Burgers' equation characteristic system:");
println!("dt/ds = {}", coefficients.a);
println!("dx/ds = {}", coefficients.b);  // Note: depends on u!
println!("du/ds = {}", coefficients.c);

// The solution u = F(x - u*t) is implicit (requires solving for u)
// Warning: Can develop shocks where characteristics intersect

```

### Python

```python
# Burgers' equation coefficients
u_sym = symbol('u')
coefficients = PdeCoefficients(
    a=expr(1),           # Coefficient of ∂u/∂t
    b=expr(u_sym),       # Coefficient of ∂u/∂x (nonlinear!)
    c=expr(0)            # RHS
)

print("Burgers' equation characteristic system:")
print(f"dt/ds = {coefficients.a}")
print(f"dx/ds = {coefficients.b}")  # Note: depends on u!
print(f"du/ds = {coefficients.c}")

# The solution u = F(x - u*t) is implicit (requires solving for u)
# Warning: Can develop shocks where characteristics intersect

```

### JavaScript

```javascript
// Burgers' equation coefficients
const uSym = symbol('u');
const coefficients = {
    a: expr(1),           // Coefficient of ∂u/∂t
    b: expr(uSym),        // Coefficient of ∂u/∂x (nonlinear!)
    c: expr(0)            // RHS
};

console.log("Burgers' equation characteristic system:");
console.log(`dt/ds = ${coefficients.a}`);
console.log(`dx/ds = ${coefficients.b}`);  // Note: depends on u!
console.log(`du/ds = ${coefficients.c}`);

// The solution u = F(x - u*t) is implicit (requires solving for u)
// Warning: Can develop shocks where characteristics intersect

```



---
chunk_id: advanced_pde_user_guide::3
topic: advanced.pde.user_guide
title: Educational Step-by-Step Solver
priority: medium
keywords:
  - user_guide
  - educational step-by-step solver
languages: [rust, python, javascript]
chunk: 4/4
---

## Educational Step-by-Step Solver

Get detailed explanations of solution process

### Rust

```rust
let solver = EducationalPDESolver::new();

let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

let equation = expr!(u + x);

// Solve with explanations
let (result, explanation) = solver.solve_pde(&equation, &u, &[x, t]);

// Display step-by-step explanation
println!("Educational Explanation:");
for (i, step) in explanation.steps.iter().enumerate() {
    println!("Step {}: {}", i + 1, step.title);
    println!("  {}", step.description);
    println!();
}

```

### Python

```python
solver = EducationalPDESolver()

u = symbol('u')
x = symbol('x')
t = symbol('t')

equation = expr('u + x')

# Solve with explanations
result, explanation = solver.solve_pde(equation, u, [x, t])

# Display step-by-step explanation
print("Educational Explanation:")
for i, step in enumerate(explanation.steps):
    print(f"Step {i + 1}: {step.title}")
    print(f"  {step.description}")
    print()

```

### JavaScript

```javascript
const solver = new EducationalPDESolver();

const u = symbol('u');
const x = symbol('x');
const t = symbol('t');

const equation = expr('u + x');

// Solve with explanations
const [result, explanation] = solver.solvePde(equation, u, [x, t]);

// Display step-by-step explanation
console.log("Educational Explanation:");
explanation.steps.forEach((step, i) => {
    console.log(`Step ${i + 1}: ${step.title}`);
    console.log(`  ${step.description}`);
    console.log();
});

```



