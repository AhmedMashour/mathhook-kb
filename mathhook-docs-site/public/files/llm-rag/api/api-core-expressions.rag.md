# Core Expression System

The Expression type is the foundation of MathHook. Expressions are immutable,
32-byte cache-optimized structures representing mathematical constructs from
numbers to complex symbolic operations.


---
chunk_id: api_core_expressions::0
topic: api.core.expressions
title: Creating Expressions with Macros
priority: medium
keywords:
  - expressions
  - creating expressions with macros
languages: [rust, python, javascript]
chunk: 1/4
---

## Creating Expressions with Macros

Use expr!() and symbol!() macros for ergonomic expression creation

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Basic arithmetic
let sum = expr!(x + y);
let product = expr!(x * y);
let power = expr!(x ^ 2);

// Complex nested expressions
let complex = expr!(sin(x ^ 2) + cos(y ^ 2));

```

### Python

```python
from mathhook import symbol, expr

x = symbol('x')
y = symbol('y')

# Basic arithmetic
sum_expr = x + y
product = x * y
power = x**2

# Complex nested expressions
from mathhook import sin, cos
complex_expr = sin(x**2) + cos(y**2)

```

### JavaScript

```javascript
import { symbol, parse } from 'mathhook';

const x = symbol('x');
const y = symbol('y');

// Parse expressions
const sum = parse('x + y');
const product = parse('x * y');
const power = parse('x^2');

// Complex nested expressions
const complex = parse('sin(x^2) + cos(y^2)');

```



---
chunk_id: api_core_expressions::1
topic: api.core.expressions
title: Immutability and Operations
priority: medium
keywords:
  - expressions
  - immutability and operations
languages: [rust, python, javascript]
chunk: 2/4
---

## Immutability and Operations

All operations return new expressions, original unchanged

### Rust

```rust
let expr = expr!(x + 1);
let doubled = expr.mul(&expr!(2));  // Returns new expression
// `expr` is unchanged - still x + 1

// Safe to use in multiple threads
use std::sync::Arc;
let expr_arc = Arc::new(expr!(x ^ 2));
let clone = Arc::clone(&expr_arc);

```

### Python

```python
expr = x + 1
doubled = expr * 2  # Returns new expression
# expr is unchanged - still x + 1

# Safe for concurrent use
import threading
shared_expr = x**2

```

### JavaScript

```javascript
const expr = parse('x + 1');
const doubled = expr.mul(2);  // Returns new expression
// expr is unchanged - still x + 1

// Immutable - safe for concurrent access

```



---
chunk_id: api_core_expressions::2
topic: api.core.expressions
title: Canonical Forms and Equality
priority: medium
keywords:
  - expressions
  - canonical forms and equality
languages: [rust, python, javascript]
chunk: 3/4
---

## Canonical Forms and Equality

Automatic normalization ensures equivalent expressions are equal

### Rust

```rust
let expr1 = expr!(x + y);
let expr2 = expr!(y + x);
assert_eq!(expr1, expr2);  // True - both normalized to x + y

// Flattening
let nested = expr!((x + y) + z);
// Automatically flattened to Add(x, y, z)

// Identity removal
let identity = expr!(x + 0);
assert_eq!(identity.simplify(), expr!(x));

```

### Python

```python
expr1 = x + y
expr2 = y + x
assert expr1 == expr2  # True - both normalized to x + y

# Flattening and identity removal
nested = (x + y) + z
identity = x + 0
assert identity.simplify() == x

```

### JavaScript

```javascript
const expr1 = parse('x + y');
const expr2 = parse('y + x');
// Both normalized to x + y

// Identity removal
const identity = parse('x + 0');
const simplified = identity.simplify();
// Result: x

```



---
chunk_id: api_core_expressions::3
topic: api.core.expressions
title: Pattern Matching and Structure
priority: medium
keywords:
  - expressions
  - pattern matching and structure
languages: [rust, python, javascript]
chunk: 4/4
---

## Pattern Matching and Structure

Work with expression structure using pattern matching

### Rust

```rust
use mathhook::Expression;

match expr {
    Expression::Add(terms) => {
        println!("Sum with {} terms", terms.len());
    }
    Expression::Mul(factors) => {
        println!("Product with {} factors", factors.len());
    }
    Expression::Pow(base, exp) => {
        println!("Power: {} ^ {}", base, exp);
    }
    Expression::Function(name, args) => {
        println!("Function {} with {} args", name, args.len());
    }
    _ => {}
}

```

### Python

```python
from mathhook import Expression

# Python uses method introspection
if expr.is_add():
    terms = expr.get_terms()
    print(f"Sum with {len(terms)} terms")
elif expr.is_mul():
    factors = expr.get_factors()
    print(f"Product with {len(factors)} factors")
elif expr.is_pow():
    base, exp = expr.get_base_exp()
    print(f"Power: {base} ^ {exp}")

```

### JavaScript

```javascript
import { Expression } from 'mathhook';

// Node.js uses type checking methods
if (expr.isAdd()) {
    const terms = expr.getTerms();
    console.log(`Sum with ${terms.length} terms`);
} else if (expr.isMul()) {
    const factors = expr.getFactors();
    console.log(`Product with ${factors.length} factors`);
} else if (expr.isPow()) {
    const [base, exp] = expr.getBaseExp();
    console.log(`Power: ${base} ^ ${exp}`);
}

```



