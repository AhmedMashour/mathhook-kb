# Polynomial Module Overview

Comprehensive symbolic polynomial manipulation capabilities in MathHook. Implements a trait-based
architecture for automatic classification, property computation, arithmetic operations, and GCD algorithms.


---
chunk_id: polynomial_overview::0
topic: polynomial.overview
title: Basic Polynomial Usage
priority: medium
keywords:
  - overview
  - basic polynomial usage
languages: [rust, python, javascript]
chunk: 1/3
---

## Basic Polynomial Usage

Create polynomials and compute properties using trait-based API

### Rust

```rust
use mathhook_core::core::polynomial::{
    PolynomialClassification,
    PolynomialProperties,
    PolynomialGcdOps
};
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// Create polynomials using expr! macro
let f = expr!((x ^ 2) + (2 * x) + 1);  // x^2 + 2x + 1
let g = expr!((x ^ 2) - 1);             // x^2 - 1

// Properties
assert_eq!(f.degree(&x), Some(2));
assert!(f.is_polynomial_in(&[x.clone()]));

// GCD computation
let gcd = f.polynomial_gcd(&g).unwrap();
// gcd = x + 1 (since f = (x+1)^2 and g = (x+1)(x-1))

```

### Python

```python
from mathhook import expr, symbol
from mathhook.polynomial import PolynomialOps

x = symbol('x')

# Create polynomials
f = expr('x^2 + 2*x + 1')
g = expr('x^2 - 1')

# Properties
assert f.degree(x) == 2
assert f.is_polynomial_in([x])

# GCD computation
gcd = f.polynomial_gcd(g)
# gcd = x + 1

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');

const x = symbol('x');

// Create polynomials
const f = expr('x^2 + 2*x + 1');
const g = expr('x^2 - 1');

// Properties
assert(f.degree(x) === 2);
assert(f.isPolynomialIn([x]));

// GCD computation
const gcd = f.polynomialGcd(g);
// gcd = x + 1

```



---
chunk_id: polynomial_overview::1
topic: polynomial.overview
title: Polynomial Classification
priority: medium
keywords:
  - overview
  - polynomial classification
languages: [rust, python, javascript]
chunk: 2/3
---

## Polynomial Classification

Automatic detection of polynomial structure and variable extraction

### Rust

```rust
use mathhook_core::core::polynomial::PolynomialClassification;
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let y = symbol!(y);

// Automatic detection
let poly = expr!((x ^ 2) + (y * x) + 1);
assert!(poly.is_polynomial());
assert!(poly.is_polynomial_in(&[x.clone(), y.clone()]));

// Variable extraction
let vars = poly.polynomial_variables();
// vars contains x and y

```

### Python

```python
from mathhook import expr, symbol

x = symbol('x')
y = symbol('y')

# Automatic detection
poly = expr('x^2 + y*x + 1')
assert poly.is_polynomial()
assert poly.is_polynomial_in([x, y])

# Variable extraction
vars = poly.polynomial_variables()
# vars contains x and y

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');

const x = symbol('x');
const y = symbol('y');

// Automatic detection
const poly = expr('x^2 + y*x + 1');
assert(poly.isPolynomial());
assert(poly.isPolynomialIn([x, y]));

// Variable extraction
const vars = poly.polynomialVariables();
// vars contains x and y

```



---
chunk_id: polynomial_overview::2
topic: polynomial.overview
title: Content and Primitive Part
priority: medium
keywords:
  - overview
  - content and primitive part
languages: [rust, python, javascript]
chunk: 3/3
---

## Content and Primitive Part

Extract GCD of coefficients and primitive polynomial

### Rust

```rust
use mathhook_core::core::polynomial::PolynomialProperties;
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let poly = expr!((6 * (x ^ 2)) + (9 * x) + 3);  // 6x^2 + 9x + 3

let content = poly.content();           // 3
let primitive = poly.primitive_part();  // 2x^2 + 3x + 1

```

### Python

```python
from mathhook import expr, symbol

x = symbol('x')
poly = expr('6*x^2 + 9*x + 3')

content = poly.content()         # 3
primitive = poly.primitive_part() # 2*x^2 + 3*x + 1

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');

const x = symbol('x');
const poly = expr('6*x^2 + 9*x + 3');

const content = poly.content();         // 3
const primitive = poly.primitivePart(); // 2*x^2 + 3*x + 1

```



