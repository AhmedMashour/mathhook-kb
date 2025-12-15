---









---

# Special Mathematical Functions

> **Topic**: `advanced.special_functions`

Comprehensive support for special mathematical functions including the Gamma
function family (Gamma, Digamma, Polygamma), Beta function, and other
advanced special functions used in mathematics, physics, and statistics.



## Mathematical Definition

Gamma function: $$\Gamma(z) = \int_0^\infty t^{z-1} e^{-t} \, dt$$

Digamma function: $$\psi(z) = \frac{d}{dz} \ln(\Gamma(z)) = \frac{\Gamma'(z)}{\Gamma(z)}$$

Polygamma function: $$\psi^{(n)}(z) = \frac{d^{n+1}}{dz^{n+1}} \ln(\Gamma(z))$$

Beta function: $$B(a, b) = \frac{\Gamma(a) \cdot \Gamma(b)}{\Gamma(a+b)}$$





## Examples


### Gamma Function

Factorial extension: Γ(n) = (n-1)! for positive integers


<details>
<summary><b>Rust</b></summary>

```rust
// Integer factorial: Γ(5) = 4!
let result = gamma(&expr!(5));
// Result: 24

// Half-integer (exact symbolic): Γ(1/2) = √π
let result = gamma(&Expression::rational(1, 2));
// Result: sqrt(pi)

// Numerical evaluation
let result = gamma(&Expression::float(3.7));
// Result: ~4.17

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from sympy import gamma, sqrt, pi

# Integer factorial
result = gamma(5)
# Result: 24

# Half-integer
result = gamma(Rational(1, 2))
# Result: sqrt(pi)

# Numerical
result = gamma(3.7)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
// Integer factorial
const result1 = gamma(5);
// Result: 24

// Half-integer
const result2 = gamma(0.5);
// Result: sqrt(pi)

// Numerical
const result3 = gamma(3.7);

```
</details>





### Digamma Function

Logarithmic derivative of Gamma: ψ(z) = Γ'(z)/Γ(z)


<details>
<summary><b>Rust</b></summary>

```rust
// Special value: ψ(1) = -γ (Euler-Mascheroni constant)
let result = digamma(&expr!(1));
// Result: -EulerGamma (symbolic)

// Using recurrence: ψ(5) = ψ(1) + 1 + 1/2 + 1/3 + 1/4
let result = digamma(&expr!(5));
// Result: -EulerGamma + 25/12

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from sympy import digamma, EulerGamma

# Special value
result = digamma(1)
# Result: -EulerGamma

# Recurrence relation
result = digamma(5)
# Result: -EulerGamma + 25/12

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
// Special value
const result1 = digamma(1);
// Result: -EulerGamma

// Recurrence
const result2 = digamma(5);

```
</details>





### Polygamma Function

Higher derivatives: ψ^(n)(z) = d^n/dz^n ψ(z)


<details>
<summary><b>Rust</b></summary>

```rust
// Trigamma (n=1): ψ^(1)(1) = π²/6
let result = polygamma(1, &expr!(1));
// Result: pi^2/6

// Tetragamma (n=2)
let result = polygamma(2, &expr!(1));
// Result: -2*zeta(3)

// Higher orders
let result = polygamma(3, &expr!(1.5));
// Result: numerical value

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from sympy import polygamma, pi, zeta

# Trigamma
result = polygamma(1, 1)
# Result: pi**2/6

# Tetragamma
result = polygamma(2, 1)
# Result: -2*zeta(3)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
// Trigamma
const result1 = polygamma(1, 1);
// Result: pi^2/6

// Tetragamma
const result2 = polygamma(2, 1);

```
</details>





### Beta Function

Beta function: B(a,b) = Γ(a)Γ(b)/Γ(a+b)


<details>
<summary><b>Rust</b></summary>

```rust
// Integer values: B(2, 3) = 1/12
let result = beta(&expr!(2), &expr!(3));

// Numerical evaluation
let result = beta(&Expression::float(2.5), &Expression::float(3.7));

// Symmetry property
assert_eq!(beta(&expr!(2), &expr!(5)), beta(&expr!(5), &expr!(2)));

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from sympy import beta

# Integer values
result = beta(2, 3)
# Result: 1/12

# Symmetry
assert beta(2, 5) == beta(5, 2)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
// Beta function
const result1 = beta(2, 3);
// Result: 1/12

// Numerical
const result2 = beta(2.5, 3.7);

```
</details>







## Performance

**Time Complexity**: O(1) for Lanczos approximation


## API Reference

- **Rust**: `mathhook_core::special_functions`
- **Python**: `mathhook.special_functions`
- **JavaScript**: `mathhook.special_functions`


## See Also


- [core.functions](../core/functions.md)

- [advanced.complex_numbers](../advanced/complex_numbers.md)

- [operations.integration](../operations/integration.md)

- [operations.series](../operations/series.md)


