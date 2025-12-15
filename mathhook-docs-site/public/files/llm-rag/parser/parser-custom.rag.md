# Custom Parsers and Extensions

Extend MathHook's parser for domain-specific mathematical notation.
Add custom functions, operators, preprocessors, and grammar modifications.


---
chunk_id: parser_custom::0
topic: parser.custom
title: Adding Custom Functions
priority: medium
keywords:
  - custom
  - adding custom functions
languages: [rust, python, javascript]
chunk: 1/5
---

## Adding Custom Functions

Register domain-specific functions

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: parser_custom::1
topic: parser.custom
title: Adding Custom Operators
priority: medium
keywords:
  - custom
  - adding custom operators
languages: [rust, python, javascript]
chunk: 2/5
---

## Adding Custom Operators

Define new infix operators with precedence

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: parser_custom::2
topic: parser.custom
title: Preprocessor Transformations
priority: medium
keywords:
  - custom
  - preprocessor transformations
languages: [rust, python, javascript]
chunk: 3/5
---

## Preprocessor Transformations

Transform input before parsing

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: parser_custom::3
topic: parser.custom
title: Domain-Specific Parser (Chemistry)
priority: medium
keywords:
  - custom
  - domain-specific parser (chemistry)
languages: [rust, python, javascript]
chunk: 4/5
---

## Domain-Specific Parser (Chemistry)

Complete chemistry equation parser

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: parser_custom::4
topic: parser.custom
title: Custom LaTeX Macros
priority: medium
keywords:
  - custom
  - custom latex macros
languages: [rust, python, javascript]
chunk: 5/5
---

## Custom LaTeX Macros

Expand LaTeX macros before parsing

### Rust

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

### Python

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

### JavaScript

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



