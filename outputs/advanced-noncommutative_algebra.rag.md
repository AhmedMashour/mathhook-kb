# Noncommutative Algebra

Support for noncommutative algebra in MathHook with four symbol types:
Scalar (commutative), Matrix, Operator, and Quaternion (all noncommutative).
Essential for quantum mechanics, linear algebra, and 3D rotations.


---
chunk_id: advanced_noncommutative_algebra::0
topic: advanced.noncommutative_algebra
title: Matrix Symbols
priority: medium
keywords:
  - noncommutative_algebra
  - matrix symbols
languages: [rust, python, javascript]
chunk: 1/4
---

## Matrix Symbols

Create noncommutative matrix symbols

### Rust

```rust
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Order matters!
let AB = expr!(A * B);  // A*B
let BA = expr!(B * A);  // B*A ≠ A*B

```

### Python

```python
A = MatrixSymbol('A', n, n)
B = MatrixSymbol('B', n, n)

AB = A * B  # Noncommutative
BA = B * A  # Different!

```

### JavaScript

```javascript
const A = symbol('A', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

const AB = A.mul(B);  // Noncommutative
const BA = B.mul(A);  // Different!

```



---
chunk_id: advanced_noncommutative_algebra::1
topic: advanced.noncommutative_algebra
title: Quantum Operators
priority: medium
keywords:
  - noncommutative_algebra
  - quantum operators
languages: [rust, python, javascript]
chunk: 2/4
---

## Quantum Operators

Position and momentum operators (canonical commutation relation)

### Rust

```rust
let x = symbol!(x; operator);
let p = symbol!(p; operator);

// Commutator: [x, p] = xp - px
let commutator = expr!((x * p) - (p * x));
// Physical result: [x, p] = iℏ

```

### Python

```python
x = Operator('x')
p = Operator('p')

# Commutator
commutator = Commutator(x, p)
# Result: I*hbar

```

### JavaScript

```javascript
const x = symbol('x', {type: 'operator'});
const p = symbol('p', {type: 'operator'});

// Commutator
const comm = x.mul(p).sub(p.mul(x));

```



---
chunk_id: advanced_noncommutative_algebra::2
topic: advanced.noncommutative_algebra
title: Quaternions
priority: medium
keywords:
  - noncommutative_algebra
  - quaternions
languages: [rust, python, javascript]
chunk: 3/4
---

## Quaternions

3D rotation with quaternion multiplication

### Rust

```rust
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);

let ij = expr!(i * j);  // i*j = k
let ji = expr!(j * i);  // j*i = -k (different!)

```

### Python

```python
from sympy.algebras.quaternion import Quaternion

i = Quaternion(0, 1, 0, 0)
j = Quaternion(0, 0, 1, 0)

ij = i * j  # k
ji = j * i  # -k

```

### JavaScript

```javascript
const i = symbol('i', {type: 'quaternion'});
const j = symbol('j', {type: 'quaternion'});

const ij = i.mul(j);  // k
const ji = j.mul(i);  // -k

```



---
chunk_id: advanced_noncommutative_algebra::3
topic: advanced.noncommutative_algebra
title: LaTeX Type Inference
priority: medium
keywords:
  - noncommutative_algebra
  - latex type inference
languages: [rust, python, javascript]
chunk: 4/4
---

## LaTeX Type Inference

Parser infers types from LaTeX notation

### Rust

```rust
let parser = Parser::new(ParserConfig::default());

// Bold notation → Matrix type
let eq1 = parser.parse(r"\mathbf{A}\mathbf{X} = \mathbf{B}").unwrap();

// Hat notation → Operator type
let eq2 = parser.parse(r"\hat{H}\hat{\psi} = E\hat{\psi}").unwrap();

```

### Python

```python
from sympy.parsing.latex import parse_latex

# Bold → Matrix
eq1 = parse_latex(r'\mathbf{A}\mathbf{X} = \mathbf{B}')

# Hat → Operator
eq2 = parse_latex(r'\hat{H}\hat{\psi} = E\hat{\psi}')

```

### JavaScript

```javascript
const parser = new Parser();

// Bold → Matrix
const eq1 = parser.parse('\\mathbf{A}\\mathbf{X} = \\mathbf{B}');

// Hat → Operator
const eq2 = parser.parse('\\hat{H}\\hat{\\psi} = E\\hat{\\psi}');

```



