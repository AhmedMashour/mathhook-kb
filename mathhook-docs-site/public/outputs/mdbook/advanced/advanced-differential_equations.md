---









---

# Differential Equation Solving

> **Topic**: `advanced.differential_equations`

Solve ordinary differential equations (ODEs) and partial differential equations
(PDEs) symbolically in MathHook, with support for initial conditions, boundary
conditions, and various solution methods.



## Mathematical Definition

Ordinary Differential Equation (ODE):
$$F\left(x, y, \frac{dy}{dx}, \frac{d^2y}{dx^2}, \ldots\right) = 0$$

Partial Differential Equation (PDE):
$$F\left(x, y, u, \frac{\partial u}{\partial x}, \frac{\partial u}{\partial y}, \ldots\right) = 0$$





## Examples


### First-Order ODE

Solve dy/dx = 2x with initial condition y(0) = 1


<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);
let y = Function::new("y", vec![x.clone()]);

let ode = expr!(diff(y, x) - 2*x);
let solver = ODESolver::new();

let solution = solver.solve(&ode, &y, Some((&x, expr!(0), expr!(1))));
// Result: y = x^2 + 1

```
</details>



<details>
<summary><b>Python</b></summary>

```python
x = symbol('x')
y = Function('y')(x)

ode = diff(y, x) - 2*x
solution = dsolve(ode, y, ics={y.subs(x, 0): 1})
# Result: y = x**2 + 1

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');
const y = func('y', [x]);

const ode = diff(y, x).sub(mul(2, x));
const solution = ode_solve(ode, y, {x0: 0, y0: 1});
// Result: y = x^2 + 1

```
</details>





### Second-Order Linear ODE

Solve y'' + y = 0 (simple harmonic oscillator)


<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);
let y = Function::new("y", vec![x.clone()]);

let ode = expr!(diff(y, x, 2) + y);
let solution = ode_solver.solve(&ode, &y, None);
// Result: y = C1*cos(x) + C2*sin(x)

```
</details>



<details>
<summary><b>Python</b></summary>

```python
x = symbol('x')
y = Function('y')(x)

ode = diff(y, x, 2) + y
solution = dsolve(ode, y)
# Result: y = C1*cos(x) + C2*sin(x)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');
const y = func('y', [x]);

const ode = diff(y, x, 2).add(y);
const solution = ode_solve(ode, y);
// Result: y = C1*cos(x) + C2*sin(x)

```
</details>








## API Reference

- **Rust**: `mathhook_core::solvers::ode`
- **Python**: `mathhook.solvers.ode`
- **JavaScript**: `mathhook.solvers.ode`


## See Also


- [operations.differentiation](../operations/differentiation.md)

- [operations.integration](../operations/integration.md)

- [operations.solving](../operations/solving.md)

- [advanced.pde](../advanced/pde.md)

- [advanced.system_solving](../advanced/system_solving.md)


