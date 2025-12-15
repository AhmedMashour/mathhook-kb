---









---

# Series Expansions

> **Topic**: `operations.series`

Expand functions as infinite series for numerical approximation and analysis.



## Mathematical Definition

$$**Taylor's Theorem:**
If $f(x)$ is infinitely differentiable at $x = a$, then:
$$f(x) = \sum_{n=0}^{\infty} \frac{f^{(n)}(a)}{n!} (x - a)^n$$

Expanded form:
$$f(x) = f(a) + f'(a)(x-a) + \frac{f''(a)}{2!}(x-a)^2 + \frac{f'''(a)}{3!}(x-a)^3 + \cdots$$

**Maclaurin Series (Special Case: a = 0):**
$$f(x) = \sum_{n=0}^{\infty} \frac{f^{(n)}(0)}{n!} x^n$$

**Common Series:**
- $e^x = 1 + x + \frac{x^2}{2!} + \frac{x^3}{3!} + \cdots$
- $\sin(x) = x - \frac{x^3}{3!} + \frac{x^5}{5!} - \cdots$
- $\cos(x) = 1 - \frac{x^2}{2!} + \frac{x^4}{4!} - \cdots$
- $\ln(1+x) = x - \frac{x^2}{2} + \frac{x^3}{3} - \cdots$ for $|x| < 1$

**Radius of Convergence ($R$):**
The series converges for $|x - a| < R$ and may diverge for $|x - a| > R$.
$$




## Examples


### Maclaurin Series (Expansion at x=0)

Standard functions at x = 0

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// exp(x) = 1 + x + x²/2! + x³/3! + ...
let exp_series = expr!(exp(x)).taylor_series(&x, &expr!(0), 5);
// Result: 1 + x + x²/2 + x³/6 + x⁴/24 + x⁵/120

// sin(x) = x - x³/3! + x⁵/5! - ...
let sin_series = expr!(sin(x)).taylor_series(&x, &expr!(0), 7);
// Result: x - x³/6 + x⁵/120 - x⁷/5040

// cos(x) = 1 - x²/2! + x⁴/4! - ...
let cos_series = expr!(cos(x)).taylor_series(&x, &expr!(0), 6);
// Result: 1 - x²/2 + x⁴/24 - x⁶/720

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, series, exp, sin, cos

x = symbol('x')

# exp(x)
exp_series = series(exp(x), x, 0, n=5)
# Result: 1 + x + x^2/2 + x^3/6 + x^4/24 + x^5/120

# sin(x)
sin_series = series(sin(x), x, 0, n=7)
# Result: x - x^3/6 + x^5/120 - x^7/5040

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, series, parse } = require('mathhook');

const x = symbol('x');

// exp(x)
const expSeries = series(parse('exp(x)'), x, 0, { n: 5 });
// Result: 1 + x + x^2/2 + x^3/6 + x^4/24 + x^5/120

```
</details>




### Taylor Series at Arbitrary Points

Expand around any point a

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// sin(x) at x = π/2:
// sin(x) = 1 - (x-π/2)²/2! + (x-π/2)⁴/4! - ...
let sin_at_pi_2 = expr!(sin(x)).taylor_series(&x, &Expression::pi_over_2(), 5);

// exp(x) at x = 1:
// exp(x) = e + e(x-1) + e(x-1)²/2! + ...
let exp_at_1 = expr!(exp(x)).taylor_series(&x, &expr!(1), 5);

// ln(x) at x = 1:
// ln(x) = (x-1) - (x-1)²/2 + (x-1)³/3 - ...
let log_at_1 = expr!(log(x)).taylor_series(&x, &expr!(1), 5);

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, series, sin, exp, log, pi

x = symbol('x')

# sin(x) at x = π/2
sin_at_pi_2 = series(sin(x), x, pi/2, n=5)

# exp(x) at x = 1
exp_at_1 = series(exp(x), x, 1, n=5)

# ln(x) at x = 1
log_at_1 = series(log(x), x, 1, n=5)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, series, parse, pi } = require('mathhook');

const x = symbol('x');

// sin(x) at x = π/2
const sinAtPi2 = series(parse('sin(x)'), x, pi/2, { n: 5 });

```
</details>




### Laurent Series (Negative Powers)

For functions with singularities

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook_core::calculus::SeriesExpansion;

let x = symbol!(x);

// 1/x near x = 0 (pole of order 1)
let pole = expr!(1 / x);
let laurent = pole.laurent_series(&x, &expr!(0), -1, 5);
// Result: x⁻¹ (principal part only)

// exp(1/x) at x = 0:
// exp(1/x) = 1 + 1/x + 1/(2!x²) + ...
let exp_pole = expr!(exp(1 / x));
let laurent2 = exp_pole.laurent_series(&x, &expr!(0), -10, 0);
// Result: 1 + x⁻¹ + x⁻²/2 + x⁻³/6 + ...

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, laurent_series, exp

x = symbol('x')

# 1/x near x = 0
pole = 1/x
laurent = laurent_series(pole, x, 0, -1, 5)
# Result: x^(-1)

# exp(1/x) at x = 0
exp_pole = exp(1/x)
laurent2 = laurent_series(exp_pole, x, 0, -10, 0)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, laurentSeries, parse } = require('mathhook');

const x = symbol('x');

// 1/x near x = 0
const pole = parse('1/x');
const laurent = laurentSeries(pole, x, 0, -1, 5);

```
</details>






## Performance

**Time Complexity**: O(n) for n terms


## API Reference

- **Rust**: `mathhook_core::calculus::SeriesExpansion`
- **Python**: `mathhook.series`
- **JavaScript**: `mathhook.series`


## See Also


- [operations.limits](../operations/limits.md)

- [operations.differentiation](../operations/differentiation.md)

- [operations.integration](../operations/integration.md)


