# Noncommutative Algebra API Reference

Complete API reference for MathHook's noncommutative algebra support,
including symbol creation macros, type queries, and equation solving.


---
chunk_id: advanced_noncommutative_api_reference::0
topic: advanced.noncommutative_api_reference
title: Symbol Creation Macros
priority: medium
keywords:
  - noncommutative_api_reference
  - symbol creation macros
languages: [rust, python, javascript]
chunk: 1/3
---

## Symbol Creation Macros

Create symbols with different types

### Rust

```rust
// Scalar (default)
let x = symbol!(x);

// Matrix (noncommutative)
let A = symbol!(A; matrix);

// Operator (noncommutative)
let p = symbol!(p; operator);

// Quaternion (noncommutative)
let i = symbol!(i; quaternion);

```

### Python

```python
# Scalar
x = symbols('x')

# Matrix
A = MatrixSymbol('A', n, n)

# Operator (quantum mechanics)
p = Operator('p')

# Quaternion
from sympy.algebras.quaternion import Quaternion
i = Quaternion(0, 1, 0, 0)

```

### JavaScript

```javascript
// Scalar
const x = symbol('x');

// Matrix
const A = symbol('A', {type: 'matrix'});

// Operator
const p = symbol('p', {type: 'operator'});

// Quaternion
const i = symbol('i', {type: 'quaternion'});

```



---
chunk_id: advanced_noncommutative_api_reference::1
topic: advanced.noncommutative_api_reference
title: Bulk Symbol Creation
priority: medium
keywords:
  - noncommutative_api_reference
  - bulk symbol creation
languages: [rust, python, javascript]
chunk: 2/3
---

## Bulk Symbol Creation

Create multiple symbols at once

### Rust

```rust
// Multiple scalars
let scalars = symbols![x, y, z];

// Multiple matrices
let matrices = symbols![A, B, C => matrix];

// Multiple operators
let operators = symbols![p, x, H => operator];

```

### Python

```python
# Multiple scalars
x, y, z = symbols('x y z')

# Multiple matrices
A, B, C = symbols('A B C', cls=MatrixSymbol)

```

### JavaScript

```javascript
// Multiple symbols
const [x, y, z] = symbols(['x', 'y', 'z']);

// Multiple matrices
const [A, B, C] = symbols(['A', 'B', 'C'], {type: 'matrix'});

```



---
chunk_id: advanced_noncommutative_api_reference::2
topic: advanced.noncommutative_api_reference
title: Type Queries
priority: medium
keywords:
  - noncommutative_api_reference
  - type queries
languages: [rust, python, javascript]
chunk: 3/3
---

## Type Queries

Check symbol type and commutativity

### Rust

```rust
let x = symbol!(x);
let A = symbol!(A; matrix);

// Type check
assert_eq!(x.symbol_type(), SymbolType::Scalar);
assert_eq!(A.symbol_type(), SymbolType::Matrix);

// Commutativity check
assert_eq!(x.commutativity(), Commutativity::Commutative);
assert_eq!(A.commutativity(), Commutativity::Noncommutative);

```

### Python

```python
x = symbols('x')
A = MatrixSymbol('A', n, n)

# Type check
print(type(x))  # Symbol
print(type(A))  # MatrixSymbol

# Commutativity (implicit in type)
print(A.is_commutative)  # False

```

### JavaScript

```javascript
const x = symbol('x');
const A = symbol('A', {type: 'matrix'});

// Type check
console.log(x.type);  // 'scalar'
console.log(A.type);  // 'matrix'

// Commutativity
console.log(x.is_commutative);  // true
console.log(A.is_commutative);  // false

```



