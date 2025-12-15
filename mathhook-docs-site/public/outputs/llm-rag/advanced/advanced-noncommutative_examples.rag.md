# Noncommutative Algebra Examples

Comprehensive examples of noncommutative algebra in MathHook covering
quantum mechanics operators, matrix algebra, quaternion rotations, and
bulk symbol creation patterns.


---
chunk_id: advanced_noncommutative_examples::0
topic: advanced.noncommutative_examples
title: Quantum Commutator
priority: medium
keywords:
  - noncommutative_examples
  - quantum commutator
languages: [rust, python, javascript]
chunk: 1/8
---

## Quantum Commutator

Position-momentum canonical commutation relation [x,p] = iℏ

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x; operator);  // Position operator
let p = symbol!(p; operator);  // Momentum operator

// Commutator: [x, p] = xp - px
let xp = expr!(x * p);
let px = expr!(p * x);
let commutator = expr!(xp - px);

// These are structurally different (noncommutative)
assert_ne!(xp.to_string(), px.to_string());

// LaTeX output preserves operator hats
let latex = commutator.to_latex(None).unwrap();
// Output: \hat{x}\hat{p} - \hat{p}\hat{x}

```

### Python

```python
from mathhook import symbol, expr

# Create operator symbols
x = symbol('x', type='operator')  # Position operator
p = symbol('p', type='operator')  # Momentum operator

# Commutator: [x, p] = xp - px
xp = x * p
px = p * x
commutator = xp - px

# These are structurally different (noncommutative)
assert str(xp) != str(px)

# LaTeX output preserves operator hats
latex = commutator.to_latex()
# Output: \hat{x}\hat{p} - \hat{p}\hat{x}

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');

// Create operator symbols
const x = symbol('x', {type: 'operator'});  // Position operator
const p = symbol('p', {type: 'operator'});  // Momentum operator

// Commutator: [x, p] = xp - px
const xp = x.mul(p);
const px = p.mul(x);
const commutator = xp.sub(px);

// These are structurally different (noncommutative)
console.log(xp.toString() !== px.toString());  // true

// LaTeX output preserves operator hats
const latex = commutator.toLatex();
// Output: \hat{x}\hat{p} - \hat{p}\hat{x}

```



---
chunk_id: advanced_noncommutative_examples::1
topic: advanced.noncommutative_examples
title: Angular Momentum Operators
priority: medium
keywords:
  - noncommutative_examples
  - angular momentum operators
languages: [rust, python, javascript]
chunk: 2/8
---

## Angular Momentum Operators

Quantum angular momentum with [Lx, Ly] = iℏLz

### Rust

```rust
use mathhook::prelude::*;

let lx = symbol!(Lx; operator);
let ly = symbol!(Ly; operator);
let lz = symbol!(Lz; operator);

// Lx*Ly product
let lx_ly = expr!(lx * ly);

// Ly*Lx product
let ly_lx = expr!(ly * lx);

// These are NOT equal (noncommutative)
assert_ne!(lx_ly.to_string(), ly_lx.to_string());

// Commutator [Lx, Ly] = Lx*Ly - Ly*Lx
let commutator = expr!(lx_ly - ly_lx);
// In quantum mechanics, this equals i*hbar*Lz

```

### Python

```python
from mathhook import symbol, expr

lx = symbol('Lx', type='operator')
ly = symbol('Ly', type='operator')
lz = symbol('Lz', type='operator')

# Lx*Ly product
lx_ly = lx * ly

# Ly*Lx product
ly_lx = ly * lx

# These are NOT equal (noncommutative)
assert str(lx_ly) != str(ly_lx)

# Commutator [Lx, Ly] = Lx*Ly - Ly*Lx
commutator = lx_ly - ly_lx
# In quantum mechanics, this equals i*hbar*Lz

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

const lx = symbol('Lx', {type: 'operator'});
const ly = symbol('Ly', {type: 'operator'});
const lz = symbol('Lz', {type: 'operator'});

// Lx*Ly product
const lx_ly = lx.mul(ly);

// Ly*Lx product
const ly_lx = ly.mul(lx);

// These are NOT equal (noncommutative)
console.log(lx_ly.toString() !== ly_lx.toString());  // true

```



---
chunk_id: advanced_noncommutative_examples::2
topic: advanced.noncommutative_examples
title: Matrix Equation Left Division
priority: medium
keywords:
  - noncommutative_examples
  - matrix equation left division
languages: [rust, python, javascript]
chunk: 3/8
---

## Matrix Equation Left Division

Solve A*X = B using left division X = A^(-1)*B

### Rust

```rust
use mathhook::prelude::*;

let solver = MatrixEquationSolver::new();
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Equation: A*X - B = 0 (i.e., A*X = B)
let equation = expr!((A * X) - B);

let result = solver.solve(&equation, &X);
// Returns: X = A^(-1)*B (left multiplication by inverse)

// Note: We multiply by A^(-1) on the LEFT because X is on the right of A

```

### Python

```python
from mathhook import symbol, MatrixEquationSolver

solver = MatrixEquationSolver()
A = symbol('A', type='matrix')
X = symbol('X', type='matrix')
B = symbol('B', type='matrix')

# Equation: A*X = B
equation = A * X - B

result = solver.solve(equation, X)
# Returns: X = A.inv() * B (left multiplication by inverse)

# Note: We multiply by A^(-1) on the LEFT because X is on the right of A

```

### JavaScript

```javascript
const { symbol, MatrixEquationSolver } = require('mathhook');

const solver = new MatrixEquationSolver();
const A = symbol('A', {type: 'matrix'});
const X = symbol('X', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

// Equation: A*X = B
const equation = A.mul(X).sub(B);

const result = solver.solve(equation, X);
// Returns: X = A.inv().mul(B) (left multiplication by inverse)

```



---
chunk_id: advanced_noncommutative_examples::3
topic: advanced.noncommutative_examples
title: Matrix Equation Right Division
priority: medium
keywords:
  - noncommutative_examples
  - matrix equation right division
languages: [rust, python, javascript]
chunk: 4/8
---

## Matrix Equation Right Division

Solve X*A = B using right division X = B*A^(-1)

### Rust

```rust
use mathhook::prelude::*;

let solver = MatrixEquationSolver::new();
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Equation: X*A - B = 0 (i.e., X*A = B)
let equation = expr!((X * A) - B);

let result = solver.solve(&equation, &X);
// Returns: X = B*A^(-1) (right multiplication by inverse)

// Note: We multiply by A^(-1) on the RIGHT because X is on the left of A

```

### Python

```python
from mathhook import symbol, MatrixEquationSolver

solver = MatrixEquationSolver()
A = symbol('A', type='matrix')
X = symbol('X', type='matrix')
B = symbol('B', type='matrix')

# Equation: X*A = B
equation = X * A - B

result = solver.solve(equation, X)
# Returns: X = B * A.inv() (right multiplication by inverse)

# Note: We multiply by A^(-1) on the RIGHT because X is on the left of A

```

### JavaScript

```javascript
const { symbol, MatrixEquationSolver } = require('mathhook');

const solver = new MatrixEquationSolver();
const A = symbol('A', {type: 'matrix'});
const X = symbol('X', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

// Equation: X*A = B
const equation = X.mul(A).sub(B);

const result = solver.solve(equation, X);
// Returns: X = B.mul(A.inv()) (right multiplication by inverse)

```



---
chunk_id: advanced_noncommutative_examples::4
topic: advanced.noncommutative_examples
title: Quaternion Multiplication
priority: medium
keywords:
  - noncommutative_examples
  - quaternion multiplication
languages: [rust, python, javascript]
chunk: 5/8
---

## Quaternion Multiplication

Noncommutative quaternion basis multiplication i*j = k, j*i = -k

### Rust

```rust
use mathhook::prelude::*;

let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);
let k = symbol!(k; quaternion);

// i*j = k
let ij = expr!(i * j);

// j*i = -k (different!)
let ji = expr!(j * i);

// Order matters - multiplication is noncommutative
assert_ne!(ij.to_string(), ji.to_string());

// All quaternion products
// i*j = k, j*i = -k
// j*k = i, k*j = -i
// k*i = j, i*k = -j

```

### Python

```python
from mathhook import symbol

i = symbol('i', type='quaternion')
j = symbol('j', type='quaternion')
k = symbol('k', type='quaternion')

# i*j = k
ij = i * j

# j*i = -k (different!)
ji = j * i

# Order matters - multiplication is noncommutative
assert str(ij) != str(ji)

# All quaternion products:
# i*j = k, j*i = -k
# j*k = i, k*j = -i
# k*i = j, i*k = -j

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

const i = symbol('i', {type: 'quaternion'});
const j = symbol('j', {type: 'quaternion'});
const k = symbol('k', {type: 'quaternion'});

// i*j = k
const ij = i.mul(j);

// j*i = -k (different!)
const ji = j.mul(i);

// Order matters - multiplication is noncommutative
console.log(ij.toString() !== ji.toString());  // true

```



---
chunk_id: advanced_noncommutative_examples::5
topic: advanced.noncommutative_examples
title: 3D Rotation with Quaternions
priority: medium
keywords:
  - noncommutative_examples
  - 3d rotation with quaternions
languages: [rust, python, javascript]
chunk: 6/8
---

## 3D Rotation with Quaternions

Rotating a vector v by quaternion q: v' = q*v*conj(q)

### Rust

```rust
use mathhook::prelude::*;

let q = symbol!(q; quaternion);       // Rotation quaternion
let v = symbol!(v; quaternion);       // Vector as pure quaternion
let q_conj = symbol!(q_conj; quaternion); // Conjugate of q

// Rotation formula: v' = q*v*q_conj
let rotation = expr!(q * v * q_conj);

// The order matters:
// q * v * q_conj ≠ q_conj * v * q

```

### Python

```python
from mathhook import symbol

q = symbol('q', type='quaternion')       # Rotation quaternion
v = symbol('v', type='quaternion')       # Vector as pure quaternion
q_conj = symbol('q_conj', type='quaternion')  # Conjugate of q

# Rotation formula: v' = q*v*q_conj
rotation = q * v * q_conj

# The order matters:
# q * v * q_conj ≠ q_conj * v * q

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

const q = symbol('q', {type: 'quaternion'});       // Rotation quaternion
const v = symbol('v', {type: 'quaternion'});       // Vector as pure quaternion
const q_conj = symbol('q_conj', {type: 'quaternion'});  // Conjugate of q

// Rotation formula: v' = q*v*q_conj
const rotation = q.mul(v).mul(q_conj);

// The order matters:
// q * v * q_conj ≠ q_conj * v * q

```



---
chunk_id: advanced_noncommutative_examples::6
topic: advanced.noncommutative_examples
title: Bulk Symbol Creation
priority: medium
keywords:
  - noncommutative_examples
  - bulk symbol creation
languages: [rust, python, javascript]
chunk: 7/8
---

## Bulk Symbol Creation

Create multiple symbols at once using the symbols![] macro

### Rust

```rust
use mathhook::prelude::*;

// Multiple scalars (default type)
let scalars = symbols![x, y, z];
let x = &scalars[0];
let y = &scalars[1];
let z = &scalars[2];

// Multiple matrices
let matrices = symbols![A, B, C => matrix];
let A = &matrices[0];
let B = &matrices[1];
let C = &matrices[2];

// Multiple operators
let operators = symbols![p, x_op, H => operator];
let p = &operators[0];
let x_op = &operators[1];
let H = &operators[2];

// Multiple quaternions
let quaternions = symbols![i, j, k => quaternion];
let i = &quaternions[0];
let j = &quaternions[1];
let k = &quaternions[2];

```

### Python

```python
from mathhook import symbols

# Multiple scalars (default type)
x, y, z = symbols('x y z')

# Multiple matrices
A, B, C = symbols('A B C', type='matrix')

# Multiple operators
p, x_op, H = symbols('p x_op H', type='operator')

# Multiple quaternions
i, j, k = symbols('i j k', type='quaternion')

```

### JavaScript

```javascript
const { symbols } = require('mathhook');

// Multiple scalars (default type)
const [x, y, z] = symbols(['x', 'y', 'z']);

// Multiple matrices
const [A, B, C] = symbols(['A', 'B', 'C'], {type: 'matrix'});

// Multiple operators
const [p, x_op, H] = symbols(['p', 'x_op', 'H'], {type: 'operator'});

// Multiple quaternions
const [i, j, k] = symbols(['i', 'j', 'k'], {type: 'quaternion'});

```



---
chunk_id: advanced_noncommutative_examples::7
topic: advanced.noncommutative_examples
title: Complete Workflow Example
priority: medium
keywords:
  - noncommutative_examples
  - complete workflow example
languages: [rust, python, javascript]
chunk: 8/8
---

## Complete Workflow Example

End-to-end example: create symbols, build equation, solve, format as LaTeX

### Rust

```rust
use mathhook::prelude::*;
use mathhook::educational::message_registry::{
    MessageBuilder, MessageCategory, MessageType
};

// 1. Create matrix symbols
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// 2. Build equation: A*X = B
let equation = expr!((A * X) - B);

// 3. Solve equation
let solver = MatrixEquationSolver::new();
let result = solver.solve(&equation, &X);

// 4. Format solution as LaTeX
if let SolverResult::Single(solution) = result {
    let latex = solution.to_latex(None).unwrap();
    println!("Solution: {}", latex);
    // Output: \mathbf{A}^{-1} \cdot \mathbf{B}
}

// 5. Get educational explanation
let msg = MessageBuilder::new(
    MessageCategory::NoncommutativeAlgebra,
    MessageType::LeftMultiplyInverse,
    0
).build();

if let Some(message) = msg {
    println!("Explanation: {}", message.description);
}

```

### Python

```python
from mathhook import symbol, MatrixEquationSolver, SolverResult
from mathhook.educational import MessageBuilder, MessageCategory, MessageType

# 1. Create matrix symbols
A = symbol('A', type='matrix')
X = symbol('X', type='matrix')
B = symbol('B', type='matrix')

# 2. Build equation: A*X = B
equation = A * X - B

# 3. Solve equation
solver = MatrixEquationSolver()
result = solver.solve(equation, X)

# 4. Format solution as LaTeX
if isinstance(result, SolverResult.Single):
    latex = result.solution.to_latex()
    print(f"Solution: {latex}")
    # Output: \mathbf{A}^{-1} \cdot \mathbf{B}

# 5. Get educational explanation
msg = MessageBuilder(
    MessageCategory.NoncommutativeAlgebra,
    MessageType.LeftMultiplyInverse,
    step=0
).build()

if msg:
    print(f"Explanation: {msg.description}")

```

### JavaScript

```javascript
const { symbol, MatrixEquationSolver, SolverResult } = require('mathhook');
const { MessageBuilder, MessageCategory, MessageType } = require('mathhook/educational');

// 1. Create matrix symbols
const A = symbol('A', {type: 'matrix'});
const X = symbol('X', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

// 2. Build equation: A*X = B
const equation = A.mul(X).sub(B);

// 3. Solve equation
const solver = new MatrixEquationSolver();
const result = solver.solve(equation, X);

// 4. Format solution as LaTeX
if (result instanceof SolverResult.Single) {
    const latex = result.solution.toLatex();
    console.log(`Solution: ${latex}`);
    // Output: \mathbf{A}^{-1} \cdot \mathbf{B}
}

// 5. Get educational explanation
const msg = new MessageBuilder(
    MessageCategory.NoncommutativeAlgebra,
    MessageType.LeftMultiplyInverse,
    0
).build();

if (msg) {
    console.log(`Explanation: ${msg.description}`);
}

```



