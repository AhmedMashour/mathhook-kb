# Expressions

The Expression type is the foundation of MathHook. Expressions are represented as an enum
with variants for different mathematical constructs including numbers, variables, operations,
functions, constants, matrices, and relations.


---
chunk_id: core_expressions::0
topic: core.expressions
title: Basic Expression Creation with Macros
priority: medium
keywords:
  - expressions
  - basic expression creation with macros
languages: [rust, python, javascript]
chunk: 1/3
---

## Basic Expression Creation with Macros

Using expr! and symbol! macros for ergonomic expression creation

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Basic arithmetic
let sum = expr!(x + y);
let product = expr!(x * y);
let power = expr!(x ^ 2);

// Complex expressions
let quadratic = expr!(a * x ^ 2 + b * x + c);

```

### Python

```python
from mathhook import symbol, expr

x = symbol('x')
y = symbol('y')

# Basic arithmetic
sum_expr = expr('x + y')
product = expr('x * y')
power = expr('x^2')

# Complex expressions
quadratic = expr('a*x^2 + b*x + c')

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook-node');

const x = symbol('x');
const y = symbol('y');

// Basic arithmetic
const sum = expr('x + y');
const product = expr('x * y');
const power = expr('x^2');

// Complex expressions
const quadratic = expr('a*x^2 + b*x + c');

```



---
chunk_id: core_expressions::1
topic: core.expressions
title: Canonical Form Normalization
priority: medium
keywords:
  - expressions
  - canonical form normalization
languages: [rust, python, javascript]
chunk: 2/3
---

## Canonical Form Normalization

Expressions are automatically normalized to canonical form

### Rust

```rust
use mathhook::prelude::*;

let expr1 = expr!(x + y);
let expr2 = expr!(y + x);

// Both normalized to same form
assert_eq!(expr1, expr2);

// Rationals reduced
let frac = Expression::rational(6, 4);
assert_eq!(frac, Expression::rational(3, 2));

```

### Python

```python
from mathhook import expr, Expression

expr1 = expr('x + y')
expr2 = expr('y + x')

# Both normalized to same form
assert expr1 == expr2

# Rationals reduced
frac = Expression.rational(6, 4)
assert frac == Expression.rational(3, 2)

```

### JavaScript

```javascript
const { expr, Expression } = require('mathhook-node');

const expr1 = expr('x + y');
const expr2 = expr('y + x');

// Both normalized to same form
console.assert(expr1.equals(expr2));

// Rationals reduced
const frac = Expression.rational(6, 4);
console.assert(frac.equals(Expression.rational(3, 2)));

```



---
chunk_id: core_expressions::2
topic: core.expressions
title: Immutable Operations
priority: medium
keywords:
  - expressions
  - immutable operations
languages: [rust, python, javascript]
chunk: 3/3
---

## Immutable Operations

All expression operations return new expressions without modifying originals

### Rust

```rust
use mathhook::prelude::*;

let expr = expr!(x + 1);
let doubled = expr.mul(&expr!(2));

// Original unchanged
println!("Original: {}", expr);  // x + 1
println!("Doubled: {}", doubled); // 2*(x + 1)

```

### Python

```python
from mathhook import expr

original = expr('x + 1')
doubled = original * 2

# Original unchanged
print(f"Original: {original}")  # x + 1
print(f"Doubled: {doubled}")    # 2*(x + 1)

```

### JavaScript

```javascript
const { expr } = require('mathhook-node');

const original = expr('x + 1');
const doubled = original.mul(2);

// Original unchanged
console.log(`Original: ${original}`);  // x + 1
console.log(`Doubled: ${doubled}`);    // 2*(x + 1)

```



