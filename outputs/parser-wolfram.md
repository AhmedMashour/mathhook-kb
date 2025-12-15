---









---

# Wolfram Language Notation

> **Topic**: `parser.wolfram`

Parse and generate Mathematica/Wolfram Language syntax for compatibility with Wolfram products.
Supports bidirectional conversion between MathHook expressions and Wolfram notation.





# Wolfram Language Notation

Parse and generate Mathematica/Wolfram Language syntax for compatibility with Wolfram products.

## Understanding Wolfram Notation

### What is Wolfram Language?

Wolfram Language (used in Mathematica, Wolfram Alpha, Wolfram Cloud) is a symbolic computation language with:

- **Capital letter functions**: `Sin`, `Cos`, `Exp` (not `sin`, `cos`, `exp`)
- **Bracket notation**: Function calls use `[]` (e.g., `Sin[x]`, not `Sin(x)`)
- **Symbolic core**: Everything is an expression tree (similar to MathHook)
- **Pattern matching**: Powerful transformation rules (MathHook: simplification)

### Why Wolfram Compatibility?

- **Academic Migration**: Many researchers use Mathematica
- **Cross-Platform**: Export MathHook results to Wolfram Alpha
- **Data Exchange**: Import/export equations between systems
- **Validation**: Compare MathHook results with Mathematica

### How It Works (Architecture)

**Parser:**
- Recognizes Wolfram function tokens (`Sin`, `Cos`, `D`, `Integrate`)
- Bracket parsing: `f[x, y, z]` → Function call
- PascalCase → snake_case conversion for custom functions

**Formatter:**
- Lowercase → Capitalized function names (`sin` → `Sin`)
- Parentheses → Brackets (`(...)` → `[...]`)
- Operator precedence matching Wolfram

## Wolfram ↔ MathHook Translation Table

| **Operation** | **Wolfram** | **MathHook** | **Notes** |
|---------------|-------------|--------------|-----------|
| Addition | `a + b` | `a + b` | Same |
| Multiplication | `a * b` or `a b` | `a * b` | Same |
| Division | `a / b` | `a / b` | Same |
| Power | `a^b` | `a^b` | Same |
| Function call | `f[x]` | `f(x)` | Brackets vs parens |
| Derivative | `D[f, x]` | `f.derivative(&x, 1)` | Functional vs method |
| Integral | `Integrate[f, x]` | `f.integrate(&x)` | Functional vs method |
| Sqrt | `Sqrt[x]` | `sqrt(x)` | Capital vs lowercase |
| Sin | `Sin[x]` | `sin(x)` | Capital vs lowercase |
| Cos | `Cos[x]` | `cos(x)` | Capital vs lowercase |
| Exp | `Exp[x]` | `exp(x)` | Capital vs lowercase |
| Log | `Log[x]` | `log(x)` | Capital vs lowercase |












## Examples


### Basic Wolfram Parsing

Parse common Wolfram Language expressions

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::parser::wolfram::parse_wolfram;

// Functions (capital letters, brackets)
let expr = parse_wolfram("Sin[x]")?;
let expr = parse_wolfram("Exp[x]")?;
let expr = parse_wolfram("Log[x]")?;

// Powers use ^ (not brackets)
let expr = parse_wolfram("x^2")?;

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook.parser import parse_wolfram

# Functions (capital letters, brackets)
expr = parse_wolfram("Sin[x]")
expr = parse_wolfram("Exp[x]")
expr = parse_wolfram("Log[x]")

# Powers use ^ (not brackets)
expr = parse_wolfram("x^2")

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { parseWolfram } = require('mathhook');

// Functions (capital letters, brackets)
const expr = parseWolfram("Sin[x]");
const expr = parseWolfram("Exp[x]");
const expr = parseWolfram("Log[x]");

// Powers use ^ (not brackets)
const expr = parseWolfram("x^2");

```
</details>




### Calculus Operations

Wolfram calculus notation

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>




### Generating Wolfram Output

Format MathHook expressions as Wolfram Language

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>




### Cross-Platform Validation

Export to Wolfram for verification

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>






## Performance

**Time Complexity**: O(n) where n = expression tree size


## API Reference

- **Rust**: `mathhook_core::parser::wolfram`
- **Python**: `mathhook.parser.wolfram`
- **JavaScript**: `mathhook.parser.wolfram`


## See Also


- [parser.latex](../parser/latex.md)

- [parser.formatting](../parser/formatting.md)

- [parser.custom](../parser/custom.md)

- [core.expressions](../core/expressions.md)

- [operations.solving](../operations/solving.md)

- [operations.calculus](../operations/calculus.md)


