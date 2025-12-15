# Substitution

Replace variables with values or expressions to evaluate, simplify, or transform expressions.


---
chunk_id: operations_substitution::0
topic: operations.substitution
title: Single Variable Substitution
priority: medium
keywords:
  - substitution
  - single variable substitution
languages: [rust, python, javascript]
chunk: 1/3
---

## Single Variable Substitution

Replace variable with number

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Substitute x = 2 into x² + 3x
let expr1 = expr!(x ^ 2 + 3 * x);
let result1 = expr1.substitute(&x, &expr!(2));
// Result: 4 + 6 = 10

// Substitute x = -1 into x³ - 2x + 1
let expr2 = expr!(x ^ 3 - 2 * x + 1);
let result2 = expr2.substitute(&x, &expr!(-1));
// Result: -1 + 2 + 1 = 2

```

### Python

```python
from mathhook import symbol

x = symbol('x')

# Substitute x = 2 into x² + 3x
expr1 = x**2 + 3*x
result1 = expr1.subs(x, 2)
# Result: 10

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');

// Substitute x = 2 into x² + 3x
const expr1 = x.pow(2).add(x.mul(3));
const result1 = expr1.substitute(x, 2);
// Result: 10

```



---
chunk_id: operations_substitution::1
topic: operations.substitution
title: Expression Substitution
priority: medium
keywords:
  - substitution
  - expression substitution
languages: [rust, python, javascript]
chunk: 2/3
---

## Expression Substitution

Replace with another expression

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Substitute x = y + 1 into x² + 3x
let expression = expr!(x ^ 2 + 3 * x);
let substituted = expression.substitute(&x, &expr!(y + 1));
// Result: (y+1)² + 3(y+1)

// Expand for cleaner form
let expanded = substituted.expand();
// Result: y² + 2y + 1 + 3y + 3 = y² + 5y + 4

```

### Python

```python
from mathhook import symbol

x = symbol('x')
y = symbol('y')

# Substitute x = y + 1 into x² + 3x
expression = x**2 + 3*x
substituted = expression.subs(x, y + 1)
# Result: (y+1)^2 + 3(y+1)

# Expand for cleaner form
expanded = substituted.expand()
# Result: y^2 + 5*y + 4

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');
const y = symbol('y');

// Substitute x = y + 1 into x² + 3x
const expression = x.pow(2).add(x.mul(3));
const substituted = expression.substitute(x, y.add(1));
// Result: (y+1)^2 + 3(y+1)

```



---
chunk_id: operations_substitution::2
topic: operations.substitution
title: U-Substitution for Integration
priority: medium
keywords:
  - substitution
  - u-substitution for integration
languages: [rust, python, javascript]
chunk: 3/3
---

## U-Substitution for Integration

Transform difficult integrals

### Rust

```rust
use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);
let u = symbol!(u);

// Integrate: ∫ 2x·e^(x²) dx
// Let u = x², then du = 2x dx
let integrand = expr!(2 * x * exp(x ^ 2));

// Manual substitution
let u_expr = expr!(x ^ 2);  // u = x²
let integrand_u = integrand.substitute(&expr!(x ^ 2), &u);
// Result: ∫ e^u du = e^u + C

// Back-substitute: e^(x²) + C
let result = expr!(exp(u)).substitute(&u, &u_expr);
// Result: e^(x²) + C

```

### Python

```python
from mathhook import symbol, integrate

x = symbol('x')
u = symbol('u')

# Integrate: ∫ 2x·e^(x²) dx
integrand = 2*x*exp(x**2)

# Manual substitution
u_expr = x**2  # u = x²
integrand_u = integrand.subs(x**2, u)
# Result: ∫ e^u du = e^u + C

# Back-substitute: e^(x²) + C
result = exp(u).subs(u, u_expr)
# Result: e^(x²) + C

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');
const u = symbol('u');

// Integrate: ∫ 2x·e^(x²) dx
const integrand = x.mul(2).mul(expr('exp(x^2)'));

// Manual substitution
const uExpr = x.pow(2);  // u = x²
const integrandU = integrand.substitute(x.pow(2), u);
// Result: ∫ e^u du = e^u + C

```



