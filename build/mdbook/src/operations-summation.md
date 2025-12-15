---









---

# Summation and Products

> **Topic**: `operations.summation`

Symbolic summation and products in MathHook provide closed-form formulas for arithmetic series, geometric series, power sums, and convergence analysis for infinite series.



## Mathematical Definition

**Arithmetic Series:**
$$\sum_{i=1}^{n} [a + (i-1)d] = \frac{n}{2} \times [2a + (n-1)d]$$

**Geometric Series (Finite):**
$$\sum_{i=1}^{n} ar^{i-1} = a \times \frac{1 - r^n}{1 - r} \quad \text{for } r \neq 1$$

**Geometric Series (Infinite):**
$$\sum_{i=1}^{\infty} ar^{i-1} = \frac{a}{1 - r} \quad \text{for } |r| < 1$$

**Power Sums (Faulhaber's Formulas):**
- $\sum_{i=1}^{n} 1 = n$
- $\sum_{i=1}^{n} i = \frac{n(n+1)}{2}$ (Gauss's formula)
- $\sum_{i=1}^{n} i^2 = \frac{n(n+1)(2n+1)}{6}$
- $\sum_{i=1}^{n} i^3 = \left[\frac{n(n+1)}{2}\right]^2$ (Nicomachus's theorem)

**p-Series Convergence:**
$$\sum_{n=1}^{\infty} \frac{1}{n^p} \begin{cases}
\text{converges} & \text{if } p > 1 \\
\text{diverges} & \text{if } p \leq 1
\end{cases}$$





## Examples


### Arithmetic Series (Gauss's Formula)

Sum of integers from 1 to n


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use summation::SummationMethods;

// Sum of integers from 1 to 10
let first_term = expr!(1);
let common_diff = expr!(1);
let num_terms = expr!(10);

let sum = SummationMethods::arithmetic_series(
    &first_term,
    &common_diff,
    &num_terms
);
println!("{}", sum);  // 55

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import arithmetic_series

# Sum of integers from 1 to 10
sum_integers = arithmetic_series(first_term=1, common_diff=1, num_terms=10)
print(sum_integers)  # 55

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { arithmeticSeries } = require('mathhook');

// Sum of integers from 1 to 10
const sumIntegers = arithmeticSeries({
  firstTerm: 1,
  commonDiff: 1,
  numTerms: 10
});
console.log(sumIntegers.toString());  // 55

```
</details>





### Geometric Series

1 + 1/2 + 1/4 + ...


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use summation::SummationMethods;

// 1 + 1/2 + 1/4 (three terms)
let first = expr!(1);
let ratio = expr!(1 / 2);
let n = expr!(3);

let sum = SummationMethods::geometric_series(&first, &ratio, &n);
println!("{}", sum);  // 7/4 (exact rational)

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import geometric_series

# 1 + 1/2 + 1/4
sum_halves = geometric_series(first_term=1, common_ratio=0.5, num_terms=3)
print(sum_halves)  # 1.75

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { geometricSeries } = require('mathhook');

const sumHalves = geometricSeries({
  firstTerm: 1,
  commonRatio: 0.5,
  numTerms: 3
});
console.log(sumHalves.toString());  // 7/4

```
</details>





### Infinite Geometric Series

Converges when |r| < 1


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use summation::SummationMethods;

// 1 + 1/3 + 1/9 + 1/27 + ...
let first = expr!(1);
let ratio = Expression::rational(1, 3);

let sum = SummationMethods::infinite_geometric_series(&first, &ratio);
println!("{}", sum);  // 3/2 (exact rational)

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import infinite_geometric_series

# 1 + 1/3 + 1/9 + ...
sum_thirds = infinite_geometric_series(first_term=1, common_ratio=1/3)
print(sum_thirds)  # 1.5

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { infiniteGeometricSeries } = require('mathhook');

const sumThirds = infiniteGeometricSeries({
  firstTerm: 1,
  commonRatio: 1/3
});
console.log(sumThirds.toString());  // 3/2

```
</details>





### Power Sums (Nicomachus's Theorem)

Sum of cubes equals square of sum


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let power = expr!(3);
let n = expr!(4);
let sum = SummationMethods::power_sum(&power, &n);
println!("{}", sum);  // 100 (1 + 8 + 27 + 64 = 10^2)

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import power_sum

# Verify Nicomachus's theorem for n=5
sum_cubes = power_sum(power=3, upper_limit=5)
sum_integers = power_sum(power=1, upper_limit=5)
print(sum_cubes)  # 225
print(sum_integers ** 2)  # 225 (15^2)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { powerSum } = require('mathhook');

const sumCubes = powerSum({ power: 3, upperLimit: 4 });
console.log(sumCubes.toString());  // 100

```
</details>







## Performance

**Time Complexity**: O(1) for all closed-form formulas


## API Reference

- **Rust**: `mathhook_core::calculus::summation::Summation`
- **Python**: `mathhook.summation`
- **JavaScript**: `mathhook.summation`


## See Also


- [operations.series](../operations/series.md)

- [operations.limits](../operations/limits.md)

- [operations.integration](../operations/integration.md)


