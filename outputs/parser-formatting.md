---









---

# Expression Formatting

> **Topic**: `parser.formatting`

Format mathematical expressions for display in multiple notations.
Supports LaTeX, Unicode, Wolfram, and custom formatters for different output targets.





# Expression Formatting

Format mathematical expressions for display in multiple notations.

## Understanding Expression Formatting

### What is Expression Formatting?

Expression formatting converts internal `Expression` structures to human-readable or machine-parseable strings. MathHook provides multiple output formats:

- **LaTeX**: Academic papers, presentations, web rendering (MathJax/KaTeX)
- **Unicode**: Pretty terminal output, notebooks, readable display
- **Wolfram**: Mathematica/Wolfram Language compatibility
- **String**: Rust Debug format for debugging

### How It Works (Architecture)

**Formatter Trait**:
```rust
pub trait Formatter {
    fn format(&self, expr: &Expression) -> String;
}
```

**Implementations**:
- `LaTeXFormatter` - Recursive descent with LaTeX command generation
- `UnicodeFormatter` - Unicode mathematical symbols (superscripts, subscripts)
- `WolframFormatter` - Wolfram Language bracket notation
- Type-aware formatting for noncommutative symbols

## Format Comparison

| **Expression** | **LaTeX** | **Unicode** | **Wolfram** |
|----------------|-----------|-------------|-------------|
| x² + 2x + 1 | `x^{2} + 2 \cdot x + 1` | `x² + 2·x + 1` | `x^2 + 2*x + 1` |
| sin(x) | `\sin(x)` | `sin(x)` | `Sin[x]` |
| √2 | `\sqrt{2}` | `√2` | `Sqrt[2]` |
| π·r² | `\pi \cdot r^{2}` | `π·r²` | `Pi*r^2` |

## Unicode Mathematical Symbols

Supported Unicode ranges:
- **Superscripts**: ⁰¹²³⁴⁵⁶⁷⁸⁹
- **Subscripts**: ₀₁₂₃₄₅₆₇₈₉
- **Greek**: α β γ δ ε ζ η θ ι κ λ μ ν ξ ο π ρ σ τ υ φ χ ψ ω
- **Operators**: · × ÷ ± ∓ ≤ ≥ ≠ ≈ ∞
- **Roots**: √ ∛ ∜
- **Set theory**: ∈ ∉ ⊂ ⊃ ⊆ ⊇ ∪ ∩ ∅












## Examples


### Basic Formatting

Format expressions in different notations

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>




### Type-Aware Formatting

Noncommutative symbols formatted correctly

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>




### Customized LaTeX Output

Configure formatter behavior

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>




### Educational Step Formatting

Format step-by-step explanations

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>






## Performance

**Time Complexity**: O(n) where n = expression tree size


## API Reference

- **Rust**: `mathhook_core::formatter`
- **Python**: `mathhook.formatter`
- **JavaScript**: `mathhook.formatter`


## See Also


- [parser.latex](../parser/latex.md)

- [parser.wolfram](../parser/wolfram.md)

- [parser.custom](../parser/custom.md)

- [core.expressions](../core/expressions.md)

- [educational.step_by_step](../educational/step_by_step.md)


