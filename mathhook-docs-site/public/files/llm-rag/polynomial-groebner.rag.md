# Groebner Bases

Groebner bases are fundamental tools in computational algebraic geometry for working with polynomial ideals,
enabling ideal membership testing, polynomial system solving, variable elimination, and geometric theorem proving.


---
chunk_id: polynomial_groebner::0
topic: polynomial.groebner
title: Basic Groebner Basis Computation
priority: medium
keywords:
  - groebner
  - basic groebner basis computation
languages: [rust, python, javascript]
chunk: 1/4
---

## Basic Groebner Basis Computation

Compute Groebner basis for a polynomial ideal

### Rust

```rust
use mathhook_core::core::polynomial::groebner::{GroebnerBasis, MonomialOrder};
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let y = symbol!(y);

// Define polynomials: f1 = x - y, f2 = y^2 - 1
let f1 = expr!(x - y);
let f2 = expr!((y ^ 2) - 1);

// Create Groebner basis
let mut gb = GroebnerBasis::new(
    vec![f1, f2],
    vec![x.clone(), y.clone()],
    MonomialOrder::Lex
);

// Compute the basis
gb.compute();

println!("Basis has {} polynomials", gb.basis.len());

```

### Python

```python
from mathhook import expr, symbol
from mathhook.polynomial.groebner import GroebnerBasis, MonomialOrder

x = symbol('x')
y = symbol('y')

# Define polynomials: f1 = x - y, f2 = y^2 - 1
f1 = expr('x - y')
f2 = expr('y^2 - 1')

# Create Groebner basis
gb = GroebnerBasis(
    [f1, f2],
    [x, y],
    MonomialOrder.Lex
)

# Compute the basis
gb.compute()

print(f"Basis has {len(gb.basis)} polynomials")

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');
const { GroebnerBasis, MonomialOrder } = require('mathhook/polynomial/groebner');

const x = symbol('x');
const y = symbol('y');

// Define polynomials: f1 = x - y, f2 = y^2 - 1
const f1 = expr('x - y');
const f2 = expr('y^2 - 1');

// Create Groebner basis
const gb = new GroebnerBasis(
    [f1, f2],
    [x, y],
    MonomialOrder.Lex
);

// Compute the basis
gb.compute();

console.log(`Basis has ${gb.basis.length} polynomials`);

```



---
chunk_id: polynomial_groebner::1
topic: polynomial.groebner
title: Sparse Polynomial Representation
priority: medium
keywords:
  - groebner
  - sparse polynomial representation
languages: [rust, python, javascript]
chunk: 2/4
---

## Sparse Polynomial Representation

Work with sparse polynomials for efficiency

### Rust

```rust
use mathhook_core::core::polynomial::groebner::{Monomial, expression_to_sparse_polynomial};
use mathhook_core::{expr, symbol};

// Create a monomial x^2 * y (exponents [2, 1])
let mono = Monomial::new(vec![2, 1]);
assert_eq!(mono.degree(), 3);

// Convert expression to sparse polynomial
let x = symbol!(x);
let y = symbol!(y);
let poly = expr!((x ^ 2) + y);

let vars = vec![x, y];
let sparse = expression_to_sparse_polynomial(&poly, &vars);

```

### Python

```python
from mathhook import expr, symbol
from mathhook.polynomial.groebner import Monomial, expression_to_sparse_polynomial

# Create a monomial x^2 * y (exponents [2, 1])
mono = Monomial([2, 1])
assert mono.degree() == 3

# Convert expression to sparse polynomial
x = symbol('x')
y = symbol('y')
poly = expr('x^2 + y')

vars = [x, y]
sparse = expression_to_sparse_polynomial(poly, vars)

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');
const { Monomial, expressionToSparsePolynomial } = require('mathhook/polynomial/groebner');

// Create a monomial x^2 * y (exponents [2, 1])
const mono = new Monomial([2, 1]);
assert(mono.degree() === 3);

// Convert expression to sparse polynomial
const x = symbol('x');
const y = symbol('y');
const poly = expr('x^2 + y');

const vars = [x, y];
const sparse = expressionToSparsePolynomial(poly, vars);

```



---
chunk_id: polynomial_groebner::2
topic: polynomial.groebner
title: Polynomial Reduction
priority: medium
keywords:
  - groebner
  - polynomial reduction
languages: [rust, python, javascript]
chunk: 3/4
---

## Polynomial Reduction

Reduce polynomial by a set of polynomials (division algorithm)

### Rust

```rust
use mathhook_core::core::polynomial::groebner::{
    poly_reduce,
    poly_reduce_completely
};

// Single-step reduction
let reduced = poly_reduce(&poly, &basis, &order);

// Complete reduction (until no further reduction possible)
let fully_reduced = poly_reduce_completely(&poly, &basis, &order);

```

### Python

```python
from mathhook.polynomial.groebner import poly_reduce, poly_reduce_completely

# Single-step reduction
reduced = poly_reduce(poly, basis, order)

# Complete reduction (until no further reduction possible)
fully_reduced = poly_reduce_completely(poly, basis, order)

```

### JavaScript

```javascript
const { polyReduce, polyReduceCompletely } = require('mathhook/polynomial/groebner');

// Single-step reduction
const reduced = polyReduce(poly, basis, order);

// Complete reduction (until no further reduction possible)
const fullyReduced = polyReduceCompletely(poly, basis, order);

```



---
chunk_id: polynomial_groebner::3
topic: polynomial.groebner
title: Bidirectional Expression Conversion
priority: medium
keywords:
  - groebner
  - bidirectional expression conversion
languages: [rust, python, javascript]
chunk: 4/4
---

## Bidirectional Expression Conversion

Convert between Expression and sparse polynomial representation

### Rust

```rust
use mathhook_core::core::polynomial::groebner::{
    expression_to_sparse_polynomial,
    sparse_polynomial_to_expression
};
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let y = symbol!(y);
let vars = vec![x.clone(), y.clone()];

// Expression to sparse
let expr = expr!((x ^ 2) + y);
let sparse = expression_to_sparse_polynomial(&expr, &vars).unwrap();

// Sparse back to expression
let back = sparse_polynomial_to_expression(&sparse, &vars);

```

### Python

```python
from mathhook import expr, symbol
from mathhook.polynomial.groebner import (
    expression_to_sparse_polynomial,
    sparse_polynomial_to_expression
)

x = symbol('x')
y = symbol('y')
vars = [x, y]

# Expression to sparse
e = expr('x^2 + y')
sparse = expression_to_sparse_polynomial(e, vars)

# Sparse back to expression
back = sparse_polynomial_to_expression(sparse, vars)

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');
const {
    expressionToSparsePolynomial,
    sparsePolynomialToExpression
} = require('mathhook/polynomial/groebner');

const x = symbol('x');
const y = symbol('y');
const vars = [x, y];

// Expression to sparse
const e = expr('x^2 + y');
const sparse = expressionToSparsePolynomial(e, vars);

// Sparse back to expression
const back = sparsePolynomialToExpression(sparse, vars);

```



