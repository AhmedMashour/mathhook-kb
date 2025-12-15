---









---

# Separable ODEs

> **Topic**: `ode.separable`

Separable ODEs are the most important and frequently encountered class of first-order differential
equations. MathHook provides a robust solver that handles both general and particular solutions
with automatic variable separation and symbolic integration.



## Mathematical Definition

$$A first-order ODE $$\frac{dy}{dx} = f(x,y)$$ is **separable** if it can be written as:

$$\frac{dy}{dx} = g(x) \cdot h(y)$$

where $g(x)$ is a function of **only** $x$ and $h(y)$ is a function of **only** $y$.
$$



# Separable ODEs

**Coverage:** ~30% of first-order ODE problems
**Priority:** Highest-priority solver in classification chain
**Complexity:** O(n) where n is integration complexity

Separable ODEs are the most important and frequently encountered class of first-order differential equations. MathHook provides a robust solver that handles both general and particular solutions with automatic variable separation and symbolic integration.

## Mathematical Background

### What is a Separable ODE?

A first-order ODE $\frac{dy}{dx} = f(x,y)$ is **separable** if it can be written as:

$$\frac{dy}{dx} = g(x) \cdot h(y)$$

where:
- $g(x)$ is a function of **only** $x$ (the independent variable)
- $h(y)$ is a function of **only** $y$ (the dependent variable)

**Key insight:** The right-hand side **factors** into a product of two single-variable functions.

### Solution Method

**Algorithm:**

1. **Separate variables:** Rewrite as $\frac{1}{h(y)} dy = g(x) dx$
2. **Integrate both sides:** $\int \frac{1}{h(y)} dy = \int g(x) dx + C$
3. **Solve for $y$ (if possible):** Obtain explicit or implicit solution
4. **Apply initial condition (if given):** Determine constant $C$

**Mathematical justification:**

Starting with $\frac{dy}{dx} = g(x)h(y)$, we multiply both sides by $\frac{dx}{h(y)}$:

$$\frac{dy}{h(y)} = g(x) dx$$

This is valid because we're treating $dy$ and $dx$ as differentials. Integrating both sides gives the general solution.

### Why This Method Works

The separation of variables method exploits the **multiplicative structure** of the equation. By dividing by $h(y)$, we isolate all $y$-dependence on one side and all $x$-dependence on the other. Since both sides must be equal, their integrals must also be equal (up to a constant).

**Implicit vs Explicit Solutions:**

- **Explicit:** $y = F(x, C)$ (can solve for $y$ directly)
- **Implicit:** $G(x, y) = C$ (cannot solve for $y$ algebraically)












## Examples


### Simple Linear ODE

Solve dy/dx = x

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let solution = solver.solve(&expr!(x), &y, &x, None)?;
// Result: y = x²/2 + C

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

solution = solver.solve('x', y, x, None)
# Result: y = x²/2 + C

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const solution = solver.solve('x', y, x, null);
// Result: y = x²/2 + C

```
</details>




### Exponential Growth

Solve dy/dx = y (exponential growth/decay model)

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let solution = solver.solve(&expr!(y), &y, &x, None)?;
// Result: y = Ce^x

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

solution = solver.solve('y', y, x, None)
# Result: y = Ce^x

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const solution = solver.solve('y', y, x, null);
// Result: y = Ce^x

```
</details>




### Product Form

Solve dy/dx = xy (nonlinear growth model)

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let solution = solver.solve(&expr!(x * y), &y, &x, None)?;
// Result: y = Ce^(x²/2)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

solution = solver.solve('x*y', y, x, None)
# Result: y = Ce^(x²/2)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const solution = solver.solve('x*y', y, x, null);
// Result: y = Ce^(x²/2)

```
</details>




### Initial Value Problem

Solve dy/dx = x with y(0) = 1

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let ic = Some((expr!(0), expr!(1))); // y(0) = 1
let solution = solver.solve(&expr!(x), &y, &x, ic)?;
// Result: y = x²/2 + 1

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

ic = (expr('0'), expr('1'))  # y(0) = 1
solution = solver.solve('x', y, x, ic)
# Result: y = x²/2 + 1

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const ic = [expr('0'), expr('1')];  // y(0) = 1
const solution = solver.solve('x', y, x, ic);
// Result: y = x²/2 + 1

```
</details>




### Rational Function

Solve dy/dx = x/y

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let rhs = expr!(x / y);
let solution = solver.solve(&rhs, &y, &x, None)?;
// Result: y² - x² = C (implicit) or y = ±√(x² + C) (explicit)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

solution = solver.solve('x/y', y, x, None)
# Result: y² - x² = C

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const solution = solver.solve('x/y', y, x, null);
// Result: y² - x² = C

```
</details>






## Performance

**Time Complexity**: O(n) where n = integration complexity


## API Reference

- **Rust**: `mathhook_core::ode::first_order::separable::SeparableODESolver`
- **Python**: `mathhook.ode.first_order.separable.SeparableODESolver`
- **JavaScript**: `mathhook.ode.firstOrder.separable.SeparableODESolver`


## See Also


- [calculus.integration](../calculus/integration.md)

- [ode.linear](../ode/linear.md)

- [ode.exact](../ode/exact.md)

- [ode.solver](../ode/solver.md)


