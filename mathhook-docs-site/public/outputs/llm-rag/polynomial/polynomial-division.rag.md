# Polynomial Division and Factorization

Polynomial division algorithms including long division, exact division, and factorization capabilities
such as square-free factorization, resultant, and discriminant computation.


---
chunk_id: polynomial_division::0
topic: polynomial.division
title: Polynomial Long Division
priority: medium
keywords:
  - division
  - polynomial long division
languages: [rust, python, javascript]
chunk: 1/5
---

## Polynomial Long Division

Compute quotient and remainder using standard division algorithm

### Rust

```rust
use mathhook_core::core::polynomial::algorithms::polynomial_long_division;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// Divide (x^2 - 1) by (x - 1)
let dividend = expr!((x ^ 2) - 1);
let divisor = expr!(x - 1);

let (quotient, remainder) = polynomial_long_division(&dividend, &divisor, &x).unwrap();

// quotient = x + 1
// remainder = 0
// Verify: dividend = divisor * quotient + remainder

```

### Python

```python
from mathhook import expr, symbol
from mathhook.polynomial.algorithms import polynomial_long_division

x = symbol('x')

# Divide (x^2 - 1) by (x - 1)
dividend = expr('x^2 - 1')
divisor = expr('x - 1')

quotient, remainder = polynomial_long_division(dividend, divisor, x)

# quotient = x + 1
# remainder = 0
# Verify: dividend = divisor * quotient + remainder

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');
const { polynomialLongDivision } = require('mathhook/polynomial/algorithms');

const x = symbol('x');

// Divide (x^2 - 1) by (x - 1)
const dividend = expr('x^2 - 1');
const divisor = expr('x - 1');

const [quotient, remainder] = polynomialLongDivision(dividend, divisor, x);

// quotient = x + 1
// remainder = 0
// Verify: dividend = divisor * quotient + remainder

```



---
chunk_id: polynomial_division::1
topic: polynomial.division
title: Exact Division
priority: medium
keywords:
  - division
  - exact division
languages: [rust, python, javascript]
chunk: 2/5
---

## Exact Division

Division that errors if remainder is non-zero

### Rust

```rust
use mathhook_core::core::polynomial::algorithms::exact_division;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// x^2 / x = x (exact)
let dividend = expr!(x ^ 2);
let divisor = expr!(x);

match exact_division(&dividend, &divisor, &x) {
    Ok(quotient) => println!("Exact quotient: {}", quotient),
    Err(e) => println!("Division not exact: {:?}", e),
}

```

### Python

```python
from mathhook import expr, symbol
from mathhook.polynomial.algorithms import exact_division

x = symbol('x')

# x^2 / x = x (exact)
dividend = expr('x^2')
divisor = expr('x')

try:
    quotient = exact_division(dividend, divisor, x)
    print(f"Exact quotient: {quotient}")
except Exception as e:
    print(f"Division not exact: {e}")

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');
const { exactDivision } = require('mathhook/polynomial/algorithms');

const x = symbol('x');

// x^2 / x = x (exact)
const dividend = expr('x^2');
const divisor = expr('x');

try {
    const quotient = exactDivision(dividend, divisor, x);
    console.log(`Exact quotient: ${quotient}`);
} catch (e) {
    console.log(`Division not exact: ${e}`);
}

```



---
chunk_id: polynomial_division::2
topic: polynomial.division
title: Trait-Based Division
priority: medium
keywords:
  - division
  - trait-based division
languages: [rust, python, javascript]
chunk: 3/5
---

## Trait-Based Division

Use PolynomialArithmetic trait for ergonomic API

### Rust

```rust
use mathhook_core::core::polynomial::PolynomialArithmetic;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

let f = expr!((x ^ 3) - 1);
let g = expr!(x - 1);

// Returns (quotient, remainder)
let (q, r) = f.poly_div(&g, &x).unwrap();
// q = x^2 + x + 1
// r = 0

```

### Python

```python
from mathhook import expr, symbol

x = symbol('x')

f = expr('x^3 - 1')
g = expr('x - 1')

# Returns (quotient, remainder)
q, r = f.poly_div(g, x)
# q = x^2 + x + 1
# r = 0

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');

const x = symbol('x');

const f = expr('x^3 - 1');
const g = expr('x - 1');

// Returns (quotient, remainder)
const [q, r] = f.polyDiv(g, x);
// q = x^2 + x + 1
// r = 0

```



---
chunk_id: polynomial_division::3
topic: polynomial.division
title: Polynomial Resultant
priority: medium
keywords:
  - division
  - polynomial resultant
languages: [rust, python, javascript]
chunk: 4/5
---

## Polynomial Resultant

Test for common roots using resultant

### Rust

```rust
use mathhook_core::core::polynomial::algorithms::polynomial_resultant;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// p1 = x - 1
let p1 = expr!(x - 1);
// p2 = x - 2
let p2 = expr!(x - 2);

let res = polynomial_resultant(&p1, &p2, &x).unwrap();
// Resultant is non-zero (distinct roots)

```

### Python

```python
from mathhook import expr, symbol
from mathhook.polynomial.algorithms import polynomial_resultant

x = symbol('x')

# p1 = x - 1
p1 = expr('x - 1')
# p2 = x - 2
p2 = expr('x - 2')

res = polynomial_resultant(p1, p2, x)
# Resultant is non-zero (distinct roots)

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');
const { polynomialResultant } = require('mathhook/polynomial/algorithms');

const x = symbol('x');

// p1 = x - 1
const p1 = expr('x - 1');
// p2 = x - 2
const p2 = expr('x - 2');

const res = polynomialResultant(p1, p2, x);
// Resultant is non-zero (distinct roots)

```



---
chunk_id: polynomial_division::4
topic: polynomial.division
title: Polynomial Discriminant
priority: medium
keywords:
  - division
  - polynomial discriminant
languages: [rust, python, javascript]
chunk: 5/5
---

## Polynomial Discriminant

Detect repeated roots using discriminant

### Rust

```rust
use mathhook_core::core::polynomial::algorithms::polynomial_discriminant;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// (x - 1)^2 = x^2 - 2x + 1 (repeated root at 1)
let poly = expr!((x ^ 2) - (2 * x) + 1);

let disc = polynomial_discriminant(&poly, &x).unwrap();
// Discriminant = 0 (repeated root)

```

### Python

```python
from mathhook import expr, symbol
from mathhook.polynomial.algorithms import polynomial_discriminant

x = symbol('x')

# (x - 1)^2 = x^2 - 2x + 1 (repeated root at 1)
poly = expr('x^2 - 2*x + 1')

disc = polynomial_discriminant(poly, x)
# Discriminant = 0 (repeated root)

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');
const { polynomialDiscriminant } = require('mathhook/polynomial/algorithms');

const x = symbol('x');

// (x - 1)^2 = x^2 - 2x + 1 (repeated root at 1)
const poly = expr('x^2 - 2*x + 1');

const disc = polynomialDiscriminant(poly, x);
// Discriminant = 0 (repeated root)

```



