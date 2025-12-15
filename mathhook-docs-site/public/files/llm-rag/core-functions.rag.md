# Function System

MathHook provides a comprehensive mathematical function system with intelligent evaluation,
symbolic manipulation, and educational explanations. Functions are first-class expressions
supporting exact symbolic computation and high-performance numerical evaluation through
a modular intelligence architecture.


---
chunk_id: core_functions::0
topic: core.functions
title: Creating Functions with Macros
priority: medium
keywords:
  - functions
  - creating functions with macros
languages: [rust, python, javascript]
chunk: 1/6
---

## Creating Functions with Macros

Using function! and expr! macros for ergonomic function creation

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Single argument functions
let sine = function!(sin, x);
let cosine = function!(cos, x);

// Multi-argument functions
let log_base = function!(log, x, 10);

// Using expr! macro
let trig_identity = expr!(sin(x)^2 + cos(x)^2);
assert_eq!(trig_identity.simplify(), Expression::integer(1));

```

### Python

```python
from mathhook import symbol, function, expr

x = symbol('x')

# Single argument functions
sine = function('sin', x)
cosine = function('cos', x)

# Multi-argument functions
log_base = function('log', x, 10)

# Using expr
trig_identity = expr('sin(x)^2 + cos(x)^2')
assert trig_identity.simplify() == 1

```

### JavaScript

```javascript
const { symbol, func, expr } = require('mathhook-node');

const x = symbol('x');

// Single argument functions
const sine = func('sin', x);
const cosine = func('cos', x);

// Multi-argument functions
const logBase = func('log', x, 10);

// Using expr
const trigIdentity = expr('sin(x)^2 + cos(x)^2');
console.assert(trigIdentity.simplify().equals(1));

```



---
chunk_id: core_functions::1
topic: core.functions
title: Trigonometric Functions with Exact Values
priority: medium
keywords:
  - functions
  - trigonometric functions with exact values
languages: [rust, python, javascript]
chunk: 2/6
---

## Trigonometric Functions with Exact Values

Automatic recognition of exact trigonometric values at special angles

### Rust

```rust
use mathhook::prelude::*;

// Exact values recognized
assert_eq!(expr!(sin(0)), expr!(0));
assert_eq!(expr!(sin(pi/6)), expr!(1/2));
assert_eq!(expr!(sin(pi/4)), expr!(sqrt(2)/2));
assert_eq!(expr!(sin(pi/2)), expr!(1));

assert_eq!(expr!(cos(0)), expr!(1));
assert_eq!(expr!(cos(pi/3)), expr!(1/2));
assert_eq!(expr!(cos(pi/2)), expr!(0));

```

### Python

```python
from mathhook import expr

# Exact values recognized
assert expr('sin(0)') == 0
assert expr('sin(pi/6)') == expr('1/2')
assert expr('sin(pi/4)') == expr('sqrt(2)/2')
assert expr('sin(pi/2)') == 1

assert expr('cos(0)') == 1
assert expr('cos(pi/3)') == expr('1/2')
assert expr('cos(pi/2)') == 0

```

### JavaScript

```javascript
const { expr } = require('mathhook-node');

// Exact values recognized
console.assert(expr('sin(0)').equals(0));
console.assert(expr('sin(pi/6)').equals(expr('1/2')));
console.assert(expr('sin(pi/4)').equals(expr('sqrt(2)/2')));
console.assert(expr('sin(pi/2)').equals(1));

console.assert(expr('cos(0)').equals(1));
console.assert(expr('cos(pi/3)').equals(expr('1/2')));
console.assert(expr('cos(pi/2)').equals(0));

```



---
chunk_id: core_functions::2
topic: core.functions
title: Logarithm and Exponential Identities
priority: medium
keywords:
  - functions
  - logarithm and exponential identities
languages: [rust, python, javascript]
chunk: 3/6
---

## Logarithm and Exponential Identities

Automatic application of logarithm laws and exponential identities

### Rust

```rust
use mathhook::prelude::*;

let a = symbol!(a);
let b = symbol!(b);
let n = symbol!(n);

// Logarithm laws
assert_eq!(expr!(ln(a*b)).expand(), expr!(ln(a) + ln(b)));
assert_eq!(expr!(ln(a/b)).expand(), expr!(ln(a) - ln(b)));
assert_eq!(expr!(ln(a^n)).expand(), expr!(n*ln(a)));

// Exponential identities
assert_eq!(expr!(e^(ln(a))).simplify(), a);
assert_eq!(expr!(ln(e^a)).simplify(), a);

```

### Python

```python
from mathhook import symbol, expr

a = symbol('a')
b = symbol('b')
n = symbol('n')

# Logarithm laws
assert expr('ln(a*b)').expand() == expr('ln(a) + ln(b)')
assert expr('ln(a/b)').expand() == expr('ln(a) - ln(b)')
assert expr('ln(a^n)').expand() == expr('n*ln(a)')

# Exponential identities
assert expr('e^(ln(a))').simplify() == a
assert expr('ln(e^a)').simplify() == a

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook-node');

const a = symbol('a');
const b = symbol('b');
const n = symbol('n');

// Logarithm laws
console.assert(expr('ln(a*b)').expand().equals(expr('ln(a) + ln(b)')));
console.assert(expr('ln(a/b)').expand().equals(expr('ln(a) - ln(b)')));
console.assert(expr('ln(a^n)').expand().equals(expr('n*ln(a)')));

// Exponential identities
console.assert(expr('e^(ln(a))').simplify().equals(a));
console.assert(expr('ln(e^a)').simplify().equals(a));

```



---
chunk_id: core_functions::3
topic: core.functions
title: Function Derivatives (Automatic Chain Rule)
priority: medium
keywords:
  - functions
  - function derivatives (automatic chain rule)
languages: [rust, python, javascript]
chunk: 4/6
---

## Function Derivatives (Automatic Chain Rule)

Functions know their derivatives with automatic chain rule application

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Basic derivatives
assert_eq!(expr!(sin(x)).derivative(&x, 1), expr!(cos(x)));
assert_eq!(expr!(cos(x)).derivative(&x, 1), expr!(-sin(x)));
assert_eq!(expr!(exp(x)).derivative(&x, 1), expr!(exp(x)));
assert_eq!(expr!(ln(x)).derivative(&x, 1), expr!(1/x));

// Chain rule automatic
let f = expr!(sin(x^2));
assert_eq!(f.derivative(&x, 1), expr!(2*x*cos(x^2)));

```

### Python

```python
from mathhook import symbol, expr

x = symbol('x')

# Basic derivatives
assert expr('sin(x)').derivative(x) == expr('cos(x)')
assert expr('cos(x)').derivative(x) == expr('-sin(x)')
assert expr('exp(x)').derivative(x) == expr('exp(x)')
assert expr('ln(x)').derivative(x) == expr('1/x')

# Chain rule automatic
f = expr('sin(x^2)')
assert f.derivative(x) == expr('2*x*cos(x^2)')

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook-node');

const x = symbol('x');

// Basic derivatives
console.assert(expr('sin(x)').derivative(x).equals(expr('cos(x)')));
console.assert(expr('cos(x)').derivative(x).equals(expr('-sin(x)')));
console.assert(expr('exp(x)').derivative(x).equals(expr('exp(x)')));
console.assert(expr('ln(x)').derivative(x).equals(expr('1/x')));

// Chain rule automatic
const f = expr('sin(x^2)');
console.assert(f.derivative(x).equals(expr('2*x*cos(x^2)')));

```



---
chunk_id: core_functions::4
topic: core.functions
title: Special Functions (Gamma and Bessel)
priority: medium
keywords:
  - functions
  - special functions (gamma and bessel)
languages: [rust, python, javascript]
chunk: 5/6
---

## Special Functions (Gamma and Bessel)

Advanced special functions for scientific and engineering applications

### Rust

```rust
use mathhook::prelude::*;

// Gamma function (factorial generalization)
assert_eq!(expr!(gamma(1)), expr!(1));   // Γ(1) = 0! = 1
assert_eq!(expr!(gamma(5)), expr!(24));  // Γ(5) = 4! = 24
assert_eq!(expr!(gamma(1/2)), expr!(sqrt(pi)));

// Bessel functions (wave propagation)
let x = symbol!(x);
let bessel_j0 = expr!(bessel_j(0, x));
let bessel_y0 = expr!(bessel_y(0, x));

```

### Python

```python
from mathhook import expr, symbol

# Gamma function
assert expr('gamma(1)') == 1
assert expr('gamma(5)') == 24
assert expr('gamma(1/2)') == expr('sqrt(pi)')

# Bessel functions
x = symbol('x')
bessel_j0 = expr('bessel_j(0, x)')
bessel_y0 = expr('bessel_y(0, x)')

```

### JavaScript

```javascript
const { expr, symbol } = require('mathhook-node');

// Gamma function
console.assert(expr('gamma(1)').equals(1));
console.assert(expr('gamma(5)').equals(24));
console.assert(expr('gamma(1/2)').equals(expr('sqrt(pi)')));

// Bessel functions
const x = symbol('x');
const besselJ0 = expr('bessel_j(0, x)');
const besselY0 = expr('bessel_y(0, x)');

```



---
chunk_id: core_functions::5
topic: core.functions
title: Real-World Physics Application
priority: medium
keywords:
  - functions
  - real-world physics application
languages: [rust, python, javascript]
chunk: 6/6
---

## Real-World Physics Application

Damped harmonic oscillator using exponential and trigonometric functions

### Rust

```rust
use mathhook::prelude::*;

// Damped harmonic motion: x(t) = A*e^(-γt)*cos(ωt + φ)
let t = symbol!(t);
let A = expr!(1);
let gamma = expr!(0.1);
let omega = expr!(2*pi);
let phi = expr!(0);

let position = expr!(A * e^(-gamma*t) * cos(omega*t + phi));
let velocity = position.derivative(&t, 1);
let acceleration = velocity.derivative(&t, 1);

// Verify: ẍ + 2γẋ + ω²x = 0
let lhs = expr!(acceleration + 2*gamma*velocity + (omega^2)*position);
assert_eq!(lhs.simplify(), expr!(0));

```

### Python

```python
from mathhook import symbol, expr

# Damped harmonic motion
t = symbol('t')
position = expr('e^(-0.1*t) * cos(2*pi*t)')
velocity = position.derivative(t)
acceleration = velocity.derivative(t)

# Differential equation verification
gamma = 0.1
omega = expr('2*pi')
lhs = expr(f'acceleration + 2*{gamma}*velocity + omega^2*position')
# Should simplify to 0

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook-node');

// Damped harmonic motion
const t = symbol('t');
const position = expr('e^(-0.1*t) * cos(2*pi*t)');
const velocity = position.derivative(t);
const acceleration = velocity.derivative(t);

// Differential equation verification
const gamma = 0.1;
const omega = expr('2*pi');
// Should satisfy: ẍ + 2γẋ + ω²x = 0

```



