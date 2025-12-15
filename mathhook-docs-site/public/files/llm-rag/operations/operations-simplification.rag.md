# Symbolic Simplification

MathHook provides comprehensive symbolic simplification for mathematical expressions, with full support for noncommutative algebra (matrices, operators, quaternions). The simplification system implements canonical forms and mathematical identities to reduce expressions to their simplest equivalent representation.


---
chunk_id: operations_simplification::0
topic: operations.simplification
title: Basic Simplification
priority: medium
keywords:
  - simplification
  - basic simplification
languages: [rust, python, javascript]
chunk: 1/3
---

## Basic Simplification

Identity elements and constant folding

### Rust

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

### Python

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

### JavaScript

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');

// Identity elements
let expr = x.add(0).mul(1);
const simplified = expr.simplify();
// Result: x

```



---
chunk_id: operations_simplification::1
topic: operations.simplification
title: Power Rule
priority: medium
keywords:
  - simplification
  - power rule
languages: [rust, python, javascript]
chunk: 2/3
---

## Power Rule

Combine like powers with same base

### Rust

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

### Python

```python
from mathhook import symbol

x = symbol('x')

# Combine like powers
expr = x**2 * x**3
simplified = expr.simplify()
# Result: x^5

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');

// Combine like powers
const expr = x.pow(2).mul(x.pow(3));
const simplified = expr.simplify();
// Result: x^5

```



---
chunk_id: operations_simplification::2
topic: operations.simplification
title: Noncommutative Matrices
priority: medium
keywords:
  - simplification
  - noncommutative matrices
languages: [rust, python, javascript]
chunk: 3/3
---

## Noncommutative Matrices

Matrix multiplication does NOT commute

### Rust

```rust
use mathhook::prelude::*;

let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Matrix multiplication does NOT commute
let expr = expr!(A * B);
// Simplification preserves order: A*B ≠ B*A

```

### Python

```python
from mathhook import symbol

A = symbol('A', matrix=True)
B = symbol('B', matrix=True)

# Matrix multiplication does NOT commute
expr = A * B
# Simplification preserves order: A*B ≠ B*A

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });

// Matrix multiplication does NOT commute
const expr = A.mul(B);
// Simplification preserves order: A*B ≠ B*A

```



