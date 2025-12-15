---









---

# Symbolic Differentiation

> **Topic**: `operations.differentiation`

Symbolic differentiation in MathHook uses automatic differentiation with the chain rule, product rule, quotient rule, and function-specific derivative rules.



## Mathematical Definition

$$**Power Rule:**
$$\frac{d}{dx} x^n = n x^{n-1}$$

**Product Rule:**
$$\frac{d}{dx} [f(x) \cdot g(x)] = f'(x) \cdot g(x) + f(x) \cdot g'(x)$$

**Quotient Rule:**
$$\frac{d}{dx} \frac{f(x)}{g(x)} = \frac{f'(x) \cdot g(x) - f(x) \cdot g'(x)}{[g(x)]^2}$$

**Chain Rule:**
$$\frac{d}{dx} f(g(x)) = f'(g(x)) \cdot g'(x)$$

**Trigonometric Derivatives:**
- $\frac{d}{dx}\sin(x) = \cos(x)$
- $\frac{d}{dx}\cos(x) = -\sin(x)$
- $\frac{d}{dx}\tan(x) = \sec^2(x)$

**Exponential and Logarithmic:**
- $\frac{d}{dx}e^x = e^x$
- $\frac{d}{dx}\ln(x) = \frac{1}{x}$
$$




## Examples


### Power Rule

d/dx(x^n) = n*x^(n-1)

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 5);
let deriv = expr.derivative(&x, 1);
// Result: 5 * x^4

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, derivative

x = symbol('x')
expr = x**5
deriv = derivative(expr, x)
# Result: 5 * x^4

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, derivative } = require('mathhook');

const x = symbol('x');
const expr = x.pow(5);
const deriv = derivative(expr, x);
// Result: 5 * x^4

```
</details>




### Product Rule

d/dx(f·g) = f'·g + f·g'

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let f = expr!(x ^ 2);
let g = expr!(x ^ 3);
let product = expr!(mul: f, g);  // x^2 * x^3

let deriv = product.derivative(&x, 1);
// Result: 2*x * x^3 + x^2 * 3*x^2 = 5*x^4

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, derivative

x = symbol('x')
f = x**2
g = x**3
product = f * g

deriv = derivative(product, x)
# Result: 5*x^4

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, derivative } = require('mathhook');

const x = symbol('x');
const product = x.pow(2).mul(x.pow(3));
const deriv = derivative(product, x);
// Result: 5*x^4

```
</details>




### Chain Rule

d/dx(f(g(x))) = f'(g(x))·g'(x)

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let inner = expr!(x ^ 2);
let outer = expr!(sin(inner));  // sin(x^2)

let deriv = outer.derivative(&x, 1);
// Result: cos(x^2) * 2*x

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, derivative, sin

x = symbol('x')
inner = x**2
outer = sin(inner)  # sin(x^2)

deriv = derivative(outer, x)
# Result: cos(x^2) * 2*x

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, derivative, parse } = require('mathhook');

const x = symbol('x');
const expr = parse('sin(x^2)');
const deriv = derivative(expr, x);
// Result: cos(x^2) * 2*x

```
</details>




### Partial Derivatives

Multivariable differentiation

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);
let expr = expr!((x ^ 2) * y);

// Partial derivative with respect to x
let df_dx = expr.derivative(&x, 1);
// Result: 2*x*y

// Partial derivative with respect to y
let df_dy = expr.derivative(&y, 1);
// Result: x^2

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, derivative

x = symbol('x')
y = symbol('y')
expr = x**2 * y

# Partial derivative with respect to x
df_dx = derivative(expr, x)
# Result: 2*x*y

# Partial derivative with respect to y
df_dy = derivative(expr, y)
# Result: x^2

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, derivative } = require('mathhook');

const x = symbol('x');
const y = symbol('y');
const expr = x.pow(2).mul(y);

// Partial derivative with respect to x
const df_dx = derivative(expr, x);
// Result: 2*x*y

// Partial derivative with respect to y
const df_dy = derivative(expr, y);
// Result: x^2

```
</details>




### Higher-Order Derivatives

Second, third, or nth order derivatives

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 4);

// First derivative: 4*x^3
let first = expr.derivative(&x, 1);

// Second derivative: 12*x^2
let second = expr.derivative(&x, 2);

// Third derivative: 24*x
let third = expr.derivative(&x, 3);

// Fourth derivative: 24
let fourth = expr.derivative(&x, 4);

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, derivative

x = symbol('x')
expr = x**4

# First derivative: 4*x^3
first = derivative(expr, x, order=1)

# Second derivative: 12*x^2
second = derivative(expr, x, order=2)

# Third derivative: 24*x
third = derivative(expr, x, order=3)

# Fourth derivative: 24
fourth = derivative(expr, x, order=4)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, derivative } = require('mathhook');

const x = symbol('x');
const expr = x.pow(4);

// First derivative: 4*x^3
const first = derivative(expr, x, { order: 1 });

// Second derivative: 12*x^2
const second = derivative(expr, x, { order: 2 });

```
</details>






## Performance

**Time Complexity**: O(n) where n = expression tree size


## API Reference

- **Rust**: `mathhook_core::calculus::derivatives::Derivative`
- **Python**: `mathhook.derivative`
- **JavaScript**: `mathhook.derivative`


## See Also


- [operations.integration](../operations/integration.md)

- [operations.limits](../operations/limits.md)

- [operations.series](../operations/series.md)


