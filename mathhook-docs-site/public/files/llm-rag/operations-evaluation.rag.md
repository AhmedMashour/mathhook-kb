# Expression Evaluation

MathHook provides two fundamental operations for working with expressions:

1. **Evaluation** - Compute numerical values with domain checking
2. **Simplification** - Algebraic reduction while staying symbolic

Understanding when to use each operation is critical for correct mathematical computation.


---
chunk_id: operations_evaluation::0
topic: operations.evaluation
title: Constants Evaluate to Numbers
priority: medium
keywords:
  - evaluation
  - constants evaluate to numbers
languages: [rust, python, javascript]
chunk: 1/3
---

## Constants Evaluate to Numbers

Direct evaluation of constant expressions

### Rust

```rust
use mathhook::prelude::*;

let sum = expr!(2 + 3);
assert_eq!(sum.evaluate().unwrap(), expr!(5));

// Domain checking catches errors
let sqrt_neg = expr!(sqrt(-1));
assert!(matches!(sqrt_neg.evaluate(), Err(MathError::DomainError { .. })));

```

### Python

```python
from mathhook import expr

sum_expr = expr('2 + 3')
assert sum_expr.evaluate() == 5

# Domain checking catches errors
sqrt_neg = expr('sqrt(-1)')
try:
    sqrt_neg.evaluate()
    assert False, "Should raise domain error"
except MathError:
    pass

```

### JavaScript

```javascript
const { expr } = require('mathhook');

const sum = expr('2 + 3');
assert(sum.evaluate() === 5);

// Domain checking catches errors
const sqrtNeg = expr('sqrt(-1)');
try {
    sqrtNeg.evaluate();
    throw new Error("Should raise domain error");
} catch (e) {
    // Expected MathError
}

```



---
chunk_id: operations_evaluation::1
topic: operations.evaluation
title: Evaluation with Context (Substitution)
priority: medium
keywords:
  - evaluation
  - evaluation with context (substitution)
languages: [rust, python, javascript]
chunk: 2/3
---

## Evaluation with Context (Substitution)

Substitute variable values and evaluate

### Rust

```rust
use mathhook_core::core::expression::eval_numeric::EvalContext;
use mathhook::prelude::*;
use std::collections::HashMap;

let x = symbol!(x);

// Substitute x = 3 and evaluate
let mut vars = HashMap::new();
vars.insert("x".to_string(), Expression::integer(3));
let ctx = EvalContext::numeric(vars);

let expr = Expression::pow(x.clone(), Expression::integer(2));
assert_eq!(expr.evaluate_with_context(&ctx).unwrap(), Expression::integer(9));

```

### Python

```python
from mathhook import symbol, Expression

x = symbol('x')
expr = x ** 2

# Substitute x = 3 and evaluate
ctx = {'x': 3}
result = expr.evaluate_with_context(ctx)
assert result == 9

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');
const expr = x.pow(2);

// Substitute x = 3 and evaluate
const ctx = { x: 3 };
const result = expr.evaluateWithContext(ctx);
assert(result === 9);

```



---
chunk_id: operations_evaluation::2
topic: operations.evaluation
title: Simplification Without Domain Checking
priority: medium
keywords:
  - evaluation
  - simplification without domain checking
languages: [rust, python, javascript]
chunk: 3/3
---

## Simplification Without Domain Checking

Simplify operates purely symbolically without domain validation

### Rust

```rust
use mathhook::prelude::*;
use mathhook_core::simplify::Simplify;

let x = symbol!(x);

// Combine like terms
let sum = expr!(x + x);
assert_eq!(sum.simplify(), expr!(2 * x));

// Apply identities
assert_eq!(expr!(x * 1).simplify(), expr!(x));
assert_eq!(expr!(0 * x).simplify(), expr!(0));

```

### Python

```python
from mathhook import symbol

x = symbol('x')

# Combine like terms
sum_expr = x + x
assert sum_expr.simplify() == 2 * x

# Apply identities
assert (x * 1).simplify() == x
assert (0 * x).simplify() == 0

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');

// Combine like terms
const sumExpr = x.add(x);
assert(sumExpr.simplify().equals(x.mul(2)));

// Apply identities
assert(x.mul(1).simplify().equals(x));
assert(x.mul(0).simplify().equals(0));

```



