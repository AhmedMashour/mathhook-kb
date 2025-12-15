# Residue Calculus and Pole Finding

Find poles of rational and transcendental functions for applications in
control theory, signal processing, and complex analysis. SymPy-validated
pole locations for trigonometric functions.


---
chunk_id: advanced_residue_calculus::0
topic: advanced.residue_calculus
title: Rational Function Poles
priority: medium
keywords:
  - residue_calculus
  - rational function poles
languages: [rust, python, javascript]
chunk: 1/3
---

## Rational Function Poles

Find poles where denominator equals zero

### Rust

```rust
let z = symbol!(z);

// Simple pole at z = 0
let f1 = expr!(1 / z);
let poles1 = f1.find_poles(&z);
// Returns: [expr!(0)]

// Pole of order 2 at z = 3
let f2 = expr!(1 / ((z - 3) ^ 2));
let poles2 = f2.find_poles(&z);
// Returns: [expr!(3)]

// Multiple simple poles
let f3 = expr!(1 / ((z - 1) * (z + 2)));
let poles3 = f3.find_poles(&z);
// Returns: [expr!(1), expr!(-2)]

```

### Python

```python
from sympy import symbols, singularities

z = symbols('z')

# Simple pole
f1 = 1/z
poles1 = singularities(f1, z)
# Returns: {0}

# Multiple poles
f3 = 1/((z-1)*(z+2))
poles3 = singularities(f3, z)
# Returns: {1, -2}

```

### JavaScript

```javascript
const z = symbol('z');

// Simple pole
const f1 = div(1, z);
const poles1 = f1.findPoles(z);
// Returns: [0]

// Multiple poles
const f3 = div(1, mul(sub(z, 1), add(z, 2)));
const poles3 = f3.findPoles(z);
// Returns: [1, -2]

```



---
chunk_id: advanced_residue_calculus::1
topic: advanced.residue_calculus
title: Trigonometric Function Poles (SymPy Validated)
priority: medium
keywords:
  - residue_calculus
  - trigonometric function poles (sympy validated)
languages: [rust, python, javascript]
chunk: 2/3
---

## Trigonometric Function Poles (SymPy Validated)

Poles of tan, cot, sec, csc functions

### Rust

```rust
let x = symbol!(x);

// tan(x) has poles at x = π/2 + nπ
let f = expr!(tan(x));
let poles = f.find_poles(&x);
// Returns principal pole: [expr!(pi / 2)]

// cot(x) has poles at x = nπ
let f = expr!(cot(x));
let poles = f.find_poles(&x);
// Returns principal pole: [expr!(0)]

// sec(x) has poles at x = π/2 + nπ
let f = expr!(sec(x));
let poles = f.find_poles(&x);
// Returns principal pole: [expr!(pi / 2)]

// csc(x) has poles at x = nπ
let f = expr!(csc(x));
let poles = f.find_poles(&x);
// Returns principal pole: [expr!(0)]

```

### Python

```python
from sympy import symbols, tan, cot, sec, csc, singularities, pi

x = symbols('x', real=True)

# tan(x) poles
poles_tan = singularities(tan(x), x)
# Principal: pi/2

# cot(x) poles
poles_cot = singularities(cot(x), x)
# Principal: 0

# sec(x) poles
poles_sec = singularities(sec(x), x)
# Principal: pi/2

# csc(x) poles
poles_csc = singularities(csc(x), x)
# Principal: 0

```

### JavaScript

```javascript
const x = symbol('x');

// tan(x) poles
const poles_tan = tan(x).findPoles(x);
// Returns: [pi/2]

// cot(x) poles
const poles_cot = cot(x).findPoles(x);
// Returns: [0]

```



---
chunk_id: advanced_residue_calculus::2
topic: advanced.residue_calculus
title: Control System Stability
priority: medium
keywords:
  - residue_calculus
  - control system stability
languages: [rust, python, javascript]
chunk: 3/3
---

## Control System Stability

Transfer function pole analysis for stability

### Rust

```rust
let s = symbol!(s);
let zeta = expr!(0.7);      // Damping ratio
let omega_n = expr!(10);    // Natural frequency
let K = expr!(100);

let denominator = expr!(s^2 + 2*zeta*omega_n*s + omega_n^2);
let H = expr!(K / denominator);

let poles = H.find_poles(&s);

// For ζ = 0.7, ωₙ = 10:
// Poles ≈ -7 ± 7.14i
// - Stable (negative real part)
// - Damped oscillation at ~7.14 rad/s

```

### Python

```python
from sympy import symbols, sqrt

s = symbols('s')
zeta = 0.7
omega_n = 10
K = 100

H = K / (s**2 + 2*zeta*omega_n*s + omega_n**2)
poles = solve(denom(H), s)

# Stability: all poles have Re(pole) < 0

```

### JavaScript

```javascript
const s = symbol('s');
const zeta = 0.7;
const omega_n = 10;
const K = 100;

const denom = add(pow(s, 2), mul(2, zeta, omega_n, s), pow(omega_n, 2));
const H = div(K, denom);

const poles = H.findPoles(s);

```



