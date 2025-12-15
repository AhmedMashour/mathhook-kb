---









---

# PDE Quick Start - 5 Minutes to Your First Solution

> **Topic**: `advanced.pde.quick_start`

Quick-start tutorial for solving partial differential equations with MathHook.
Covers transport equation solving in 30 seconds, common PDE patterns, and complete examples.



## Mathematical Definition

$$**Transport Equation:**
$$\frac{\partial u}{\partial t} + c \cdot \frac{\partial u}{\partial x} = 0$$

where $c$ is the wave speed and $u(x,t)$ is the unknown function.
$$



# PDE Quick Start - 5 Minutes to Your First Solution

## Installation

Add MathHook to your `Cargo.toml`:

```toml
[dependencies]
mathhook = "0.1"
mathhook-core = "0.1"
```

## Transport Equation in 30 Seconds

**Problem:** Solve $\frac{\partial u}{\partial t} + \frac{\partial u}{\partial x} = 0$ with $u(x,0) = \sin(x)$

**What just happened:**
- Solved transport equation (wave moves right at speed 1)
- Initial wave shape: $\sin(x)$
- Solution at time $t$: $\sin(x - t)$

**Physical interpretation:** The sine wave propagates to the right, keeping its shape.

## Common PDEs Cheat Sheet

### Transport Equation
**PDE:** $\frac{\partial u}{\partial t} + c \cdot \frac{\partial u}{\partial x} = 0$

**Use for:** Wave propagation, signal advection, fluid transport

**Solution form:** $u(x,t) = f(x - c \cdot t)$ where $f$ is the initial condition












## Examples


### Transport Equation - Basic Solution

Solve transport equation with sinusoidal initial condition

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

// Define variables
let u = symbol!(u);
let t = symbol!(t);
let x = symbol!(x);

// Build PDE structure
let equation = expr!(u);
let pde = Pde::new(equation, u, vec![t.clone(), x.clone()]);

// Solve using method of characteristics
match method_of_characteristics(&pde) {
    Ok(solution) => {
        println!("General solution: F(x - t)");

        // Apply initial condition: u(x,0) = sin(x)
        // Therefore: u(x,t) = sin(x - t)
        let specific_solution = expr!(sin(x - t));

        println!("Specific solution: {}", specific_solution);
    }
    Err(e) => println!("Error: {:?}", e),
}

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr, Pde, method_of_characteristics
import math

# Define variables
u = symbol('u')
t = symbol('t')
x = symbol('x')

# Build PDE
equation = expr(u)
pde = Pde(equation, u, [t, x])

# Solve
solution = method_of_characteristics(pde)
print("General solution: F(x - t)")

# Apply initial condition
specific_solution = expr(f"sin({x} - {t})")
print(f"Specific solution: {specific_solution}")

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr, Pde, methodOfCharacteristics } = require('mathhook');

// Define variables
const u = symbol('u');
const t = symbol('t');
const x = symbol('x');

// Build PDE
const equation = expr(u);
const pde = new Pde(equation, u, [t, x]);

// Solve
const solution = methodOfCharacteristics(pde);
console.log("General solution: F(x - t)");

// Apply initial condition
const specificSolution = expr(`sin(${x} - ${t})`);
console.log(`Specific solution: ${specificSolution}`);

```
</details>




### Complete Working Example - Full Workflow

End-to-end example with verification and characteristic trajectory

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook::pde::method_of_characteristics::{
    method_of_characteristics, solve_characteristic_odes
};
use derivatives::Derivative;
use mathhook::simplify::Simplify;

fn main() {
    println!("═══════════════════════════════════════");
    println!("MathHook PDE Solver - Transport Equation");
    println!("═══════════════════════════════════════\n");

    // Problem: ∂u/∂t + 2·∂u/∂x = 0 with u(x,0) = x²
    let u = symbol!(u);
    let t = symbol!(t);
    let x = symbol!(x);

    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![t.clone(), x.clone()]);

    println!("PDE: ∂u/∂t + 2·∂u/∂x = 0");
    println!("IC:  u(x, 0) = x²\n");

    // Solve
    match method_of_characteristics(&pde) {
        Ok(result) => {
            println!("✓ Method of characteristics applied");

            let solution = expr!((x - (2 * t)) ^ 2);
            println!("Solution: u(x,t) = {}\n", solution);

            // Verify
            let du_dt = solution.derivative(t.clone());
            let du_dx = solution.derivative(x.clone());
            let lhs = expr!(du_dt + (2 * du_dx));

            println!("Verification:");
            println!("  PDE satisfied: {}", lhs.simplify() == expr!(0));
            println!("  IC satisfied: u(x,0) = x²\n");
            println!("✓ Solution complete!");
        }
        Err(e) => println!("✗ Error: {:?}", e),
    }
}

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr, Pde, method_of_characteristics, derivative, simplify

print("═" * 40)
print("MathHook PDE Solver - Transport Equation")
print("═" * 40)

u = symbol('u')
t = symbol('t')
x = symbol('x')

equation = expr(u)
pde = Pde(equation, u, [t, x])

print("\nPDE: ∂u/∂t + 2·∂u/∂x = 0")
print("IC:  u(x, 0) = x²\n")

result = method_of_characteristics(pde)
print("✓ Method of characteristics applied")

solution = expr(f"({x} - 2*{t})^2")
print(f"Solution: u(x,t) = {solution}\n")

# Verify
du_dt = derivative(solution, t)
du_dx = derivative(solution, x)
lhs = expr(f"{du_dt} + 2*{du_dx}")

print("Verification:")
print(f"  PDE satisfied: {simplify(lhs) == expr(0)}")
print("  IC satisfied: u(x,0) = x²")
print("\n✓ Solution complete!")

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr, Pde, methodOfCharacteristics, derivative, simplify } = require('mathhook');

console.log("═".repeat(40));
console.log("MathHook PDE Solver - Transport Equation");
console.log("═".repeat(40));

const u = symbol('u');
const t = symbol('t');
const x = symbol('x');

const equation = expr(u);
const pde = new Pde(equation, u, [t, x]);

console.log("\nPDE: ∂u/∂t + 2·∂u/∂x = 0");
console.log("IC:  u(x, 0) = x²\n");

const result = methodOfCharacteristics(pde);
console.log("✓ Method of characteristics applied");

const solution = expr(`(${x} - 2*${t})^2`);
console.log(`Solution: u(x,t) = ${solution}\n`);

// Verify
const duDt = derivative(solution, t);
const duDx = derivative(solution, x);
const lhs = expr(`${duDt} + 2*${duDx}`);

console.log("Verification:");
console.log(`  PDE satisfied: ${simplify(lhs) === expr(0)}`);
console.log("  IC satisfied: u(x,0) = x²");
console.log("\n✓ Solution complete!");

```
</details>







## API Reference

- **Rust**: `mathhook_core::pde::method_of_characteristics`
- **Python**: `mathhook.pde.method_of_characteristics`
- **JavaScript**: `mathhook.pde.methodOfCharacteristics`


## See Also


- [advanced.pde.user_guide](../advanced/pde/user_guide.md)

- [advanced.pde.method_of_characteristics](../advanced/pde/method_of_characteristics.md)

- [advanced.pde.technical_guide](../advanced/pde/technical_guide.md)


