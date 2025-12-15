---









---

# Polynomial Module Overview

> **Topic**: `polynomial.overview`

Comprehensive symbolic polynomial manipulation capabilities in MathHook. Implements a trait-based
architecture for automatic classification, property computation, arithmetic operations, and GCD algorithms.



## Mathematical Definition

A polynomial in variable $x$ over a ring $R$ is an expression of the form:

$$f(x) = a_n x^n + a_{n-1} x^{n-1} + \cdots + a_1 x + a_0$$

where $a_i \in R$ are coefficients and $n \in \mathbb{N}$ is the degree.

For multivariate polynomials in variables $x_1, x_2, \ldots, x_k$:

$$f(x_1, \ldots, x_k) = \sum_{\alpha \in \mathbb{N}^k} c_\alpha x_1^{\alpha_1} \cdots x_k^{\alpha_k}$$




The polynomial module provides comprehensive symbolic polynomial manipulation capabilities in MathHook. It implements a trait-based architecture for automatic classification, property computation, arithmetic operations, and GCD algorithms.

## Architecture

The module uses **decomposed traits** for clean separation of concerns:

| Trait | Purpose |
|-------|---------|
| `PolynomialClassification` | Type detection and variable extraction |
| `PolynomialProperties` | Degree, leading coefficient, content, primitive part |
| `PolynomialArithmetic` | Division, multiplication, addition |
| `PolynomialGcdOps` | GCD, LCM, cofactors |
| `PolynomialEducational` | Step-by-step explanations (opt-in) |

### Design Philosophy

1. **Automatic Classification**: Users don't need to manually wrap expressions - the system detects polynomial structure automatically
2. **Trait Composition**: Functionality is split into focused traits rather than one monolithic interface
3. **Performance First**: Thread-local LRU caching for expensive operations like degree computation
4. **Educational Support**: Optional step-by-step explanations for learning

## Key Concepts

### Univariate vs Multivariate

| Type | Description | Example |
|------|-------------|---------|
| **Univariate** | Single variable | `x^2 + 2x + 1` |
| **Multivariate** | Multiple variables | `x^2 + xy + y^2` |

The module automatically detects and routes to appropriate algorithms.

### Content and Primitive Part

For a polynomial $f(x) = a_n x^n + \cdots + a_1 x + a_0$:

- **Content**: $\gcd(a_n, \ldots, a_1, a_0)$ - the GCD of all coefficients
- **Primitive Part**: $f(x) / \text{content}(f)$ - the polynomial with content factored out

### Caching Strategy

Property computations are cached using thread-local LRU caching:

- **Automatic**: No user intervention required
- **Thread-safe**: Each thread has its own cache
- **Size-limited**: LRU eviction prevents memory bloat
- **Hash-based**: Uses pointer address + discriminant for fast lookup












## Examples


### Basic Polynomial Usage

Create polynomials and compute properties using trait-based API


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

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
</details>



<details>
<summary><b>JavaScript</b></summary>

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
</details>





### Polynomial Classification

Automatic detection of polynomial structure and variable extraction


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

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
</details>



<details>
<summary><b>JavaScript</b></summary>

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
</details>





### Content and Primitive Part

Extract GCD of coefficients and primitive polynomial


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook_core::core::polynomial::PolynomialProperties;
use mathhook_core::{expr, symbol};

let x = symbol!(x);
let poly = expr!((6 * (x ^ 2)) + (9 * x) + 3);  // 6x^2 + 9x + 3

let content = poly.content();           // 3
let primitive = poly.primitive_part();  // 2x^2 + 3x + 1

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import expr, symbol

x = symbol('x')
poly = expr('6*x^2 + 9*x + 3')

content = poly.content()         # 3
primitive = poly.primitive_part() # 2*x^2 + 3*x + 1

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { expr, symbol } = require('mathhook');

const x = symbol('x');
const poly = expr('6*x^2 + 9*x + 3');

const content = poly.content();         // 3
const primitive = poly.primitivePart(); // 2*x^2 + 3*x + 1

```
</details>







## Performance

**Time Complexity**: O(n) for property computation with LRU caching


## API Reference

- **Rust**: `mathhook_core::polynomial`
- **Python**: `mathhook.polynomial`
- **JavaScript**: `mathhook.polynomial`


## See Also


- [polynomial.gcd](../polynomial/gcd.md)

- [polynomial.division](../polynomial/division.md)

- [polynomial.groebner](../polynomial/groebner.md)

- [polynomial.special-families](../polynomial/special-families.md)

- [polynomial.finite-field](../polynomial/finite-field.md)


