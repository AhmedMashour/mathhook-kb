---









---

# Groebner Bases

> **Topic**: `polynomial.groebner`

Groebner bases are fundamental tools in computational algebraic geometry for working with polynomial ideals,
enabling ideal membership testing, polynomial system solving, variable elimination, and geometric theorem proving.



## Mathematical Definition

**Groebner Basis Definition**: A set $G = \{g_1, \ldots, g_m\}$ is a Groebner basis for ideal $I$
with respect to monomial order $<$ if:

$$\langle \text{LT}(g_1), \ldots, \text{LT}(g_m) \rangle = \langle \text{LT}(I) \rangle$$

where $\text{LT}$ denotes the leading term and $\langle \cdot \rangle$ denotes the ideal generated.

**S-Polynomial**: The S-polynomial of $f$ and $g$ is:

$$S(f,g) = \frac{\text{lcm}(\text{LT}(f), \text{LT}(g))}{\text{LT}(f)} \cdot f - \frac{\text{lcm}(\text{LT}(f), \text{LT}(g))}{\text{LT}(g)} \cdot g$$

**Buchberger's Criterion**: $G$ is a Groebner basis if and only if for all pairs $g_i, g_j \in G$:

$$S(g_i, g_j) \xrightarrow{G}_+ 0$$

(i.e., $S(g_i, g_j)$ reduces to 0 modulo $G$)

**Monomial Orders**:
- **Lex**: $x^\alpha > x^\beta \iff$ first non-zero entry of $\alpha - \beta$ is positive
- **Grlex**: Total degree first, then lex
- **Grevlex**: Total degree first, then reverse lex from right




Groebner bases are a fundamental tool in computational algebraic geometry for working with polynomial ideals.

## Overview

A Groebner basis is a special generating set for a polynomial ideal that has many useful computational properties:

- **Ideal Membership Testing**: Determine if a polynomial belongs to an ideal
- **Polynomial System Solving**: Find common solutions to systems of polynomial equations
- **Variable Elimination**: Eliminate variables from polynomial systems
- **Geometric Theorem Proving**: Prove geometric theorems algebraically

## Monomial Orders

The choice of monomial order affects the structure of the Groebner basis:

| Order | Description | Use Case |
|-------|-------------|----------|
| `Lex` | Lexicographic | Variable elimination |
| `Grlex` | Graded lexicographic | Balanced computation |
| `Grevlex` | Graded reverse lexicographic | Efficient computation |

### Lexicographic Order (Lex)

Compares exponents from left to right:
- $x^2y > xy^2$ (2 > 1 in first position)
- $xy^3 > xz^5$ (y > z in second position)

Best for: Variable elimination, solving systems

### Graded Lexicographic (Grlex)

Compares total degree first, then lexicographic:
- $xy^2 > x^2$ (degree 3 > 2)
- $x^2 > xy$ (same degree, $x^2 > xy$ lexicographically)

Best for: Balanced trade-off between structure and efficiency

### Graded Reverse Lexicographic (Grevlex)

Compares total degree first, then reverse lexicographic from right:
- $xy^2 > x^2y$ (same degree, compare from right)

Best for: Efficient computation (often produces smaller bases)

## Buchberger's Algorithm

The classic algorithm for computing Groebner bases:

### Algorithm Steps

1. **Initialize**: Start with the input polynomials
2. **S-pairs**: For each pair of polynomials, compute the S-polynomial
3. **Reduce**: Reduce each S-polynomial by the current basis
4. **Add**: If reduction is non-zero, add to basis
5. **Repeat**: Continue until no new polynomials are added

## Applications

### Solving Polynomial Systems

Compute a Groebner basis with Lex order to get elimination ideals.

### Ideal Membership

Test if a polynomial belongs to an ideal by checking if its remainder under the
Groebner basis is zero.

### Elimination

With Lex order x > y > z, the Groebner basis contains polynomials in:
- Only z (elimination of x and y)
- y and z (elimination of x)
- x, y, and z












## Examples


### Basic Groebner Basis Computation

Compute Groebner basis for a polynomial ideal


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

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
</details>



<details>
<summary><b>JavaScript</b></summary>

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
</details>





### Sparse Polynomial Representation

Work with sparse polynomials for efficiency


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

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
</details>



<details>
<summary><b>JavaScript</b></summary>

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
</details>





### Polynomial Reduction

Reduce polynomial by a set of polynomials (division algorithm)


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook.polynomial.groebner import poly_reduce, poly_reduce_completely

# Single-step reduction
reduced = poly_reduce(poly, basis, order)

# Complete reduction (until no further reduction possible)
fully_reduced = poly_reduce_completely(poly, basis, order)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { polyReduce, polyReduceCompletely } = require('mathhook/polynomial/groebner');

// Single-step reduction
const reduced = polyReduce(poly, basis, order);

// Complete reduction (until no further reduction possible)
const fullyReduced = polyReduceCompletely(poly, basis, order);

```
</details>





### Bidirectional Expression Conversion

Convert between Expression and sparse polynomial representation


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

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
</details>



<details>
<summary><b>JavaScript</b></summary>

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
</details>







## Performance

**Time Complexity**: Doubly exponential worst case, practical for small systems


## API Reference

- **Rust**: `mathhook_core::polynomial::groebner`
- **Python**: `mathhook.polynomial.groebner`
- **JavaScript**: `mathhook.polynomial.groebner`


## See Also


- [polynomial.overview](../polynomial/overview.md)

- [polynomial.gcd](../polynomial/gcd.md)

- [polynomial.division](../polynomial/division.md)


