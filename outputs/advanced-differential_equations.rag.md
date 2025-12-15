# Differential Equation Solving

Solve ordinary differential equations (ODEs) and partial differential equations
(PDEs) symbolically in MathHook, with support for initial conditions, boundary
conditions, and various solution methods.


---
chunk_id: advanced_differential_equations::0
topic: advanced.differential_equations
title: First-Order ODE
priority: medium
keywords:
  - differential_equations
  - first-order ode
languages: [rust, python, javascript]
chunk: 1/2
---

## First-Order ODE

Solve dy/dx = 2x with initial condition y(0) = 1

### Rust

```rust
let x = symbol!(x);
let y = Function::new("y", vec![x.clone()]);

let ode = expr!(diff(y, x) - 2*x);
let solver = ODESolver::new();

let solution = solver.solve(&ode, &y, Some((&x, expr!(0), expr!(1))));
// Result: y = x^2 + 1

```

### Python

```python
x = symbol('x')
y = Function('y')(x)

ode = diff(y, x) - 2*x
solution = dsolve(ode, y, ics={y.subs(x, 0): 1})
# Result: y = x**2 + 1

```

### JavaScript

```javascript
const x = symbol('x');
const y = func('y', [x]);

const ode = diff(y, x).sub(mul(2, x));
const solution = ode_solve(ode, y, {x0: 0, y0: 1});
// Result: y = x^2 + 1

```



---
chunk_id: advanced_differential_equations::1
topic: advanced.differential_equations
title: Second-Order Linear ODE
priority: medium
keywords:
  - differential_equations
  - second-order linear ode
languages: [rust, python, javascript]
chunk: 2/2
---

## Second-Order Linear ODE

Solve y'' + y = 0 (simple harmonic oscillator)

### Rust

```rust
let x = symbol!(x);
let y = Function::new("y", vec![x.clone()]);

let ode = expr!(diff(y, x, 2) + y);
let solution = ode_solver.solve(&ode, &y, None);
// Result: y = C1*cos(x) + C2*sin(x)

```

### Python

```python
x = symbol('x')
y = Function('y')(x)

ode = diff(y, x, 2) + y
solution = dsolve(ode, y)
# Result: y = C1*cos(x) + C2*sin(x)

```

### JavaScript

```javascript
const x = symbol('x');
const y = func('y', [x]);

const ode = diff(y, x, 2).add(y);
const solution = ode_solve(ode, y);
// Result: y = C1*cos(x) + C2*sin(x)

```



