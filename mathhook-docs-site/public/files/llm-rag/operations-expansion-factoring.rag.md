# Expansion and Factoring

Transform expressions between expanded and factored forms for easier manipulation and analysis.


---
chunk_id: operations_expansion-factoring::0
topic: operations.expansion-factoring
title: Simple Products
priority: medium
keywords:
  - expansion-factoring
  - simple products
languages: [rust, python, javascript]
chunk: 1/3
---

## Simple Products

Expand products of sums

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Expand 2(x + 3)
let expr1 = expr!(2 * (x + 3));
let expanded1 = expr1.expand();
// Result: 2x + 6

// Expand (x + 1)(x + 2)
let expr2 = expr!((x + 1) * (x + 2));
let expanded2 = expr2.expand();
// Result: x² + 3x + 2

// Expand (x + y)(x - y) - difference of squares
let y = symbol!(y);
let expr3 = expr!((x + y) * (x - y));
let expanded3 = expr3.expand();
// Result: x² - y²

```

### Python

```python
from mathhook import symbol

x = symbol('x')

# Expand 2(x + 3)
expr1 = 2 * (x + 3)
expanded1 = expr1.expand()
# Result: 2*x + 6

# Expand (x + 1)(x + 2)
expr2 = (x + 1) * (x + 2)
expanded2 = expr2.expand()
# Result: x**2 + 3*x + 2

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');

// Expand 2(x + 3)
const expr1 = x.add(3).mul(2);
const expanded1 = expr1.expand();
// Result: 2*x + 6

```



---
chunk_id: operations_expansion-factoring::1
topic: operations.expansion-factoring
title: Power Expansion
priority: medium
keywords:
  - expansion-factoring
  - power expansion
languages: [rust, python, javascript]
chunk: 2/3
---

## Power Expansion

Expand expressions raised to integer powers

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Expand (x + 1)^2
let expr1 = expr!((x + 1) ^ 2);
let expanded1 = expr1.expand();
// Result: x² + 2x + 1

// Expand (x + y)^3
let expr2 = expr!((x + y) ^ 3);
let expanded2 = expr2.expand();
// Result: x³ + 3x²y + 3xy² + y³

// Expand (x - 2)^4
let expr3 = expr!((x - 2) ^ 4);
let expanded3 = expr3.expand();
// Result: x⁴ - 8x³ + 24x² - 32x + 16

```

### Python

```python
from mathhook import symbol

x = symbol('x')
y = symbol('y')

# Expand (x + 1)^2
expr1 = (x + 1)**2
expanded1 = expr1.expand()
# Result: x**2 + 2*x + 1

# Expand (x + y)^3
expr2 = (x + y)**3
expanded2 = expr2.expand()
# Result: x**3 + 3*x**2*y + 3*x*y**2 + y**3

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');
const y = symbol('y');

// Expand (x + 1)^2
const expr1 = x.add(1).pow(2);
const expanded1 = expr1.expand();
// Result: x^2 + 2*x + 1

```



---
chunk_id: operations_expansion-factoring::2
topic: operations.expansion-factoring
title: Noncommutative Matrix Expansion
priority: medium
keywords:
  - expansion-factoring
  - noncommutative matrix expansion
languages: [rust, python, javascript]
chunk: 3/3
---

## Noncommutative Matrix Expansion

For matrices, order matters - (A+B)² has 4 terms

### Rust

```rust
use mathhook::prelude::*;

// Create matrix symbols
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let C = symbol!(C; matrix);

// Expand (A + B)^2 - preserves noncommutativity
let expr = expr!((A + B) ^ 2);
let expanded = expr.expand();
// Result: A² + AB + BA + B²   (4 terms, NOT A² + 2AB + B²)

// Expand (A + B)(C)
let expr2 = expr!((A + B) * C);
let expanded2 = expr2.expand();
// Result: AC + BC   (order preserved: A*C first, then B*C)

```

### Python

```python
from mathhook import symbol

# Create matrix symbols
A = symbol('A', matrix=True)
B = symbol('B', matrix=True)
C = symbol('C', matrix=True)

# Expand (A + B)^2 - preserves noncommutativity
expr = (A + B)**2
expanded = expr.expand()
# Result: A**2 + A*B + B*A + B**2   (4 terms, NOT A**2 + 2*A*B + B**2)

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

// Create matrix symbols
const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });

// Expand (A + B)^2 - preserves noncommutativity
const expr = A.add(B).pow(2);
const expanded = expr.expand();
// Result: A^2 + AB + BA + B^2   (4 terms)

```



