---









---

# Expansion and Factoring

> **Topic**: `operations.expansion-factoring`

Transform expressions between expanded and factored forms for easier manipulation and analysis.



## Mathematical Definition

$$**Distributive Law:**
$$a(b + c) = ab + ac$$

**Binomial Expansion:**
$$(x + y)^n = \sum_{k=0}^{n} \binom{n}{k} x^{n-k} y^k$$

For small powers:
- $$(x + y)^2 = x^2 + 2xy + y^2$$
- $$(x + y)^3 = x^3 + 3x^2y + 3xy^2 + y^3$$
- $$(x - y)^2 = x^2 - 2xy + y^2$$

**Special Products:**
- **Difference of Squares:** $(x + y)(x - y) = x^2 - y^2$
- **Perfect Square Trinomial:** $(x + y)^2 = x^2 + 2xy + y^2$

**Noncommutative Expansion:**
For matrices (noncommutative):
$$(A + B)^2 = A^2 + AB + BA + B^2 \quad \text{(4 terms, cannot combine } AB \text{ and } BA\text{)}$$
$$




## Examples


### Simple Products

Expand products of sums

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Expand 2(x + 3)
let expr1 = expr!(2 * (x + 3));
let expanded1 = expr1.expand();
// Result: 2x + 6

// Expand (x + 1)(x + 2)
let expr2 = expr!((x + 1) * (x + 2));
let expanded2 = expr2.expand();
// Result: x² + 3x + 2

// Expand (x + y)(x - y) - difference of squares
let y = symbol!(y);
let expr3 = expr!((x + y) * (x - y));
let expanded3 = expr3.expand();
// Result: x² - y²

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol

x = symbol('x')

# Expand 2(x + 3)
expr1 = 2 * (x + 3)
expanded1 = expr1.expand()
# Result: 2*x + 6

# Expand (x + 1)(x + 2)
expr2 = (x + 1) * (x + 2)
expanded2 = expr2.expand()
# Result: x**2 + 3*x + 2

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');

// Expand 2(x + 3)
const expr1 = x.add(3).mul(2);
const expanded1 = expr1.expand();
// Result: 2*x + 6

```
</details>




### Power Expansion

Expand expressions raised to integer powers

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Expand (x + 1)^2
let expr1 = expr!((x + 1) ^ 2);
let expanded1 = expr1.expand();
// Result: x² + 2x + 1

// Expand (x + y)^3
let expr2 = expr!((x + y) ^ 3);
let expanded2 = expr2.expand();
// Result: x³ + 3x²y + 3xy² + y³

// Expand (x - 2)^4
let expr3 = expr!((x - 2) ^ 4);
let expanded3 = expr3.expand();
// Result: x⁴ - 8x³ + 24x² - 32x + 16

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol

x = symbol('x')
y = symbol('y')

# Expand (x + 1)^2
expr1 = (x + 1)**2
expanded1 = expr1.expand()
# Result: x**2 + 2*x + 1

# Expand (x + y)^3
expr2 = (x + y)**3
expanded2 = expr2.expand()
# Result: x**3 + 3*x**2*y + 3*x*y**2 + y**3

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');
const y = symbol('y');

// Expand (x + 1)^2
const expr1 = x.add(1).pow(2);
const expanded1 = expr1.expand();
// Result: x^2 + 2*x + 1

```
</details>




### Noncommutative Matrix Expansion

For matrices, order matters - (A+B)² has 4 terms

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

// Create matrix symbols
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let C = symbol!(C; matrix);

// Expand (A + B)^2 - preserves noncommutativity
let expr = expr!((A + B) ^ 2);
let expanded = expr.expand();
// Result: A² + AB + BA + B²   (4 terms, NOT A² + 2AB + B²)

// Expand (A + B)(C)
let expr2 = expr!((A + B) * C);
let expanded2 = expr2.expand();
// Result: AC + BC   (order preserved: A*C first, then B*C)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol

# Create matrix symbols
A = symbol('A', matrix=True)
B = symbol('B', matrix=True)
C = symbol('C', matrix=True)

# Expand (A + B)^2 - preserves noncommutativity
expr = (A + B)**2
expanded = expr.expand()
# Result: A**2 + A*B + B*A + B**2   (4 terms, NOT A**2 + 2*A*B + B**2)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook');

// Create matrix symbols
const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });

// Expand (A + B)^2 - preserves noncommutativity
const expr = A.add(B).pow(2);
const expanded = expr.expand();
// Result: A^2 + AB + BA + B^2   (4 terms)

```
</details>






## Performance

**Time Complexity**: O(2^n) terms for (a+b)^n noncommutative, O(n) for commutative


## API Reference

- **Rust**: `mathhook_core::expand::Expand`
- **Python**: `mathhook.expand`
- **JavaScript**: `mathhook.expand`


## See Also


- [operations.simplification](../operations/simplification.md)

- [operations.differentiation](../operations/differentiation.md)

- [operations.integration](../operations/integration.md)

- [features.noncommutative](../features/noncommutative.md)


