# Finite Field Arithmetic

Arithmetic in Z_p (integers modulo a prime p) essential for modular GCD algorithms. Provides
field elements, polynomial operations over finite fields, and Chinese Remainder Theorem reconstruction.


---
chunk_id: polynomial_finite-field::0
topic: polynomial.finite-field
title: Field Element Arithmetic
priority: medium
keywords:
  - finite-field
  - field element arithmetic
languages: [rust, python, javascript]
chunk: 1/6
---

## Field Element Arithmetic

Basic operations in Z_p with automatic modular reduction

### Rust

```rust
use mathhook_core::core::polynomial::finite_field::FieldElement;

// Create elements in Z_7
let a = FieldElement::new(3, 7);  // 3 mod 7
let b = FieldElement::new(5, 7);  // 5 mod 7

// Arithmetic
let sum = a + b;       // 8 mod 7 = 1
let diff = a - b;      // -2 mod 7 = 5
let prod = a * b;      // 15 mod 7 = 1
let quot = a / b;      // 3 * 5^(-1) mod 7 = 3 * 3 = 9 mod 7 = 2

// Inverse
let inv = b.inverse(); // 5^(-1) mod 7 = 3 (since 5*3 = 15 = 1 mod 7)

```

### Python

```python
from mathhook.polynomial.finite_field import FieldElement

# Create elements in Z_7
a = FieldElement(3, 7)  # 3 mod 7
b = FieldElement(5, 7)  # 5 mod 7

# Arithmetic
sum_val = a + b       # 8 mod 7 = 1
diff = a - b          # -2 mod 7 = 5
prod = a * b          # 15 mod 7 = 1
quot = a / b          # 3 * 5^(-1) mod 7 = 3 * 3 = 9 mod 7 = 2

# Inverse
inv = b.inverse()     # 5^(-1) mod 7 = 3

```

### JavaScript

```javascript
const { FieldElement } = require('mathhook/polynomial/finite_field');

// Create elements in Z_7
const a = new FieldElement(3, 7);  // 3 mod 7
const b = new FieldElement(5, 7);  // 5 mod 7

// Arithmetic
const sum = a.add(b);      // 8 mod 7 = 1
const diff = a.sub(b);     // -2 mod 7 = 5
const prod = a.mul(b);     // 15 mod 7 = 1
const quot = a.div(b);     // 3 * 5^(-1) mod 7 = 2

// Inverse
const inv = b.inverse();   // 5^(-1) mod 7 = 3

```



---
chunk_id: polynomial_finite-field::1
topic: polynomial.finite-field
title: Polynomial Operations in Z_p[x]
priority: medium
keywords:
  - finite-field
  - polynomial operations in z_p[x]
languages: [rust, python, javascript]
chunk: 2/6
---

## Polynomial Operations in Z_p[x]

Create and manipulate polynomials over finite fields

### Rust

```rust
use mathhook_core::core::polynomial::finite_field::PolyZp;

// Create polynomial x^2 + 2x + 1 in Z_5[x]
let p = PolyZp::from_coeffs(vec![1, 2, 1], 5);  // [a_0, a_1, a_2]

// Polynomial properties
let deg = p.degree();           // Some(2)
let coeffs = p.coefficients();  // [1, 2, 1]

// Create from integer coefficients (auto-reduce mod p)
let q = PolyZp::from_coeffs(vec![7, -3, 6], 5);  // becomes [2, 2, 1]

```

### Python

```python
from mathhook.polynomial.finite_field import PolyZp

# Create polynomial x^2 + 2x + 1 in Z_5[x]
p = PolyZp.from_coeffs([1, 2, 1], 5)  # [a_0, a_1, a_2]

# Polynomial properties
deg = p.degree()           # 2
coeffs = p.coefficients()  # [1, 2, 1]

# Create from integer coefficients (auto-reduce mod p)
q = PolyZp.from_coeffs([7, -3, 6], 5)  # becomes [2, 2, 1]

```

### JavaScript

```javascript
const { PolyZp } = require('mathhook/polynomial/finite_field');

// Create polynomial x^2 + 2x + 1 in Z_5[x]
const p = PolyZp.fromCoeffs([1, 2, 1], 5);  // [a_0, a_1, a_2]

// Polynomial properties
const deg = p.degree();           // 2
const coeffs = p.coefficients();  // [1, 2, 1]

// Create from integer coefficients (auto-reduce mod p)
const q = PolyZp.fromCoeffs([7, -3, 6], 5);  // becomes [2, 2, 1]

```



---
chunk_id: polynomial_finite-field::2
topic: polynomial.finite-field
title: Polynomial Arithmetic in Z_p[x]
priority: medium
keywords:
  - finite-field
  - polynomial arithmetic in z_p[x]
languages: [rust, python, javascript]
chunk: 3/6
---

## Polynomial Arithmetic in Z_p[x]

Add, multiply, divide polynomials over finite fields

### Rust

```rust
use mathhook_core::core::polynomial::finite_field::PolyZp;

let f = PolyZp::from_coeffs(vec![1, 0, 1], 5);  // x^2 + 1
let g = PolyZp::from_coeffs(vec![1, 1], 5);     // x + 1

// Addition
let sum = f.add(&g);

// Multiplication
let prod = f.mul(&g);

// Division (quotient and remainder)
let (quotient, remainder) = f.div_rem(&g);

// Scalar multiplication
let scaled = f.scalar_mul(3);  // 3(x^2 + 1) = 3x^2 + 3

```

### Python

```python
from mathhook.polynomial.finite_field import PolyZp

f = PolyZp.from_coeffs([1, 0, 1], 5)  # x^2 + 1
g = PolyZp.from_coeffs([1, 1], 5)     # x + 1

# Addition
sum_poly = f.add(g)

# Multiplication
prod = f.mul(g)

# Division (quotient and remainder)
quotient, remainder = f.div_rem(g)

# Scalar multiplication
scaled = f.scalar_mul(3)  # 3(x^2 + 1) = 3x^2 + 3

```

### JavaScript

```javascript
const { PolyZp } = require('mathhook/polynomial/finite_field');

const f = PolyZp.fromCoeffs([1, 0, 1], 5);  // x^2 + 1
const g = PolyZp.fromCoeffs([1, 1], 5);     // x + 1

// Addition
const sum = f.add(g);

// Multiplication
const prod = f.mul(g);

// Division (quotient and remainder)
const [quotient, remainder] = f.divRem(g);

// Scalar multiplication
const scaled = f.scalarMul(3);  // 3(x^2 + 1) = 3x^2 + 3

```



---
chunk_id: polynomial_finite-field::3
topic: polynomial.finite-field
title: GCD in Z_p[x]
priority: medium
keywords:
  - finite-field
  - gcd in z_p[x]
languages: [rust, python, javascript]
chunk: 4/6
---

## GCD in Z_p[x]

Compute GCD using Euclidean algorithm in finite field polynomial ring

### Rust

```rust
use mathhook_core::core::polynomial::finite_field::PolyZp;

let f = PolyZp::from_coeffs(vec![4, 0, 0, 1], 5);  // x^3 + 4 in Z_5[x]
let g = PolyZp::from_coeffs(vec![1, 1], 5);         // x + 1 in Z_5[x]

// Compute GCD
let gcd = f.gcd(&g).unwrap();

```

### Python

```python
from mathhook.polynomial.finite_field import PolyZp

f = PolyZp.from_coeffs([4, 0, 0, 1], 5)  # x^3 + 4 in Z_5[x]
g = PolyZp.from_coeffs([1, 1], 5)         # x + 1 in Z_5[x]

# Compute GCD
gcd = f.gcd(g)

```

### JavaScript

```javascript
const { PolyZp } = require('mathhook/polynomial/finite_field');

const f = PolyZp.fromCoeffs([4, 0, 0, 1], 5);  // x^3 + 4 in Z_5[x]
const g = PolyZp.fromCoeffs([1, 1], 5);         // x + 1 in Z_5[x]

// Compute GCD
const gcd = f.gcd(g);

```



---
chunk_id: polynomial_finite-field::4
topic: polynomial.finite-field
title: Extended GCD with Bezout Coefficients
priority: medium
keywords:
  - finite-field
  - extended gcd with bezout coefficients
languages: [rust, python, javascript]
chunk: 5/6
---

## Extended GCD with Bezout Coefficients

Get GCD along with coefficients satisfying gcd = s*f + t*g

### Rust

```rust
use mathhook_core::core::polynomial::finite_field::PolyZp;

let f = PolyZp::from_coeffs(vec![1, 0, 1], 5);  // x^2 + 1
let g = PolyZp::from_coeffs(vec![1, 1], 5);     // x + 1

// Extended GCD: gcd = s*f + t*g
let (gcd, s, t) = f.extended_gcd(&g);

```

### Python

```python
from mathhook.polynomial.finite_field import PolyZp

f = PolyZp.from_coeffs([1, 0, 1], 5)  # x^2 + 1
g = PolyZp.from_coeffs([1, 1], 5)     # x + 1

# Extended GCD: gcd = s*f + t*g
gcd, s, t = f.extended_gcd(g)

```

### JavaScript

```javascript
const { PolyZp } = require('mathhook/polynomial/finite_field');

const f = PolyZp.fromCoeffs([1, 0, 1], 5);  // x^2 + 1
const g = PolyZp.fromCoeffs([1, 1], 5);     // x + 1

// Extended GCD: gcd = s*f + t*g
const [gcd, s, t] = f.extendedGcd(g);

```



---
chunk_id: polynomial_finite-field::5
topic: polynomial.finite-field
title: CRT Reconstruction
priority: medium
keywords:
  - finite-field
  - crt reconstruction
languages: [rust, python, javascript]
chunk: 6/6
---

## CRT Reconstruction

Combine results from multiple primes using Chinese Remainder Theorem

### Rust

```rust
use mathhook_core::core::polynomial::algorithms::zippel_gcd::helpers::crt_combine_u128;

// Combine results from two primes
let coef1 = 3;      // result mod p1
let mod1 = 7u128;   // first prime
let coef2 = 5;      // result mod p2
let mod2 = 11u128;  // second prime

let combined = crt_combine_u128(coef1, mod1, coef2, mod2);
// combined is the unique value in range 0 to 77 satisfying both constraints

```

### Python

```python
from mathhook.polynomial.algorithms.zippel_gcd.helpers import crt_combine

# Combine results from two primes
coef1 = 3      # result mod p1
mod1 = 7       # first prime
coef2 = 5      # result mod p2
mod2 = 11      # second prime

combined = crt_combine(coef1, mod1, coef2, mod2)
# combined is the unique value in range 0 to 77 satisfying both constraints

```

### JavaScript

```javascript
const { crtCombine } = require('mathhook/polynomial/algorithms/zippel_gcd/helpers');

// Combine results from two primes
const coef1 = 3;      // result mod p1
const mod1 = 7;       // first prime
const coef2 = 5;      // result mod p2
const mod2 = 11;      // second prime

const combined = crtCombine(coef1, mod1, coef2, mod2);
// combined is the unique value in range 0 to 77 satisfying both constraints

```



