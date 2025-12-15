---









---

# Complete PDE Examples

> **Topic**: `advanced.pde.examples`

Three complete, real-world examples demonstrating MathHook's PDE solving capabilities across heat, wave, and Laplace equations.
Each example includes full problem setup, mathematical formulation, MathHook implementation, and physical interpretation.



## Mathematical Definition

**Example 1: Heat Diffusion**
$$\frac{\partial u}{\partial t} = \alpha \frac{\partial^2 u}{\partial x^2}$$

**Example 2: Wave Propagation**
$$\frac{\partial^2 u}{\partial t^2} = c^2 \frac{\partial^2 u}{\partial x^2}$$

**Example 3: Electrostatic Potential**
$$\frac{\partial^2 u}{\partial x^2} + \frac{\partial^2 u}{\partial y^2} = 0$$




# Complete PDE Examples

Three complete, real-world examples demonstrating MathHook's PDE solving capabilities.

## Example 1: Heat Diffusion in Steel Rod

**Physical Problem**: A 1-meter steel rod is initially heated to 100°C. Both ends are plunged into ice water (0°C). How does temperature evolve?

### Complete Solution

**STEP 1: Define Variables**

Temperature $u$, position $x$ (0 to 1 meter), time $t$

**STEP 2: Create PDE**

Heat equation: $\frac{\partial u}{\partial t} = \alpha \frac{\partial^2 u}{\partial x^2}$

**STEP 3: Material Properties**

Steel: $\alpha = k/(\rho c_p) \approx 1.3 \times 10^{-5}$ m²/s

**STEP 4: Boundary Conditions**

- $u(0,t) = 0°C$ (left end in ice water)
- $u(1,t) = 0°C$ (right end in ice water)

**STEP 5: Initial Condition**

$u(x,0) = 100°C$ (uniform initial temperature)

**STEP 6: Solve**

Using `HeatEquationSolver`

**STEP 7: Examine Solution**

Solution structure shows eigenvalues $\lambda_n = (n\pi/L)^2$ and exponential decay modes.

**Physical Interpretation**:
- Eigenvalues determine spatial modes
- Higher modes decay faster (∝ $n^2$)
- Temperature → 0°C as $t \to \infty$ (boundary temperature)

## Example 2: Vibrating Guitar String

**Physical Problem**: An E4 guitar string (0.65 m) is plucked 5 mm at the center and released. Describe the vibration.

### Complete Solution

**STEP 1: Define Variables**

Displacement $u$, position $x$ along string, time $t$

**STEP 2: Create PDE**

Wave equation: $\frac{\partial^2 u}{\partial t^2} = c^2 \frac{\partial^2 u}{\partial x^2}$

**STEP 3: Physical Parameters**

Steel E string: $T = 73.4$ N, $\rho = 3.75 \times 10^{-4}$ kg/m

Wave speed: $c = \sqrt{T/\rho} \approx 442$ m/s

**STEP 4: Boundary Conditions**

- $u(0,t) = 0$ (left end fixed)
- $u(L,t) = 0$ (right end fixed, $L = 0.65$ m)

**STEP 5: Initial Conditions**

- Initial position: triangular pluck at center (5 mm displacement)
- Initial velocity: released from rest ($\partial u/\partial t = 0$)

**STEP 6: Solve**

Using `WaveEquationSolver`

**STEP 7: Analyze Musical Properties**

**Musical Harmonics:**
- Fundamental: $f_1 = c/(2L) \approx 340$ Hz (close to E4 = 329.63 Hz)
- Overtones: $f_n = n f_1$
  - $f_2 = 680$ Hz (octave)
  - $f_3 = 1020$ Hz (octave + fifth)
  - $f_4 = 1360$ Hz (two octaves)

**Standing Wave Nodes:**
- Mode 1: nodes at $x = 0, 0.65$ m
- Mode 2: nodes at $x = 0, 0.325, 0.65$ m
- Mode 3: nodes at $x = 0, 0.217, 0.433, 0.65$ m

## Example 3: Electrostatic Potential in Rectangular Plate

**Physical Problem**: A 10 cm × 5 cm conducting plate has bottom/sides grounded (0 V) and top edge at 100 V. Find the potential distribution.

### Complete Solution

**STEP 1: Define Variables**

Electrostatic potential $u$, horizontal position $x$, vertical position $y$

**STEP 2: Create PDE**

Laplace equation: $\frac{\partial^2 u}{\partial x^2} + \frac{\partial^2 u}{\partial y^2} = 0$

**STEP 3: Boundary Conditions**

- $u(0,y) = 0$ V (left edge grounded)
- $u(a,y) = 0$ V (right edge grounded, $a = 0.1$ m)
- $u(x,0) = 0$ V (bottom edge grounded)
- $u(x,b) = 100$ V (top edge at fixed potential, $b = 0.05$ m)

**STEP 4: Solve**

Using `LaplaceEquationSolver`

**STEP 5: Examine Solution**

Solution structure shows:
- X-direction eigenvalues: $\lambda_n = (n\pi/a)^2$
- Hyperbolic sine functions in $y$-direction
- Smooth variation from 0 V to 100 V

**Physical Interpretation:**
- Potential varies smoothly from 0 V (bottom/sides) to 100 V (top)
- No local maxima/minima inside (maximum principle)
- Electric field $\mathbf{E} = -\nabla u$ points from high to low potential
- Field strongest near top edge (steepest gradient)

**Estimated potential at center (5 cm, 2.5 cm):**
$u(5, 2.5) \approx 48$ V (halfway between 0 V and 100 V)

## Common Pitfalls

### Pitfall 1: Expecting Numerical Coefficients

❌ WRONG: Coefficients are symbolic
```rust
for coeff in result.coefficients {
    let numerical_value = coeff.evaluate()?;  // ERROR: Can't evaluate A_1
}
```

✅ CORRECT: Acknowledge symbolic nature
```rust
for (n, coeff) in result.coefficients.iter().enumerate() {
    println!("Coefficient A_{} (symbolic): {}", n + 1, coeff);
}
```

### Pitfall 2: Using Non-Standard Variable Names

❌ MAY NOT CLASSIFY:
```rust
let r = symbol!(r);         // Radial
let theta = symbol!(theta); // Angular
```

✅ USE STANDARD NAMES:
```rust
let x = symbol!(x);
let y = symbol!(y);
let t = symbol!(t);
```

### Pitfall 3: Non-Homogeneous BCs Without Transformation

❌ UNSUPPORTED DIRECTLY:
```rust
let bc = BoundaryCondition::dirichlet(expr!(50), ...);  // Non-zero
```

✅ TRANSFORM FIRST:
1. Find steady-state $u_s(x)$ satisfying BCs
2. Solve for $v(x,t) = u(x,t) - u_s(x)$ with homogeneous BCs
3. Add back: $u(x,t) = v(x,t) + u_s(x)$

## Summary

**Three complete examples** demonstrate:
1. ✅ Heat equation: Thermal diffusion in steel
2. ✅ Wave equation: Musical string vibrations
3. ✅ Laplace equation: Electrostatic potential

**All examples show**:
- Correct eigenvalue computation
- Proper solution structure
- Physical interpretation
- Symbolic coefficient limitation

**Next steps**: Use these patterns for your own PDE problems, keeping limitations in mind (Dirichlet BCs only, symbolic coefficients).












## Examples


### Heat Diffusion in Steel Rod - Complete Implementation

1-meter steel rod cooling from 100°C with ice water at ends. Full implementation with error handling.


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

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

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr, Pde, BoundaryCondition, BoundaryLocation, InitialCondition, HeatEquationSolver

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

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr, Pde, BoundaryCondition, BoundaryLocation, InitialCondition, HeatEquationSolver } = require('mathhook');

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

```
</details>





### Vibrating Guitar String - Musical Analysis

E4 guitar string with musical frequency analysis and standing wave nodes.


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

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

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr, Pde, BoundaryCondition, BoundaryLocation, InitialCondition, WaveEquationSolver

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

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr, Pde, BoundaryCondition, BoundaryLocation, InitialCondition, WaveEquationSolver } = require('mathhook');

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

```
</details>





### Electrostatic Potential in Rectangular Plate

10cm × 5cm plate with grounded sides and fixed potential top edge.


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

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

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr, Pde, BoundaryCondition, BoundaryLocation, LaplaceEquationSolver

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

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr, Pde, BoundaryCondition, BoundaryLocation, LaplaceEquationSolver } = require('mathhook');

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

```
</details>







## Performance

**Time Complexity**: Varies by example: O(n) for n modes


## API Reference

- **Rust**: `mathhook_core::pde`
- **Python**: `mathhook.pde`
- **JavaScript**: `mathhook.pde`


## See Also


- [advanced.pde.heat_equation](../advanced/pde/heat_equation.md)

- [advanced.pde.wave_equation](../advanced/pde/wave_equation.md)

- [advanced.pde.laplace_equation](../advanced/pde/laplace_equation.md)

- [advanced.pde.separation_of_variables](../advanced/pde/separation_of_variables.md)


