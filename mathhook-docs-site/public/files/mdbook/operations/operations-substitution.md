---









---

# Substitution

> **Topic**: `operations.substitution`

Replace variables with values or expressions to evaluate, simplify, or transform expressions.



## Mathematical Definition

**Function Evaluation:**
$$f(a) = f(x)|_{x=a}$$
Substitute $x = a$ into function $f(x)$.

**Composition:**
$$f(g(x)) = f(u)|_{u=g(x)}$$
Substitute $u = g(x)$ into function $f(u)$.

**U-Substitution (Integration):**
$$\int f(g(x)) \cdot g'(x) \, dx = \int f(u) \, du$$
where $u = g(x)$ and $du = g'(x) \, dx$.

**Change of Variables (Multivariable):**
$$\frac{\partial f}{\partial x} = \frac{\partial f}{\partial u} \cdot \frac{\partial u}{\partial x}$$





## Examples


### Single Variable Substitution

Replace variable with number


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Substitute x = 2 into x² + 3x
let expr1 = expr!(x ^ 2 + 3 * x);
let result1 = expr1.substitute(&x, &expr!(2));
// Result: 4 + 6 = 10

// Substitute x = -1 into x³ - 2x + 1
let expr2 = expr!(x ^ 3 - 2 * x + 1);
let result2 = expr2.substitute(&x, &expr!(-1));
// Result: -1 + 2 + 1 = 2

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol

x = symbol('x')

# Substitute x = 2 into x² + 3x
expr1 = x**2 + 3*x
result1 = expr1.subs(x, 2)
# Result: 10

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');

// Substitute x = 2 into x² + 3x
const expr1 = x.pow(2).add(x.mul(3));
const result1 = expr1.substitute(x, 2);
// Result: 10

```
</details>





### Expression Substitution

Replace with another expression


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Substitute x = y + 1 into x² + 3x
let expression = expr!(x ^ 2 + 3 * x);
let substituted = expression.substitute(&x, &expr!(y + 1));
// Result: (y+1)² + 3(y+1)

// Expand for cleaner form
let expanded = substituted.expand();
// Result: y² + 2y + 1 + 3y + 3 = y² + 5y + 4

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol

x = symbol('x')
y = symbol('y')

# Substitute x = y + 1 into x² + 3x
expression = x**2 + 3*x
substituted = expression.subs(x, y + 1)
# Result: (y+1)^2 + 3(y+1)

# Expand for cleaner form
expanded = substituted.expand()
# Result: y^2 + 5*y + 4

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');
const y = symbol('y');

// Substitute x = y + 1 into x² + 3x
const expression = x.pow(2).add(x.mul(3));
const substituted = expression.substitute(x, y.add(1));
// Result: (y+1)^2 + 3(y+1)

```
</details>





### U-Substitution for Integration

Transform difficult integrals


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);
let u = symbol!(u);

// Integrate: ∫ 2x·e^(x²) dx
// Let u = x², then du = 2x dx
let integrand = expr!(2 * x * exp(x ^ 2));

// Manual substitution
let u_expr = expr!(x ^ 2);  // u = x²
let integrand_u = integrand.substitute(&expr!(x ^ 2), &u);
// Result: ∫ e^u du = e^u + C

// Back-substitute: e^(x²) + C
let result = expr!(exp(u)).substitute(&u, &u_expr);
// Result: e^(x²) + C

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, integrate

x = symbol('x')
u = symbol('u')

# Integrate: ∫ 2x·e^(x²) dx
integrand = 2*x*exp(x**2)

# Manual substitution
u_expr = x**2  # u = x²
integrand_u = integrand.subs(x**2, u)
# Result: ∫ e^u du = e^u + C

# Back-substitute: e^(x²) + C
result = exp(u).subs(u, u_expr)
# Result: e^(x²) + C

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');
const u = symbol('u');

// Integrate: ∫ 2x·e^(x²) dx
const integrand = x.mul(2).mul(expr('exp(x^2)'));

// Manual substitution
const uExpr = x.pow(2);  // u = x²
const integrandU = integrand.substitute(x.pow(2), u);
// Result: ∫ e^u du = e^u + C

```
</details>







## Performance

**Time Complexity**: O(n) where n = expression tree size


## API Reference

- **Rust**: `mathhook_core::substitution::Substitute`
- **Python**: `mathhook.substitute`
- **JavaScript**: `mathhook.substitute`


## See Also


- [operations.solving](../operations/solving.md)

- [operations.integration](../operations/integration.md)

- [operations.differentiation](../operations/differentiation.md)

- [operations.simplification](../operations/simplification.md)


