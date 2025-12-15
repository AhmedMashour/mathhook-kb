---









---

# LaTeX Notation

> **Topic**: `parser.latex`

Parse and generate beautiful LaTeX notation for mathematical expressions.
MathHook provides full bidirectional support: Parse LaTeX → Expression, Expression → LaTeX.
Includes automatic type inference, implicit multiplication, and comprehensive coverage of 150+ LaTeX commands.





# LaTeX Notation

Parse and generate beautiful LaTeX notation for mathematical expressions.

## Understanding LaTeX Parsing

### What is LaTeX Notation?

LaTeX is the standard mathematical typesetting language used in academic papers, textbooks, and presentations. MathHook provides:

- **Full bidirectional support**: Parse LaTeX → Expression, Expression → LaTeX
- **Automatic type inference**: `\mathbf{A}` creates matrix symbols, `\hat{p}` creates operator symbols
- **Implicit multiplication**: Handles `2x`, `\pi x`, `(a)(b)` correctly
- **Comprehensive coverage**: 150+ LaTeX commands for functions, symbols, operators, calculus

### How It Works (Architecture)

**Two-Stage Processing:**

1. **Lexer** (Token Generation):
   - Inserts implicit multiplication tokens (`2x` → `2*x`)
   - Classifies tokens (number, identifier, function, operator)
   - O(1) HashMap lookups for LaTeX commands (`\sin`, `\pi`, `\alpha`)

2. **Parser** (LALRPOP Grammar):
   - LR(1) parser with operator precedence
   - Right-associative exponentiation: `2^3^4` → `2^(3^4)`
   - Context-aware function resolution (indexed functions, calculus operators)

**Performance:**
- >100K simple expressions/second
- Thread-local caching for common expressions
- Zero-copy string processing where possible

## Complete LaTeX Command Reference

**Trigonometric Functions:**
```
\sin, \cos, \tan, \sec, \csc, \cot
\sinh, \cosh, \tanh, \sech, \csch, \coth
\arcsin, \arccos, \arctan, \arcsec, \arccsc, \arccot
```

**Logarithmic Functions:**
```
\ln, \log, \log_10, \log_2
```

**Calculus Operators:**
```
\int, \iint, \iiint, \oint          # Integrals
\sum, \prod                          # Summation, product
\lim                                 # Limits
\partial                             # Partial derivatives
\nabla                               # Nabla operator
```

**Greek Letters (Lowercase):**
```
\alpha, \beta, \gamma, \delta, \epsilon, \zeta, \eta, \theta
\iota, \kappa, \lambda, \mu, \nu, \xi, \omicron, \pi, \rho
\sigma, \tau, \upsilon, \phi, \chi, \psi, \omega
```

**Greek Letters (Uppercase, often functions):**
```
\Gamma        # Gamma function
\Delta        # Dirac delta
\Psi          # Digamma function
\Zeta         # Riemann zeta function
```

**Mathematical Constants:**
```
\pi           # π ≈ 3.14159...
\phi, \varphi # Golden ratio φ ≈ 1.618...
\infty        # ∞
\EulerGamma   # Euler-Mascheroni constant γ ≈ 0.5772...
```

**Operators:**
```
\cdot         # Multiplication (·)
\times        # Cross product (×)
\div          # Division (÷)
\pm, \mp      # Plus-minus (±), minus-plus (∓)
\leq, \geq    # ≤, ≥
\neq          # ≠
\equiv        # ≡ (equivalence)
\approx       # ≈ (approximately)
\sim          # ~ (similar to)
\propto       # ∝ (proportional to)
```

**Formatting:**
```
\mathbf{A}                 # Bold (inferred as matrix symbol)
\hat{p}                    # Hat (inferred as operator symbol)
\vec{v}                    # Vector arrow
\overline{z}               # Overline (often conjugate)
\tilde{x}                  # Tilde
\bar{x}                    # Bar
\text{if}                  # Text mode
\mathcal{F}                # Calligraphic (fancy fonts)
\mathbb{R}                 # Blackboard bold (ℝ, ℤ, ℚ)
```












## Examples


### Basic LaTeX Parsing

Parse common LaTeX expressions

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>




### Expression to LaTeX

Format expressions as LaTeX

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use mathhook::formatter::latex::LaTeXFormatter;

let x = symbol!(x);
let expr = expr!(x^2 / 2);
let latex = expr.to_latex(None)?;
// Returns: \frac{x^{2}}{2}

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr, LaTeXFormatter

x = symbol('x')
expr = expr('x^2 / 2')
latex = expr.to_latex(None)
# Returns: \frac{x^{2}}{2}

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr, LaTeXFormatter } = require('mathhook');

const x = symbol('x');
const expr = expr('x^2 / 2');
const latex = expr.toLatex(null);
// Returns: \frac{x^{2}}{2}

```
</details>




### Noncommutative Type Inference

Automatic symbol type inference from LaTeX notation

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::parser::latex::parse_latex;

// Matrix symbols (noncommutative): \mathbf{A}
let expr = parse_latex(r"\mathbf{A}\mathbf{X} = \mathbf{B}")?;
// Creates matrix symbols A, X, B where A*X ≠ X*A

// Operator symbols (noncommutative): \hat{p}
let expr = parse_latex(r"[\hat{x}, \hat{p}] = i\hbar")?;
// Creates operator symbols (quantum mechanics commutator)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook.parser import parse_latex

# Matrix symbols (noncommutative): \mathbf{A}
expr = parse_latex(r"\mathbf{A}\mathbf{X} = \mathbf{B}")
# Creates matrix symbols A, X, B where A*X ≠ X*A

# Operator symbols (noncommutative): \hat{p}
expr = parse_latex(r"[\hat{x}, \hat{p}] = i\hbar")
# Creates operator symbols (quantum mechanics commutator)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { parseLatex } = require('mathhook');

// Matrix symbols (noncommutative): \mathbf{A}
const expr = parseLatex(String.raw`\mathbf{A}\mathbf{X} = \mathbf{B}`);
// Creates matrix symbols A, X, B where A*X ≠ X*A

// Operator symbols (noncommutative): \hat{p}
const expr = parseLatex(String.raw`[\hat{x}, \hat{p}] = i\hbar`);
// Creates operator symbols (quantum mechanics commutator)

```
</details>




### Calculus Notation

Parse calculus operations in LaTeX

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>






## Performance

**Time Complexity**: O(n) where n = expression tree size


## API Reference

- **Rust**: `mathhook_core::parser::latex`
- **Python**: `mathhook.parser.latex`
- **JavaScript**: `mathhook.parser.latex`


## See Also


- [parser.formatting](../parser/formatting.md)

- [parser.wolfram](../parser/wolfram.md)

- [parser.custom](../parser/custom.md)

- [parser.noncommutative_design](../parser/noncommutative_design.md)

- [core.expressions](../core/expressions.md)


