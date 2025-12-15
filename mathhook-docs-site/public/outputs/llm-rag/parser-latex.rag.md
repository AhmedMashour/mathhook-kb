# LaTeX Notation

Parse and generate beautiful LaTeX notation for mathematical expressions.
MathHook provides full bidirectional support: Parse LaTeX → Expression, Expression → LaTeX.
Includes automatic type inference, implicit multiplication, and comprehensive coverage of 150+ LaTeX commands.


---
chunk_id: parser_latex::0
topic: parser.latex
title: Basic LaTeX Parsing
priority: medium
keywords:
  - latex
  - basic latex parsing
languages: [rust, python, javascript]
chunk: 1/4
---

## Basic LaTeX Parsing

Parse common LaTeX expressions

### Rust

```rust
use mathhook::parser::{Parser, ParserConfig};

let parser = Parser::new(ParserConfig::default());

// Fractions
let expr = parser.parse(r"\frac{x^2 + 1}{x - 1}")?;

// Functions
let expr = parser.parse(r"\sin(x) + \cos(y)")?;

// Square roots
let expr = parser.parse(r"\sqrt{x^2 + y^2}")?;

```

### Python

```python
from mathhook import Parser, ParserConfig

parser = Parser(ParserConfig.default())

# Fractions
expr = parser.parse(r"\frac{x^2 + 1}{x - 1}")

# Functions
expr = parser.parse(r"\sin(x) + \cos(y)")

# Square roots
expr = parser.parse(r"\sqrt{x^2 + y^2}")

```

### JavaScript

```javascript
const { Parser, ParserConfig } = require('mathhook');

const parser = new Parser(ParserConfig.default());

// Fractions
const expr = parser.parse(String.raw`\frac{x^2 + 1}{x - 1}`);

// Functions
const expr = parser.parse(String.raw`\sin(x) + \cos(y)`);

// Square roots
const expr = parser.parse(String.raw`\sqrt{x^2 + y^2}`);

```



---
chunk_id: parser_latex::1
topic: parser.latex
title: Expression to LaTeX
priority: medium
keywords:
  - latex
  - expression to latex
languages: [rust, python, javascript]
chunk: 2/4
---

## Expression to LaTeX

Format expressions as LaTeX

### Rust

```rust
use mathhook::prelude::*;
use mathhook::formatter::latex::LaTeXFormatter;

let x = symbol!(x);
let expr = expr!(x^2 / 2);
let latex = expr.to_latex(None)?;
// Returns: \frac{x^{2}}{2}

```

### Python

```python
from mathhook import symbol, expr, LaTeXFormatter

x = symbol('x')
expr = expr('x^2 / 2')
latex = expr.to_latex(None)
# Returns: \frac{x^{2}}{2}

```

### JavaScript

```javascript
const { symbol, expr, LaTeXFormatter } = require('mathhook');

const x = symbol('x');
const expr = expr('x^2 / 2');
const latex = expr.toLatex(null);
// Returns: \frac{x^{2}}{2}

```



---
chunk_id: parser_latex::2
topic: parser.latex
title: Noncommutative Type Inference
priority: medium
keywords:
  - latex
  - noncommutative type inference
languages: [rust, python, javascript]
chunk: 3/4
---

## Noncommutative Type Inference

Automatic symbol type inference from LaTeX notation

### Rust

```rust
use mathhook::parser::latex::parse_latex;

// Matrix symbols (noncommutative): \mathbf{A}
let expr = parse_latex(r"\mathbf{A}\mathbf{X} = \mathbf{B}")?;
// Creates matrix symbols A, X, B where A*X ≠ X*A

// Operator symbols (noncommutative): \hat{p}
let expr = parse_latex(r"[\hat{x}, \hat{p}] = i\hbar")?;
// Creates operator symbols (quantum mechanics commutator)

```

### Python

```python
from mathhook.parser import parse_latex

# Matrix symbols (noncommutative): \mathbf{A}
expr = parse_latex(r"\mathbf{A}\mathbf{X} = \mathbf{B}")
# Creates matrix symbols A, X, B where A*X ≠ X*A

# Operator symbols (noncommutative): \hat{p}
expr = parse_latex(r"[\hat{x}, \hat{p}] = i\hbar")
# Creates operator symbols (quantum mechanics commutator)

```

### JavaScript

```javascript
const { parseLatex } = require('mathhook');

// Matrix symbols (noncommutative): \mathbf{A}
const expr = parseLatex(String.raw`\mathbf{A}\mathbf{X} = \mathbf{B}`);
// Creates matrix symbols A, X, B where A*X ≠ X*A

// Operator symbols (noncommutative): \hat{p}
const expr = parseLatex(String.raw`[\hat{x}, \hat{p}] = i\hbar`);
// Creates operator symbols (quantum mechanics commutator)

```



---
chunk_id: parser_latex::3
topic: parser.latex
title: Calculus Notation
priority: medium
keywords:
  - latex
  - calculus notation
languages: [rust, python, javascript]
chunk: 4/4
---

## Calculus Notation

Parse calculus operations in LaTeX

### Rust

```rust
use mathhook::parser::latex::parse_latex;

// Indefinite integral
let expr = parse_latex(r"\int x^2 \, dx")?;

// Definite integral
let expr = parse_latex(r"\int_0^{\infty} e^{-x} \, dx")?;

// Summations
let expr = parse_latex(r"\sum_{i=1}^{n} i^2")?;

// Limits
let expr = parse_latex(r"\lim_{x \to 0} \frac{\sin(x)}{x}")?;

```

### Python

```python
from mathhook.parser import parse_latex

# Indefinite integral
expr = parse_latex(r"\int x^2 \, dx")

# Definite integral
expr = parse_latex(r"\int_0^{\infty} e^{-x} \, dx")

# Summations
expr = parse_latex(r"\sum_{i=1}^{n} i^2")

# Limits
expr = parse_latex(r"\lim_{x \to 0} \frac{\sin(x)}{x}")

```

### JavaScript

```javascript
const { parseLatex } = require('mathhook');

// Indefinite integral
const expr = parseLatex(String.raw`\int x^2 \, dx`);

// Definite integral
const expr = parseLatex(String.raw`\int_0^{\infty} e^{-x} \, dx`);

// Summations
const expr = parseLatex(String.raw`\sum_{i=1}^{n} i^2`);

// Limits
const expr = parseLatex(String.raw`\lim_{x \to 0} \frac{\sin(x)}{x}`);

```



