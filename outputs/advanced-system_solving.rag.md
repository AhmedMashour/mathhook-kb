# System of Equations Solving

Solve systems of equations (linear and nonlinear) with multiple unknowns
using substitution, elimination, matrix methods, and Newton's method for
nonlinear systems.


---
chunk_id: advanced_system_solving::0
topic: advanced.system_solving
title: Linear System (2×2)
priority: medium
keywords:
  - system_solving
  - linear system (2×2)
languages: [rust, python, javascript]
chunk: 1/4
---

## Linear System (2×2)

Solve { 2x + y = 5, x - y = 1 }

### Rust

```rust
let x = symbol!(x);
let y = symbol!(y);

// Method 1: Equations as list
let solver = SystemSolver::new();
let equations = vec![
    expr!(2*x + y - 5),
    expr!(x - y - 1),
];
let vars = vec![x.clone(), y.clone()];

let solution = solver.solve_system(&equations, &vars);
// Result: { x = 2, y = 1 }

// Method 2: Matrix form Ax = b
let A = Expression::matrix(vec![
    vec![expr!(2), expr!(1)],
    vec![expr!(1), expr!(-1)],
]);
let b = Expression::matrix(vec![
    vec![expr!(5)],
    vec![expr!(1)],
]);

let solution_matrix = expr!(A^(-1) * b);
// Result: [[2], [1]]

```

### Python

```python
from sympy import symbols, solve, Matrix

x, y = symbols('x y')

# Method 1: Equations
equations = [2*x + y - 5, x - y - 1]
solution = solve(equations, [x, y])
# Result: {x: 2, y: 1}

# Method 2: Matrix form
A = Matrix([[2, 1], [1, -1]])
b = Matrix([[5], [1]])
solution_matrix = A.inv() * b
# Result: Matrix([[2], [1]])

```

### JavaScript

```javascript
const x = symbol('x');
const y = symbol('y');

// Equations
const equations = [
    sub(add(mul(2, x), y), 5),
    sub(sub(x, y), 1)
];

const solution = solve(equations, [x, y]);
// Result: {x: 2, y: 1}

// Matrix form
const A = matrix([[2, 1], [1, -1]]);
const b = matrix([[5], [1]]);
const sol = A.inv().mul(b);

```



---
chunk_id: advanced_system_solving::1
topic: advanced.system_solving
title: Nonlinear System
priority: medium
keywords:
  - system_solving
  - nonlinear system
languages: [rust, python, javascript]
chunk: 2/4
---

## Nonlinear System

Solve { x^2 + y^2 = 25, x + y = 5 }

### Rust

```rust
let x = symbol!(x);
let y = symbol!(y);

// Step 1: Solve linear for y: y = 5 - x
// Step 2: Substitute into nonlinear
let substituted = expr!(x^2 + (5 - x)^2 - 25);
// Simplifies to: 2x^2 - 10x = 0 → x(x - 5) = 0

// Solutions: x = 0 or x = 5
// Corresponding y values: y = 5 or y = 0
// Two solutions: (0, 5) and (5, 0)

```

### Python

```python
from sympy import symbols, solve

x, y = symbols('x y')

equations = [x**2 + y**2 - 25, x + y - 5]
solutions = solve(equations, [x, y])
# Result: [(0, 5), (5, 0)]

```

### JavaScript

```javascript
const x = symbol('x');
const y = symbol('y');

const equations = [
    sub(add(pow(x, 2), pow(y, 2)), 25),
    sub(add(x, y), 5)
];

const solutions = solve(equations, [x, y]);
// Result: [[0, 5], [5, 0]]

```



---
chunk_id: advanced_system_solving::2
topic: advanced.system_solving
title: Three Variables
priority: medium
keywords:
  - system_solving
  - three variables
languages: [rust, python, javascript]
chunk: 3/4
---

## Three Variables

Solve { x + y + z = 6, 2x - y + z = 3, x + 2y - z = 2 }

### Rust

```rust
let x = symbol!(x);
let y = symbol!(y);
let z = symbol!(z);

// Matrix form
let A = Expression::matrix(vec![
    vec![expr!(1), expr!(1), expr!(1)],
    vec![expr!(2), expr!(-1), expr!(1)],
    vec![expr!(1), expr!(2), expr!(-1)],
]);

let b = Expression::matrix(vec![
    vec![expr!(6)],
    vec![expr!(3)],
    vec![expr!(2)],
]);

let solution = expr!(A^(-1) * b);
// Result: x = 1, y = 2, z = 3

```

### Python

```python
from sympy import symbols, solve, Matrix

x, y, z = symbols('x y z')

A = Matrix([[1, 1, 1], [2, -1, 1], [1, 2, -1]])
b = Matrix([[6], [3], [2]])

solution = A.inv() * b
# Result: Matrix([[1], [2], [3]])

```

### JavaScript

```javascript
const [x, y, z] = symbols(['x', 'y', 'z']);

const A = matrix([[1, 1, 1], [2, -1, 1], [1, 2, -1]]);
const b = matrix([[6], [3], [2]]);

const solution = A.inv().mul(b);

```



---
chunk_id: advanced_system_solving::3
topic: advanced.system_solving
title: Overdetermined System (Least Squares)
priority: medium
keywords:
  - system_solving
  - overdetermined system (least squares)
languages: [rust, python, javascript]
chunk: 4/4
---

## Overdetermined System (Least Squares)

More equations than unknowns: find best approximate solution

### Rust

```rust
// System: { x + y = 1, 2x + 2y = 3, x - y = 0 }
// Inconsistent! Use least squares.

let A = Expression::matrix(vec![
    vec![expr!(1), expr!(1)],
    vec![expr!(2), expr!(2)],
    vec![expr!(1), expr!(-1)],
]);

let b = Expression::matrix(vec![
    vec![expr!(1)],
    vec![expr!(3)],
    vec![expr!(0)],
]);

// Least squares: (A^T A)^(-1) A^T b
let AT = expr!(transpose(A));
let ATA = expr!(AT * A);
let ATA_inv = expr!(ATA^(-1));
let ATb = expr!(AT * b);

let x_ls = expr!(ATA_inv * ATb);
// Result: Best approximate solution

```

### Python

```python
from sympy import Matrix

A = Matrix([[1, 1], [2, 2], [1, -1]])
b = Matrix([[1], [3], [0]])

# Least squares
x_ls = (A.T * A).inv() * A.T * b

```

### JavaScript

```javascript
const A = matrix([[1, 1], [2, 2], [1, -1]]);
const b = matrix([[1], [3], [0]]);

// Least squares
const AT = A.transpose();
const x_ls = AT.mul(A).inv().mul(AT).mul(b);

```



