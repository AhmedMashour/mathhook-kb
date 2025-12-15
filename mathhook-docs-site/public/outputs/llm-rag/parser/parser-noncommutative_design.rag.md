# Parser Design for Noncommutative Algebra

Design documentation for MathHook's type-aware LaTeX parser that automatically infers symbol types.
Enables seamless support for noncommutative algebra without explicit type annotations in mathematical expressions.


---
chunk_id: parser_noncommutative_design::0
topic: parser.noncommutative_design
title: Matrix Type Inference
priority: medium
keywords:
  - noncommutative_design
  - matrix type inference
languages: [rust, python, javascript]
chunk: 1/3
---

## Matrix Type Inference

LaTeX bold notation creates matrix symbols

### Rust

```rust
use mathhook::parser::latex::parse_latex;

// Bold notation → Matrix symbols
let expr = parse_latex(r"\mathbf{A}\mathbf{B} \neq \mathbf{B}\mathbf{A}")?;

// A and B are noncommutative matrices
// A*B ≠ B*A in general

```

### Python

```python
from mathhook.parser import parse_latex

# Bold notation → Matrix symbols
expr = parse_latex(r"\mathbf{A}\mathbf{B} \neq \mathbf{B}\mathbf{A}")

# A and B are noncommutative matrices
# A*B ≠ B*A in general

```

### JavaScript

```javascript
const { parseLatex } = require('mathhook');

// Bold notation → Matrix symbols
const expr = parseLatex(String.raw`\mathbf{A}\mathbf{B} \neq \mathbf{B}\mathbf{A}`);

// A and B are noncommutative matrices
// A*B ≠ B*A in general

```



---
chunk_id: parser_noncommutative_design::1
topic: parser.noncommutative_design
title: Operator Type Inference
priority: medium
keywords:
  - noncommutative_design
  - operator type inference
languages: [rust, python, javascript]
chunk: 2/3
---

## Operator Type Inference

LaTeX hat notation creates operator symbols

### Rust

```rust
use mathhook::parser::latex::parse_latex;

// Hat notation → Operator symbols
let expr = parse_latex(r"[\hat{x}, \hat{p}] = i\hbar")?;

// Canonical commutation relation
// x and p are noncommutative operators

```

### Python

```python
from mathhook.parser import parse_latex

# Hat notation → Operator symbols
expr = parse_latex(r"[\hat{x}, \hat{p}] = i\hbar")

# Canonical commutation relation
# x and p are noncommutative operators

```

### JavaScript

```javascript
const { parseLatex } = require('mathhook');

// Hat notation → Operator symbols
const expr = parseLatex(String.raw`[\hat{x}, \hat{p}] = i\hbar`);

// Canonical commutation relation
// x and p are noncommutative operators

```



---
chunk_id: parser_noncommutative_design::2
topic: parser.noncommutative_design
title: Mixed Type Expression
priority: medium
keywords:
  - noncommutative_design
  - mixed type expression
languages: [rust, python, javascript]
chunk: 3/3
---

## Mixed Type Expression

Different symbol types in same expression

### Rust

```rust
use mathhook::parser::latex::parse_latex;

// Quantum mechanics: scalar + operators + matrix
let expr = parse_latex(r"\hbar \omega \hat{a}^\dagger \hat{a} + \mathbf{H}_0")?;

// ℏ and ω: scalars (commutative)
// â†, â: operators (noncommutative)
// H₀: matrix (noncommutative)

```

### Python

```python
from mathhook.parser import parse_latex

# Quantum mechanics: scalar + operators + matrix
expr = parse_latex(r"\hbar \omega \hat{a}^\dagger \hat{a} + \mathbf{H}_0")

# ℏ and ω: scalars (commutative)
# â†, â: operators (noncommutative)
# H₀: matrix (noncommutative)

```

### JavaScript

```javascript
const { parseLatex } = require('mathhook');

// Quantum mechanics: scalar + operators + matrix
const expr = parseLatex(String.raw`\hbar \omega \hat{a}^\dagger \hat{a} + \mathbf{H}_0`);

// ℏ and ω: scalars (commutative)
// â†, â: operators (noncommutative)
// H₀: matrix (noncommutative)

```



