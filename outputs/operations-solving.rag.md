# Solving Equations

Find solutions to equations symbolically and numerically.


---
chunk_id: operations_solving::0
topic: operations.solving
title: Linear Equations
priority: medium
keywords:
  - solving
  - linear equations
languages: [rust, python, javascript]
chunk: 1/5
---

## Linear Equations

Solve ax + b = 0

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Solve: 2x + 3 = 0
let eq1 = expr!(2 * x + 3);
let mut solver = MathSolver::new();
let sol1 = solver.solve(&eq1, &x);
// Result: x = -3/2

// Solve: 5x - 10 = 0
let eq2 = expr!(5 * x - 10);
let sol2 = solver.solve(&eq2, &x);
// Result: x = 2

```

### Python

```python
from mathhook import symbol, solve

x = symbol('x')

# Solve: 2x + 3 = 0
eq1 = 2*x + 3
sol1 = solve(eq1, x)
# Result: x = -3/2

# Solve: 5x - 10 = 0
eq2 = 5*x - 10
sol2 = solve(eq2, x)
# Result: x = 2

```

### JavaScript

```javascript
const { symbol, solve } = require('mathhook');

const x = symbol('x');

// Solve: 2x + 3 = 0
const eq1 = x.mul(2).add(3);
const sol1 = solve(eq1, x);
// Result: x = -3/2

```



---
chunk_id: operations_solving::1
topic: operations.solving
title: Quadratic Equations
priority: medium
keywords:
  - solving
  - quadratic equations
languages: [rust, python, javascript]
chunk: 2/5
---

## Quadratic Equations

Solve ax² + bx + c = 0

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Solve: x² - 5x + 6 = 0
let eq1 = expr!(x ^ 2 - 5 * x + 6);
let mut solver = MathSolver::new();
let solutions = solver.solve(&eq1, &x);
// Result: [x = 2, x = 3]

// Solve: x² - 4 = 0 (difference of squares)
let eq2 = expr!(x ^ 2 - 4);
let sol2 = solver.solve(&eq2, &x);
// Result: [x = -2, x = 2]

```

### Python

```python
from mathhook import symbol, solve

x = symbol('x')

# Solve: x² - 5x + 6 = 0
eq1 = x**2 - 5*x + 6
solutions = solve(eq1, x)
# Result: [2, 3]

# Solve: x² - 4 = 0
eq2 = x**2 - 4
sol2 = solve(eq2, x)
# Result: [-2, 2]

```

### JavaScript

```javascript
const { symbol, solve } = require('mathhook');

const x = symbol('x');

// Solve: x² - 5x + 6 = 0
const eq1 = x.pow(2).sub(x.mul(5)).add(6);
const solutions = solve(eq1, x);
// Result: [2, 3]

```



---
chunk_id: operations_solving::2
topic: operations.solving
title: Complex Roots
priority: medium
keywords:
  - solving
  - complex roots
languages: [rust, python, javascript]
chunk: 3/5
---

## Complex Roots

When discriminant is negative

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Solve: x² + 1 = 0 (complex roots)
let equation = expr!(x ^ 2 + 1);
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x);
// Result: [x = i, x = -i]

// Solve: x² - 2x + 5 = 0
// Discriminant: 4 - 20 = -16 < 0 (complex roots)
let eq2 = expr!(x ^ 2 - 2 * x + 5);
let sol2 = solver.solve(&eq2, &x);
// Result: [x = 1 + 2i, x = 1 - 2i]

```

### Python

```python
from mathhook import symbol, solve, I

x = symbol('x')

# Solve: x² + 1 = 0
equation = x**2 + 1
solutions = solve(equation, x)
# Result: [I, -I]

# Solve: x² - 2x + 5 = 0
eq2 = x**2 - 2*x + 5
sol2 = solve(eq2, x)
# Result: [1 + 2*I, 1 - 2*I]

```

### JavaScript

```javascript
const { symbol, solve } = require('mathhook');

const x = symbol('x');

// Solve: x² + 1 = 0
const equation = x.pow(2).add(1);
const solutions = solve(equation, x);
// Result: [i, -i]

```



---
chunk_id: operations_solving::3
topic: operations.solving
title: Transcendental Equations
priority: medium
keywords:
  - solving
  - transcendental equations
languages: [rust, python, javascript]
chunk: 4/5
---

## Transcendental Equations

Trigonometric, exponential, logarithmic

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Solve: sin(x) = 0
let eq1 = expr!(sin(x));
let mut solver = MathSolver::new();
let solutions = solver.solve(&eq1, &x);
// Result: [x = 0, x = π, x = 2π, ...] (infinitely many)

// Solve: e^x = 5
let eq2 = expr!(exp(x) - 5);
let sol2 = solver.solve(&eq2, &x);
// Result: x = ln(5)

// Solve: log(x) = 2
let eq3 = expr!(log(x) - 2);
let sol3 = solver.solve(&eq3, &x);
// Result: x = e² (if natural log)

```

### Python

```python
from mathhook import symbol, solve, sin, exp, log

x = symbol('x')

# Solve: sin(x) = 0
eq1 = sin(x)
solutions = solve(eq1, x)
# Result: [0, π, 2π, ...]

# Solve: e^x = 5
eq2 = exp(x) - 5
sol2 = solve(eq2, x)
# Result: log(5)

```

### JavaScript

```javascript
const { symbol, solve, parse } = require('mathhook');

const x = symbol('x');

// Solve: sin(x) = 0
const eq1 = parse('sin(x)');
const solutions = solve(eq1, x);
// Result: [0, π, 2π, ...]

```



---
chunk_id: operations_solving::4
topic: operations.solving
title: Matrix Equations (Noncommutative)
priority: medium
keywords:
  - solving
  - matrix equations (noncommutative)
languages: [rust, python, javascript]
chunk: 5/5
---

## Matrix Equations (Noncommutative)

Left and right division for matrices

### Rust

```rust
use mathhook::prelude::*;

// Matrix symbols
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Left division: A*X = B → X = A⁻¹*B
let left_eq = expr!(A * X - B);
let mut solver = MathSolver::new();
let solution_left = solver.solve(&left_eq, &X);
// Result: X = A⁻¹*B

// Right division: X*A = B → X = B*A⁻¹
let right_eq = expr!(X * A - B);
let solution_right = solver.solve(&right_eq, &X);
// Result: X = B*A⁻¹

// Note: A⁻¹*B ≠ B*A⁻¹ for matrices!

```

### Python

```python
from mathhook import symbol, solve

# Matrix symbols
A = symbol('A', matrix=True)
X = symbol('X', matrix=True)
B = symbol('B', matrix=True)

# Left division: A*X = B → X = A⁻¹*B
left_eq = A*X - B
solution_left = solve(left_eq, X)
# Result: X = A^(-1)*B

```

### JavaScript

```javascript
const { symbol, solve } = require('mathhook');

// Matrix symbols
const A = symbol('A', { type: 'matrix' });
const X = symbol('X', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });

// Left division: A*X = B → X = A⁻¹*B
const leftEq = A.mul(X).sub(B);
const solutionLeft = solve(leftEq, X);
// Result: X = A^(-1)*B

```



