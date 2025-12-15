---









---

# Custom Parsers and Extensions

> **Topic**: `parser.custom`

Extend MathHook's parser for domain-specific mathematical notation.
Add custom functions, operators, preprocessors, and grammar modifications.





# Custom Parsers and Extensions

Extend MathHook's parser for domain-specific mathematical notation.

## Understanding Parser Extension

### What Can You Extend?

MathHook's parser is modular and extensible. You can add:

- **Custom Functions**: Domain-specific functions (chemistry, physics, engineering)
- **Custom Operators**: New infix/prefix/postfix operators
- **Custom Notation**: LaTeX macros, specialized symbols
- **Parser Preprocessors**: Transform input before parsing
- **Lexer Tokens**: New token types for specialized syntax

### When to Extend the Parser

**Use Built-In Features When:**
- Standard mathematical notation suffices
- Functions can be named conventionally
- LaTeX or Wolfram notation covers your needs

**Extend the Parser When:**
- Domain-specific notation is essential (chemistry: `→`, physics: `⊗`)
- Custom operators with special precedence
- Proprietary mathematical notation
- Legacy system compatibility

### Architecture Overview

```
Input String
    ↓
Preprocessor (optional) - Transform syntax before parsing
    ↓
Lexer - Tokenize input (recognizes custom tokens)
    ↓
Parser (LALRPOP) - Build expression tree
    ↓
Post-Processor (optional) - Transform parsed expression
    ↓
Expression
```

## Domain-Specific Examples

### Chemistry Notation

Custom parser for chemical equations:
- `→` for yields
- `⇌` for equilibrium
- Subscripts for molecular formulas

### Quantum Mechanics

Specialized notation:
- `⊗` for tensor product
- `⟨|⟩` for inner product
- `[,]` for commutator
- `{,}` for anticommutator

### Financial Mathematics

Finance-specific functions:
- NPV (Net Present Value)
- IRR (Internal Rate of Return)
- FV/PV (Future/Present Value)
- % operator for percentages

### Control Theory

System notation:
- `*` as convolution
- `ℒ` for Laplace transform
- `//` for feedback connection












## Examples


### Adding Custom Functions

Register domain-specific functions


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::parser::ParserBuilder;

let parser = ParserBuilder::new()
    .add_function("erf", "error_function")
    .add_function("Si", "sine_integral")
    .add_function("Ci", "cosine_integral")
    .build();

let expr = parser.parse("erf(x) + Si(x)")?;
// Parsed as: error_function(x) + sine_integral(x)

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook.parser import ParserBuilder

parser = (ParserBuilder()
    .add_function("erf", "error_function")
    .add_function("Si", "sine_integral")
    .add_function("Ci", "cosine_integral")
    .build())

expr = parser.parse("erf(x) + Si(x)")
# Parsed as: error_function(x) + sine_integral(x)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { ParserBuilder } = require('mathhook');

const parser = new ParserBuilder()
    .addFunction("erf", "error_function")
    .addFunction("Si", "sine_integral")
    .addFunction("Ci", "cosine_integral")
    .build();

const expr = parser.parse("erf(x) + Si(x)");
// Parsed as: error_function(x) + sine_integral(x)

```
</details>





### Adding Custom Operators

Define new infix operators with precedence


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::parser::ParserBuilder;
use mathhook::parser::Precedence;

let parser = ParserBuilder::new()
    .add_operator("×", "*")      // Cross product symbol
    .add_operator("⊗", "tensor") // Tensor product
    .add_operator_with_precedence(
        "⊕",
        "direct_sum",
        Precedence::Addition
    )
    .build();

let expr = parser.parse("A ⊗ B")?;
// Parsed as: tensor(A, B)

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook.parser import ParserBuilder, Precedence

parser = (ParserBuilder()
    .add_operator("×", "*")
    .add_operator("⊗", "tensor")
    .add_operator_with_precedence(
        "⊕",
        "direct_sum",
        Precedence.ADDITION
    )
    .build())

expr = parser.parse("A ⊗ B")
# Parsed as: tensor(A, B)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { ParserBuilder, Precedence } = require('mathhook');

const parser = new ParserBuilder()
    .addOperator("×", "*")
    .addOperator("⊗", "tensor")
    .addOperatorWithPrecedence(
        "⊕",
        "direct_sum",
        Precedence.ADDITION
    )
    .build();

const expr = parser.parse("A ⊗ B");
// Parsed as: tensor(A, B)

```
</details>





### Preprocessor Transformations

Transform input before parsing


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::parser::ParserBuilder;

let parser = ParserBuilder::new()
    .add_preprocessor(|input| {
        input.replace("→", "->")   // Arrow notation
             .replace("×", "*")     // Cross product
             .replace("÷", "/")     // Division symbol
    })
    .build();

let expr = parser.parse("x → ∞")?;
// Preprocessed to: x -> ∞
// Then parsed normally

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook.parser import ParserBuilder

def preprocess(input_str):
    return (input_str
        .replace("→", "->")
        .replace("×", "*")
        .replace("÷", "/"))

parser = (ParserBuilder()
    .add_preprocessor(preprocess)
    .build())

expr = parser.parse("x → ∞")
# Preprocessed to: x -> ∞
# Then parsed normally

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { ParserBuilder } = require('mathhook');

const parser = new ParserBuilder()
    .addPreprocessor((input) => {
        return input
            .replace(/→/g, "->")
            .replace(/×/g, "*")
            .replace(/÷/g, "/");
    })
    .build();

const expr = parser.parse("x → ∞");
// Preprocessed to: x -> ∞
// Then parsed normally

```
</details>





### Domain-Specific Parser (Chemistry)

Complete chemistry equation parser


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::parser::ParserBuilder;

fn create_chemistry_parser() -> Parser {
    ParserBuilder::new()
        .add_operator("→", "yields")
        .add_operator("⇌", "equilibrium")
        .add_operator("+", "plus")
        .add_preprocessor(|input| {
            // H2O → H_2*O
            expand_chemical_formulas(input)
        })
        .add_postprocessor(|expr| {
            balance_equation(expr)
        })
        .build()
}

let parser = create_chemistry_parser();
let reaction = parser.parse("H₂ + O₂ → H₂O")?;
let balanced = reaction.balance();  // 2H₂ + O₂ → 2H₂O

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook.parser import ParserBuilder

def create_chemistry_parser():
    return (ParserBuilder()
        .add_operator("→", "yields")
        .add_operator("⇌", "equilibrium")
        .add_operator("+", "plus")
        .add_preprocessor(expand_chemical_formulas)
        .add_postprocessor(balance_equation)
        .build())

parser = create_chemistry_parser()
reaction = parser.parse("H₂ + O₂ → H₂O")
balanced = reaction.balance()  # 2H₂ + O₂ → 2H₂O

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { ParserBuilder } = require('mathhook');

function createChemistryParser() {
    return new ParserBuilder()
        .addOperator("→", "yields")
        .addOperator("⇌", "equilibrium")
        .addOperator("+", "plus")
        .addPreprocessor(expandChemicalFormulas)
        .addPostprocessor(balanceEquation)
        .build();
}

const parser = createChemistryParser();
const reaction = parser.parse("H₂ + O₂ → H₂O");
const balanced = reaction.balance();  // 2H₂ + O₂ → 2H₂O

```
</details>





### Custom LaTeX Macros

Expand LaTeX macros before parsing


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::parser::LatexParserBuilder;

let parser = LatexParserBuilder::new()
    .add_macro(r"\RR", r"\mathbb{R}")    // Real numbers
    .add_macro(r"\CC", r"\mathbb{C}")    // Complex numbers
    .add_macro(r"\NN", r"\mathbb{N}")    // Natural numbers
    .add_macro(r"\dd", r"\mathrm{d}")    // Differential d
    .build();

let expr = parser.parse(r"f: \RR \to \CC")?;
// Expands to: f: \mathbb{R} \to \mathbb{C}

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook.parser import LatexParserBuilder

parser = (LatexParserBuilder()
    .add_macro(r"\RR", r"\mathbb{R}")
    .add_macro(r"\CC", r"\mathbb{C}")
    .add_macro(r"\NN", r"\mathbb{N}")
    .add_macro(r"\dd", r"\mathrm{d}")
    .build())

expr = parser.parse(r"f: \RR \to \CC")
# Expands to: f: \mathbb{R} \to \mathbb{C}

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { LatexParserBuilder } = require('mathhook');

const parser = new LatexParserBuilder()
    .addMacro(String.raw`\RR`, String.raw`\mathbb{R}`)
    .addMacro(String.raw`\CC`, String.raw`\mathbb{C}`)
    .addMacro(String.raw`\NN`, String.raw`\mathbb{N}`)
    .addMacro(String.raw`\dd`, String.raw`\mathrm{d}`)
    .build();

const expr = parser.parse(String.raw`f: \RR \to \CC`);
// Expands to: f: \mathbb{R} \to \mathbb{C}

```
</details>








## API Reference

- **Rust**: `mathhook_core::parser::extensions`
- **Python**: `mathhook.parser.extensions`
- **JavaScript**: `mathhook.parser.extensions`


## See Also


- [parser.latex](../parser/latex.md)

- [parser.wolfram](../parser/wolfram.md)

- [parser.formatting](../parser/formatting.md)

- [operations.functions](../operations/functions.md)

- [advanced.pattern_matching](../advanced/pattern_matching.md)


