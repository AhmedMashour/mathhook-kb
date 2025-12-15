# Piecewise Functions

Define functions with different formulas in different regions, essential for
modeling discontinuous behavior, conditional logic, step functions, and
threshold-based systems.


---
chunk_id: advanced_piecewise::0
topic: advanced.piecewise
title: Absolute Value Function
priority: medium
keywords:
  - piecewise
  - absolute value function
languages: [rust, python, javascript]
chunk: 1/4
---

## Absolute Value Function

|x| = { x if x ≥ 0, -x if x < 0 }

### Rust

```rust
let x = symbol!(x);

let abs_x = Expression::piecewise(
    vec![
        (expr!(x), expr!(x >= 0)),
        (expr!(-x), expr!(x < 0)),
    ],
    None,
);

```

### Python

```python
from sympy import symbols, Piecewise

x = symbols('x')
abs_x = Piecewise((x, x >= 0), (-x, x < 0))

```

### JavaScript

```javascript
const x = symbol('x');

const abs_x = piecewise([
    [x, ge(x, 0)],
    [neg(x), lt(x, 0)]
]);

```



---
chunk_id: advanced_piecewise::1
topic: advanced.piecewise
title: Heaviside Step Function
priority: medium
keywords:
  - piecewise
  - heaviside step function
languages: [rust, python, javascript]
chunk: 2/4
---

## Heaviside Step Function

H(x) = { 0 if x < 0, 1 if x ≥ 0 }

### Rust

```rust
let x = symbol!(x);

let heaviside = Expression::piecewise(
    vec![
        (expr!(0), expr!(x < 0)),
        (expr!(1), expr!(x >= 0)),
    ],
    None,
);

```

### Python

```python
from sympy import symbols, Heaviside

x = symbols('x')
H = Heaviside(x)  # Built-in Heaviside function

```

### JavaScript

```javascript
const x = symbol('x');

const H = piecewise([
    [0, lt(x, 0)],
    [1, ge(x, 0)]
]);

```



---
chunk_id: advanced_piecewise::2
topic: advanced.piecewise
title: Tax Bracket Example
priority: medium
keywords:
  - piecewise
  - tax bracket example
languages: [rust, python, javascript]
chunk: 3/4
---

## Tax Bracket Example

Progressive tax with income thresholds

### Rust

```rust
let income = symbol!(income);

// 10% on first $10k, 12% on next $30k, 22% on remainder
let tax = Expression::piecewise(
    vec![
        (expr!(0.10 * income), expr!(income <= 10000)),
        (expr!(1000 + 0.12 * (income - 10000)), expr!(income <= 40000)),
    ],
    Some(expr!(4600 + 0.22 * (income - 40000))),
);

// Calculate tax for $50,000
let tax_owed = tax.substitute(&income, &expr!(50000));
// Result: 4600 + 0.22 * 10000 = $6,800

```

### Python

```python
from sympy import symbols, Piecewise

income = symbols('income')

tax = Piecewise(
    (0.10 * income, income <= 10000),
    (1000 + 0.12 * (income - 10000), income <= 40000),
    (4600 + 0.22 * (income - 40000), True)
)

tax_owed = tax.subs(income, 50000)
# Result: 6800

```

### JavaScript

```javascript
const income = symbol('income');

const tax = piecewise([
    [mul(0.10, income), le(income, 10000)],
    [add(1000, mul(0.12, sub(income, 10000))), le(income, 40000)],
    [add(4600, mul(0.22, sub(income, 40000))), true]
]);

const tax_owed = tax.subs(income, 50000);

```



---
chunk_id: advanced_piecewise::3
topic: advanced.piecewise
title: Differentiation of Piecewise
priority: medium
keywords:
  - piecewise
  - differentiation of piecewise
languages: [rust, python, javascript]
chunk: 4/4
---

## Differentiation of Piecewise

Derivative computed piece-by-piece

### Rust

```rust
let x = symbol!(x);

// f(x) = { x^2 if x ≥ 0, -x^2 if x < 0 }
let f = Expression::piecewise(
    vec![
        (expr!(x^2), expr!(x >= 0)),
        (expr!(-x^2), expr!(x < 0)),
    ],
    None,
);

// Derivative
let df = f.derivative(&x, 1);
// Result: { 2x if x ≥ 0, -2x if x < 0 }

```

### Python

```python
from sympy import symbols, Piecewise, diff

x = symbols('x')
f = Piecewise((x**2, x >= 0), (-x**2, x < 0))

df = diff(f, x)
# Result: Piecewise((2*x, x > 0), (-2*x, x < 0))

```

### JavaScript

```javascript
const x = symbol('x');

const f = piecewise([
    [pow(x, 2), ge(x, 0)],
    [neg(pow(x, 2)), lt(x, 0)]
]);

const df = diff(f, x);

```



