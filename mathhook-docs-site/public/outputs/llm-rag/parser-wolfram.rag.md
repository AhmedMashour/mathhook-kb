# Wolfram Language Notation

Parse and generate Mathematica/Wolfram Language syntax for compatibility with Wolfram products.
Supports bidirectional conversion between MathHook expressions and Wolfram notation.


---
chunk_id: parser_wolfram::0
topic: parser.wolfram
title: Basic Wolfram Parsing
priority: medium
keywords:
  - wolfram
  - basic wolfram parsing
languages: [rust, python, javascript]
chunk: 1/4
---

## Basic Wolfram Parsing

Parse common Wolfram Language expressions

### Rust

```rust
use mathhook::parser::wolfram::parse_wolfram;

// Functions (capital letters, brackets)
let expr = parse_wolfram("Sin[x]")?;
let expr = parse_wolfram("Exp[x]")?;
let expr = parse_wolfram("Log[x]")?;

// Powers use ^ (not brackets)
let expr = parse_wolfram("x^2")?;

```

### Python

```python
from mathhook.parser import parse_wolfram

# Functions (capital letters, brackets)
expr = parse_wolfram("Sin[x]")
expr = parse_wolfram("Exp[x]")
expr = parse_wolfram("Log[x]")

# Powers use ^ (not brackets)
expr = parse_wolfram("x^2")

```

### JavaScript

```javascript
const { parseWolfram } = require('mathhook');

// Functions (capital letters, brackets)
const expr = parseWolfram("Sin[x]");
const expr = parseWolfram("Exp[x]");
const expr = parseWolfram("Log[x]");

// Powers use ^ (not brackets)
const expr = parseWolfram("x^2");

```



---
chunk_id: parser_wolfram::1
topic: parser.wolfram
title: Calculus Operations
priority: medium
keywords:
  - wolfram
  - calculus operations
languages: [rust, python, javascript]
chunk: 2/4
---

## Calculus Operations

Wolfram calculus notation

### Rust

```rust
use mathhook::parser::wolfram::parse_wolfram;

// Derivative: D[expr, var]
let expr = parse_wolfram("D[x^2, x]")?;  // 2x

// Integral: Integrate[expr, var]
let expr = parse_wolfram("Integrate[x^2, x]")?;  // x^3/3

// Definite integral: Integrate[expr, {var, a, b}]
let expr = parse_wolfram("Integrate[x^2, {x, 0, 1}]")?;

// Limit: Limit[expr, var -> value]
let expr = parse_wolfram("Limit[Sin[x]/x, x -> 0]")?;

```

### Python

```python
from mathhook.parser import parse_wolfram

# Derivative: D[expr, var]
expr = parse_wolfram("D[x^2, x]")  # 2x

# Integral: Integrate[expr, var]
expr = parse_wolfram("Integrate[x^2, x]")  # x^3/3

# Definite integral: Integrate[expr, {var, a, b}]
expr = parse_wolfram("Integrate[x^2, {x, 0, 1}]")

# Limit: Limit[expr, var -> value]
expr = parse_wolfram("Limit[Sin[x]/x, x -> 0]")

```

### JavaScript

```javascript
const { parseWolfram } = require('mathhook');

// Derivative: D[expr, var]
const expr = parseWolfram("D[x^2, x]");  // 2x

// Integral: Integrate[expr, var]
const expr = parseWolfram("Integrate[x^2, x]");  // x^3/3

// Definite integral: Integrate[expr, {var, a, b}]
const expr = parseWolfram("Integrate[x^2, {x, 0, 1}]");

// Limit: Limit[expr, var -> value]
const expr = parseWolfram("Limit[Sin[x]/x, x -> 0]");

```



---
chunk_id: parser_wolfram::2
topic: parser.wolfram
title: Generating Wolfram Output
priority: medium
keywords:
  - wolfram
  - generating wolfram output
languages: [rust, python, javascript]
chunk: 3/4
---

## Generating Wolfram Output

Format MathHook expressions as Wolfram Language

### Rust

```rust
use mathhook::prelude::*;
use mathhook::formatter::wolfram::WolframFormatter;

let formatter = WolframFormatter::new();
let x = symbol!(x);

// Functions
let expr = expr!(sin(x));
println!("{}", formatter.format(&expr));  // Sin[x]

// Complex expressions
let expr = expr!((x + 1) / (x - 1));
println!("{}", formatter.format(&expr));  // (x + 1)/(x - 1)

```

### Python

```python
from mathhook import symbol, expr, WolframFormatter

formatter = WolframFormatter()
x = symbol('x')

# Functions
expr_obj = expr('sin(x)')
print(formatter.format(expr_obj))  # Sin[x]

# Complex expressions
expr_obj = expr('(x + 1) / (x - 1)')
print(formatter.format(expr_obj))  # (x + 1)/(x - 1)

```

### JavaScript

```javascript
const { symbol, expr, WolframFormatter } = require('mathhook');

const formatter = new WolframFormatter();
const x = symbol('x');

// Functions
const exprObj = expr('sin(x)');
console.log(formatter.format(exprObj));  // Sin[x]

// Complex expressions
const exprObj = expr('(x + 1) / (x - 1)');
console.log(formatter.format(exprObj));  // (x + 1)/(x - 1)

```



---
chunk_id: parser_wolfram::3
topic: parser.wolfram
title: Cross-Platform Validation
priority: medium
keywords:
  - wolfram
  - cross-platform validation
languages: [rust, python, javascript]
chunk: 4/4
---

## Cross-Platform Validation

Export to Wolfram for verification

### Rust

```rust
use mathhook::prelude::*;
use mathhook::formatter::wolfram::WolframFormatter;

// Compute derivative in MathHook
let x = symbol!(x);
let f = expr!(x^3 + 2*x^2 + x);
let df = f.derivative(&x, 1);

// Export to Wolfram for verification
let formatter = WolframFormatter::new();
let wolfram_code = formatter.format(&df);

println!("Verify in Wolfram Alpha:");
println!("Simplify[{}]", wolfram_code);

```

### Python

```python
from mathhook import symbol, expr, WolframFormatter

# Compute derivative in MathHook
x = symbol('x')
f = expr('x^3 + 2*x^2 + x')
df = f.derivative(x, 1)

# Export to Wolfram for verification
formatter = WolframFormatter()
wolfram_code = formatter.format(df)

print("Verify in Wolfram Alpha:")
print(f"Simplify[{wolfram_code}]")

```

### JavaScript

```javascript
const { symbol, expr, WolframFormatter } = require('mathhook');

// Compute derivative in MathHook
const x = symbol('x');
const f = expr('x^3 + 2*x^2 + x');
const df = f.derivative(x, 1);

// Export to Wolfram for verification
const formatter = new WolframFormatter();
const wolframCode = formatter.format(df);

console.log("Verify in Wolfram Alpha:");
console.log(`Simplify[${wolframCode}]`);

```



