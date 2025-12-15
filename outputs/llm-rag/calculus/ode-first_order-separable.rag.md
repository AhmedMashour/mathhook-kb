# Separable Differential Equations

Solve first-order ordinary differential equations that can be separated into functions of x and y independently

---
chunk_id: ode_first_order_separable::0
topic: ode.first_order.separable
title: Simple Exponential Growth
priority: high
keywords:
  - separable
  - simple exponential growth
languages: [rust, python, javascript]
chunk: 1/3
description: "Learn to solve separable differential equations symbolically with MathHook. Step-by-step guide with Newton's cooling, exponential growth, and logistic models. Fast, accurate ODE solving for Rust, Python, JavaScript."
seo_keywords:
  - separable differential equations
  - ODE solver
  - separation of variables
  - first order ODE
  - symbolic ODE solving
canonical_url: "https://docs.mathhook.org/ode/first-order/separable"
---

## Simple Exponential Growth

The simplest separable ODE models exponential growth: dy/dx = y.
This can be separated as dy/y = dx, integrating gives ln|y| = x + C,
which simplifies to y = Ce^x. This models population growth, radioactive
decay (with negative constant), and compound interest.


### Rust

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

### Python

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

### JavaScript

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

### Expected Output

```
y = 3*e^x
```



---
chunk_id: ode_first_order_separable::1
topic: ode.first_order.separable
title: Logistic Growth Model
priority: high
keywords:
  - separable
  - logistic growth model
languages: [rust, python, javascript]
chunk: 2/3
description: "Learn to solve separable differential equations symbolically with MathHook. Step-by-step guide with Newton's cooling, exponential growth, and logistic models. Fast, accurate ODE solving for Rust, Python, JavaScript."
seo_keywords:
  - separable differential equations
  - ODE solver
  - separation of variables
  - first order ODE
  - symbolic ODE solving
canonical_url: "https://docs.mathhook.org/ode/first-order/separable"
---

## Logistic Growth Model

The logistic equation dy/dx = y(1-y) models population growth with carrying
capacity. Separating variables: dy/(y(1-y)) = dx. Using partial fractions:
(1/y + 1/(1-y))dy = dx. Integrating: ln|y/(1-y)| = x + C.
Solving for y: y = 1/(1 + Ce^(-x)). This S-curve is fundamental in ecology,
epidemiology, and marketing.


### Rust

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

### Python

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

### JavaScript

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

### Expected Output

```
y = 1/(1 + 9*e^(-x))
```



---
chunk_id: ode_first_order_separable::2
topic: ode.first_order.separable
title: Newton's Law of Cooling
priority: high
keywords:
  - separable
  - newton's law of cooling
languages: [rust, python, javascript]
chunk: 3/3
description: "Learn to solve separable differential equations symbolically with MathHook. Step-by-step guide with Newton's cooling, exponential growth, and logistic models. Fast, accurate ODE solving for Rust, Python, JavaScript."
seo_keywords:
  - separable differential equations
  - ODE solver
  - separation of variables
  - first order ODE
  - symbolic ODE solving
canonical_url: "https://docs.mathhook.org/ode/first-order/separable"
---

## Newton's Law of Cooling

Temperature change follows dT/dt = k(T - T_ambient). For T_ambient = 20°C,
this becomes dT/dt = k(T - 20). Separating: dT/(T-20) = k*dt.
Integrating: ln|T-20| = kt + C, so T = 20 + Ce^(kt). If T(0) = 100°C and
k = -0.05, then T = 20 + 80*e^(-0.05t). This models coffee cooling,
forensic time-of-death estimation, and HVAC systems.


### Rust

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

### Python

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

### JavaScript

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

### Expected Output

```
T = 20 + 80*e^(-0.05t)
```



