# Summation and Products

Symbolic summation and products in MathHook provide closed-form formulas for arithmetic series, geometric series, power sums, and convergence analysis for infinite series.


---
chunk_id: operations_summation::0
topic: operations.summation
title: Arithmetic Series (Gauss's Formula)
priority: medium
keywords:
  - summation
  - arithmetic series (gauss's formula)
languages: [rust, python, javascript]
chunk: 1/4
---

## Arithmetic Series (Gauss's Formula)

Sum of integers from 1 to n

### Rust

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

### Python

```python
from mathhook import arithmetic_series

# Sum of integers from 1 to 10
sum_integers = arithmetic_series(first_term=1, common_diff=1, num_terms=10)
print(sum_integers)  # 55

```

### JavaScript

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



---
chunk_id: operations_summation::1
topic: operations.summation
title: Geometric Series
priority: medium
keywords:
  - summation
  - geometric series
languages: [rust, python, javascript]
chunk: 2/4
---

## Geometric Series

1 + 1/2 + 1/4 + ...

### Rust

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

### Python

```python
from mathhook import geometric_series

# 1 + 1/2 + 1/4
sum_halves = geometric_series(first_term=1, common_ratio=0.5, num_terms=3)
print(sum_halves)  # 1.75

```

### JavaScript

```javascript
const { geometricSeries } = require('mathhook');

const sumHalves = geometricSeries({
  firstTerm: 1,
  commonRatio: 0.5,
  numTerms: 3
});
console.log(sumHalves.toString());  // 7/4

```



---
chunk_id: operations_summation::2
topic: operations.summation
title: Infinite Geometric Series
priority: medium
keywords:
  - summation
  - infinite geometric series
languages: [rust, python, javascript]
chunk: 3/4
---

## Infinite Geometric Series

Converges when |r| < 1

### Rust

```rust
use mathhook::prelude::*;
use summation::SummationMethods;

// 1 + 1/3 + 1/9 + 1/27 + ...
let first = expr!(1);
let ratio = Expression::rational(1, 3);

let sum = SummationMethods::infinite_geometric_series(&first, &ratio);
println!("{}", sum);  // 3/2 (exact rational)

```

### Python

```python
from mathhook import infinite_geometric_series

# 1 + 1/3 + 1/9 + ...
sum_thirds = infinite_geometric_series(first_term=1, common_ratio=1/3)
print(sum_thirds)  # 1.5

```

### JavaScript

```javascript
const { infiniteGeometricSeries } = require('mathhook');

const sumThirds = infiniteGeometricSeries({
  firstTerm: 1,
  commonRatio: 1/3
});
console.log(sumThirds.toString());  // 3/2

```



---
chunk_id: operations_summation::3
topic: operations.summation
title: Power Sums (Nicomachus's Theorem)
priority: medium
keywords:
  - summation
  - power sums (nicomachus's theorem)
languages: [rust, python, javascript]
chunk: 4/4
---

## Power Sums (Nicomachus's Theorem)

Sum of cubes equals square of sum

### Rust

```rust
use mathhook::prelude::*;

let power = expr!(3);
let n = expr!(4);
let sum = SummationMethods::power_sum(&power, &n);
println!("{}", sum);  // 100 (1 + 8 + 27 + 64 = 10^2)

```

### Python

```python
from mathhook import power_sum

# Verify Nicomachus's theorem for n=5
sum_cubes = power_sum(power=3, upper_limit=5)
sum_integers = power_sum(power=1, upper_limit=5)
print(sum_cubes)  # 225
print(sum_integers ** 2)  # 225 (15^2)

```

### JavaScript

```javascript
const { powerSum } = require('mathhook');

const sumCubes = powerSum({ power: 3, upperLimit: 4 });
console.log(sumCubes.toString());  // 100

```



