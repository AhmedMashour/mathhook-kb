---









---

# Expression Evaluation

> **Topic**: `operations.evaluation`

MathHook provides two fundamental operations for working with expressions:

1. **Evaluation** - Compute numerical values with domain checking
2. **Simplification** - Algebraic reduction while staying symbolic

Understanding when to use each operation is critical for correct mathematical computation.



## Mathematical Definition

**Function Evaluation:**
$$f(a) = f(x)|_{x=a}$$

**Evaluation with Context:**
For expression $f(x_1, \ldots, x_n)$ and substitutions $\{x_i \mapsto v_i\}$:
$$\text{evaluate}(f, \{x_i \mapsto v_i\}) = f(v_1, \ldots, v_n)$$

**Domain Constraints:**
- $\sqrt{x}$ requires $x \geq 0$ in $\mathbb{R}$
- $\log(x)$ requires $x > 0$ (pole at 0)
- $\tan(x)$ has poles at $\frac{\pi}{2} + n\pi$
- $\arcsin(x), \arccos(x)$ require $|x| \leq 1$ in $\mathbb{R}$




## Quick Decision Guide

Need a numerical value?
├─ YES → Use evaluate() or evaluate_with_context()
│   ├─ With variables? → evaluate_with_context(context)
│   └─ Constants only? → evaluate()
│
└─ NO → Need algebraic simplification?
    ├─ YES → Use simplify()
    └─ NO → Keep expression as-is

## Key Differences

| Operation | Purpose | Domain Checking | Substitution | Returns |
|-----------|---------|-----------------|--------------|---------|
| **evaluate()** | Numerical computation | ✅ Yes | ❌ No | Result<Expression, MathError> |
| **evaluate_with_context()** | Substitution + computation | ✅ Yes | ✅ Yes | Result<Expression, MathError> |
| **simplify()** | Algebraic reduction | ❌ No | ❌ No | Expression |

## Domain Constraints Checked

- sqrt(x): Requires x ≥ 0 in real domain
- log(x): Requires x > 0 (pole at 0)
- tan(x): Has poles at π/2 + nπ
- arcsin(x), arccos(x): Require |x| ≤ 1 in real domain
- Division by zero: Checked in x/y and x^(-n)

## Evaluation Context Options

```rust
pub struct EvalContext {
    pub variables: HashMap<String, Expression>,  // Variable substitutions
    pub precision: u32,                          // Numerical precision
    pub simplify_first: bool,                    // Simplify before evaluation?
    pub numeric: bool,                           // Perform numerical evaluation?
}
```












## Examples


### Constants Evaluate to Numbers

Direct evaluation of constant expressions


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let sum = expr!(2 + 3);
assert_eq!(sum.evaluate().unwrap(), expr!(5));

// Domain checking catches errors
let sqrt_neg = expr!(sqrt(-1));
assert!(matches!(sqrt_neg.evaluate(), Err(MathError::DomainError { .. })));

```
</details>



<details>
<summary><b>Python</b></summary>

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
</details>



<details>
<summary><b>JavaScript</b></summary>

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
</details>





### Evaluation with Context (Substitution)

Substitute variable values and evaluate


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, Expression

x = symbol('x')
expr = x ** 2

# Substitute x = 3 and evaluate
ctx = {'x': 3}
result = expr.evaluate_with_context(ctx)
assert result == 9

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');
const expr = x.pow(2);

// Substitute x = 3 and evaluate
const ctx = { x: 3 };
const result = expr.evaluateWithContext(ctx);
assert(result === 9);

```
</details>





### Simplification Without Domain Checking

Simplify operates purely symbolically without domain validation


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

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
</details>



<details>
<summary><b>JavaScript</b></summary>

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
</details>







## Performance

**Time Complexity**: O(n) for expression tree size n


## API Reference

- **Rust**: `mathhook_core::core::expression::eval_numeric::evaluate`
- **Python**: `mathhook.evaluate`
- **JavaScript**: `mathhook.evaluate`


## See Also


- [operations.substitution](../operations/substitution.md)

- [operations.simplification](../operations/simplification.md)


