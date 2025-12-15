# Separation of Variables for PDEs

Separation of variables is the fundamental technique for solving linear partial differential
equations (PDEs) with boundary conditions. This method transforms a PDE into a system of
ordinary differential equations (ODEs) that can be solved independently, then combines the
solutions into an infinite series.


---
chunk_id: pde_separation-of-variables::0
topic: pde.separation-of-variables
title: Heat Equation with Dirichlet BCs
priority: medium
keywords:
  - separation-of-variables
  - heat equation with dirichlet bcs
languages: [rust, python, javascript]
chunk: 1/3
---

## Heat Equation with Dirichlet BCs

Solve 1D heat equation with fixed boundary conditions

### Rust

```rust
use mathhook::prelude::*;

let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);
let alpha = symbol!(alpha);

let equation = expr!(u);
let pde = Pde::new(equation, u, vec![x.clone(), t.clone()]);

// Boundary conditions: u(0,t) = 0, u(π,t) = 0
let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
let bc_right = BoundaryCondition::dirichlet_at(x.clone(), expr!(pi), expr!(0));
let bcs = vec![bc_left, bc_right];

// Initial condition: u(x,0) = sin(x)
let ic = InitialCondition::value(expr!(sin(x)));
let ics = vec![ic];

let solution = separate_variables(&pde, &bcs, &ics)?;
// Result: eigenvalues [1, 4, 9, 16, ...], eigenfunctions [sin(x), sin(2x), ...]

```

### Python

```python
from mathhook import symbol, expr
from mathhook.pde import Pde, BoundaryCondition, InitialCondition, separate_variables

u = symbol('u')
x = symbol('x')
t = symbol('t')

pde = Pde(u, u, [x, t])

# Boundary conditions
bc_left = BoundaryCondition.dirichlet_at(x, expr('0'), expr('0'))
bc_right = BoundaryCondition.dirichlet_at(x, expr('pi'), expr('0'))
bcs = [bc_left, bc_right]

# Initial condition
ic = InitialCondition.value(expr('sin(x)'))
ics = [ic]

solution = separate_variables(pde, bcs, ics)
# Result: eigenvalues [1, 4, 9, 16, ...], eigenfunctions [sin(x), sin(2x), ...]

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');
const { Pde, BoundaryCondition, InitialCondition, separateVariables } = require('mathhook/pde');

const u = symbol('u');
const x = symbol('x');
const t = symbol('t');

const pde = new Pde(u, u, [x, t]);

// Boundary conditions
const bcLeft = BoundaryCondition.dirichletAt(x, expr('0'), expr('0'));
const bcRight = BoundaryCondition.dirichletAt(x, expr('pi'), expr('0'));
const bcs = [bcLeft, bcRight];

// Initial condition
const ic = InitialCondition.value(expr('sin(x)'));
const ics = [ic];

const solution = separateVariables(pde, bcs, ics);
// Result: eigenvalues [1, 4, 9, 16, ...], eigenfunctions [sin(x), sin(2x), ...]

```



---
chunk_id: pde_separation-of-variables::1
topic: pde.separation-of-variables
title: Wave Equation
priority: medium
keywords:
  - separation-of-variables
  - wave equation
languages: [rust, python, javascript]
chunk: 2/3
---

## Wave Equation

Solve 1D wave equation with Dirichlet boundary conditions

### Rust

```rust
use mathhook::prelude::*;

let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);
let L = symbol!(L);

let pde = Pde::new(expr!(u), u, vec![x.clone(), t.clone()]);

let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
let bc_right = BoundaryCondition::dirichlet_at(x.clone(), expr!(L), expr!(0));
let bcs = vec![bc_left, bc_right];

// Initial displacement and velocity
let ic_displacement = InitialCondition::value(expr!(sin(pi * x / L)));
let ic_velocity = InitialCondition::derivative(expr!(0));
let ics = vec![ic_displacement, ic_velocity];

let solution = separate_variables(&pde, &bcs, &ics)?;

```

### Python

```python
from mathhook import symbol, expr
from mathhook.pde import Pde, BoundaryCondition, InitialCondition, separate_variables

u = symbol('u')
x = symbol('x')
t = symbol('t')
L = symbol('L')

pde = Pde(u, u, [x, t])

bc_left = BoundaryCondition.dirichlet_at(x, expr('0'), expr('0'))
bc_right = BoundaryCondition.dirichlet_at(x, L, expr('0'))
bcs = [bc_left, bc_right]

ic_displacement = InitialCondition.value(expr('sin(pi*x/L)'))
ic_velocity = InitialCondition.derivative(expr('0'))
ics = [ic_displacement, ic_velocity]

solution = separate_variables(pde, bcs, ics)

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');
const { Pde, BoundaryCondition, InitialCondition, separateVariables } = require('mathhook/pde');

const u = symbol('u');
const x = symbol('x');
const t = symbol('t');
const L = symbol('L');

const pde = new Pde(u, u, [x, t]);

const bcLeft = BoundaryCondition.dirichletAt(x, expr('0'), expr('0'));
const bcRight = BoundaryCondition.dirichletAt(x, L, expr('0'));
const bcs = [bcLeft, bcRight];

const icDisplacement = InitialCondition.value(expr('sin(pi*x/L)'));
const icVelocity = InitialCondition.derivative(expr('0'));
const ics = [icDisplacement, icVelocity];

const solution = separateVariables(pde, bcs, ics);

```



---
chunk_id: pde_separation-of-variables::2
topic: pde.separation-of-variables
title: Laplace Equation on Rectangle
priority: medium
keywords:
  - separation-of-variables
  - laplace equation on rectangle
languages: [rust, python, javascript]
chunk: 3/3
---

## Laplace Equation on Rectangle

Solve Laplace's equation on rectangular domain

### Rust

```rust
use mathhook::prelude::*;

let u = symbol!(u);
let x = symbol!(x);
let y = symbol!(y);
let a = symbol!(a);

let pde = Pde::new(expr!(u), u, vec![x.clone(), y.clone()]);

let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
let bc_right = BoundaryCondition::dirichlet_at(x.clone(), expr!(a), expr!(0));
let bcs = vec![bc_left, bc_right];

let ics = vec![];  // Laplace is elliptic, not time-dependent

let solution = separate_variables(&pde, &bcs, &ics)?;

```

### Python

```python
from mathhook import symbol, expr
from mathhook.pde import Pde, BoundaryCondition, separate_variables

u = symbol('u')
x = symbol('x')
y = symbol('y')
a = symbol('a')

pde = Pde(u, u, [x, y])

bc_left = BoundaryCondition.dirichlet_at(x, expr('0'), expr('0'))
bc_right = BoundaryCondition.dirichlet_at(x, a, expr('0'))
bcs = [bc_left, bc_right]

ics = []  # Laplace is elliptic

solution = separate_variables(pde, bcs, ics)

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');
const { Pde, BoundaryCondition, separateVariables } = require('mathhook/pde');

const u = symbol('u');
const x = symbol('x');
const y = symbol('y');
const a = symbol('a');

const pde = new Pde(u, u, [x, y]);

const bcLeft = BoundaryCondition.dirichletAt(x, expr('0'), expr('0'));
const bcRight = BoundaryCondition.dirichletAt(x, a, expr('0'));
const bcs = [bcLeft, bcRight];

const ics = [];  // Laplace is elliptic

const solution = separateVariables(pde, bcs, ics);

```



