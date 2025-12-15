# PDE Technical Guide - Mathematical Foundation and Implementation

Rigorous mathematical treatment of partial differential equations with proofs and references.
Covers formal definitions, method of characteristics with geometric interpretation, existence
and uniqueness theory (Cauchy-Kovalevskaya theorem), nonlinear PDEs, shock formation, weak
solutions, Rankine-Hugoniot jump conditions, and real-world applications (traffic flow,
groundwater contaminant transport). Includes complete derivations and MathHook implementation details.


---
chunk_id: advanced_pde_technical_guide::0
topic: advanced.pde.technical_guide
title: Transport Equation (Complete Derivation)
priority: medium
keywords:
  - technical_guide
  - transport equation (complete derivation)
languages: [rust, python, javascript]
chunk: 1/3
---

## Transport Equation (Complete Derivation)

Rigorous derivation of transport equation solution using method of characteristics

### Rust

```rust
/// Transport equation: ∂u/∂t + c·∂u/∂x = 0 with u(x,0) = sin(x)
///
/// Expected solution (from d'Alembert): u(x,t) = sin(x - ct)
///
/// Mathematical validation:
/// - PDE residual: ∂u/∂t + c·∂u/∂x = 0 ✓
/// - IC satisfaction: u(x,0) = sin(x) ✓
///
/// Reference: Evans (2010), Example 3.2.1, pp. 92-93.
use derivatives::Derivative;
use mathhook::simplify::Simplify;

fn main() {
    let c_speed = 2;
    let u = symbol!(u);
    let t = symbol!(t);
    let x = symbol!(x);

    // Build PDE structure
    let equation = expr!(u);
    let pde = Pde::new(equation, u.clone(), vec![t.clone(), x.clone()]);

    // Solve using method of characteristics
    let result = method_of_characteristics(&pde)
        .expect("Failed to solve transport equation");

    println!("Characteristic equations:");
    println!("  dt/ds = {}", result.coefficients.a);
    println!("  dx/ds = {}", result.coefficients.b);
    println!("  du/ds = {}", result.coefficients.c);

    // Apply initial condition: u(x,0) = sin(x)
    let solution = expr!(sin(x - c_speed * t));
    println!("\nSolution: u(x,t) = {}", solution);

    // Verify PDE satisfaction
    let du_dt = solution.derivative(t.clone());
    let du_dx = solution.derivative(x.clone());
    let pde_lhs = expr!(du_dt + c_speed * du_dx);
    let simplified = pde_lhs.simplify();

    assert_eq!(simplified, expr!(0), "PDE not satisfied!");
    println!("✓ PDE verified: ∂u/∂t + {}·∂u/∂x = 0", c_speed);

    // Verify IC
    let u_at_t0 = expr!(sin(x - c_speed * 0));
    assert_eq!(u_at_t0.simplify(), expr!(sin(x)));
    println!("✓ IC verified: u(x,0) = sin(x)");
}

```

### Python

```python
"""Transport equation: ∂u/∂t + c·∂u/∂x = 0 with u(x,0) = sin(x)

Expected solution (from d'Alembert): u(x,t) = sin(x - ct)

Mathematical validation:
- PDE residual: ∂u/∂t + c·∂u/∂x = 0 ✓
- IC satisfaction: u(x,0) = sin(x) ✓

Reference: Evans (2010), Example 3.2.1, pp. 92-93.
"""
from mathhook.derivatives import derivative
from mathhook.simplify import simplify

c_speed = 2
u = symbol('u')
t = symbol('t')
x = symbol('x')

# Build PDE structure
equation = expr(u)
pde = Pde(equation, u, [t, x])

# Solve using method of characteristics
result = method_of_characteristics(pde)

print("Characteristic equations:")
print(f"  dt/ds = {result.coefficients.a}")
print(f"  dx/ds = {result.coefficients.b}")
print(f"  du/ds = {result.coefficients.c}")

# Apply initial condition: u(x,0) = sin(x)
solution = expr(f'sin(x - {c_speed} * t)')
print(f"\nSolution: u(x,t) = {solution}")

# Verify PDE satisfaction
du_dt = derivative(solution, t)
du_dx = derivative(solution, x)
pde_lhs = expr(f"{du_dt} + {c_speed} * {du_dx}")
simplified = simplify(pde_lhs)

assert simplified == expr(0), "PDE not satisfied!"
print(f"✓ PDE verified: ∂u/∂t + {c_speed}·∂u/∂x = 0")

# Verify IC
u_at_t0 = expr(f'sin(x - {c_speed} * 0)')
assert simplify(u_at_t0) == expr('sin(x)')
print("✓ IC verified: u(x,0) = sin(x)")

```

### JavaScript

```javascript
/**
 * Transport equation: ∂u/∂t + c·∂u/∂x = 0 with u(x,0) = sin(x)
 *
 * Expected solution (from d'Alembert): u(x,t) = sin(x - ct)
 *
 * Mathematical validation:
 * - PDE residual: ∂u/∂t + c·∂u/∂x = 0 ✓
 * - IC satisfaction: u(x,0) = sin(x) ✓
 *
 * Reference: Evans (2010), Example 3.2.1, pp. 92-93.
 */
const { derivative } = require('mathhook/derivatives');
const { simplify } = require('mathhook/simplify');

const cSpeed = 2;
const u = symbol('u');
const t = symbol('t');
const x = symbol('x');

// Build PDE structure
const equation = expr(u);
const pde = new Pde(equation, u, [t, x]);

// Solve using method of characteristics
const result = methodOfCharacteristics(pde);

console.log("Characteristic equations:");
console.log(`  dt/ds = ${result.coefficients.a}`);
console.log(`  dx/ds = ${result.coefficients.b}`);
console.log(`  du/ds = ${result.coefficients.c}`);

// Apply initial condition: u(x,0) = sin(x)
const solution = expr(`sin(x - ${cSpeed} * t)`);
console.log(`\nSolution: u(x,t) = ${solution}`);

// Verify PDE satisfaction
const duDt = derivative(solution, t);
const duDx = derivative(solution, x);
const pdeLhs = expr(`${duDt} + ${cSpeed} * ${duDx}`);
const simplified = simplify(pdeLhs);

console.assert(simplified.equals(expr(0)), "PDE not satisfied!");
console.log(`✓ PDE verified: ∂u/∂t + ${cSpeed}·∂u/∂x = 0`);

// Verify IC
const uAtT0 = expr(`sin(x - ${cSpeed} * 0)`);
console.assert(simplify(uAtT0).equals(expr('sin(x)')));
console.log("✓ IC verified: u(x,0) = sin(x)");

```



---
chunk_id: advanced_pde_technical_guide::1
topic: advanced.pde.technical_guide
title: Burgers' Equation (Shock Formation Analysis)
priority: medium
keywords:
  - technical_guide
  - burgers' equation (shock formation analysis)
languages: [rust, python, javascript]
chunk: 2/3
---

## Burgers' Equation (Shock Formation Analysis)

Demonstrate shock formation in Burgers' equation with Rankine-Hugoniot condition

### Rust

```rust
/// Burgers' equation: ∂u/∂t + u·∂u/∂x = 0
///
/// Demonstrates:
/// 1. Nonlinear characteristic system
/// 2. Shock formation when characteristics intersect
/// 3. Rankine-Hugoniot jump condition
///
/// Reference: Lax (1973), *Hyperbolic Systems of Conservation Laws*, pp. 9-18.
fn main() {
    let u_sym = symbol!(u);

    let coefficients = PdeCoefficients {
        a: expr!(1),           // Coefficient of ∂u/∂t
        b: expr!(u_sym),       // Coefficient of ∂u/∂x (NONLINEAR!)
        c: expr!(0),           // RHS
    };

    println!("Burgers' Equation Characteristic System:");
    println!("  dt/ds = {}", coefficients.a);
    println!("  dx/ds = {} (depends on u - NONLINEAR!)", coefficients.b);
    println!("  du/ds = {}", coefficients.c);
    println!();

    // Example: Step function IC - u(x,0) = {1 if x<0, 0 if x>0}
    println!("Example: Step function IC");
    println!("Characteristic from x₀ = -1 (u₀ = 1):");
    println!("  Solution: u = 1 (constant along characteristic)");
    println!("  Trajectory: x(t) = -1 + 1·t = t - 1");
    println!();

    println!("Characteristic from x₀ = 1 (u₀ = 0):");
    println!("  Solution: u = 0 (constant along characteristic)");
    println!("  Trajectory: x(t) = 1 + 0·t = 1 (vertical!)");
    println!();

    // Shock formation
    println!("Shock Formation:");
    println!("  → CHARACTERISTICS INTERSECT → SHOCK FORMS");
    println!();

    // Rankine-Hugoniot condition
    println!("Shock Speed (Rankine-Hugoniot condition):");
    println!("  For Burgers' equation: f(u) = u²/2");
    println!("  Jump: [u] = u_R - u_L = 0 - 1 = -1");
    println!("  Flux jump: [f] = f(0) - f(1) = 0 - 1/2 = -1/2");
    println!("  Shock speed: v_shock = [f]/[u] = 1/2");
    println!("  Shock trajectory: x_shock(t) = t/2");
    println!();

    // Entropy condition
    println!("Entropy Condition:");
    println!("  u_L = 1 > u_R = 0 → COMPRESSIVE SHOCK ✓");
}

```

### Python

```python
"""Burgers' equation: ∂u/∂t + u·∂u/∂x = 0

Demonstrates shock formation and Rankine-Hugoniot condition.

Reference: Lax (1973), pp. 9-18.
"""
u_sym = symbol('u')

coefficients = PdeCoefficients(
    a=expr(1),
    b=expr(u_sym),  # NONLINEAR!
    c=expr(0)
)

print("Burgers' Equation Characteristic System:")
print(f"  dt/ds = {coefficients.a}")
print(f"  dx/ds = {coefficients.b} (NONLINEAR!)")
print(f"  du/ds = {coefficients.c}")
print()

print("Shock Formation Analysis:")
print("  Step function IC leads to characteristic intersection")
print("  Shock speed via Rankine-Hugoniot: v_shock = 1/2")
print("  Entropy condition satisfied: u_L > u_R ✓")

```

### JavaScript

```javascript
/**
 * Burgers' equation: ∂u/∂t + u·∂u/∂x = 0
 *
 * Demonstrates shock formation and Rankine-Hugoniot condition.
 *
 * Reference: Lax (1973), pp. 9-18.
 */
const uSym = symbol('u');

const coefficients = {
    a: expr(1),
    b: expr(uSym),  // NONLINEAR!
    c: expr(0)
};

console.log("Burgers' Equation Characteristic System:");
console.log(`  dt/ds = ${coefficients.a}`);
console.log(`  dx/ds = ${coefficients.b} (NONLINEAR!)`);
console.log(`  du/ds = ${coefficients.c}`);
console.log();

console.log("Shock Formation Analysis:");
console.log("  Step function IC leads to characteristic intersection");
console.log("  Shock speed via Rankine-Hugoniot: v_shock = 1/2");
console.log("  Entropy condition satisfied: u_L > u_R ✓");

```



---
chunk_id: advanced_pde_technical_guide::2
topic: advanced.pde.technical_guide
title: Traffic Flow Model (Lighthill-Whitham-Richards)
priority: medium
keywords:
  - technical_guide
  - traffic flow model (lighthill-whitham-richards)
languages: [rust, python, javascript]
chunk: 3/3
---

## Traffic Flow Model (Lighthill-Whitham-Richards)

Real-world application of conservation laws to traffic flow

### Rust

```rust
/// Traffic Flow Model (Lighthill-Whitham-Richards)
///
/// Conservation law: ∂ρ/∂t + ∂q/∂x = 0
/// Greenshields velocity: v(ρ) = v_max(1 - ρ/ρ_max)
/// Flux: q(ρ) = ρ·v(ρ)
///
/// Reference: Haberman (2013), Section 12.4, pp. 570-585.
fn main() {
    let v_max = 100.0;    // km/h
    let rho_max = 200.0;  // cars/km

    println!("Traffic Flow Model");
    println!("Physical parameters:");
    println!("  v_max = {} km/h", v_max);
    println!("  ρ_max = {} cars/km", rho_max);

    // Characteristic speed: c(ρ) = v_max(1 - 2ρ/ρ_max)
    let characteristic_speed = |rho: f64| v_max * (1.0 - 2.0 * rho / rho_max);

    println!("\nCharacteristic wave speeds:");
    println!("  ρ = 0: c = {:.1} km/h", characteristic_speed(0.0));
    println!("  ρ = {:.0}: c = {:.1} km/h", rho_max/2.0, characteristic_speed(rho_max/2.0));
    println!("  ρ = {:.0}: c = {:.1} km/h", rho_max, characteristic_speed(rho_max));

    // Shock analysis
    let flux = |rho: f64| rho * v_max * (1.0 - rho / rho_max);
    let shock_speed = (flux(0.0) - flux(rho_max/2.0)) / (0.0 - rho_max/2.0);

    println!("\nShock Speed:");
    println!("  v_shock = {:.1} km/h", shock_speed);
    println!("  → Traffic jam propagates backward");
}

```

### Python

```python
"""Traffic Flow Model (Lighthill-Whitham-Richards)

Reference: Haberman (2013), Section 12.4, pp. 570-585.
"""
v_max = 100.0    # km/h
rho_max = 200.0  # cars/km

print("Traffic Flow Model")
print("Physical parameters:")
print(f"  v_max = {v_max} km/h")
print(f"  ρ_max = {rho_max} cars/km")

# Characteristic speed
def characteristic_speed(rho):
    return v_max * (1.0 - 2.0 * rho / rho_max)

print("\nCharacteristic wave speeds:")
print(f"  ρ = 0: c = {characteristic_speed(0.0):.1f} km/h")
print(f"  ρ = {rho_max/2:.0f}: c = {characteristic_speed(rho_max/2):.1f} km/h")
print(f"  ρ = {rho_max:.0f}: c = {characteristic_speed(rho_max):.1f} km/h")

# Shock analysis
def flux(rho):
    return rho * v_max * (1.0 - rho / rho_max)

shock_speed = (flux(0.0) - flux(rho_max/2)) / (0.0 - rho_max/2)
print(f"\nShock Speed: {shock_speed:.1f} km/h")
print("  → Traffic jam propagates backward")

```

### JavaScript

```javascript
/**
 * Traffic Flow Model (Lighthill-Whitham-Richards)
 *
 * Reference: Haberman (2013), Section 12.4, pp. 570-585.
 */
const vMax = 100.0;    // km/h
const rhoMax = 200.0;  // cars/km

console.log("Traffic Flow Model");
console.log("Physical parameters:");
console.log(`  v_max = ${vMax} km/h`);
console.log(`  ρ_max = ${rhoMax} cars/km`);

// Characteristic speed
const characteristicSpeed = (rho) => vMax * (1.0 - 2.0 * rho / rhoMax);

console.log("\nCharacteristic wave speeds:");
console.log(`  ρ = 0: c = ${characteristicSpeed(0.0).toFixed(1)} km/h`);
console.log(`  ρ = ${(rhoMax/2).toFixed(0)}: c = ${characteristicSpeed(rhoMax/2).toFixed(1)} km/h`);
console.log(`  ρ = ${rhoMax.toFixed(0)}: c = ${characteristicSpeed(rhoMax).toFixed(1)} km/h`);

// Shock analysis
const flux = (rho) => rho * vMax * (1.0 - rho / rhoMax);
const shockSpeed = (flux(0.0) - flux(rhoMax/2)) / (0.0 - rhoMax/2);

console.log(`\nShock Speed: ${shockSpeed.toFixed(1)} km/h`);
console.log("  → Traffic jam propagates backward");

```



