---









---

# Limits

> **Topic**: `operations.limits`

Compute limits of expressions as variables approach values, infinity, or points of discontinuity.



## Mathematical Definition

**Epsilon-Delta Definition (ε-δ):**
$$\lim_{x \to a} f(x) = L$$
means: For every $\varepsilon > 0$, there exists $\delta > 0$ such that:
$$0 < |x - a| < \delta \implies |f(x) - L| < \varepsilon$$

**Limit Laws:**
1. **Sum/Difference:** $\lim_{x \to a} [f(x) \pm g(x)] = \lim_{x \to a} f(x) \pm \lim_{x \to a} g(x)$
2. **Product:** $\lim_{x \to a} [f(x) \cdot g(x)] = \lim_{x \to a} f(x) \cdot \lim_{x \to a} g(x)$
3. **Quotient:** $\lim_{x \to a} \frac{f(x)}{g(x)} = \frac{\lim_{x \to a} f(x)}{\lim_{x \to a} g(x)}$ (if denominator $\neq 0$)

**L'Hôpital's Rule (0/0 or ∞/∞):**
$$\lim_{x \to a} \frac{f(x)}{g(x)} = \lim_{x \to a} \frac{f'(x)}{g'(x)}$$
(if the limit on the right exists)





## Examples


### Direct Substitution

For continuous functions, substitute directly


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Limit: lim(x→2) x² = 4
let expr1 = expr!(x ^ 2);
let limit1 = expr1.limit(&x, &expr!(2));
// Result: 4

// Limit: lim(x→π) sin(x) = 0
let expr2 = expr!(sin(x));
let limit2 = expr2.limit(&x, &Expression::pi());
// Result: 0

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, limit, pi

x = symbol('x')

# Limit: lim(x→2) x² = 4
expr1 = x**2
limit1 = limit(expr1, x, 2)
# Result: 4

# Limit: lim(x→π) sin(x) = 0
expr2 = sin(x)
limit2 = limit(expr2, x, pi)
# Result: 0

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, limit } = require('mathhook');

const x = symbol('x');

// Limit: lim(x→2) x² = 4
const expr1 = x.pow(2);
const limit1 = limit(expr1, x, 2);
// Result: 4

```
</details>





### L'Hôpital's Rule (0/0 Form)

Use derivatives to resolve indeterminate forms


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Limit: lim(x→0) sin(x)/x = 1 (0/0 form)
// Apply L'Hôpital: lim(x→0) cos(x)/1 = 1
let expr = expr!(sin(x) / x);
let limit = expr.limit(&x, &expr!(0));
// Result: 1

// Limit: lim(x→0) (1 - cos(x))/x² = 1/2 (0/0 form)
let expr2 = expr!((1 - cos(x)) / (x ^ 2));
let limit2 = expr2.limit(&x, &expr!(0));
// Result: 1/2

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, limit, sin, cos

x = symbol('x')

# Limit: lim(x→0) sin(x)/x = 1
expr = sin(x)/x
result = limit(expr, x, 0)
# Result: 1

# Limit: lim(x→0) (1 - cos(x))/x²
expr2 = (1 - cos(x))/x**2
result2 = limit(expr2, x, 0)
# Result: 1/2

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, limit, parse } = require('mathhook');

const x = symbol('x');

// Limit: lim(x→0) sin(x)/x
const expr = parse('sin(x)/x');
const result = limit(expr, x, 0);
// Result: 1

```
</details>





### Limits at Infinity

Behavior as x approaches ±∞


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook::core::Expression;

let x = symbol!(x);

// Limit: lim(x→∞) (2x² + 1)/(x² + 3) = 2
let expr1 = expr!((2 * (x ^ 2) + 1) / ((x ^ 2) + 3));
let limit1 = expr1.limit(&x, &Expression::infinity());
// Result: 2

// Limit: lim(x→∞) (x + 1)/(x² + 1) = 0
let expr2 = expr!((x + 1) / ((x ^ 2) + 1));
let limit2 = expr2.limit(&x, &Expression::infinity());
// Result: 0

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, limit, oo

x = symbol('x')

# Limit: lim(x→∞) (2x² + 1)/(x² + 3)
expr1 = (2*x**2 + 1)/(x**2 + 3)
limit1 = limit(expr1, x, oo)
# Result: 2

# Limit: lim(x→∞) (x + 1)/(x² + 1)
expr2 = (x + 1)/(x**2 + 1)
limit2 = limit(expr2, x, oo)
# Result: 0

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, limit, Infinity } = require('mathhook');

const x = symbol('x');

// Limit: lim(x→∞) (2x² + 1)/(x² + 3)
const expr1 = parse('(2*x^2 + 1)/(x^2 + 3)');
const limit1 = limit(expr1, x, Infinity);
// Result: 2

```
</details>








## API Reference

- **Rust**: `mathhook_core::calculus::limits::Limit`
- **Python**: `mathhook.limit`
- **JavaScript**: `mathhook.limit`


## See Also


- [operations.differentiation](../operations/differentiation.md)

- [operations.series](../operations/series.md)

- [operations.integration](../operations/integration.md)


