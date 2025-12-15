---









---

# Polynomial Division and Factorization

> **Topic**: `polynomial.division`

Polynomial division algorithms including long division, exact division, and factorization capabilities
such as square-free factorization, resultant, and discriminant computation.



## Mathematical Definition

**Polynomial Long Division**: For polynomials $f(x), g(x)$ with $\deg(g) \leq \deg(f)$:

$$f(x) = g(x) \cdot q(x) + r(x)$$

where $q(x)$ is the quotient and $r(x)$ is the remainder with $\deg(r) < \deg(g)$.

**Resultant**: The resultant $\text{Res}(f, g)$ of polynomials $f, g$ of degrees $m, n$ is:

$$\text{Res}(f, g) = a_n^m \cdot b_m^n \cdot \prod_{i,j} (\alpha_i - \beta_j)$$

where $\alpha_i, \beta_j$ are roots of $f, g$ respectively. Properties:
- $\text{Res}(f, g) = 0 \iff f$ and $g$ share a common root
- Computed as determinant of Sylvester matrix

**Discriminant**: For polynomial $f(x)$ of degree $n$ with leading coefficient $a_n$:

$$\text{disc}(f) = \frac{(-1)^{n(n-1)/2}}{a_n} \cdot \text{Res}(f, f')$$

Properties:
- $\text{disc}(f) = 0 \iff f$ has a repeated root
- For quadratic $ax^2 + bx + c$: $\text{disc} = b^2 - 4ac$




This chapter covers polynomial division algorithms and factorization capabilities in MathHook.

## Polynomial Division

### Long Division

The standard polynomial long division algorithm computes quotient and remainder.

### Exact Division

When you expect the division to be exact (zero remainder), use exact division which
returns an error if the division is not exact.

## Factorization

### Common Factor Extraction

Extract the greatest common factor from all terms.

### Numeric Coefficient Factoring

Separate the numeric coefficient from the polynomial.

### Square-Free Factorization

Decompose a polynomial into square-free factors.

## Resultant and Discriminant

### Resultant

The resultant of two polynomials is zero if and only if they share a common root.

### Discriminant

The discriminant indicates whether a polynomial has repeated roots.

### Coprimality Test

Check if two polynomials are coprime (GCD is constant).












## Examples


### Polynomial Long Division

Compute quotient and remainder using standard division algorithm


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

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
</details>



<details>
<summary><b>JavaScript</b></summary>

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
</details>





### Exact Division

Division that errors if remainder is non-zero


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

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
</details>



<details>
<summary><b>JavaScript</b></summary>

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
</details>





### Trait-Based Division

Use PolynomialArithmetic trait for ergonomic API


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

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
</details>



<details>
<summary><b>JavaScript</b></summary>

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
</details>





### Polynomial Resultant

Test for common roots using resultant


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

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
</details>



<details>
<summary><b>JavaScript</b></summary>

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
</details>





### Polynomial Discriminant

Detect repeated roots using discriminant


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook_core::core::polynomial::algorithms::polynomial_discriminant;
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// (x - 1)^2 = x^2 - 2x + 1 (repeated root at 1)
let poly = expr!((x ^ 2) - (2 * x) + 1);

let disc = polynomial_discriminant(&poly, &x).unwrap();
// Discriminant = 0 (repeated root)

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import expr, symbol
from mathhook.polynomial.algorithms import polynomial_discriminant

x = symbol('x')

# (x - 1)^2 = x^2 - 2x + 1 (repeated root at 1)
poly = expr('x^2 - 2*x + 1')

disc = polynomial_discriminant(poly, x)
# Discriminant = 0 (repeated root)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { expr, symbol } = require('mathhook');
const { polynomialDiscriminant } = require('mathhook/polynomial/algorithms');

const x = symbol('x');

// (x - 1)^2 = x^2 - 2x + 1 (repeated root at 1)
const poly = expr('x^2 - 2*x + 1');

const disc = polynomialDiscriminant(poly, x);
// Discriminant = 0 (repeated root)

```
</details>







## Performance

**Time Complexity**: O(d^2) for division, O(d^3) for resultant


## API Reference

- **Rust**: `mathhook_core::polynomial::algorithms::division`
- **Python**: `mathhook.polynomial.algorithms.division`
- **JavaScript**: `mathhook.polynomial.algorithms.division`


## See Also


- [polynomial.gcd](../polynomial/gcd.md)

- [polynomial.overview](../polynomial/overview.md)

- [polynomial.groebner](../polynomial/groebner.md)


