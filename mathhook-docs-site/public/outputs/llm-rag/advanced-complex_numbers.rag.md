# Complex Number Operations

Work with complex numbers in MathHook including imaginary unit i, complex
arithmetic, polar form, Euler's formula, and operations like conjugate,
magnitude, and argument.


---
chunk_id: advanced_complex_numbers::0
topic: advanced.complex_numbers
title: Basic Complex Arithmetic
priority: medium
keywords:
  - complex_numbers
  - basic complex arithmetic
languages: [rust, python, javascript]
chunk: 1/3
---

## Basic Complex Arithmetic



### Rust

```rust
let i = Expression::i();
let z1 = expr!(3 + 4*i);
let z2 = expr!(1 - 2*i);

let sum = expr!(z1 + z2);      // 4 + 2i
let product = expr!(z1 * z2);  // 11 - 2i

```

### Python

```python
i = expr('I')
z1 = expr('3 + 4*I')
z2 = expr('1 - 2*I')

sum_z = z1 + z2       # 4 + 2*I
product = z1 * z2     # 11 - 2*I

```

### JavaScript

```javascript
const i = expr('I');
const z1 = expr('3 + 4*I');
const z2 = expr('1 - 2*I');

const sum = z1.add(z2);      // 4 + 2*I
const product = z1.mul(z2);  // 11 - 2*I

```



---
chunk_id: advanced_complex_numbers::1
topic: advanced.complex_numbers
title: Euler's Formula
priority: medium
keywords:
  - complex_numbers
  - euler's formula
languages: [rust, python, javascript]
chunk: 2/3
---

## Euler's Formula



### Rust

```rust
let theta = symbol!(theta);
let euler = expr!(exp(i * theta));

// Expands to: cos(theta) + i*sin(theta)
let expanded = euler.expand();

```

### Python

```python
theta = symbol('theta')
euler = exp(I * theta)

# Expands to: cos(theta) + I*sin(theta)
expanded = expand(euler)

```

### JavaScript

```javascript
const theta = symbol('theta');
const euler = exp(mul(I, theta));

// Expands to: cos(theta) + I*sin(theta)
const expanded = euler.expand();

```



---
chunk_id: advanced_complex_numbers::2
topic: advanced.complex_numbers
title: Polar Form Conversion
priority: medium
keywords:
  - complex_numbers
  - polar form conversion
languages: [rust, python, javascript]
chunk: 3/3
---

## Polar Form Conversion



### Rust

```rust
let z = expr!(3 + 4*i);

let magnitude = expr!(abs(z));  // 5
let angle = expr!(arg(z));      // arctan(4/3)

// Polar form: r*exp(i*theta)
let polar = expr!(magnitude * exp(i * angle));

```

### Python

```python
z = expr('3 + 4*I')

magnitude = abs(z)  # 5
angle = arg(z)      # atan(4/3)

# Polar form
polar = magnitude * exp(I * angle)

```

### JavaScript

```javascript
const z = expr('3 + 4*I');

const magnitude = abs(z);  // 5
const angle = arg(z);      // atan(4/3)

// Polar form
const polar = magnitude.mul(exp(I.mul(angle)));

```



