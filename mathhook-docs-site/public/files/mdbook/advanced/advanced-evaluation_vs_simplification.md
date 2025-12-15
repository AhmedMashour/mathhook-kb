---









---

# Evaluation vs Simplification

> **Topic**: `advanced.evaluation_vs_simplification`

Understand the critical differences between evaluation (computing numerical results
with domain checking) and simplification (algebraic transformation) in MathHook's
symbolic engine. Knowing when to use each operation is essential for correct
mathematical computation.



## Mathematical Definition

**Evaluation** maps expressions to numerical values:
$$\text{eval}: E \times \sigma \to \mathbb{R} \cup \mathbb{C} \cup \{\text{Error}\}$$
where $E$ is the set of expressions and $\sigma$ is a variable substitution.

**Simplification** maps expressions to equivalent canonical forms:
$$\text{simplify}: E \to E \quad \text{such that } \forall e \in E: e \equiv \text{simplify}(e)$$




# Evaluation vs Simplification

MathHook provides two fundamental operations for working with expressions:

1. **Evaluation** (`evaluate()`, `evaluate_with_context()`) - Compute numerical values
2. **Simplification** (`simplify()`) - Algebraic reduction

## The Key Principle

> **`evaluate()` ≠ `simplify()`** - They serve different purposes and should not be used interchangeably.

| Aspect | Evaluation | Simplification |
|--------|-----------|----------------|
| **Purpose** | Compute numerical values | Reduce algebraic complexity |
| **Input** | Expression (+ optional variables) | Expression only |
| **Output** | Numerical result or error | Simpler symbolic form |
| **Domain Checking** | ✅ Yes (catches mathematical errors) | ❌ No |
| **Substitution** | ✅ Yes (with context) | ❌ No |
| **Error Handling** | `Result<Expression, MathError>` | `Expression` |

## Core Concepts

### Evaluation: Numerical Computation

Evaluation converts symbolic expressions into concrete numerical values with
domain checking:

- **Domain Checking**: Catches mathematical errors (sqrt(-1), log(0), division by zero)
- **Recursive Evaluation**: Evaluates entire expression tree
- **Error Propagation**: Errors bubble up from nested expressions

### Simplification: Algebraic Reduction

Simplification transforms expressions into equivalent but simpler symbolic forms:

- **Algebraic Equivalence**: Output is mathematically equivalent to input
- **No Domain Checking**: Operates purely symbolically
- **Idempotency**: Simplifying twice yields the same result

## Decision Guide

### Use `evaluate()` when:
- You need a numerical result
- You want domain validation
- Expression contains only constants

### Use `evaluate_with_context()` when:
- Expression contains variables you need to substitute
- You want control over evaluation behavior
- You're solving equations or evaluating formulas

### Use `simplify()` when:
- You need algebraic reduction
- You want to reduce expression complexity
- You're preparing for symbolic operations

## Common Pitfalls

### ❌ Expecting Numbers from `simplify()`
```rust
let x = symbol!(x);
let result = expr!(x + x).simplify();
// Returns: 2*x (still symbolic, NOT a number!)
```

### ❌ Using `evaluate()` Without Substitution
```rust
let x = symbol!(x);
let result = expr!(x + 1).evaluate().unwrap();
// Returns: x + 1 (symbolic, can't substitute without context)
```

### ❌ Ignoring Domain Errors
```rust
let result = expr!(sqrt(-1)).evaluate().unwrap(); // PANIC!
// Always handle Result properly
```












## Examples


### Basic Evaluation vs Simplification

Shows the fundamental difference between the two operations


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook_core::{expr, symbol};

// Simplify: algebraic reduction
let x = symbol!(x);
let simplified = expr!(x + x + x).simplify();
assert_eq!(simplified, expr!(3 * x));  // Still symbolic

// Evaluate: numerical computation
let result = expr!(2 + 3).evaluate().unwrap();
assert_eq!(result, expr!(5));  // Numerical value

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr

# Simplify: algebraic reduction
x = symbol('x')
simplified = (x + x + x).simplify()
# Result: 3*x (still symbolic)

# Evaluate: numerical computation
result = expr('2 + 3').evaluate()
# Result: 5 (numerical value)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook');

// Simplify: algebraic reduction
const x = symbol('x');
const simplified = expr(x.add(x).add(x)).simplify();
// Result: 3*x (still symbolic)

// Evaluate: numerical computation
const result = expr('2 + 3').evaluate();
// Result: 5 (numerical value)

```
</details>





### Evaluation with Variable Substitution

Using evaluate_with_context for variable substitution


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook_core::core::expression::eval_numeric::EvalContext;
use mathhook_core::{expr, symbol};
use std::collections::HashMap;

let x = symbol!(x);
let y = symbol!(y);

// Create context with variable values
let mut vars = HashMap::new();
vars.insert("x".to_string(), expr!(3));
vars.insert("y".to_string(), expr!(4));
let ctx = EvalContext::numeric(vars);

// Evaluate x² + 2xy + y² at (x=3, y=4)
let formula = expr!(x^2 + 2*x*y + y^2);
let result = formula.evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(49)); // (3 + 4)² = 49

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr, EvalContext

x = symbol('x')
y = symbol('y')

# Create context with variable values
ctx = EvalContext({'x': 3, 'y': 4})

# Evaluate x² + 2xy + y² at (x=3, y=4)
formula = x**2 + 2*x*y + y**2
result = formula.evaluate_with_context(ctx)
# Result: 49  (which is (3+4)²)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr, EvalContext } = require('mathhook');

const x = symbol('x');
const y = symbol('y');

// Create context with variable values
const ctx = new EvalContext({x: 3, y: 4});

// Evaluate x² + 2xy + y² at (x=3, y=4)
const formula = x.pow(2).add(x.mul(y).mul(2)).add(y.pow(2));
const result = formula.evaluateWithContext(ctx);
// Result: 49  (which is (3+4)²)

```
</details>





### Domain Error Handling

Evaluation catches mathematical domain errors


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook_core::{expr, MathError};

// sqrt(-1) triggers domain error
match expr!(sqrt(-1)).evaluate() {
    Ok(result) => println!("Result: {}", result),
    Err(MathError::DomainError { operation, value, reason }) => {
        eprintln!("Domain error in {}: {} ({})", operation, value, reason);
    }
    Err(e) => eprintln!("Error: {:?}", e),
}

// log(0) triggers domain error
assert!(expr!(log(0)).evaluate().is_err());

// Division by zero
assert!(expr!(1 / 0).evaluate().is_err());

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import expr, MathError

# sqrt(-1) triggers domain error
try:
    result = expr('sqrt(-1)').evaluate()
except MathError as e:
    print(f"Domain error: {e}")

# log(0) triggers domain error
try:
    result = expr('log(0)').evaluate()
except MathError as e:
    print(f"Domain error: {e}")

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { expr, MathError } = require('mathhook');

// sqrt(-1) triggers domain error
try {
    const result = expr('sqrt(-1)').evaluate();
} catch (e) {
    if (e instanceof MathError) {
        console.error(`Domain error: ${e.message}`);
    }
}

// log(0) triggers domain error - also throws
// Division by zero - also throws

```
</details>





### Simplification for Algebraic Manipulation

Simplification applies algebraic identities without domain checking


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// Combine like terms
assert_eq!(expr!(x + x + x).simplify(), expr!(3 * x));

// Remove identity elements
assert_eq!(expr!(x * 1).simplify(), expr!(x));
assert_eq!(expr!(x + 0).simplify(), expr!(x));

// Zero propagation
assert_eq!(expr!(0 * x).simplify(), expr!(0));

// Trigonometric identities
assert_eq!(expr!(sin(x)^2 + cos(x)^2).simplify(), expr!(1));

// Simplify doesn't check domain (stays symbolic)
let result = expr!(sqrt(x)).simplify(); // OK, stays sqrt(x)

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr, simplify

x = symbol('x')

# Combine like terms
assert simplify(x + x + x) == 3*x

# Remove identity elements
assert simplify(x * 1) == x
assert simplify(x + 0) == x

# Zero propagation
assert simplify(0 * x) == 0

# Trigonometric identities
from mathhook import sin, cos
assert simplify(sin(x)**2 + cos(x)**2) == 1

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr, simplify, sin, cos } = require('mathhook');

const x = symbol('x');

// Combine like terms
console.log(simplify(x.add(x).add(x)));  // 3*x

// Remove identity elements
console.log(simplify(x.mul(1)));  // x
console.log(simplify(x.add(0)));  // x

// Zero propagation
console.log(simplify(x.mul(0)));  // 0

// Trigonometric identities
console.log(simplify(sin(x).pow(2).add(cos(x).pow(2))));  // 1

```
</details>








## API Reference

- **Rust**: `mathhook_core::core::expression::eval_numeric`
- **Python**: `mathhook.evaluation`
- **JavaScript**: `mathhook.evaluation`


## See Also


- [operations.simplification](../operations/simplification.md)

- [operations.evaluation](../operations/evaluation.md)

- [operations.substitution](../operations/substitution.md)

- [core.expressions](../core/expressions.md)

- [appendix.errors](../appendix/errors.md)


