# Special Mathematical Functions

Comprehensive support for special mathematical functions including the Gamma
function family (Gamma, Digamma, Polygamma), Beta function, and other
advanced special functions used in mathematics, physics, and statistics.


---
chunk_id: advanced_special_functions::0
topic: advanced.special_functions
title: Gamma Function
priority: medium
keywords:
  - special_functions
  - gamma function
languages: [rust, python, javascript]
chunk: 1/4
---

## Gamma Function

Factorial extension: Γ(n) = (n-1)! for positive integers

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: advanced_special_functions::1
topic: advanced.special_functions
title: Digamma Function
priority: medium
keywords:
  - special_functions
  - digamma function
languages: [rust, python, javascript]
chunk: 2/4
---

## Digamma Function

Logarithmic derivative of Gamma: ψ(z) = Γ'(z)/Γ(z)

### Rust

```rust
// Special value: ψ(1) = -γ (Euler-Mascheroni constant)
let result = digamma(&expr!(1));
// Result: -EulerGamma (symbolic)

// Using recurrence: ψ(5) = ψ(1) + 1 + 1/2 + 1/3 + 1/4
let result = digamma(&expr!(5));
// Result: -EulerGamma + 25/12

```

### Python

```python
from sympy import digamma, EulerGamma

# Special value
result = digamma(1)
# Result: -EulerGamma

# Recurrence relation
result = digamma(5)
# Result: -EulerGamma + 25/12

```

### JavaScript

```javascript
// Special value
const result1 = digamma(1);
// Result: -EulerGamma

// Recurrence
const result2 = digamma(5);

```



---
chunk_id: advanced_special_functions::2
topic: advanced.special_functions
title: Polygamma Function
priority: medium
keywords:
  - special_functions
  - polygamma function
languages: [rust, python, javascript]
chunk: 3/4
---

## Polygamma Function

Higher derivatives: ψ^(n)(z) = d^n/dz^n ψ(z)

### Rust

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

### Python

```python
from sympy import polygamma, pi, zeta

# Trigamma
result = polygamma(1, 1)
# Result: pi**2/6

# Tetragamma
result = polygamma(2, 1)
# Result: -2*zeta(3)

```

### JavaScript

```javascript
// Trigamma
const result1 = polygamma(1, 1);
// Result: pi^2/6

// Tetragamma
const result2 = polygamma(2, 1);

```



---
chunk_id: advanced_special_functions::3
topic: advanced.special_functions
title: Beta Function
priority: medium
keywords:
  - special_functions
  - beta function
languages: [rust, python, javascript]
chunk: 4/4
---

## Beta Function

Beta function: B(a,b) = Γ(a)Γ(b)/Γ(a+b)

### Rust

```rust
// Integer values: B(2, 3) = 1/12
let result = beta(&expr!(2), &expr!(3));

// Numerical evaluation
let result = beta(&Expression::float(2.5), &Expression::float(3.7));

// Symmetry property
assert_eq!(beta(&expr!(2), &expr!(5)), beta(&expr!(5), &expr!(2)));

```

### Python

```python
from sympy import beta

# Integer values
result = beta(2, 3)
# Result: 1/12

# Symmetry
assert beta(2, 5) == beta(5, 2)

```

### JavaScript

```javascript
// Beta function
const result1 = beta(2, 3);
// Result: 1/12

// Numerical
const result2 = beta(2.5, 3.7);

```



