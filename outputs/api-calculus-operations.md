---









---

# Symbolic Calculus: Differentiation and Integration

> **Topic**: `api.calculus.operations`

Symbolic differentiation and integration using automatic differentiation rules,
integration strategies, and the Risch algorithm. Supports chain rule, product rule,
quotient rule, and comprehensive integration techniques from table lookup to complete
Risch algorithm for elementary functions.





# Symbolic Calculus Operations

## Differentiation

### Overview
Symbolic differentiation using automatic differentiation with:
- **Power Rule**: $\frac{d}{dx} x^n = nx^{n-1}$
- **Product Rule**: $\frac{d}{dx}(fg) = f'g + fg'$
- **Quotient Rule**: $\frac{d}{dx}\frac{f}{g} = \frac{f'g - fg'}{g^2}$
- **Chain Rule**: $\frac{d}{dx}f(g(x)) = f'(g(x)) \cdot g'(x)$

### Supported Functions
- Trigonometric: sin, cos, tan, cot, sec, csc
- Inverse trig: arcsin, arctan, arccos
- Exponential/Logarithmic: exp, log, ln
- Hyperbolic: sinh, cosh, tanh

## Integration

### 8-Layer Strategy Architecture
1. **Table Lookup**: O(1) hash lookup for 500+ common patterns
2. **Rational Functions**: Partial fraction decomposition
3. **Function Registry**: Built-in antiderivatives
4. **Integration by Parts**: LIATE heuristic
5. **U-Substitution**: Chain rule patterns
6. **Trigonometric**: Trig identities and reduction
7. **Risch Algorithm**: Complete algorithm for elementary functions
8. **Symbolic Fallback**: Return unevaluated integral

### Coverage and Performance
- **Coverage**: 93-95% of elementary integrals
- **Fast Path**: Layers 1-4 (90% of integrals, <1ms)
- **Medium Path**: Layers 5-6 (5-8%, 1-10ms)
- **Slow Path**: Layer 7 (2-5%, 10-2000ms)












## Examples


### Basic Differentiation

Compute derivatives using power rule and chain rule

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 3);

// First derivative: 3x^2
let derivative = expr.derivative(&x, 1);

// Second derivative: 6x
let second_derivative = expr.derivative(&x, 2);

// Complex function with chain rule
let expr = expr!(sin(x ^ 2));
let deriv = expr.derivative(&x, 1);
// Result: cos(x^2) * 2x

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, derivative

x = symbol('x')
expr = x**3

# First derivative: 3x^2
df = derivative(expr, x)

# Second derivative: 6x
d2f = derivative(expr, x, order=2)

# Complex function with chain rule
from mathhook import sin
expr = sin(x**2)
deriv = derivative(expr, x)
# Result: cos(x^2) * 2x

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { symbol, parse, derivative } from 'mathhook';

const x = symbol('x');
const expr = parse('x^3');

// First derivative: 3x^2
const df = derivative(expr, x);

// Second derivative: 6x
const d2f = derivative(expr, x, { order: 2 });

// Complex function with chain rule
const expr2 = parse('sin(x^2)');
const deriv = derivative(expr2, x);
// Result: cos(x^2) * 2x

```
</details>




### Product and Quotient Rules

Differentiate products and quotients

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Product rule: d/dx(x^2 * x^3) = 2x * x^3 + x^2 * 3x^2 = 5x^4
let f = expr!(x ^ 2);
let g = expr!(x ^ 3);
let product = expr!(mul: f, g);
let deriv = product.derivative(&x, 1);
// Result: 5*x^4

// Quotient rule: d/dx(x^2 / (x+1))
let numerator = expr!(x ^ 2);
let denominator = expr!(x + 1);
let quotient = expr!(div: numerator, denominator);
let deriv = quotient.derivative(&x, 1);
// Result: (2*x*(x+1) - x^2*1) / (x+1)^2

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, derivative

x = symbol('x')

# Product rule
f = x**2
g = x**3
product = f * g
deriv = derivative(product, x)
# Result: 5*x^4

# Quotient rule
numerator = x**2
denominator = x + 1
quotient = numerator / denominator
deriv = derivative(quotient, x)
# Result: (2*x*(x+1) - x^2) / (x+1)^2

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { symbol, parse, derivative } from 'mathhook';

const x = symbol('x');

// Product rule
const product = parse('x^2 * x^3');
const deriv1 = derivative(product, x);
// Result: 5*x^4

// Quotient rule
const quotient = parse('x^2 / (x + 1)');
const deriv2 = derivative(quotient, x);
// Result: (2*x*(x+1) - x^2) / (x+1)^2

```
</details>




### Partial Derivatives (Multivariable)

Compute partial derivatives with respect to each variable

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
import { symbol, parse, derivative } from 'mathhook';

const x = symbol('x');
const y = symbol('y');
const expr = parse('x^2 * y');

// Partial derivative with respect to x
const df_dx = derivative(expr, x);
// Result: 2*x*y

// Partial derivative with respect to y
const df_dy = derivative(expr, y);
// Result: x^2

```
</details>




### Basic Integration

Symbolic integration using layered strategy

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook::calculus::integrals::Integration;

let x = symbol!(x);

// Simple polynomial (Layer 1: Table Lookup)
let expr = expr!(x ^ 2);
let result = expr.integrate(x.clone());
// Result: x^3/3 + C

// Rational function (Layer 2: Partial fractions)
let expr = expr!(1 / (x + 1));
let result = expr.integrate(x.clone());
// Result: ln|x+1| + C

// Trigonometric (Layer 3: Function registry)
let expr = expr!(sin(x));
let result = expr.integrate(x.clone());
// Result: -cos(x) + C

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, integrate

x = symbol('x')

# Simple polynomial
expr = x**2
result = integrate(expr, x)
# Result: x^3/3 + C

# Rational function
expr = 1 / (x + 1)
result = integrate(expr, x)
# Result: ln|x+1| + C

# Trigonometric
from mathhook import sin
expr = sin(x)
result = integrate(expr, x)
# Result: -cos(x) + C

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { symbol, parse, integrate } from 'mathhook';

const x = symbol('x');

// Simple polynomial
const expr = parse('x^2');
const result = integrate(expr, x);
// Result: x^3/3 + C

// Rational function
const expr2 = parse('1 / (x + 1)');
const result2 = integrate(expr2, x);
// Result: ln|x+1| + C

// Trigonometric
const expr3 = parse('sin(x)');
const result3 = integrate(expr3, x);
// Result: -cos(x) + C

```
</details>




### Integration by Parts and Substitution

Advanced integration techniques

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook::calculus::integrals::Integration;

let x = symbol!(x);

// Integration by parts: ∫x*e^x dx
let expr = expr!(x * exp(x));
let result = expr.integrate(x.clone());
// Result: e^x(x-1) + C

// U-substitution: ∫2x*sin(x^2) dx
let expr = expr!(2 * x * sin(x ^ 2));
let result = expr.integrate(x.clone());
// Result: -cos(x^2) + C

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, integrate, exp, sin

x = symbol('x')

# Integration by parts
expr = x * exp(x)
result = integrate(expr, x)
# Result: e^x(x-1) + C

# U-substitution
expr = 2 * x * sin(x**2)
result = integrate(expr, x)
# Result: -cos(x^2) + C

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { symbol, parse, integrate } from 'mathhook';

const x = symbol('x');

// Integration by parts
const expr = parse('x * exp(x)');
const result = integrate(expr, x);
// Result: e^x(x-1) + C

// U-substitution
const expr2 = parse('2 * x * sin(x^2)');
const result2 = integrate(expr2, x);
// Result: -cos(x^2) + C

```
</details>




### Real-World Application: Velocity and Acceleration

Physics application of derivatives

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let t = symbol!(t);
let position = expr!((t ^ 3) - (6 * (t ^ 2)) + (9 * t));

let velocity = position.derivative(&t, 1);
// v(t) = 3t^2 - 12t + 9

let acceleration = position.derivative(&t, 2);
// a(t) = 6t - 12

// Find when velocity is zero (critical points)
// Solve: 3t^2 - 12t + 9 = 0

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, derivative

t = symbol('t')
position = t**3 - 6*t**2 + 9*t

velocity = derivative(position, t)
# v(t) = 3t^2 - 12t + 9

acceleration = derivative(position, t, order=2)
# a(t) = 6t - 12

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { symbol, parse, derivative } from 'mathhook';

const t = symbol('t');
const position = parse('t^3 - 6*t^2 + 9*t');

const velocity = derivative(position, t);
// v(t) = 3t^2 - 12t + 9

const acceleration = derivative(position, t, { order: 2 });
// a(t) = 6t - 12

```
</details>







## API Reference

- **Rust**: `mathhook_core::calculus::{derivative, integrate}`
- **Python**: `mathhook.calculus.{derivative, integrate}`
- **JavaScript**: `mathhook.calculus.{derivative, integrate}`


## See Also


- [api.operations.limits](../api/operations/limits.md)

- [api.operations.series](../api/operations/series.md)

- [api.operations.solving](../api/operations/solving.md)

- [api.advanced.differential_equations](../api/advanced/differential_equations.md)

- [api.educational.step_by_step](../api/educational/step_by_step.md)


