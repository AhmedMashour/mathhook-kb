# Separable ODEs

Separable ODEs are the most important and frequently encountered class of first-order differential
equations. MathHook provides a robust solver that handles both general and particular solutions
with automatic variable separation and symbolic integration.


---
chunk_id: ode_separable::0
topic: ode.separable
title: Simple Linear ODE
priority: medium
keywords:
  - separable
  - simple linear ode
languages: [rust, python, javascript]
chunk: 1/5
---

## Simple Linear ODE

Solve dy/dx = x

### Rust

```rust
use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let solution = solver.solve(&expr!(x), &y, &x, None)?;
// Result: y = x²/2 + C

```

### Python

```python
from mathhook import symbol
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

solution = solver.solve('x', y, x, None)
# Result: y = x²/2 + C

```

### JavaScript

```javascript
const { symbol } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const solution = solver.solve('x', y, x, null);
// Result: y = x²/2 + C

```



---
chunk_id: ode_separable::1
topic: ode.separable
title: Exponential Growth
priority: medium
keywords:
  - separable
  - exponential growth
languages: [rust, python, javascript]
chunk: 2/5
---

## Exponential Growth

Solve dy/dx = y (exponential growth/decay model)

### Rust

```rust
use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let solution = solver.solve(&expr!(y), &y, &x, None)?;
// Result: y = Ce^x

```

### Python

```python
from mathhook import symbol
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

solution = solver.solve('y', y, x, None)
# Result: y = Ce^x

```

### JavaScript

```javascript
const { symbol } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const solution = solver.solve('y', y, x, null);
// Result: y = Ce^x

```



---
chunk_id: ode_separable::2
topic: ode.separable
title: Product Form
priority: medium
keywords:
  - separable
  - product form
languages: [rust, python, javascript]
chunk: 3/5
---

## Product Form

Solve dy/dx = xy (nonlinear growth model)

### Rust

```rust
use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let solution = solver.solve(&expr!(x * y), &y, &x, None)?;
// Result: y = Ce^(x²/2)

```

### Python

```python
from mathhook import symbol
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

solution = solver.solve('x*y', y, x, None)
# Result: y = Ce^(x²/2)

```

### JavaScript

```javascript
const { symbol } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const solution = solver.solve('x*y', y, x, null);
// Result: y = Ce^(x²/2)

```



---
chunk_id: ode_separable::3
topic: ode.separable
title: Initial Value Problem
priority: medium
keywords:
  - separable
  - initial value problem
languages: [rust, python, javascript]
chunk: 4/5
---

## Initial Value Problem

Solve dy/dx = x with y(0) = 1

### Rust

```rust
use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let ic = Some((expr!(0), expr!(1))); // y(0) = 1
let solution = solver.solve(&expr!(x), &y, &x, ic)?;
// Result: y = x²/2 + 1

```

### Python

```python
from mathhook import symbol, expr
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

ic = (expr('0'), expr('1'))  # y(0) = 1
solution = solver.solve('x', y, x, ic)
# Result: y = x²/2 + 1

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const ic = [expr('0'), expr('1')];  // y(0) = 1
const solution = solver.solve('x', y, x, ic);
// Result: y = x²/2 + 1

```



---
chunk_id: ode_separable::4
topic: ode.separable
title: Rational Function
priority: medium
keywords:
  - separable
  - rational function
languages: [rust, python, javascript]
chunk: 5/5
---

## Rational Function

Solve dy/dx = x/y

### Rust

```rust
use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let rhs = expr!(x / y);
let solution = solver.solve(&rhs, &y, &x, None)?;
// Result: y² - x² = C (implicit) or y = ±√(x² + C) (explicit)

```

### Python

```python
from mathhook import symbol, expr
from mathhook.ode.first_order.separable import SeparableODESolver

x = symbol('x')
y = symbol('y')
solver = SeparableODESolver()

solution = solver.solve('x/y', y, x, None)
# Result: y² - x² = C

```

### JavaScript

```javascript
const { symbol } = require('mathhook');
const { SeparableODESolver } = require('mathhook/ode/firstOrder/separable');

const x = symbol('x');
const y = symbol('y');
const solver = new SeparableODESolver();

const solution = solver.solve('x/y', y, x, null);
// Result: y² - x² = C

```



