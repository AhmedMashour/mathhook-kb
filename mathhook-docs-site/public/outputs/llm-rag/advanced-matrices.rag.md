# Matrix Operations

Work with matrices symbolically and numerically in MathHook, with full support
for noncommutative algebra where order matters. Create matrices, perform
operations, and solve matrix equations.


---
chunk_id: advanced_matrices::0
topic: advanced.matrices
title: Creating Matrices
priority: medium
keywords:
  - matrices
  - creating matrices
languages: [rust, python, javascript]
chunk: 1/4
---

## Creating Matrices

Create matrix symbols and numeric matrices

### Rust

```rust
// Matrix symbols (noncommutative)
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Numeric 2×2 matrix
let M = Expression::matrix(vec![
    vec![expr!(1), expr!(2)],
    vec![expr!(3), expr!(4)],
]);

```

### Python

```python
# Matrix symbols
A = MatrixSymbol('A', n, m)
B = MatrixSymbol('B', m, p)

# Numeric matrix
M = Matrix([[1, 2], [3, 4]])

```

### JavaScript

```javascript
// Matrix symbols
const A = symbol('A', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

// Numeric matrix
const M = matrix([[1, 2], [3, 4]]);

```



---
chunk_id: advanced_matrices::1
topic: advanced.matrices
title: Matrix Multiplication (Noncommutative)
priority: medium
keywords:
  - matrices
  - matrix multiplication (noncommutative)
languages: [rust, python, javascript]
chunk: 2/4
---

## Matrix Multiplication (Noncommutative)

A*B ≠ B*A in general

### Rust

```rust
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

let AB = expr!(A * B);  // A*B
let BA = expr!(B * A);  // B*A ≠ A*B

```

### Python

```python
A = MatrixSymbol('A', n, n)
B = MatrixSymbol('B', n, n)

AB = A * B  # Matrix product
BA = B * A  # Different result!

```

### JavaScript

```javascript
const A = symbol('A', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

const AB = A.mul(B);  // A*B
const BA = B.mul(A);  // B*A ≠ A*B

```



---
chunk_id: advanced_matrices::2
topic: advanced.matrices
title: Solving Linear System Ax=b
priority: medium
keywords:
  - matrices
  - solving linear system ax=b
languages: [rust, python, javascript]
chunk: 3/4
---

## Solving Linear System Ax=b

Solve matrix equation using inverse

### Rust

```rust
let A = Expression::matrix(vec![
    vec![expr!(2), expr!(1)],
    vec![expr!(1), expr!(-1)],
]);
let b = Expression::matrix(vec![
    vec![expr!(5)],
    vec![expr!(1)],
]);

// Solution: x = A^(-1)*b
let x = expr!(A^(-1) * b);
// Result: [[2], [1]]

```

### Python

```python
A = Matrix([[2, 1], [1, -1]])
b = Matrix([[5], [1]])

# Solution
x = A.inv() * b
# Result: Matrix([[2], [1]])

```

### JavaScript

```javascript
const A = matrix([[2, 1], [1, -1]]);
const b = matrix([[5], [1]]);

// Solution
const x = A.inv().mul(b);
// Result: [[2], [1]]

```



---
chunk_id: advanced_matrices::3
topic: advanced.matrices
title: Matrix Equation A*X=B (Left Division)
priority: medium
keywords:
  - matrices
  - matrix equation a*x=b (left division)
languages: [rust, python, javascript]
chunk: 4/4
---

## Matrix Equation A*X=B (Left Division)

Solve for matrix unknown X

### Rust

```rust
let solver = MatrixEquationSolver::new();
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

let equation = expr!((A * X) - B);
let solution = solver.solve(&equation, &X);
// Returns: X = A^(-1)*B (left division)

```

### Python

```python
A, X, B = symbols('A X B', matrix=True)
equation = Eq(A*X, B)
solution = solve(equation, X)
# Returns: X = A^(-1)*B

```

### JavaScript

```javascript
const A = symbol('A', {type: 'matrix'});
const X = symbol('X', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

const equation = A.mul(X).sub(B);
const solution = solve(equation, X);
// Returns: X = A.inv().mul(B)

```



