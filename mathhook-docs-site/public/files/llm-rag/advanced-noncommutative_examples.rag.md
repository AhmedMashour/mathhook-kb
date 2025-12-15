# Noncommutative Algebra Examples

Comprehensive examples of noncommutative algebra in MathHook covering
quantum mechanics, matrix algebra, and quaternion rotations.


---
chunk_id: advanced_noncommutative_examples::0
topic: advanced.noncommutative_examples
title: Quantum Commutator
priority: medium
keywords:
  - noncommutative_examples
  - quantum commutator
languages: [rust, python, javascript]
chunk: 1/3
---

## Quantum Commutator

Position-momentum canonical commutation relation [x,p] = iℏ

### Rust

```rust
let x = symbol!(x; operator);
let p = symbol!(p; operator);

// Commutator: [x, p] = xp - px
let xp = expr!(x * p);
let px = expr!(p * x);
let commutator = expr!(xp - px);

// LaTeX output
let latex = commutator.to_latex(None).unwrap();
// Output: \hat{x}\hat{p} - \hat{p}\hat{x}

```

### Python

```python
from sympy.physics.quantum import Operator, Commutator

x = Operator('x')
p = Operator('p')

# Commutator
comm = Commutator(x, p)
# Result: I*hbar (in quantum mechanics)

```

### JavaScript

```javascript
const x = symbol('x', {type: 'operator'});
const p = symbol('p', {type: 'operator'});

// Commutator
const xp = x.mul(p);
const px = p.mul(x);
const comm = xp.sub(px);

```



---
chunk_id: advanced_noncommutative_examples::1
topic: advanced.noncommutative_examples
title: Matrix Equation Left Division
priority: medium
keywords:
  - noncommutative_examples
  - matrix equation left division
languages: [rust, python, javascript]
chunk: 2/3
---

## Matrix Equation Left Division

Solve A*X = B using left division X = A^(-1)*B

### Rust

```rust
let solver = MatrixEquationSolver::new();
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Equation: A*X - B = 0
let equation = expr!((A * X) - B);

let result = solver.solve(&equation, &X);
// Returns: X = A^(-1)*B (left division)

```

### Python

```python
from sympy import symbols, MatrixSymbol, solve, Eq

A = MatrixSymbol('A', n, n)
X = MatrixSymbol('X', n, n)
B = MatrixSymbol('B', n, n)

# Solve A*X = B
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



---
chunk_id: advanced_noncommutative_examples::2
topic: advanced.noncommutative_examples
title: Quaternion Multiplication
priority: medium
keywords:
  - noncommutative_examples
  - quaternion multiplication
languages: [rust, python, javascript]
chunk: 3/3
---

## Quaternion Multiplication

Noncommutative quaternion basis multiplication i*j = k, j*i = -k

### Rust

```rust
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);

// i*j = k
let ij = expr!(i * j);

// j*i = -k (different!)
let ji = expr!(j * i);

// Order matters
assert_ne!(ij.to_string(), ji.to_string());

```

### Python

```python
from sympy.algebras.quaternion import Quaternion

i = Quaternion(0, 1, 0, 0)
j = Quaternion(0, 0, 1, 0)
k = Quaternion(0, 0, 0, 1)

# Verify: i*j = k
assert i * j == k

# Verify: j*i = -k (noncommutative!)
assert j * i == -k

```

### JavaScript

```javascript
const i = symbol('i', {type: 'quaternion'});
const j = symbol('j', {type: 'quaternion'});

const ij = i.mul(j);  // k
const ji = j.mul(i);  // -k

```



