---









---

# GCD Algorithms

> **Topic**: `polynomial.gcd`

Multiple GCD (Greatest Common Divisor) algorithms for polynomials, optimized for different use cases
including univariate, multivariate, and modular GCD using Zippel's algorithm.



## Mathematical Definition

$$For polynomials $f, g \in R[x]$ over a ring $R$, the greatest common divisor $\gcd(f, g)$ is the
monic polynomial $d$ of maximum degree such that:

$$d \mid f \quad \text{and} \quad d \mid g$$

and for any other polynomial $h$ where $h \mid f$ and $h \mid g$, we have $h \mid d$.

**Euclidean Algorithm**: For univariate polynomials over a field:

$$\gcd(f, g) = \begin{cases}
f & \text{if } g = 0 \\
\gcd(g, f \bmod g) & \text{otherwise}
\end{cases}$$

**Zippel's Modular Algorithm**:
1. Extract content: $f = c_f \cdot f_p$, $g = c_g \cdot g_p$
2. Compute $\gcd(f_p, g_p)$ in $\mathbb{Z}_p[x]$ for prime $p$
3. Use CRT to reconstruct $\gcd$ in $\mathbb{Z}[x]$
$$



MathHook provides multiple GCD (Greatest Common Divisor) algorithms for polynomials, optimized for different use cases.

## Algorithm Selection Guide

### Quick Decision Tree

```
Need GCD of two polynomials?
├─ Both are i64 integers? → integer_gcd(a, b)
├─ Don't know the structure? → polynomial_gcd(&p1, &p2)
├─ Single variable (x)? → univariate_gcd(&p1, &p2, &x)
├─ Need cofactors too? → modular_gcd_univariate(&p1, &p2, &x)
└─ Multiple variables (x, y, z)? → multivariate_gcd(&p1, &p2, &[x, y, z])
```

## Zippel's Modular GCD Algorithm

For performance-critical applications, the Zippel algorithm provides industrial-strength GCD computation using modular arithmetic.

### How It Works

1. **Content Extraction**: Separate integer content from primitive parts
2. **Prime Selection**: Choose primes that don't divide leading coefficients
3. **Modular GCD**: Compute GCD in Z_p[x] using Euclidean algorithm
4. **CRT Reconstruction**: Combine results from multiple primes using Chinese Remainder Theorem
5. **Trial Division**: Verify the result divides both inputs

### Configuration Options

- **max_eval_points**: Maximum number of evaluation points per variable
- **use_sparse**: Whether to use sparse optimization
- **starting_prime_idx**: Prime index to start with

## Performance Characteristics

| Algorithm | Complexity | Best For |
|-----------|------------|----------|
| Integer GCD | O(log(min(a,b))) | Small integers |
| Univariate Modular | O(d^2) | Single variable polynomials |
| Multivariate Zippel | O(d^n) | Sparse multivariate |
| Groebner-based | Doubly exponential | Ideal membership |

Where `d` is degree and `n` is number of variables.












## Examples


### General-Purpose GCD

Use PolynomialGcdOps trait for automatic algorithm selection

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>




### Univariate Modular GCD with Cofactors

Returns GCD and cofactors for Bezout identity verification

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>




### Multivariate GCD with Zippel Algorithm

Compute GCD for polynomials in multiple variables

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>




### Content and Primitive Part Decomposition

Fundamental operation for GCD computation

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook.polynomial.algorithms import extract_content, primitive_part

coeffs = [6, 12, 18]  # 6 + 12x + 18x^2

# Extract content (GCD of coefficients)
content = extract_content(coeffs)  # 6

# Get primitive part
cont, pp = primitive_part(coeffs)  # (6, [1, 2, 3])

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { extractContent, primitivePart } = require('mathhook/polynomial/algorithms');

const coeffs = [6, 12, 18];  // 6 + 12x + 18x^2

// Extract content (GCD of coefficients)
const content = extractContent(coeffs);  // 6

// Get primitive part
const [cont, pp] = primitivePart(coeffs);  // (6, [1, 2, 3])

```
</details>






## Performance

**Time Complexity**: O(d^2) for univariate, O(d^n) for multivariate


## API Reference

- **Rust**: `mathhook_core::polynomial::algorithms::gcd`
- **Python**: `mathhook.polynomial.algorithms.gcd`
- **JavaScript**: `mathhook.polynomial.algorithms.gcd`


## See Also


- [polynomial.overview](../polynomial/overview.md)

- [polynomial.finite-field](../polynomial/finite-field.md)

- [polynomial.division](../polynomial/division.md)


