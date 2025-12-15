# Symbolic Integration

MathHook's integration system provides symbolic integration capabilities with an 8-layer strategy architecture from fast heuristics to complete Risch algorithm. Coverage: 93-95% of elementary integrals.


---
chunk_id: operations_integration::0
topic: operations.integration
title: Basic Integration (Layer 1: Table Lookup)
priority: medium
keywords:
  - integration
  - basic integration (layer 1: table lookup)
languages: [rust, python, javascript]
chunk: 1/3
---

## Basic Integration (Layer 1: Table Lookup)

Direct table hits for common patterns

### Rust

```rust
use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);

// Polynomial: ∫x^3 dx = x^4/4 + C
let poly = expr!(x ^ 3);
let result = poly.integrate(x.clone());
// Result: x^4/4 + C

// Rational: ∫1/(x+1) dx = ln|x+1| + C
let rational = expr!(1 / (x + 1));
let result = rational.integrate(x.clone());
// Result: ln|x+1| + C

// Trigonometric: ∫sin(x) dx = -cos(x) + C
let trig = expr!(sin(x));
let result = trig.integrate(x.clone());
// Result: -cos(x) + C

```

### Python

```python
from mathhook import symbol, integrate

x = symbol('x')

# Polynomial
poly = x**3
result = integrate(poly, x)
# Result: x**4/4

# Rational
rational = 1/(x+1)
result = integrate(rational, x)
# Result: log(x+1)

# Trigonometric
trig = sin(x)
result = integrate(trig, x)
# Result: -cos(x)

```

### JavaScript

```javascript
const { symbol, integrate } = require('mathhook');

const x = symbol('x');

// Polynomial
const poly = x.pow(3);
const result = integrate(poly, x);
// Result: x^4/4

```



---
chunk_id: operations_integration::1
topic: operations.integration
title: Integration by Parts (Layer 4: LIATE)
priority: medium
keywords:
  - integration
  - integration by parts (layer 4: liate)
languages: [rust, python, javascript]
chunk: 2/3
---

## Integration by Parts (Layer 4: LIATE)

∫u dv = uv - ∫v du using LIATE rule

### Rust

```rust
use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);

// ∫x*e^x dx: u = x (algebraic), dv = e^x (exponential)
let expr = expr!(x * exp(x));
let result = expr.integrate(x.clone());
// Result: x*e^x - e^x + C = e^x(x-1) + C

// ∫x*sin(x) dx: u = x (algebraic), dv = sin(x) (trig)
let expr2 = expr!(x * sin(x));
let result2 = expr2.integrate(x.clone());
// Result: -x*cos(x) + sin(x) + C

```

### Python

```python
from mathhook import symbol, integrate, exp, sin

x = symbol('x')

# ∫x*e^x dx
expr = x * exp(x)
result = integrate(expr, x)
# Result: x*exp(x) - exp(x)

# ∫x*sin(x) dx
expr2 = x * sin(x)
result2 = integrate(expr2, x)
# Result: -x*cos(x) + sin(x)

```

### JavaScript

```javascript
const { symbol, integrate, parse } = require('mathhook');

const x = symbol('x');

// ∫x*e^x dx
const expr = parse('x*exp(x)');
const result = integrate(expr, x);
// Result: x*exp(x) - exp(x)

```



---
chunk_id: operations_integration::2
topic: operations.integration
title: U-Substitution (Layer 5)
priority: medium
keywords:
  - integration
  - u-substitution (layer 5)
languages: [rust, python, javascript]
chunk: 3/3
---

## U-Substitution (Layer 5)

∫f(g(x))*g'(x) dx = ∫f(u) du

### Rust

```rust
use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);

// ∫2x*sin(x^2) dx: u = x^2, du = 2x dx
let expr = expr!(2 * x * sin(x ^ 2));
let result = expr.integrate(x.clone());
// Result: -cos(x^2) + C

// ∫2x*e^(x^2) dx: u = x^2, du = 2x dx
let expr2 = expr!(2 * x * exp(x ^ 2));
let result2 = expr2.integrate(x.clone());
// Result: e^(x^2) + C

```

### Python

```python
from mathhook import symbol, integrate, sin, exp

x = symbol('x')

# ∫2x*sin(x^2) dx
expr = 2*x*sin(x**2)
result = integrate(expr, x)
# Result: -cos(x^2)

# ∫2x*e^(x^2) dx
expr2 = 2*x*exp(x**2)
result2 = integrate(expr2, x)
# Result: exp(x^2)

```

### JavaScript

```javascript
const { symbol, integrate, parse } = require('mathhook');

const x = symbol('x');

// ∫2x*sin(x^2) dx
const expr = parse('2*x*sin(x^2)');
const result = integrate(expr, x);
// Result: -cos(x^2)

```



