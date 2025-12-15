---









---

# LaTeX Parsing and Mathematical Notation

> **Topic**: `api.parser.latex`

Parse and generate LaTeX notation for mathematical expressions. Full bidirectional
support: LaTeX → Expression and Expression → LaTeX. Automatic type inference for
matrix symbols (\mathbf{A}), operator symbols (\hat{p}), and implicit multiplication.





# LaTeX Parsing and Notation

## Overview

MathHook provides comprehensive LaTeX support:
- **Bidirectional**: Parse LaTeX → Expression, Expression → LaTeX
- **Type Inference**: `\mathbf{A}` creates matrix symbols, `\hat{p}` creates operators
- **Implicit Multiplication**: Handles `2x`, `\pi x`, `(a)(b)` correctly
- **150+ Commands**: Functions, symbols, operators, calculus notation

## Architecture

### Two-Stage Processing

**1. Lexer (Token Generation)**:
- Inserts implicit multiplication tokens (`2x` → `2*x`)
- Classifies tokens (number, identifier, function, operator)
- O(1) HashMap lookups for LaTeX commands (`\sin`, `\pi`, `\alpha`)

**2. Parser (LALRPOP Grammar)**:
- LR(1) parser with operator precedence
- Right-associative exponentiation: `2^3^4` → `2^(3^4)`
- Context-aware function resolution

### Performance
- >100K simple expressions/second
- Thread-local caching for common expressions
- Zero-copy string processing where possible

## Supported LaTeX

### Greek Letters
- Lowercase: `\alpha`, `\beta`, `\gamma`, ..., `\omega`
- Uppercase: `\Gamma`, `\Delta`, `\Theta`, ..., `\Omega`

### Mathematical Constants
- `\pi`: π (pi)
- `e`: Euler's number
- `\phi`: Golden ratio
- `i`: Imaginary unit

### Fractions and Roots
- `\frac{a}{b}`: Fractions
- `\sqrt{x}`: Square root
- `\sqrt[n]{x}`: nth root

### Trigonometric Functions
- `\sin`, `\cos`, `\tan`, `\cot`, `\sec`, `\csc`
- `\arcsin`, `\arccos`, `\arctan`
- `\sinh`, `\cosh`, `\tanh`

### Calculus Notation
- `\int`: Integral
- `\frac{d}{dx}`: Derivative
- `\lim`: Limit
- `\sum`: Summation
- `\prod`: Product

### Matrix Notation
- `\mathbf{A}`: Matrix symbol (bold, noncommutative)
- `\hat{p}`: Operator symbol (quantum mechanics)
- `\begin{matrix}...\end{matrix}`: Matrix construction












## Examples


### Basic LaTeX Parsing

Parse standard mathematical expressions

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::parser::{Parser, ParserConfig};

let parser = Parser::new(ParserConfig::default());

// Basic arithmetic
let expr = parser.parse(r"2 + 3 \cdot 4")?;  // 2 + 3*4

// Fractions
let expr = parser.parse(r"\frac{x^2 + 1}{x - 1}")?;

// Functions
let expr = parser.parse(r"\sin(x) + \cos(y)")?;

// Square roots
let expr = parser.parse(r"\sqrt{x^2 + y^2}")?;

// Exponents
let expr = parser.parse(r"e^{-x^2}")?;  // Gaussian

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook.parser import parse_latex

# Basic arithmetic
expr = parse_latex(r"2 + 3 \cdot 4")  # 2 + 3*4

# Fractions
expr = parse_latex(r"\frac{x^2 + 1}{x - 1}")

# Functions
expr = parse_latex(r"\sin(x) + \cos(y)")

# Square roots
expr = parse_latex(r"\sqrt{x^2 + y^2}")

# Exponents
expr = parse_latex(r"e^{-x^2}")

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { parseLatex } from 'mathhook';

// Basic arithmetic
const expr = parseLatex(String.raw`2 + 3 \cdot 4`);

// Fractions
const expr2 = parseLatex(String.raw`\frac{x^2 + 1}{x - 1}`);

// Functions
const expr3 = parseLatex(String.raw`\sin(x) + \cos(y)`);

// Square roots
const expr4 = parseLatex(String.raw`\sqrt{x^2 + y^2}`);

```
</details>




### Greek Letters and Constants

Parse Greek symbols and mathematical constants

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::parser::Parser;

let parser = Parser::new(ParserConfig::default());

// Greek symbols (lowercase)
let expr = parser.parse(r"\alpha + \beta + \gamma")?;

// Greek symbols (uppercase functions)
let expr = parser.parse(r"\Gamma(n)")?;  // Gamma function

// Mathematical constants
let expr = parser.parse(r"\pi r^2")?;          // π*r²
let expr = parser.parse(r"e^{i\pi} + 1")?;     // Euler's identity
let expr = parser.parse(r"\phi = \frac{1+\sqrt{5}}{2}")?;  // Golden ratio

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook.parser import parse_latex

# Greek symbols
expr = parse_latex(r"\alpha + \beta + \gamma")

# Gamma function
expr = parse_latex(r"\Gamma(n)")

# Constants
expr = parse_latex(r"\pi r^2")
expr = parse_latex(r"e^{i\pi} + 1")
expr = parse_latex(r"\phi = \frac{1+\sqrt{5}}{2}")

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { parseLatex } from 'mathhook';

// Greek symbols
const expr = parseLatex(String.raw`\alpha + \beta + \gamma`);

// Gamma function
const expr2 = parseLatex(String.raw`\Gamma(n)`);

// Constants
const expr3 = parseLatex(String.raw`\pi r^2`);
const expr4 = parseLatex(String.raw`e^{i\pi} + 1`);

```
</details>




### Matrix and Operator Symbols

Automatic type inference from LaTeX notation

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::parser::Parser;

let parser = Parser::new(ParserConfig::default());

// Matrix symbols (bold, noncommutative)
let expr = parser.parse(r"\mathbf{A} \mathbf{B}")?;
// Creates: symbol!(A; matrix) * symbol!(B; matrix)

// Operator symbols (quantum mechanics)
let expr = parser.parse(r"\hat{p} \hat{x}")?;
// Creates: symbol!(p; operator) * symbol!(x; operator)

// Mixed scalar and matrix
let expr = parser.parse(r"x \mathbf{A}")?;
// Creates: symbol!(x) * symbol!(A; matrix)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook.parser import parse_latex

# Matrix symbols (automatic inference)
expr = parse_latex(r"\mathbf{A} \mathbf{B}")
# Creates matrix symbols A, B

# Operator symbols
expr = parse_latex(r"\hat{p} \hat{x}")
# Creates operator symbols p, x

# Mixed
expr = parse_latex(r"x \mathbf{A}")

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { parseLatex } from 'mathhook';

// Matrix symbols
const expr = parseLatex(String.raw`\mathbf{A} \mathbf{B}`);

// Operator symbols
const expr2 = parseLatex(String.raw`\hat{p} \hat{x}`);

// Mixed scalar and matrix
const expr3 = parseLatex(String.raw`x \mathbf{A}`);

```
</details>




### Generating LaTeX Output

Convert expressions back to LaTeX

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook::formatter::latex::LaTeXFormatter;

let x = symbol!(x);

// Simple expression
let expr = expr!(x^2 / 2);
let latex = expr.to_latex(None)?;
// Returns: "\frac{x^{2}}{2}"

// Matrix expression
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let expr = expr!(A * B);
let latex = expr.to_latex(None)?;
// Returns: "\mathbf{A}\mathbf{B}"

// Complex expression
let expr = expr!(sin(x) + cos(x^2));
let latex = expr.to_latex(None)?;
// Returns: "\sin\left(x\right) + \cos\left(x^{2}\right)"

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol
from mathhook.formatter import to_latex

x = symbol('x')

# Simple expression
expr = x**2 / 2
latex = to_latex(expr)
# Returns: "\frac{x^{2}}{2}"

# Matrix expression
A = symbol('A', matrix=True)
B = symbol('B', matrix=True)
expr = A * B
latex = to_latex(expr)
# Returns: "\mathbf{A}\mathbf{B}"

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { symbol, parse, toLatex } from 'mathhook';

const x = symbol('x');

// Simple expression
const expr = parse('x^2 / 2');
const latex = toLatex(expr);
// Returns: "\frac{x^{2}}{2}"

// Matrix expression
const A = symbol('A', { type: 'matrix' });
const B = symbol('B', { type: 'matrix' });
const expr2 = parse('A * B');
const latex2 = toLatex(expr2);
// Returns: "\mathbf{A}\mathbf{B}"

```
</details>




### Implicit Multiplication

Automatic insertion of multiplication operators

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::parser::Parser;

let parser = Parser::new(ParserConfig::default());

// Number-variable: 2x → 2*x
let expr = parser.parse("2x")?;

// Parentheses: (a)(b) → a*b
let expr = parser.parse("(a)(b)")?;

// Functions: sin(x)cos(y) → sin(x)*cos(y)
let expr = parser.parse(r"\sin(x)\cos(y)")?;

// Mixed: 2πr → 2*π*r
let expr = parser.parse(r"2\pi r")?;

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook.parser import parse_latex

# Implicit multiplication handled automatically
expr = parse_latex("2x")           # 2*x
expr = parse_latex("(a)(b)")       # a*b
expr = parse_latex(r"\sin(x)\cos(y)")  # sin(x)*cos(y)
expr = parse_latex(r"2\pi r")      # 2*π*r

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { parseLatex } from 'mathhook';

// Implicit multiplication
const expr = parseLatex("2x");           // 2*x
const expr2 = parseLatex("(a)(b)");      // a*b
const expr3 = parseLatex(String.raw`\sin(x)\cos(y)`);  // sin(x)*cos(y)
const expr4 = parseLatex(String.raw`2\pi r`);  // 2*π*r

```
</details>




### Calculus Notation

Parse derivatives, integrals, limits

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::parser::Parser;

let parser = Parser::new(ParserConfig::default());

// Derivative notation
let expr = parser.parse(r"\frac{d}{dx} x^2")?;

// Integral notation
let expr = parser.parse(r"\int x^2 \, dx")?;

// Definite integral
let expr = parser.parse(r"\int_0^1 x^2 \, dx")?;

// Limit notation
let expr = parser.parse(r"\lim_{x \to 0} \frac{\sin(x)}{x}")?;

// Summation
let expr = parser.parse(r"\sum_{i=1}^{n} i^2")?;

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook.parser import parse_latex

# Derivative
expr = parse_latex(r"\frac{d}{dx} x^2")

# Integral
expr = parse_latex(r"\int x^2 \, dx")

# Definite integral
expr = parse_latex(r"\int_0^1 x^2 \, dx")

# Limit
expr = parse_latex(r"\lim_{x \to 0} \frac{\sin(x)}{x}")

# Summation
expr = parse_latex(r"\sum_{i=1}^{n} i^2")

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { parseLatex } from 'mathhook';

// Derivative
const expr = parseLatex(String.raw`\frac{d}{dx} x^2`);

// Integral
const expr2 = parseLatex(String.raw`\int x^2 \, dx`);

// Definite integral
const expr3 = parseLatex(String.raw`\int_0^1 x^2 \, dx`);

// Limit
const expr4 = parseLatex(String.raw`\lim_{x \to 0} \frac{\sin(x)}{x}`);

```
</details>






## Performance

**Time Complexity**: O(n) for n-character LaTeX string


## API Reference

- **Rust**: `mathhook_core::parser::{Parser, ParserConfig}`
- **Python**: `mathhook.parser.parse_latex`
- **JavaScript**: `mathhook.parser.parseLatex`


## See Also


- [api.parser.wolfram](../api/parser/wolfram.md)

- [api.parser.custom](../api/parser/custom.md)

- [api.core.expressions](../api/core/expressions.md)

- [api.advanced.noncommutative_algebra](../api/advanced/noncommutative_algebra.md)


