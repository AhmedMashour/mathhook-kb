# Symbolic Differentiation

Computes the derivative of an expression with respect to a variable using symbolic mathematics

---
chunk_id: calculus_derivative::0
topic: calculus.derivative
title: Power Rule
priority: medium
keywords:
  - derivative
  - power rule
languages: [rust, python, javascript]
chunk: 1/3
description: "Compute symbolic derivatives with MathHook's powerful CAS. Supports chain rule, product rule, power rule. Fast, accurate, and easy to use for Rust, Python, and JavaScript."
seo_keywords:
  - symbolic differentiation
  - derivative calculator
  - automatic differentiation
  - calculus library
  - mathematical derivatives
canonical_url: "https://docs.mathhook.org/calculus/derivative"
---

## Power Rule

Demonstrates differentiation of polynomial expressions using the power rule

### Rust

```rust
use mathhook::expr;
let x = symbol!(x);
let f = expr!(x ^ 3);
let derivative = f.derivative(&x, 1);
assert_eq!(derivative, expr!(3 * (x ^ 2)));

```

### Python

```python
from mathhook import expr, symbol
x = symbol('x')
f = expr('x^3')
derivative = f.derivative(x, 1)
assert derivative == expr('3*x^2')

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook');
const x = symbol('x');
const f = expr('x^3');
const derivative = f.derivative(x, 1);
console.log(derivative.toString()); // "3*x^2"

```

### Expected Output

```
3*x^2
```



---
chunk_id: calculus_derivative::1
topic: calculus.derivative
title: Chain Rule
priority: medium
keywords:
  - derivative
  - chain rule
languages: [rust, python, javascript]
chunk: 2/3
description: "Compute symbolic derivatives with MathHook's powerful CAS. Supports chain rule, product rule, power rule. Fast, accurate, and easy to use for Rust, Python, and JavaScript."
seo_keywords:
  - symbolic differentiation
  - derivative calculator
  - automatic differentiation
  - calculus library
  - mathematical derivatives
canonical_url: "https://docs.mathhook.org/calculus/derivative"
---

## Chain Rule

Demonstrates differentiation of composite functions using the chain rule

### Rust

```rust
let f = expr!(sin(x ^ 2));
let derivative = f.derivative(&x, 1);
// Returns: 2*x*cos(x^2)

```

### Python

```python
f = expr('sin(x^2)')
derivative = f.derivative(x, 1)
# Returns: 2*x*cos(x^2)

```

### JavaScript

```javascript
const f = expr('sin(x^2)');
const derivative = f.derivative(x, 1);
// Returns: 2*x*cos(x^2)

```

### Expected Output

```
2*x*cos(x^2)
```



---
chunk_id: calculus_derivative::2
topic: calculus.derivative
title: Product Rule
priority: medium
keywords:
  - derivative
  - product rule
languages: [rust, python, javascript]
chunk: 3/3
description: "Compute symbolic derivatives with MathHook's powerful CAS. Supports chain rule, product rule, power rule. Fast, accurate, and easy to use for Rust, Python, and JavaScript."
seo_keywords:
  - symbolic differentiation
  - derivative calculator
  - automatic differentiation
  - calculus library
  - mathematical derivatives
canonical_url: "https://docs.mathhook.org/calculus/derivative"
---

## Product Rule

Demonstrates differentiation of products using the product rule

### Rust

```rust
let f = expr!(x * sin(x));
let derivative = f.derivative(&x, 1);
// Returns: sin(x) + x*cos(x)

```

### Python

```python
f = expr('x*sin(x)')
derivative = f.derivative(x, 1)
# Returns: sin(x) + x*cos(x)

```

### JavaScript

```javascript
const f = expr('x*sin(x)');
const derivative = f.derivative(x, 1);
// Returns: sin(x) + x*cos(x)

```

### Expected Output

```
sin(x) + x*cos(x)
```



