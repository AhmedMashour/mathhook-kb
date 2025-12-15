---









---

# Symbolic Simplification

> **Topic**: `api.algebra.simplification`

Comprehensive symbolic simplification for mathematical expressions, with full
support for noncommutative algebra (matrices, operators, quaternions). Implements
canonical forms and mathematical identities to reduce expressions to simplest form.





# Symbolic Simplification

## Overview

MathHook's simplification system transforms expressions to canonical form through:
- **Arithmetic Simplification**: Collect like terms, flatten operations, remove identities
- **Power Rule**: Combine like powers ($x^a \cdot x^b \rightarrow x^{a+b}$)
- **Noncommutative Algebra**: Preserve order for matrices, operators, quaternions
- **Rational Arithmetic**: Exact computation with arbitrary precision

## Capabilities

### Arithmetic Operations
- **Addition**: Collects like terms, flattens nested sums, removes 0
- **Multiplication**: Combines factors, flattens products, removes 1, applies power rule
- **Power**: Simplifies exponents, distributes when safe (commutative only)

### Noncommutative Algebra
- **Matrices**: Preserves order ($AB \neq BA$)
- **Operators**: Quantum mechanics commutators $[x,p] = xp - px$
- **Quaternions**: $ij = k$ but $ji = -k$

### Numerical Stability
- **Checked arithmetic**: Integer operations use checked_mul, checked_add
- **BigInt promotion**: Automatic on overflow
- **Iterative flattening**: Avoids stack overflow for deeply nested expressions

## Performance

### Targets
- **Simplification time**: <1ms for expressions with <100 nodes
- **Memory**: Minimal allocations through iterative flattening
- **Cache efficiency**: 32-byte expression size (2 per cache line)

### Optimization Strategies
- **Iterative flattening**: Avoids recursion stack overflow
- **Early exit**: Returns immediately for identity elements
- **Power combining**: O(n) grouping of like powers












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
simplified = expr.simplify()
# Result: 5

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { symbol, parse } from 'mathhook';

const x = symbol('x');

// Identity elements
const expr = parse('(x + 0) * 1');
const simplified = expr.simplify();
// Result: x

// Constant folding
const expr2 = parse('2 + 3');
const simplified2 = expr2.simplify();
// Result: 5

```
</details>




### Power Rule Simplification

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

# Multiple powers
expr = x**2 * x**3 * x**4
simplified = expr.simplify()
# Result: x^9

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { symbol, parse } from 'mathhook';

const x = symbol('x');

// Combine like powers
const expr = parse('x^2 * x^3');
const simplified = expr.simplify();
// Result: x^5

// Multiple powers
const expr2 = parse('x^2 * x^3 * x^4');
const simplified2 = expr2.simplify();
// Result: x^9

```
</details>




### Noncommutative Algebra

Preserve order for noncommutative symbols

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

// Scalar symbols (commutative) - factors can be sorted
let x = symbol!(x);
let y = symbol!(y);
let expr = expr!(y * x);
let simplified = expr.simplify();
// Result: x * y (sorted alphabetically)

// Matrix symbols (noncommutative) - order preserved
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let expr = expr!(B * A);
let simplified = expr.simplify();
// Result: B * A (original order preserved)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol

# Scalar symbols (commutative)
x = symbol('x')
y = symbol('y')
expr = y * x
simplified = expr.simplify()
# Result: x * y (sorted)

# Matrix symbols (noncommutative)
A = symbol('A', matrix=True)
B = symbol('B', matrix=True)
expr = B * A
simplified = expr.simplify()
# Result: B * A (order preserved)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { symbol, parse } from 'mathhook';

// Scalar symbols (commutative)
const x = symbol('x');
const y = symbol('y');
const expr = parse('y * x');
const simplified = expr.simplify();
// Result: x * y (sorted)

// Matrix symbols (noncommutative)
const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });
const expr2 = parse('B * A');
const simplified2 = expr2.simplify();
// Result: B * A (order preserved)

```
</details>




### Power Distribution (Commutative Only)

Distribute powers for scalars, not for matrices

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

// Scalars (commutative): distributes power
let x = symbol!(x);
let y = symbol!(y);
let expr = expr!((x * y) ^ 2);
let simplified = expr.simplify();
// Result: x^2 * y^2 (distributed)

// Matrices (noncommutative): does NOT distribute
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let expr = expr!((A * B) ^ 2);
let simplified = expr.simplify();
// Result: (A*B)^2 (NOT distributed to A^2 * B^2)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol

# Scalars (commutative)
x = symbol('x')
y = symbol('y')
expr = (x * y)**2
simplified = expr.simplify()
# Result: x^2 * y^2

# Matrices (noncommutative)
A = symbol('A', matrix=True)
B = symbol('B', matrix=True)
expr = (A * B)**2
simplified = expr.simplify()
# Result: (A*B)^2

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { symbol, parse } from 'mathhook';

// Scalars (commutative)
const expr = parse('(x * y)^2');
const simplified = expr.simplify();
// Result: x^2 * y^2

// Matrices (noncommutative)
const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });
const expr2 = parse('(A * B)^2');
const simplified2 = expr2.simplify();
// Result: (A*B)^2

```
</details>




### Rational Arithmetic

Exact rational computation with arbitrary precision

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let expr = expr!(1/3 + 1/6);  // Rational arithmetic
let simplified = expr.simplify();
// Result: 1/2 (exact rational, not 0.5)

// Arbitrary precision
let expr = expr!(1/999999999 + 1/999999999);
let simplified = expr.simplify();
// Result: 2/999999999 (exact, no overflow)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import expr as parse_expr

expr = parse_expr('1/3 + 1/6')
simplified = expr.simplify()
# Result: 1/2 (exact rational)

# Arbitrary precision
expr = parse_expr('1/999999999 + 1/999999999')
simplified = expr.simplify()
# Result: 2/999999999 (exact)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { parse } from 'mathhook';

const expr = parse('1/3 + 1/6');
const simplified = expr.simplify();
// Result: 1/2 (exact rational)

// Arbitrary precision
const expr2 = parse('1/999999999 + 1/999999999');
const simplified2 = expr2.simplify();
// Result: 2/999999999 (exact)

```
</details>






## Performance

**Time Complexity**: O(n) for n-node expression tree


## API Reference

- **Rust**: `mathhook_core::simplify::Simplify`
- **Python**: `mathhook.simplify`
- **JavaScript**: `mathhook.simplify`


## See Also


- [api.core.expressions](../api/core/expressions.md)

- [api.operations.expansion_factoring](../api/operations/expansion_factoring.md)

- [api.operations.substitution](../api/operations/substitution.md)

- [api.advanced.noncommutative_algebra](../api/advanced/noncommutative_algebra.md)


