# Noncommutative Algebra API Reference

Complete API reference for MathHook's noncommutative algebra support,
including symbol creation macros, type queries, expression creation,
equation solving, and LaTeX formatting.


---
chunk_id: advanced_noncommutative_api_reference::0
topic: advanced.noncommutative_api_reference
title: symbol!(name) - Create Scalar Symbol
priority: medium
keywords:
  - noncommutative_api_reference
  - symbol!(name) - create scalar symbol
languages: [rust, python, javascript]
chunk: 1/8
---

## symbol!(name) - Create Scalar Symbol

Creates a scalar (commutative) symbol with the given name

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let theta = symbol!(theta);

// Scalars are commutative
assert_eq!(x.symbol_type(), SymbolType::Scalar);
assert_eq!(x.commutativity(), Commutativity::Commutative);

```

### Python

```python
from mathhook import symbol

x = symbol('x')
theta = symbol('theta')

# Scalars are commutative
assert x.symbol_type == 'scalar'
assert x.is_commutative == True

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

const x = symbol('x');
const theta = symbol('theta');

// Scalars are commutative
console.log(x.symbolType);  // 'scalar'
console.log(x.isCommutative);  // true

```



---
chunk_id: advanced_noncommutative_api_reference::1
topic: advanced.noncommutative_api_reference
title: symbol!(name; type) - Create Typed Symbol
priority: medium
keywords:
  - noncommutative_api_reference
  - symbol!(name; type) - create typed symbol
languages: [rust, python, javascript]
chunk: 2/8
---

## symbol!(name; type) - Create Typed Symbol

Creates a symbol with specified type (matrix, operator, quaternion)

### Rust

```rust
use mathhook::prelude::*;

// Matrix (noncommutative)
let A = symbol!(A; matrix);
assert_eq!(A.symbol_type(), SymbolType::Matrix);
assert_eq!(A.commutativity(), Commutativity::Noncommutative);

// Operator (noncommutative)
let p = symbol!(p; operator);
assert_eq!(p.symbol_type(), SymbolType::Operator);

// Quaternion (noncommutative)
let i = symbol!(i; quaternion);
assert_eq!(i.symbol_type(), SymbolType::Quaternion);

```

### Python

```python
from mathhook import symbol

# Matrix (noncommutative)
A = symbol('A', type='matrix')
assert A.symbol_type == 'matrix'
assert A.is_commutative == False

# Operator (noncommutative)
p = symbol('p', type='operator')
assert p.symbol_type == 'operator'

# Quaternion (noncommutative)
i = symbol('i', type='quaternion')
assert i.symbol_type == 'quaternion'

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

// Matrix (noncommutative)
const A = symbol('A', {type: 'matrix'});
console.log(A.symbolType);  // 'matrix'
console.log(A.isCommutative);  // false

// Operator (noncommutative)
const p = symbol('p', {type: 'operator'});
console.log(p.symbolType);  // 'operator'

// Quaternion (noncommutative)
const i = symbol('i', {type: 'quaternion'});
console.log(i.symbolType);  // 'quaternion'

```



---
chunk_id: advanced_noncommutative_api_reference::2
topic: advanced.noncommutative_api_reference
title: symbols![...] - Bulk Symbol Creation
priority: medium
keywords:
  - noncommutative_api_reference
  - symbols![...] - bulk symbol creation
languages: [rust, python, javascript]
chunk: 3/8
---

## symbols![...] - Bulk Symbol Creation

Create multiple symbols at once with same type

### Rust

```rust
use mathhook::prelude::*;

// Multiple scalars (default)
let scalars = symbols![x, y, z];
assert_eq!(scalars.len(), 3);

// Multiple matrices
let matrices = symbols![A, B, C => matrix];
assert_eq!(matrices[0].symbol_type(), SymbolType::Matrix);

// Multiple operators
let operators = symbols![p, x, H => operator];
assert_eq!(operators[0].symbol_type(), SymbolType::Operator);

// Multiple quaternions
let quaternions = symbols![i, j, k => quaternion];
assert_eq!(quaternions[0].symbol_type(), SymbolType::Quaternion);

```

### Python

```python
from mathhook import symbols

# Multiple scalars (default)
x, y, z = symbols('x y z')

# Multiple matrices
A, B, C = symbols('A B C', type='matrix')
assert A.symbol_type == 'matrix'

# Multiple operators
p, x_op, H = symbols('p x_op H', type='operator')
assert p.symbol_type == 'operator'

# Multiple quaternions
i, j, k = symbols('i j k', type='quaternion')
assert i.symbol_type == 'quaternion'

```

### JavaScript

```javascript
const { symbols } = require('mathhook');

// Multiple scalars (default)
const [x, y, z] = symbols(['x', 'y', 'z']);

// Multiple matrices
const [A, B, C] = symbols(['A', 'B', 'C'], {type: 'matrix'});
console.log(A.symbolType);  // 'matrix'

// Multiple operators
const [p, x_op, H] = symbols(['p', 'x_op', 'H'], {type: 'operator'});

// Multiple quaternions
const [i, j, k] = symbols(['i', 'j', 'k'], {type: 'quaternion'});

```



---
chunk_id: advanced_noncommutative_api_reference::3
topic: advanced.noncommutative_api_reference
title: SymbolType Enum and Query Methods
priority: medium
keywords:
  - noncommutative_api_reference
  - symboltype enum and query methods
languages: [rust, python, javascript]
chunk: 4/8
---

## SymbolType Enum and Query Methods

Check symbol type and commutativity

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let A = symbol!(A; matrix);

// Type check
assert_eq!(x.symbol_type(), SymbolType::Scalar);
assert_eq!(A.symbol_type(), SymbolType::Matrix);

// Commutativity check
assert_eq!(x.commutativity(), Commutativity::Commutative);
assert_eq!(A.commutativity(), Commutativity::Noncommutative);

// Match on type
match A.symbol_type() {
    SymbolType::Scalar => println!("Scalar"),
    SymbolType::Matrix => println!("Matrix"),
    SymbolType::Operator => println!("Operator"),
    SymbolType::Quaternion => println!("Quaternion"),
}

```

### Python

```python
from mathhook import symbol, SymbolType, Commutativity

x = symbol('x')
A = symbol('A', type='matrix')

# Type check
assert x.symbol_type == SymbolType.Scalar
assert A.symbol_type == SymbolType.Matrix

# Commutativity check
assert x.commutativity == Commutativity.Commutative
assert A.commutativity == Commutativity.Noncommutative

# Check type
if A.symbol_type == SymbolType.Matrix:
    print("Matrix")

```

### JavaScript

```javascript
const { symbol, SymbolType, Commutativity } = require('mathhook');

const x = symbol('x');
const A = symbol('A', {type: 'matrix'});

// Type check
console.log(x.symbolType === SymbolType.Scalar);  // true
console.log(A.symbolType === SymbolType.Matrix);  // true

// Commutativity check
console.log(x.commutativity === Commutativity.Commutative);  // true
console.log(A.commutativity === Commutativity.Noncommutative);  // true

```



---
chunk_id: advanced_noncommutative_api_reference::4
topic: advanced.noncommutative_api_reference
title: Expression::mul - Order Matters!
priority: medium
keywords:
  - noncommutative_api_reference
  - expression::mul - order matters!
languages: [rust, python, javascript]
chunk: 5/8
---

## Expression::mul - Order Matters!

Creating multiplication expressions - order preserved for noncommutative

### Rust

```rust
use mathhook::prelude::*;

let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// A*B ≠ B*A in general
let ab = Expression::mul(vec![
    Expression::symbol(A.clone()),
    Expression::symbol(B.clone())
]);

let ba = Expression::mul(vec![
    Expression::symbol(B),
    Expression::symbol(A)
]);

// Structurally different
assert_ne!(ab.to_string(), ba.to_string());

// Using expr! macro (preferred)
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let ab = expr!(A * B);
let ba = expr!(B * A);
assert_ne!(ab.to_string(), ba.to_string());

```

### Python

```python
from mathhook import symbol, Expression

A = symbol('A', type='matrix')
B = symbol('B', type='matrix')

# A*B ≠ B*A in general
ab = A * B
ba = B * A

# Structurally different
assert str(ab) != str(ba)

```

### JavaScript

```javascript
const { symbol, Expression } = require('mathhook');

const A = symbol('A', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

// A*B ≠ B*A in general
const ab = A.mul(B);
const ba = B.mul(A);

// Structurally different
console.log(ab.toString() !== ba.toString());  // true

```



---
chunk_id: advanced_noncommutative_api_reference::5
topic: advanced.noncommutative_api_reference
title: MatrixEquationSolver
priority: medium
keywords:
  - noncommutative_api_reference
  - matrixequationsolver
languages: [rust, python, javascript]
chunk: 6/8
---

## MatrixEquationSolver

Solve matrix equations accounting for noncommutativity

### Rust

```rust
use mathhook::prelude::*;

let solver = MatrixEquationSolver::new();

let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// A*X = B → X = A^(-1)*B (left multiply by A^(-1))
let eq1 = expr!((A * X) - B);
let result1 = solver.solve(&eq1, &X);
// Returns: X = A^(-1) * B

// X*A = B → X = B*A^(-1) (right multiply by A^(-1))
let eq2 = expr!((X * A) - B);
let result2 = solver.solve(&eq2, &X);
// Returns: X = B * A^(-1)

```

### Python

```python
from mathhook import symbol, MatrixEquationSolver

solver = MatrixEquationSolver()

A = symbol('A', type='matrix')
X = symbol('X', type='matrix')
B = symbol('B', type='matrix')

# A*X = B → X = A^(-1)*B (left multiply by A^(-1))
eq1 = A * X - B
result1 = solver.solve(eq1, X)
# Returns: X = A.inv() * B

# X*A = B → X = B*A^(-1) (right multiply by A^(-1))
eq2 = X * A - B
result2 = solver.solve(eq2, X)
# Returns: X = B * A.inv()

```

### JavaScript

```javascript
const { symbol, MatrixEquationSolver } = require('mathhook');

const solver = new MatrixEquationSolver();

const A = symbol('A', {type: 'matrix'});
const X = symbol('X', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

// A*X = B → X = A^(-1)*B (left multiply by A^(-1))
const eq1 = A.mul(X).sub(B);
const result1 = solver.solve(eq1, X);
// Returns: X = A.inv().mul(B)

// X*A = B → X = B*A^(-1) (right multiply by A^(-1))
const eq2 = X.mul(A).sub(B);
const result2 = solver.solve(eq2, X);
// Returns: X = B.mul(A.inv())

```



---
chunk_id: advanced_noncommutative_api_reference::6
topic: advanced.noncommutative_api_reference
title: to_latex() - Type-Specific Formatting
priority: medium
keywords:
  - noncommutative_api_reference
  - to_latex() - type-specific formatting
languages: [rust, python, javascript]
chunk: 7/8
---

## to_latex() - Type-Specific Formatting

LaTeX formatting respects symbol types

### Rust

```rust
use mathhook::prelude::*;

// Scalar: standard notation
let x = symbol!(x);
let x_latex = Expression::symbol(x).to_latex(None).unwrap();
// Output: "x"

// Matrix: bold notation
let A = symbol!(A; matrix);
let a_latex = Expression::symbol(A).to_latex(None).unwrap();
// Output: "\mathbf{A}"

// Operator: hat notation
let p = symbol!(p; operator);
let p_latex = Expression::symbol(p).to_latex(None).unwrap();
// Output: "\hat{p}"

// Quaternion: standard notation
let i = symbol!(i; quaternion);
let i_latex = Expression::symbol(i).to_latex(None).unwrap();
// Output: "i"

```

### Python

```python
from mathhook import symbol

# Scalar: standard notation
x = symbol('x')
x_latex = x.to_latex()
# Output: "x"

# Matrix: bold notation
A = symbol('A', type='matrix')
a_latex = A.to_latex()
# Output: "\mathbf{A}"

# Operator: hat notation
p = symbol('p', type='operator')
p_latex = p.to_latex()
# Output: "\hat{p}"

# Quaternion: standard notation
i = symbol('i', type='quaternion')
i_latex = i.to_latex()
# Output: "i"

```

### JavaScript

```javascript
const { symbol } = require('mathhook');

// Scalar: standard notation
const x = symbol('x');
const xLatex = x.toLatex();
// Output: "x"

// Matrix: bold notation
const A = symbol('A', {type: 'matrix'});
const aLatex = A.toLatex();
// Output: "\mathbf{A}"

// Operator: hat notation
const p = symbol('p', {type: 'operator'});
const pLatex = p.toLatex();
// Output: "\hat{p}"

// Quaternion: standard notation
const i = symbol('i', {type: 'quaternion'});
const iLatex = i.toLatex();
// Output: "i"

```



---
chunk_id: advanced_noncommutative_api_reference::7
topic: advanced.noncommutative_api_reference
title: Error Handling
priority: medium
keywords:
  - noncommutative_api_reference
  - error handling
languages: [rust, python, javascript]
chunk: 8/8
---

## Error Handling

Handle errors from formatting and solving operations

### Rust

```rust
use mathhook::prelude::*;

let A = symbol!(A; matrix);
let expr = Expression::symbol(A);

// Handle formatting errors
match expr.to_latex(None) {
    Ok(latex) => println!("LaTeX: {}", latex),
    Err(e) => eprintln!("Formatting error: {}", e),
}

// Handle solver results
let solver = MatrixEquationSolver::new();
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);
let equation = expr!((A * X) - B);

match solver.solve(&equation, &X) {
    SolverResult::Single(solution) => {
        println!("Solution: {}", solution);
    }
    SolverResult::Multiple(solutions) => {
        println!("Multiple solutions: {:?}", solutions);
    }
    SolverResult::None => {
        println!("No solution exists");
    }
}

```

### Python

```python
from mathhook import symbol, MatrixEquationSolver, SolverResult

A = symbol('A', type='matrix')

# Handle formatting errors
try:
    latex = A.to_latex()
    print(f"LaTeX: {latex}")
except Exception as e:
    print(f"Formatting error: {e}")

# Handle solver results
solver = MatrixEquationSolver()
X = symbol('X', type='matrix')
B = symbol('B', type='matrix')
equation = A * X - B

result = solver.solve(equation, X)
if isinstance(result, SolverResult.Single):
    print(f"Solution: {result.solution}")
elif isinstance(result, SolverResult.Multiple):
    print(f"Multiple solutions: {result.solutions}")
elif isinstance(result, SolverResult.None_):
    print("No solution exists")

```

### JavaScript

```javascript
const { symbol, MatrixEquationSolver, SolverResult } = require('mathhook');

const A = symbol('A', {type: 'matrix'});

// Handle formatting errors
try {
    const latex = A.toLatex();
    console.log(`LaTeX: ${latex}`);
} catch (e) {
    console.error(`Formatting error: ${e.message}`);
}

// Handle solver results
const solver = new MatrixEquationSolver();
const X = symbol('X', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});
const equation = A.mul(X).sub(B);

const result = solver.solve(equation, X);
if (result instanceof SolverResult.Single) {
    console.log(`Solution: ${result.solution}`);
} else if (result instanceof SolverResult.Multiple) {
    console.log(`Multiple solutions: ${result.solutions}`);
} else if (result instanceof SolverResult.None) {
    console.log("No solution exists");
}

```



