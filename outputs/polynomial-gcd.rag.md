# GCD Algorithms

Multiple GCD (Greatest Common Divisor) algorithms for polynomials, optimized for different use cases
including univariate, multivariate, and modular GCD using Zippel's algorithm.


---
chunk_id: polynomial_gcd::0
topic: polynomial.gcd
title: General-Purpose GCD
priority: medium
keywords:
  - gcd
  - general-purpose gcd
languages: [rust, python, javascript]
chunk: 1/4
---

## General-Purpose GCD

Use PolynomialGcdOps trait for automatic algorithm selection

### Rust

```rust
use mathhook_core::core::polynomial::PolynomialGcdOps;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// f = x^2 - 1 = (x-1)(x+1)
let f = expr!((x ^ 2) - 1);
// g = x^2 - 2x + 1 = (x-1)^2
let g = expr!((x ^ 2) - (2 * x) + 1);

// Compute GCD
let gcd = f.polynomial_gcd(&g).unwrap();
// gcd = x - 1

// Compute LCM
let lcm = f.polynomial_lcm(&g).unwrap();
// lcm = (x-1)^2(x+1)

```

### Python

```python
from mathhook import expr, symbol
from mathhook.polynomial import PolynomialOps

x = symbol('x')

# f = x^2 - 1 = (x-1)(x+1)
f = expr('x^2 - 1')
# g = x^2 - 2x + 1 = (x-1)^2
g = expr('x^2 - 2*x + 1')

# Compute GCD
gcd = f.polynomial_gcd(g)
# gcd = x - 1

# Compute LCM
lcm = f.polynomial_lcm(g)
# lcm = (x-1)^2(x+1)

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');

const x = symbol('x');

// f = x^2 - 1 = (x-1)(x+1)
const f = expr('x^2 - 1');
// g = x^2 - 2x + 1 = (x-1)^2
const g = expr('x^2 - 2*x + 1');

// Compute GCD
const gcd = f.polynomialGcd(g);
// gcd = x - 1

// Compute LCM
const lcm = f.polynomialLcm(g);
// lcm = (x-1)^2(x+1)

```



---
chunk_id: polynomial_gcd::1
topic: polynomial.gcd
title: Univariate Modular GCD with Cofactors
priority: medium
keywords:
  - gcd
  - univariate modular gcd with cofactors
languages: [rust, python, javascript]
chunk: 2/4
---

## Univariate Modular GCD with Cofactors

Returns GCD and cofactors for Bezout identity verification

### Rust

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::modular_gcd_univariate;
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let f = expr!((x ^ 2) - 1);
let g = expr!(x - 1);

// Returns (gcd, cofactor_f, cofactor_g)
let (gcd, cof_f, cof_g) = modular_gcd_univariate(&f, &g, &x).unwrap();

// Verify: f = gcd * cof_f, g = gcd * cof_g

```

### Python

```python
from mathhook import expr, symbol
from mathhook.polynomial.algorithms import modular_gcd_univariate

x = symbol('x')
f = expr('x^2 - 1')
g = expr('x - 1')

# Returns (gcd, cofactor_f, cofactor_g)
gcd, cof_f, cof_g = modular_gcd_univariate(f, g, x)

# Verify: f = gcd * cof_f, g = gcd * cof_g

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');
const { modularGcdUnivariate } = require('mathhook/polynomial/algorithms');

const x = symbol('x');
const f = expr('x^2 - 1');
const g = expr('x - 1');

// Returns (gcd, cofactor_f, cofactor_g)
const [gcd, cofF, cofG] = modularGcdUnivariate(f, g, x);

// Verify: f = gcd * cofF, g = gcd * cofG

```



---
chunk_id: polynomial_gcd::2
topic: polynomial.gcd
title: Multivariate GCD with Zippel Algorithm
priority: medium
keywords:
  - gcd
  - multivariate gcd with zippel algorithm
languages: [rust, python, javascript]
chunk: 3/4
---

## Multivariate GCD with Zippel Algorithm

Compute GCD for polynomials in multiple variables

### Rust

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::{
    multivariate_gcd_zippel,
    MultivariateGcdConfig
};
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let y = symbol!(y);

// f = x*y, g = x*y + x
let f = expr!(x * y);
let g = expr!((x * y) + x);

let config = MultivariateGcdConfig::default();
let (gcd, _, _) = multivariate_gcd_zippel(&f, &g, &[x, y], config).unwrap();
// gcd = x

```

### Python

```python
from mathhook import expr, symbol
from mathhook.polynomial.algorithms import multivariate_gcd_zippel, MultivariateGcdConfig

x = symbol('x')
y = symbol('y')

# f = x*y, g = x*y + x
f = expr('x*y')
g = expr('x*y + x')

config = MultivariateGcdConfig()
gcd, _, _ = multivariate_gcd_zippel(f, g, [x, y], config)
# gcd = x

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');
const { multivariateGcdZippel, MultivariateGcdConfig } = require('mathhook/polynomial/algorithms');

const x = symbol('x');
const y = symbol('y');

// f = x*y, g = x*y + x
const f = expr('x*y');
const g = expr('x*y + x');

const config = new MultivariateGcdConfig();
const [gcd, _, __] = multivariateGcdZippel(f, g, [x, y], config);
// gcd = x

```



---
chunk_id: polynomial_gcd::3
topic: polynomial.gcd
title: Content and Primitive Part Decomposition
priority: medium
keywords:
  - gcd
  - content and primitive part decomposition
languages: [rust, python, javascript]
chunk: 4/4
---

## Content and Primitive Part Decomposition

Fundamental operation for GCD computation

### Rust

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::{
    extract_content,
    primitive_part
};

let coeffs = vec![6, 12, 18];  // 6 + 12x + 18x^2

// Extract content (GCD of coefficients)
let content = extract_content(&coeffs);  // 6

// Get primitive part
let (cont, pp) = primitive_part(&coeffs);  // (6, [1, 2, 3])

```

### Python

```python
from mathhook.polynomial.algorithms import extract_content, primitive_part

coeffs = [6, 12, 18]  # 6 + 12x + 18x^2

# Extract content (GCD of coefficients)
content = extract_content(coeffs)  # 6

# Get primitive part
cont, pp = primitive_part(coeffs)  # (6, [1, 2, 3])

```

### JavaScript

```javascript
const { extractContent, primitivePart } = require('mathhook/polynomial/algorithms');

const coeffs = [6, 12, 18];  // 6 + 12x + 18x^2

// Extract content (GCD of coefficients)
const content = extractContent(coeffs);  // 6

// Get primitive part
const [cont, pp] = primitivePart(coeffs);  // (6, [1, 2, 3])

```



