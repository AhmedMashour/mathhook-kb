# Expression Formatting

Format mathematical expressions for display in multiple notations.
Supports LaTeX, Unicode, Wolfram, and custom formatters for different output targets.


---
chunk_id: parser_formatting::0
topic: parser.formatting
title: Basic Formatting
priority: medium
keywords:
  - formatting
  - basic formatting
languages: [rust, python, javascript]
chunk: 1/4
---

## Basic Formatting

Format expressions in different notations

### Rust

```rust
use mathhook::prelude::*;
use mathhook::formatter::{LatexFormatter, UnicodeFormatter, WolframFormatter};

let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);

// LaTeX
let latex = LatexFormatter::new().format(&expr);
println!("{}", latex);   // x^{2} + 2 \cdot x + 1

// Unicode (pretty-print)
let unicode = UnicodeFormatter::new().format(&expr);
println!("{}", unicode); // x² + 2·x + 1

// Wolfram
let wolfram = WolframFormatter::new().format(&expr);
println!("{}", wolfram); // x^2 + 2*x + 1

```

### Python

```python
from mathhook import symbol, expr
from mathhook.formatter import LatexFormatter, UnicodeFormatter, WolframFormatter

x = symbol('x')
expr_obj = expr('x^2 + 2*x + 1')

# LaTeX
latex = LatexFormatter().format(expr_obj)
print(latex)   # x^{2} + 2 \cdot x + 1

# Unicode (pretty-print)
unicode = UnicodeFormatter().format(expr_obj)
print(unicode) # x² + 2·x + 1

# Wolfram
wolfram = WolframFormatter().format(expr_obj)
print(wolfram) # x^2 + 2*x + 1

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');
const { LatexFormatter, UnicodeFormatter, WolframFormatter } = require('mathhook');

const x = symbol('x');
const exprObj = expr('x^2 + 2*x + 1');

// LaTeX
const latex = new LatexFormatter().format(exprObj);
console.log(latex);   // x^{2} + 2 \cdot x + 1

// Unicode (pretty-print)
const unicode = new UnicodeFormatter().format(exprObj);
console.log(unicode); // x² + 2·x + 1

// Wolfram
const wolfram = new WolframFormatter().format(exprObj);
console.log(wolfram); // x^2 + 2*x + 1

```



---
chunk_id: parser_formatting::1
topic: parser.formatting
title: Type-Aware Formatting
priority: medium
keywords:
  - formatting
  - type-aware formatting
languages: [rust, python, javascript]
chunk: 2/4
---

## Type-Aware Formatting

Noncommutative symbols formatted correctly

### Rust

```rust
use mathhook::prelude::*;
use mathhook::formatter::latex::LatexFormatter;

// Matrix symbols (bold)
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let matrix_expr = expr!(A * B);

let formatter = LatexFormatter::new();
println!("{}", formatter.format(&matrix_expr));
// Output: \mathbf{A}\mathbf{B}

// Operator symbols (hat)
let p = symbol!(p; operator);
let x = symbol!(x; operator);
let op_expr = expr!(p * x);

println!("{}", formatter.format(&op_expr));
// Output: \hat{p}\hat{x}

```

### Python

```python
from mathhook import symbol, expr
from mathhook.formatter import LatexFormatter

# Matrix symbols (bold)
A = symbol('A', type='matrix')
B = symbol('B', type='matrix')
matrix_expr = expr('A * B')

formatter = LatexFormatter()
print(formatter.format(matrix_expr))
# Output: \mathbf{A}\mathbf{B}

# Operator symbols (hat)
p = symbol('p', type='operator')
x = symbol('x', type='operator')
op_expr = expr('p * x')

print(formatter.format(op_expr))
# Output: \hat{p}\hat{x}

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');
const { LatexFormatter } = require('mathhook');

// Matrix symbols (bold)
const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });
const matrixExpr = expr('A * B');

const formatter = new LatexFormatter();
console.log(formatter.format(matrixExpr));
// Output: \mathbf{A}\mathbf{B}

// Operator symbols (hat)
const p = symbol('p', { type: 'operator' });
const x = symbol('x', { type: 'operator' });
const opExpr = expr('p * x');

console.log(formatter.format(opExpr));
// Output: \hat{p}\hat{x}

```



---
chunk_id: parser_formatting::2
topic: parser.formatting
title: Customized LaTeX Output
priority: medium
keywords:
  - formatting
  - customized latex output
languages: [rust, python, javascript]
chunk: 3/4
---

## Customized LaTeX Output

Configure formatter behavior

### Rust

```rust
use mathhook::prelude::*;
use mathhook::formatter::latex::LatexFormatter;

// Configure formatter
let formatter = LatexFormatter::new()
    .with_precision(6)           // Float precision
    .with_explicit_multiplication(true)  // Show all * as \cdot
    .with_compact_fractions(false);      // Use \frac always

let expr = expr!(2*x / 3);
println!("{}", formatter.format(&expr));
// Output: \frac{2 \cdot x}{3}

```

### Python

```python
from mathhook import symbol, expr
from mathhook.formatter import LatexFormatter

# Configure formatter
formatter = LatexFormatter(
    precision=6,
    explicit_multiplication=True,
    compact_fractions=False
)

expr_obj = expr('2*x / 3')
print(formatter.format(expr_obj))
# Output: \frac{2 \cdot x}{3}

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');
const { LatexFormatter } = require('mathhook');

// Configure formatter
const formatter = new LatexFormatter({
    precision: 6,
    explicitMultiplication: true,
    compactFractions: false
});

const exprObj = expr('2*x / 3');
console.log(formatter.format(exprObj));
// Output: \frac{2 \cdot x}{3}

```



---
chunk_id: parser_formatting::3
topic: parser.formatting
title: Educational Step Formatting
priority: medium
keywords:
  - formatting
  - educational step formatting
languages: [rust, python, javascript]
chunk: 4/4
---

## Educational Step Formatting

Format step-by-step explanations

### Rust

```rust
use mathhook::prelude::*;
use mathhook::formatter::latex::LatexFormatter;

let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);

// Generate step-by-step LaTeX
let formatter = LatexFormatter::new();

println!("Step 1: Start with {}", formatter.format(&expr));
let factored = expr.factor();  // (x+1)^2
println!("Step 2: Factor as {}", formatter.format(&factored));

```

### Python

```python
from mathhook import symbol, expr
from mathhook.formatter import LatexFormatter

x = symbol('x')
expr_obj = expr('x^2 + 2*x + 1')

# Generate step-by-step LaTeX
formatter = LatexFormatter()

print(f"Step 1: Start with {formatter.format(expr_obj)}")
factored = expr_obj.factor()  # (x+1)^2
print(f"Step 2: Factor as {formatter.format(factored)}")

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');
const { LatexFormatter } = require('mathhook');

const x = symbol('x');
const exprObj = expr('x^2 + 2*x + 1');

// Generate step-by-step LaTeX
const formatter = new LatexFormatter();

console.log(`Step 1: Start with ${formatter.format(exprObj)}`);
const factored = exprObj.factor();  // (x+1)^2
console.log(`Step 2: Factor as ${formatter.format(factored)}`);

```



