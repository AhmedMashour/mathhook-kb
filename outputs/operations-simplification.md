---









---

# Symbolic Simplification

> **Topic**: `operations.simplification`

MathHook provides comprehensive symbolic simplification for mathematical expressions, with full support for noncommutative algebra (matrices, operators, quaternions). The simplification system implements canonical forms and mathematical identities to reduce expressions to their simplest equivalent representation.



## Mathematical Definition

$$**Power Rule:**
$$x^a \cdot x^b \rightarrow x^{a+b}$$

**Noncommutative Algebra:**
For noncommutative symbols (matrices, operators):
- $AB \neq BA$ in general
- $(A + B)^2 = A^2 + AB + BA + B^2$ (4 terms, not 3)

**Rational Arithmetic:**
- Exact representation: $\frac{1}{3}$ stays as rational, not float
- Automatic simplification: Reduces fractions to lowest terms
$$




## Examples


### Basic Simplification

Identity elements and constant folding

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Identity elements
let expr = expr!((x + 0) * 1);
let simplified = expr.simplify();
// Result: x

// Constant folding
let expr = expr!(2 + 3);
let simplified = expr.simplify();
// Result: 5

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol

x = symbol('x')

# Identity elements
expr = (x + 0) * 1
simplified = expr.simplify()
# Result: x

# Constant folding
expr = 2 + 3
# Result: 5

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');

// Identity elements
let expr = x.add(0).mul(1);
const simplified = expr.simplify();
// Result: x

```
</details>




### Power Rule

Combine like powers with same base

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Combine like powers
let expr = expr!((x^2) * (x^3));
let simplified = expr.simplify();
// Result: x^5

// Multiple powers
let expr = expr!((x^2) * (x^3) * (x^4));
let simplified = expr.simplify();
// Result: x^9

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol

x = symbol('x')

# Combine like powers
expr = x**2 * x**3
simplified = expr.simplify()
# Result: x^5

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');

// Combine like powers
const expr = x.pow(2).mul(x.pow(3));
const simplified = expr.simplify();
// Result: x^5

```
</details>




### Noncommutative Matrices

Matrix multiplication does NOT commute

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Matrix multiplication does NOT commute
let expr = expr!(A * B);
// Simplification preserves order: A*B ≠ B*A

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol

A = symbol('A', matrix=True)
B = symbol('B', matrix=True)

# Matrix multiplication does NOT commute
expr = A * B
# Simplification preserves order: A*B ≠ B*A

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook');

const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });

// Matrix multiplication does NOT commute
const expr = A.mul(B);
// Simplification preserves order: A*B ≠ B*A

```
</details>






## Performance

**Time Complexity**: O(n) for expression tree size n


## API Reference

- **Rust**: `mathhook_core::simplify::Simplify`
- **Python**: `mathhook.simplify`
- **JavaScript**: `mathhook.simplify`


## See Also


- [operations.expansion-factoring](../operations/expansion-factoring.md)

- [features.noncommutative](../features/noncommutative.md)

- [advanced.numerical-stability](../advanced/numerical-stability.md)


