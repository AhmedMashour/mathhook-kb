# Symbols and Numbers

Symbols represent mathematical variables (x, y, θ, etc.) using efficient string interning.
Numbers support integers, rationals, floats, and complex numbers with exact symbolic representation
for precise mathematical computation.


---
chunk_id: core_symbols-numbers::0
topic: core.symbols-numbers
title: Symbol Creation and Equality
priority: medium
keywords:
  - symbols-numbers
  - symbol creation and equality
languages: [rust, python, javascript]
chunk: 1/3
---

## Symbol Creation and Equality

Creating symbols with string interning for O(1) equality checks

### Rust

```rust
use mathhook::prelude::*;

let x1 = symbol!(x);
let x2 = symbol!(x);
let y = symbol!(y);

// O(1) pointer comparison
assert_eq!(x1, x2);
assert_ne!(x1, y);

```

### Python

```python
from mathhook import symbol

x1 = symbol('x')
x2 = symbol('x')
y = symbol('y')

# Fast equality check
assert x1 == x2
assert x1 != y

```

### JavaScript

```javascript
const { symbol } = require('mathhook-node');

const x1 = symbol('x');
const x2 = symbol('x');
const y = symbol('y');

// Fast equality check
console.assert(x1.equals(x2));
console.assert(!x1.equals(y));

```



---
chunk_id: core_symbols-numbers::1
topic: core.symbols-numbers
title: Exact Rational Arithmetic
priority: medium
keywords:
  - symbols-numbers
  - exact rational arithmetic
languages: [rust, python, javascript]
chunk: 2/3
---

## Exact Rational Arithmetic

Using rationals for exact fractional computation

### Rust

```rust
use mathhook::prelude::*;

// Exact: 1/3
let third = Expression::rational(1, 3);
let result = expr!(3 * third);
assert_eq!(result, Expression::integer(1));

// Auto-reduction: 6/4 = 3/2
let frac = Expression::rational(6, 4);
assert_eq!(frac, Expression::rational(3, 2));

```

### Python

```python
from mathhook import Expression, expr

# Exact: 1/3
third = Expression.rational(1, 3)
result = expr('3 * third')
assert result == Expression.integer(1)

# Auto-reduction: 6/4 = 3/2
frac = Expression.rational(6, 4)
assert frac == Expression.rational(3, 2)

```

### JavaScript

```javascript
const { Expression, expr } = require('mathhook-node');

// Exact: 1/3
const third = Expression.rational(1, 3);
const result = expr('3 * third');
console.assert(result.equals(Expression.integer(1)));

// Auto-reduction: 6/4 = 3/2
const frac = Expression.rational(6, 4);
console.assert(frac.equals(Expression.rational(3, 2)));

```



---
chunk_id: core_symbols-numbers::2
topic: core.symbols-numbers
title: Complex Numbers
priority: medium
keywords:
  - symbols-numbers
  - complex numbers
languages: [rust, python, javascript]
chunk: 3/3
---

## Complex Numbers

Working with complex numbers and imaginary unit

### Rust

```rust
use mathhook::prelude::*;

// 3 + 4i
let z = Expression::complex(
    Expression::integer(3),
    Expression::integer(4)
);

// Magnitude: |z| = sqrt(3^2 + 4^2) = 5
let magnitude = expr!(sqrt((3^2) + (4^2)));
assert_eq!(magnitude.simplify(), Expression::integer(5));

```

### Python

```python
from mathhook import Expression, expr

# 3 + 4i
z = Expression.complex(3, 4)

# Magnitude: |z| = 5
magnitude = expr('sqrt(3^2 + 4^2)')
assert magnitude.simplify() == Expression.integer(5)

```

### JavaScript

```javascript
const { Expression, expr } = require('mathhook-node');

// 3 + 4i
const z = Expression.complex(3, 4);

// Magnitude: |z| = 5
const magnitude = expr('sqrt(3^2 + 4^2)');
console.assert(magnitude.simplify().equals(Expression.integer(5)));

```



