---









---

# Symbolic Integration

> **Topic**: `operations.integration`

MathHook's integration system provides symbolic integration capabilities with an 8-layer strategy architecture from fast heuristics to complete Risch algorithm. Coverage: 93-95% of elementary integrals.



## Mathematical Definition

**Fundamental Theorem of Calculus:**
$$\int_a^b f(x) \, dx = F(b) - F(a)$$
where $F'(x) = f(x)$.

**Integration by Parts:**
$$\int u \, dv = uv - \int v \, du$$

**U-Substitution:**
$$\int f(g(x)) \cdot g'(x) \, dx = \int f(u) \, du$$
where $u = g(x)$ and $du = g'(x) \, dx$.

**Power Rule:**
$$\int x^n \, dx = \frac{x^{n+1}}{n+1} + C \quad (n \neq -1)$$

**Logarithm Special Case:**
$$\int \frac{1}{x} \, dx = \ln|x| + C$$




## 8-Layer Strategy Dispatcher

The integration strategy tries techniques in this exact order:

```
Layer 1: Table Lookup             - O(1) hash lookup for common patterns
Layer 2: Rational Functions       - Partial fraction decomposition
Layer 3: Function Registry        - Built-in function antiderivatives
Layer 4: Integration by Parts     - LIATE heuristic for products
Layer 5: U-Substitution           - Chain rule patterns
Layer 6: Trigonometric            - Trig identities and reduction
Layer 7: Risch Algorithm          - Complete algorithm for elementary functions
Layer 8: Symbolic Fallback        - Return unevaluated integral
```

**Performance Profile:**
- Layers 1-4: Microseconds to milliseconds (fast path, 90% of integrals)
- Layer 5-6: Milliseconds (medium complexity, 5-8%)
- Layer 7: Milliseconds to seconds (hard cases, 2-5%)












## Examples


### Basic Integration (Layer 1: Table Lookup)

Direct table hits for common patterns


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);

// Polynomial: ∫x^3 dx = x^4/4 + C
let poly = expr!(x ^ 3);
let result = poly.integrate(x.clone());
// Result: x^4/4 + C

// Rational: ∫1/(x+1) dx = ln|x+1| + C
let rational = expr!(1 / (x + 1));
let result = rational.integrate(x.clone());
// Result: ln|x+1| + C

// Trigonometric: ∫sin(x) dx = -cos(x) + C
let trig = expr!(sin(x));
let result = trig.integrate(x.clone());
// Result: -cos(x) + C

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, integrate

x = symbol('x')

# Polynomial
poly = x**3
result = integrate(poly, x)
# Result: x**4/4

# Rational
rational = 1/(x+1)
result = integrate(rational, x)
# Result: log(x+1)

# Trigonometric
trig = sin(x)
result = integrate(trig, x)
# Result: -cos(x)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, integrate } = require('mathhook');

const x = symbol('x');

// Polynomial
const poly = x.pow(3);
const result = integrate(poly, x);
// Result: x^4/4

```
</details>





### Integration by Parts (Layer 4: LIATE)

∫u dv = uv - ∫v du using LIATE rule


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);

// ∫x*e^x dx: u = x (algebraic), dv = e^x (exponential)
let expr = expr!(x * exp(x));
let result = expr.integrate(x.clone());
// Result: x*e^x - e^x + C = e^x(x-1) + C

// ∫x*sin(x) dx: u = x (algebraic), dv = sin(x) (trig)
let expr2 = expr!(x * sin(x));
let result2 = expr2.integrate(x.clone());
// Result: -x*cos(x) + sin(x) + C

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, integrate, exp, sin

x = symbol('x')

# ∫x*e^x dx
expr = x * exp(x)
result = integrate(expr, x)
# Result: x*exp(x) - exp(x)

# ∫x*sin(x) dx
expr2 = x * sin(x)
result2 = integrate(expr2, x)
# Result: -x*cos(x) + sin(x)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, integrate, parse } = require('mathhook');

const x = symbol('x');

// ∫x*e^x dx
const expr = parse('x*exp(x)');
const result = integrate(expr, x);
// Result: x*exp(x) - exp(x)

```
</details>





### U-Substitution (Layer 5)

∫f(g(x))*g'(x) dx = ∫f(u) du


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);

// ∫2x*sin(x^2) dx: u = x^2, du = 2x dx
let expr = expr!(2 * x * sin(x ^ 2));
let result = expr.integrate(x.clone());
// Result: -cos(x^2) + C

// ∫2x*e^(x^2) dx: u = x^2, du = 2x dx
let expr2 = expr!(2 * x * exp(x ^ 2));
let result2 = expr2.integrate(x.clone());
// Result: e^(x^2) + C

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, integrate, sin, exp

x = symbol('x')

# ∫2x*sin(x^2) dx
expr = 2*x*sin(x**2)
result = integrate(expr, x)
# Result: -cos(x^2)

# ∫2x*e^(x^2) dx
expr2 = 2*x*exp(x**2)
result2 = integrate(expr2, x)
# Result: exp(x^2)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, integrate, parse } = require('mathhook');

const x = symbol('x');

// ∫2x*sin(x^2) dx
const expr = parse('2*x*sin(x^2)');
const result = integrate(expr, x);
// Result: -cos(x^2)

```
</details>







## Performance

**Time Complexity**: Varies by layer: O(1) for table, O(n^3) for rational functions, polynomial for Risch


## API Reference

- **Rust**: `mathhook_core::calculus::integrals::Integration`
- **Python**: `mathhook.integrate`
- **JavaScript**: `mathhook.integrate`


## See Also


- [operations.differentiation](../operations/differentiation.md)

- [operations.substitution](../operations/substitution.md)

- [operations.limits](../operations/limits.md)


