---
description: "Learn to solve separable differential equations symbolically with MathHook. Step-by-step guide with Newton's cooling, exponential growth, and logistic models. Fast, accurate ODE solving for Rust, Python, JavaScript."
keywords: separable differential equations, ODE solver, separation of variables, first order ODE, symbolic ODE solving
canonical_url: "https://docs.mathhook.org/ode/first-order/separable"
og:title: "Separable Differential Equations - Complete Guide | MathHook CAS"
og:description: "Master separable ODEs with symbolic solving. Includes exponential growth, Newton's cooling, logistic models. Interactive examples for Rust, Python, and JavaScript. Professional computer algebra system."
og:image: "https://docs.mathhook.org/images/ode-separable-og.png"
twitter:card: "summary_large_image"
schema_type: "TechArticle"
language: "en"
---

# Separable Differential Equations

> **Topic**: `ode.first_order.separable`

Solve first-order ordinary differential equations that can be separated into functions of x and y independently


## Mathematical Definition

$$\frac{dy}{dx} = g(x)h(y) \quad \Rightarrow \quad \int \frac{dy}{h(y)} = \int g(x)dx + C$$



Imagine you're a detective investigating a crime scene, and you find a body.
The coroner needs to estimate time of death. How? By measuring body temperature
and using Newton's Law of Cooling—a separable differential equation. From modeling
population growth to predicting how fast your coffee cools, separable ODEs are
everywhere in nature and engineering.



## What Makes an ODE Separable?

A first-order ODE is **separable** if it can be written in the form:

$$\\frac{dy}{dx} = g(x)h(y)$$

Notice that the right-hand side factors into a function of $x$ multiplied by
a function of $y$. This special structure allows us to algebraically manipulate
the equation to get all $y$ terms on one side and all $x$ terms on the other.

### Recognition Patterns

**Separable:**
- $dy/dx = xy$ (factors as $x \\cdot y$)
- $dy/dx = e^{x+y} = e^x \\cdot e^y$ (exponential property)
- $dy/dx = \\sin(x)\\cos(y)$ (product of functions)

**NOT Separable:**
- $dy/dx = x + y$ (sum, not product)
- $dy/dx = xy + y^2$ (can't factor into g(x)h(y))
- $dy/dx = \\sin(xy)$ (mixed argument)

### The Key Insight

If we can write $dy/dx = g(x)h(y)$, then we can rearrange:

$$\\frac{dy}{h(y)} = g(x)dx$$

Now both sides are ready to integrate independently!


## The Separation of Variables Method

The solution process has four clear steps:

### Step 1: Verify Separability

Check if the equation can be factored as $dy/dx = g(x)h(y)$.

### Step 2: Separate Variables

Rearrange to get all $y$ terms (including $dy$) on one side and all $x$ terms
(including $dx$) on the other:

$$\\frac{dy}{h(y)} = g(x)dx$$

### Step 3: Integrate Both Sides

$$\\int \\frac{dy}{h(y)} = \\int g(x)dx + C$$

The constant of integration $C$ appears on the right side (convention).

### Step 4: Solve for y (if possible)

Try to solve algebraically for $y$ as an explicit function of $x$. Sometimes
this isn't possible, and we get an **implicit solution** instead (which is
still valid!).

### Example: dy/dx = xy

1. **Verify:** $g(x) = x$, $h(y) = y$ ✓
2. **Separate:** $dy/y = x\\,dx$
3. **Integrate:** $\\ln|y| = x^2/2 + C$
4. **Solve:** $y = Ae^{x^2/2}$ (where $A = \\pm e^C$)


## Initial Value Problems

An **initial condition** $y(x_0) = y_0$ allows us to find the particular
solution (a specific curve) rather than the general solution (a family of curves).

### Method

1. Find the general solution with constant $C$
2. Substitute the initial condition: $y(x_0) = y_0$
3. Solve for $C$
4. Write the particular solution

### Example: Exponential Growth with Initial Population

**Problem:** $dy/dx = 0.5y$, $y(0) = 100$

**General solution:**
- Separate: $dy/y = 0.5\\,dx$
- Integrate: $\\ln|y| = 0.5x + C$
- Solve: $y = Ae^{0.5x}$

**Apply initial condition:**
- $y(0) = 100 \\Rightarrow Ae^0 = A = 100$

**Particular solution:** $y = 100e^{0.5x}$

This represents a population that doubles every $\\ln(2)/0.5 \\approx 1.386$ time units.


## Real-World Applications

Separable ODEs model countless phenomena in science and engineering.

### Population Dynamics

**Exponential Growth:** $dP/dt = rP$
- Solution: $P(t) = P_0 e^{rt}$
- Models: bacteria, investments, viral videos

**Logistic Growth:** $dP/dt = rP(1 - P/K)$
- Solution: $P(t) = K/(1 + Ae^{-rt})$
- Models: species with limited resources, product adoption

### Physics

**Newton's Law of Cooling:** $dT/dt = k(T - T_{ambient})$
- Forensic science: time of death estimation
- Engineering: HVAC system design
- Daily life: beverage cooling

**Falling Objects with Air Resistance:** $dv/dt = g - kv$
- Terminal velocity: $v_{terminal} = g/k$
- Skydiving, raindrop dynamics

### Chemistry

**First-Order Reactions:** $dA/dt = -kA$
- Radioactive decay
- Drug metabolism
- Chemical kinetics

### Mixing Problems

Tank with inflow and outflow: $dy/dt = r_{in}c_{in} - r_{out}(y/V)$
- Water treatment plants
- Pharmaceutical manufacturing
- Environmental pollution modeling




---

## Deep Dives


### Existence and Uniqueness Theorem

The **Picard-Lindelöf theorem** guarantees a unique solution to
$dy/dx = f(x,y)$ with $y(x_0) = y_0$ if $f$ is continuous and
Lipschitz continuous in $y$ near $(x_0, y_0)$.

For separable equations $dy/dx = g(x)h(y)$:
- **Existence:** Guaranteed if $g(x)$ and $h(y)$ are continuous
- **Uniqueness:** Guaranteed except where $h(y_0) = 0$

This explains why singular solutions (where $h(y) = 0$) can appear!


### Direction Fields and Solution Curves

A **direction field** (slope field) visualizes the ODE by drawing
small line segments with slope $f(x,y)$ at each point $(x,y)$.

For separable equations, the direction field has special structure:
- Along lines where $h(y) = 0$, slopes are zero (horizontal)
- Along lines where $g(x) = 0$, slopes are zero (vertical)
- Solution curves follow the flow of the direction field

This geometric perspective helps understand singular solutions and
the qualitative behavior without solving explicitly.





---

## Implementation Notes

MathHook's implementation uses a pattern matching system to detect
separability. The key challenge is factoring arbitrary expressions
into $g(x)h(y)$ form. Current approach:

1. Extract all factors from the RHS expression
2. Classify each factor as "depends on x only", "depends on y only", or "mixed"
3. If any factor is "mixed", equation is not separable
4. Otherwise, group x-factors into g(x) and y-factors into h(y)

This is $O(m)$ for expression size $m$, using a single traversal.




---

## Complexity Analysis

**Separability Check:** $O(m)$ for expression size $m$
**Variable Separation:** $O(m)$ algebraic manipulation
**Integration (symbolic):** $O(n \\log n)$ to $O(n^2)$ depending on integrand
**Solving for y:** $O(m)$ to $O(m^2)$ depending on implicit complexity
**Total:** Dominated by integration, typically $O(n \\log n)$ to $O(n^2)$

**Optimizations:**
- Cache integration results for common patterns (e.g., $\\int e^x dx$)
- Use lookup table for elementary integrals
- Parallel integration of both sides (independent operations)





## Examples


### Simple Exponential Growth

The simplest separable ODE models exponential growth: dy/dx = y.
This can be separated as dy/y = dx, integrating gives ln|y| = x + C,
which simplifies to y = Ce^x. This models population growth, radioactive
decay (with negative constant), and compound interest.


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook_core::{symbol, expr};
use mathhook_core::ode::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);

// dy/dx = y (exponential growth)
let rhs = expr!(y);
let solver = SeparableODESolver::new();

// General solution: y = C*e^x
let solution = solver.solve(&rhs, &y, &x, None)?;
println!("General solution: {}", solution);

// Particular solution with y(0) = 3
let ic = (expr!(0), expr!(3));
let particular = solver.solve(&rhs, &y, &x, Some(ic))?;
println!("Particular solution: {}", particular); // y = 3*e^x

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr
from mathhook.ode import SeparableODESolver

x = symbol('x')
y = symbol('y')

# dy/dx = y (exponential growth)
rhs = y
solver = SeparableODESolver()

# General solution: y = C*e^x
solution = solver.solve(rhs, y, x)
print(f"General solution: {solution}")

# Particular solution with y(0) = 3
particular = solver.solve(rhs, y, x, initial=(0, 3))
print(f"Particular solution: {particular}")  # y = 3*e^x

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode');

const x = symbol('x');
const y = symbol('y');

// dy/dx = y (exponential growth)
const rhs = y;
const solver = new SeparableODESolver();

// General solution: y = C*e^x
const solution = solver.solve(rhs, y, x);
console.log(`General solution: ${solution}`);

// Particular solution with y(0) = 3
const particular = solver.solve(rhs, y, x, { initial: [0, 3] });
console.log(`Particular solution: ${particular}`); // y = 3*e^x

```
</details>


**Expected Output:**
```
y = 3*e^x
```



### Logistic Growth Model

The logistic equation dy/dx = y(1-y) models population growth with carrying
capacity. Separating variables: dy/(y(1-y)) = dx. Using partial fractions:
(1/y + 1/(1-y))dy = dx. Integrating: ln|y/(1-y)| = x + C.
Solving for y: y = 1/(1 + Ce^(-x)). This S-curve is fundamental in ecology,
epidemiology, and marketing.


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook_core::{symbol, expr};
use mathhook_core::ode::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);

// dy/dx = y(1-y) (logistic growth)
let rhs = expr!(y * (1 - y));
let solver = SeparableODESolver::new();

// General solution: y = 1/(1 + C*e^(-x))
let solution = solver.solve(&rhs, &y, &x, None)?;

// With y(0) = 0.1 (10% initial population)
let ic = (expr!(0), expr!(0.1));
let particular = solver.solve(&rhs, &y, &x, Some(ic))?;
println!("{}", particular); // y = 1/(1 + 9*e^(-x))

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr
from mathhook.ode import SeparableODESolver

x, y = symbol('x'), symbol('y')

# dy/dx = y(1-y) (logistic growth)
rhs = y * (1 - y)
solver = SeparableODESolver()

# With y(0) = 0.1 (10% initial population)
solution = solver.solve(rhs, y, x, initial=(0, 0.1))
print(solution)  # y = 1/(1 + 9*e^(-x))

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode');

const x = symbol('x');
const y = symbol('y');

// dy/dx = y(1-y) (logistic growth)
const rhs = expr`${y} * (1 - ${y})`;
const solver = new SeparableODESolver();

// With y(0) = 0.1 (10% initial population)
const solution = solver.solve(rhs, y, x, { initial: [0, 0.1] });
console.log(solution); // y = 1/(1 + 9*e^(-x))

```
</details>


**Expected Output:**
```
y = 1/(1 + 9*e^(-x))
```



### Newton's Law of Cooling

Temperature change follows dT/dt = k(T - T_ambient). For T_ambient = 20°C,
this becomes dT/dt = k(T - 20). Separating: dT/(T-20) = k*dt.
Integrating: ln|T-20| = kt + C, so T = 20 + Ce^(kt). If T(0) = 100°C and
k = -0.05, then T = 20 + 80*e^(-0.05t). This models coffee cooling,
forensic time-of-death estimation, and HVAC systems.


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook_core::{symbol, expr};
use mathhook_core::ode::SeparableODESolver;

let t = symbol!(t);
let T = symbol!(T);
let k = expr!(-0.05);

// dT/dt = k(T - 20)
let rhs = expr!(k * (T - 20));
let solver = SeparableODESolver::new();

// T(0) = 100 (coffee starts at 100°C)
let ic = (expr!(0), expr!(100));
let solution = solver.solve(&rhs, &T, &t, Some(ic))?;
println!("Temperature: {}", solution); // T = 20 + 80*e^(-0.05t)

// Evaluate at t = 10 minutes
let temp_at_10 = solution.evaluate(&t, &expr!(10))?;
println!("After 10 min: {:.1}°C", temp_at_10);

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr
from mathhook.ode import SeparableODESolver

t = symbol('t')
T = symbol('T')
k = -0.05

# dT/dt = k(T - 20)
rhs = k * (T - 20)
solver = SeparableODESolver()

# T(0) = 100 (coffee starts at 100°C)
solution = solver.solve(rhs, T, t, initial=(0, 100))
print(f"Temperature: {solution}")  # T = 20 + 80*e^(-0.05t)

# Evaluate at t = 10 minutes
temp_at_10 = solution.subs(t, 10)
print(f"After 10 min: {temp_at_10:.1f}°C")

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode');

const t = symbol('t');
const T = symbol('T');
const k = -0.05;

// dT/dt = k(T - 20)
const rhs = expr`${k} * (${T} - 20)`;
const solver = new SeparableODESolver();

// T(0) = 100 (coffee starts at 100°C)
const solution = solver.solve(rhs, T, t, { initial: [0, 100] });
console.log(`Temperature: ${solution}`); // T = 20 + 80*e^(-0.05t)

// Evaluate at t = 10 minutes
const tempAt10 = solution.subs(t, 10);
console.log(`After 10 min: ${tempAt10.toFixed(1)}°C`);

```
</details>


**Expected Output:**
```
T = 20 + 80*e^(-0.05t)
```





## Performance

**Time Complexity**: O(n log n) to O(n^2) where n is integration complexity


## API Reference

- **Rust**: `mathhook_core::ode`
- **Python**: `mathhook.ode.separable`
- **JavaScript**: `mathhook.ode.separable`


## See Also


- [ode.first_order.linear](../ode/first_order/linear.md)

- [ode.first_order.exact](../ode/first_order/exact.md)

- [ode.numerical.runge_kutta](../ode/numerical/runge_kutta.md)

- [calculus.integral](../calculus/integral.md)

- [algebra.solve](../algebra/solve.md)


