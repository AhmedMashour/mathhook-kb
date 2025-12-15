# Symbolic Simplification

Comprehensive symbolic simplification for mathematical expressions, with full
support for noncommutative algebra (matrices, operators, quaternions). Implements
canonical forms and mathematical identities to reduce expressions to simplest form.


---
chunk_id: api_algebra_simplification::0
topic: api.algebra.simplification
title: Basic Simplification
priority: medium
keywords:
  - simplification
  - basic simplification
languages: [rust, python, javascript]
chunk: 1/5
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
simplified = expr.simplify()
# Result: 5

```

### JavaScript

```javascript
import { symbol, parse } from 'mathhook';

const x = symbol('x');

// Identity elements
const expr = parse('(x + 0) * 1');
const simplified = expr.simplify();
// Result: x

// Constant folding
const expr2 = parse('2 + 3');
const simplified2 = expr2.simplify();
// Result: 5

```



---
chunk_id: api_algebra_simplification::1
topic: api.algebra.simplification
title: Power Rule Simplification
priority: medium
keywords:
  - simplification
  - power rule simplification
languages: [rust, python, javascript]
chunk: 2/5
---

## Power Rule Simplification

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

# Multiple powers
expr = x**2 * x**3 * x**4
simplified = expr.simplify()
# Result: x^9

```

### JavaScript

```javascript
import { symbol, parse } from 'mathhook';

const x = symbol('x');

// Combine like powers
const expr = parse('x^2 * x^3');
const simplified = expr.simplify();
// Result: x^5

// Multiple powers
const expr2 = parse('x^2 * x^3 * x^4');
const simplified2 = expr2.simplify();
// Result: x^9

```



---
chunk_id: api_algebra_simplification::2
topic: api.algebra.simplification
title: Noncommutative Algebra
priority: medium
keywords:
  - simplification
  - noncommutative algebra
languages: [rust, python, javascript]
chunk: 3/5
---

## Noncommutative Algebra

Preserve order for noncommutative symbols

### Rust

```rust
use mathhook::prelude::*;

// Scalar symbols (commutative) - factors can be sorted
let x = symbol!(x);
let y = symbol!(y);
let expr = expr!(y * x);
let simplified = expr.simplify();
// Result: x * y (sorted alphabetically)

// Matrix symbols (noncommutative) - order preserved
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let expr = expr!(B * A);
let simplified = expr.simplify();
// Result: B * A (original order preserved)

```

### Python

```python
from mathhook import symbol

# Scalar symbols (commutative)
x = symbol('x')
y = symbol('y')
expr = y * x
simplified = expr.simplify()
# Result: x * y (sorted)

# Matrix symbols (noncommutative)
A = symbol('A', matrix=True)
B = symbol('B', matrix=True)
expr = B * A
simplified = expr.simplify()
# Result: B * A (order preserved)

```

### JavaScript

```javascript
import { symbol, parse } from 'mathhook';

// Scalar symbols (commutative)
const x = symbol('x');
const y = symbol('y');
const expr = parse('y * x');
const simplified = expr.simplify();
// Result: x * y (sorted)

// Matrix symbols (noncommutative)
const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });
const expr2 = parse('B * A');
const simplified2 = expr2.simplify();
// Result: B * A (order preserved)

```



---
chunk_id: api_algebra_simplification::3
topic: api.algebra.simplification
title: Power Distribution (Commutative Only)
priority: medium
keywords:
  - simplification
  - power distribution (commutative only)
languages: [rust, python, javascript]
chunk: 4/5
---

## Power Distribution (Commutative Only)

Distribute powers for scalars, not for matrices

### Rust

```rust
use mathhook::prelude::*;

// Scalars (commutative): distributes power
let x = symbol!(x);
let y = symbol!(y);
let expr = expr!((x * y) ^ 2);
let simplified = expr.simplify();
// Result: x^2 * y^2 (distributed)

// Matrices (noncommutative): does NOT distribute
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let expr = expr!((A * B) ^ 2);
let simplified = expr.simplify();
// Result: (A*B)^2 (NOT distributed to A^2 * B^2)

```

### Python

```python
from mathhook import symbol

# Scalars (commutative)
x = symbol('x')
y = symbol('y')
expr = (x * y)**2
simplified = expr.simplify()
# Result: x^2 * y^2

# Matrices (noncommutative)
A = symbol('A', matrix=True)
B = symbol('B', matrix=True)
expr = (A * B)**2
simplified = expr.simplify()
# Result: (A*B)^2

```

### JavaScript

```javascript
import { symbol, parse } from 'mathhook';

// Scalars (commutative)
const expr = parse('(x * y)^2');
const simplified = expr.simplify();
// Result: x^2 * y^2

// Matrices (noncommutative)
const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });
const expr2 = parse('(A * B)^2');
const simplified2 = expr2.simplify();
// Result: (A*B)^2

```



---
chunk_id: api_algebra_simplification::4
topic: api.algebra.simplification
title: Rational Arithmetic
priority: medium
keywords:
  - simplification
  - rational arithmetic
languages: [rust, python, javascript]
chunk: 5/5
---

## Rational Arithmetic

Exact rational computation with arbitrary precision

### Rust

```rust
use mathhook::prelude::*;

let expr = expr!(1/3 + 1/6);  // Rational arithmetic
let simplified = expr.simplify();
// Result: 1/2 (exact rational, not 0.5)

// Arbitrary precision
let expr = expr!(1/999999999 + 1/999999999);
let simplified = expr.simplify();
// Result: 2/999999999 (exact, no overflow)

```

### Python

```python
from mathhook import expr as parse_expr

expr = parse_expr('1/3 + 1/6')
simplified = expr.simplify()
# Result: 1/2 (exact rational)

# Arbitrary precision
expr = parse_expr('1/999999999 + 1/999999999')
simplified = expr.simplify()
# Result: 2/999999999 (exact)

```

### JavaScript

```javascript
import { parse } from 'mathhook';

const expr = parse('1/3 + 1/6');
const simplified = expr.simplify();
// Result: 1/2 (exact rational)

// Arbitrary precision
const expr2 = parse('1/999999999 + 1/999999999');
const simplified2 = expr2.simplify();
// Result: 2/999999999 (exact)

```



