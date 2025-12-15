# Special Polynomial Families

Classical orthogonal polynomial families including Legendre, Chebyshev (1st and 2nd kind),
Hermite, and Laguerre polynomials with both symbolic expansion and numerical evaluation capabilities.


---
chunk_id: polynomial_special-families::0
topic: polynomial.special-families
title: Legendre Polynomials
priority: medium
keywords:
  - special-families
  - legendre polynomials
languages: [rust, python, javascript]
chunk: 1/5
---

## Legendre Polynomials

Solutions to Legendre's differential equation

### Rust

```rust
use mathhook_core::core::polynomial::special_families::Legendre;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

let x = symbol!(x);

// Symbolic expansion
let p0 = Legendre::polynomial(0, &x);  // 1
let p1 = Legendre::polynomial(1, &x);  // x
let p2 = Legendre::polynomial(2, &x);  // (3x^2 - 1)/2

// Numerical evaluation
let val = Legendre::evaluate(2, 0.5);  // P_2(0.5) = -0.125

// Recurrence: P_{n+1} = ((2n+1)x*P_n - n*P_{n-1}) / (n+1)
let (a, b, c) = Legendre::recurrence_coefficients(2);

```

### Python

```python
from mathhook import symbol
from mathhook.polynomial.special_families import Legendre

x = symbol('x')

# Symbolic expansion
p0 = Legendre.polynomial(0, x)  # 1
p1 = Legendre.polynomial(1, x)  # x
p2 = Legendre.polynomial(2, x)  # (3*x^2 - 1)/2

# Numerical evaluation
val = Legendre.evaluate(2, 0.5)  # P_2(0.5) = -0.125

# Recurrence: P_{n+1} = ((2n+1)*x*P_n - n*P_{n-1}) / (n+1)
a, b, c = Legendre.recurrence_coefficients(2)

```

### JavaScript

```javascript
const { symbol } = require('mathhook');
const { Legendre } = require('mathhook/polynomial/special_families');

const x = symbol('x');

// Symbolic expansion
const p0 = Legendre.polynomial(0, x);  // 1
const p1 = Legendre.polynomial(1, x);  // x
const p2 = Legendre.polynomial(2, x);  // (3*x^2 - 1)/2

// Numerical evaluation
const val = Legendre.evaluate(2, 0.5);  // P_2(0.5) = -0.125

// Recurrence: P_{n+1} = ((2n+1)*x*P_n - n*P_{n-1}) / (n+1)
const [a, b, c] = Legendre.recurrenceCoefficients(2);

```



---
chunk_id: polynomial_special-families::1
topic: polynomial.special-families
title: Chebyshev Polynomials (First Kind)
priority: medium
keywords:
  - special-families
  - chebyshev polynomials (first kind)
languages: [rust, python, javascript]
chunk: 2/5
---

## Chebyshev Polynomials (First Kind)

Defined by T_n(cos(theta)) = cos(n*theta)

### Rust

```rust
use mathhook_core::core::polynomial::special_families::ChebyshevT;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

let x = symbol!(x);

// Symbolic
let t0 = ChebyshevT::polynomial(0, &x);  // 1
let t1 = ChebyshevT::polynomial(1, &x);  // x
let t2 = ChebyshevT::polynomial(2, &x);  // 2x^2 - 1

// Numerical
let val = ChebyshevT::evaluate(2, 0.5);  // T_2(0.5) = -0.5

// Recurrence: T_{n+1} = 2x*T_n - T_{n-1}

```

### Python

```python
from mathhook import symbol
from mathhook.polynomial.special_families import ChebyshevT

x = symbol('x')

# Symbolic
t0 = ChebyshevT.polynomial(0, x)  # 1
t1 = ChebyshevT.polynomial(1, x)  # x
t2 = ChebyshevT.polynomial(2, x)  # 2*x^2 - 1

# Numerical
val = ChebyshevT.evaluate(2, 0.5)  # T_2(0.5) = -0.5

# Recurrence: T_{n+1} = 2*x*T_n - T_{n-1}

```

### JavaScript

```javascript
const { symbol } = require('mathhook');
const { ChebyshevT } = require('mathhook/polynomial/special_families');

const x = symbol('x');

// Symbolic
const t0 = ChebyshevT.polynomial(0, x);  // 1
const t1 = ChebyshevT.polynomial(1, x);  // x
const t2 = ChebyshevT.polynomial(2, x);  // 2*x^2 - 1

// Numerical
const val = ChebyshevT.evaluate(2, 0.5);  // T_2(0.5) = -0.5

// Recurrence: T_{n+1} = 2*x*T_n - T_{n-1}

```



---
chunk_id: polynomial_special-families::2
topic: polynomial.special-families
title: Hermite Polynomials
priority: medium
keywords:
  - special-families
  - hermite polynomials
languages: [rust, python, javascript]
chunk: 3/5
---

## Hermite Polynomials

Solutions to Hermite's equation (physicist's convention)

### Rust

```rust
use mathhook_core::core::polynomial::special_families::Hermite;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

let x = symbol!(x);

// Symbolic
let h0 = Hermite::polynomial(0, &x);  // 1
let h1 = Hermite::polynomial(1, &x);  // 2x
let h2 = Hermite::polynomial(2, &x);  // 4x^2 - 2

// Numerical
let val = Hermite::evaluate(1, 0.5);  // H_1(0.5) = 1

// Recurrence: H_{n+1} = 2x*H_n - 2n*H_{n-1}

```

### Python

```python
from mathhook import symbol
from mathhook.polynomial.special_families import Hermite

x = symbol('x')

# Symbolic
h0 = Hermite.polynomial(0, x)  # 1
h1 = Hermite.polynomial(1, x)  # 2*x
h2 = Hermite.polynomial(2, x)  # 4*x^2 - 2

# Numerical
val = Hermite.evaluate(1, 0.5)  # H_1(0.5) = 1

# Recurrence: H_{n+1} = 2*x*H_n - 2*n*H_{n-1}

```

### JavaScript

```javascript
const { symbol } = require('mathhook');
const { Hermite } = require('mathhook/polynomial/special_families');

const x = symbol('x');

// Symbolic
const h0 = Hermite.polynomial(0, x);  // 1
const h1 = Hermite.polynomial(1, x);  // 2*x
const h2 = Hermite.polynomial(2, x);  // 4*x^2 - 2

// Numerical
const val = Hermite.evaluate(1, 0.5);  // H_1(0.5) = 1

// Recurrence: H_{n+1} = 2*x*H_n - 2*n*H_{n-1}

```



---
chunk_id: polynomial_special-families::3
topic: polynomial.special-families
title: Laguerre Polynomials
priority: medium
keywords:
  - special-families
  - laguerre polynomials
languages: [rust, python, javascript]
chunk: 4/5
---

## Laguerre Polynomials

Solutions to Laguerre's equation

### Rust

```rust
use mathhook_core::core::polynomial::special_families::Laguerre;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

let x = symbol!(x);

// Symbolic
let l0 = Laguerre::polynomial(0, &x);  // 1
let l1 = Laguerre::polynomial(1, &x);  // 1 - x
let l2 = Laguerre::polynomial(2, &x);  // (x^2 - 4x + 2)/2

// Numerical
let val = Laguerre::evaluate(1, 0.5);  // L_1(0.5) = 0.5

// Recurrence: L_{n+1} = ((2n+1-x)*L_n - n*L_{n-1}) / (n+1)

```

### Python

```python
from mathhook import symbol
from mathhook.polynomial.special_families import Laguerre

x = symbol('x')

# Symbolic
l0 = Laguerre.polynomial(0, x)  # 1
l1 = Laguerre.polynomial(1, x)  # 1 - x
l2 = Laguerre.polynomial(2, x)  # (x^2 - 4*x + 2)/2

# Numerical
val = Laguerre.evaluate(1, 0.5)  # L_1(0.5) = 0.5

# Recurrence: L_{n+1} = ((2*n+1-x)*L_n - n*L_{n-1}) / (n+1)

```

### JavaScript

```javascript
const { symbol } = require('mathhook');
const { Laguerre } = require('mathhook/polynomial/special_families');

const x = symbol('x');

// Symbolic
const l0 = Laguerre.polynomial(0, x);  // 1
const l1 = Laguerre.polynomial(1, x);  // 1 - x
const l2 = Laguerre.polynomial(2, x);  // (x^2 - 4*x + 2)/2

// Numerical
const val = Laguerre.evaluate(1, 0.5);  // L_1(0.5) = 0.5

// Recurrence: L_{n+1} = ((2*n+1-x)*L_n - n*L_{n-1}) / (n+1)

```



---
chunk_id: polynomial_special-families::4
topic: polynomial.special-families
title: Variable Substitution
priority: medium
keywords:
  - special-families
  - variable substitution
languages: [rust, python, javascript]
chunk: 5/5
---

## Variable Substitution

Use any variable symbol in polynomial generation

### Rust

```rust
use mathhook_core::core::polynomial::special_families::Legendre;
use mathhook_core::core::polynomial::special_families::OrthogonalPolynomial;
use mathhook_core::symbol;

// Use variable t instead of x
let t = symbol!(t);
let p2_t = Legendre::polynomial(2, &t);
// Result uses t: (3t^2 - 1)/2

```

### Python

```python
from mathhook import symbol
from mathhook.polynomial.special_families import Legendre

# Use variable t instead of x
t = symbol('t')
p2_t = Legendre.polynomial(2, t)
# Result uses t: (3*t^2 - 1)/2

```

### JavaScript

```javascript
const { symbol } = require('mathhook');
const { Legendre } = require('mathhook/polynomial/special_families');

// Use variable t instead of x
const t = symbol('t');
const p2T = Legendre.polynomial(2, t);
// Result uses t: (3*t^2 - 1)/2

```



