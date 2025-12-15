# Evaluation vs Simplification

Understand the critical differences between evaluation (computing numerical results
with domain checking) and simplification (algebraic transformation) in MathHook's
symbolic engine. Knowing when to use each operation is essential for correct
mathematical computation.


---
chunk_id: advanced_evaluation_vs_simplification::0
topic: advanced.evaluation_vs_simplification
title: Basic Evaluation vs Simplification
priority: medium
keywords:
  - evaluation_vs_simplification
  - basic evaluation vs simplification
languages: [rust, python, javascript]
chunk: 1/4
---

## Basic Evaluation vs Simplification

Shows the fundamental difference between the two operations

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: advanced_evaluation_vs_simplification::1
topic: advanced.evaluation_vs_simplification
title: Evaluation with Variable Substitution
priority: medium
keywords:
  - evaluation_vs_simplification
  - evaluation with variable substitution
languages: [rust, python, javascript]
chunk: 2/4
---

## Evaluation with Variable Substitution

Using evaluate_with_context for variable substitution

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: advanced_evaluation_vs_simplification::2
topic: advanced.evaluation_vs_simplification
title: Domain Error Handling
priority: medium
keywords:
  - evaluation_vs_simplification
  - domain error handling
languages: [rust, python, javascript]
chunk: 3/4
---

## Domain Error Handling

Evaluation catches mathematical domain errors

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: advanced_evaluation_vs_simplification::3
topic: advanced.evaluation_vs_simplification
title: Simplification for Algebraic Manipulation
priority: medium
keywords:
  - evaluation_vs_simplification
  - simplification for algebraic manipulation
languages: [rust, python, javascript]
chunk: 4/4
---

## Simplification for Algebraic Manipulation

Simplification applies algebraic identities without domain checking

### Rust

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

### Python

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

### JavaScript

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



