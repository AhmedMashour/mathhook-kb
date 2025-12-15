# Schema Reference

> Complete specification of the MathHook Knowledge Base schema format.

## Design Philosophy

**Hybrid Schema Design**: Core content (required) + Output hints (optional) + Engine fills gaps

This ensures:
- Minimal required fields for quick schema creation
- Rich optional fields for comprehensive docs
- Format-specific customization without cluttering core

## Schema Structure

```yaml
# === CORE CONTENT (Required) ===
topic: "calculus.derivative"
title: "Symbolic Differentiation"
description: |
  Computes the derivative of an expression with respect to a variable.

# === EXAMPLES (Required) ===
examples:
  - title: "Power Rule"
    explanation: "Derivative of x^n is n*x^(n-1)"
    code:
      python: |
        result = derivative(x**3, x)  # 3*x^2
      rust: |
        let df = expr!(x^3).derivative(&x, 1);
      nodejs: |
        const df = derivative(expr("x^3"), x);

# === OPTIONAL FIELDS ===
mathematical_definition: |
  $$\frac{d}{dx} f(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$$

code_refs:
  rust: "mathhook_core::calculus::derivative"
  python: "mathhook.calculus.derivative"
  nodejs: "mathhook.calculus.derivative"

use_cases:
  - "Physics: Computing velocity from position"
  - "Machine Learning: Backpropagation gradients"

related_topics:
  - "calculus.integral"
  - "calculus.chain_rule"

# === OUTPUT HINTS (Optional) ===
outputs:
  jupyter:
    include_interactive_plots: true
  vue_site:
    seo_keywords: ["differentiation", "calculus"]
```

## Core Fields (Required)

### `topic`

**Type**: `string`
**Format**: `category.subcategory` (dot-separated hierarchy)

```yaml
topic: "calculus.derivative"
topic: "ode.first_order.separable"
topic: "algebra.simplify"
```

### `title`

**Type**: `string`
**Description**: Human-readable title displayed in documentation

```yaml
title: "Symbolic Differentiation"
```

### `description`

**Type**: `string` (multiline supported)
**Length**: 1-3 sentences recommended

```yaml
description: |
  Computes the derivative of an expression with respect to a variable
  using symbolic differentiation rules (power rule, chain rule, etc.).
```

### `examples`

**Type**: `array` of `Example`
**Minimum**: 1 example required

Each example requires:
- `title`: Example name
- `explanation`: What the example demonstrates
- `code.python`, `code.rust`, `code.nodejs`: Multi-language code

```yaml
examples:
  - title: "Basic Example"
    explanation: "Shows the power rule in action"
    code:
      python: "result = derivative(x**2, x)"
      rust: "let df = expr!(x^2).derivative(&x, 1);"
      nodejs: "const df = derivative(expr('x^2'), x);"
    expected_output: "2*x"  # Optional
```

## Optional Metadata

### `mathematical_definition`

**Type**: `string` (LaTeX supported)
**Usage**: Display mathematical formula at top of documentation

```yaml
mathematical_definition: |
  $$\frac{d}{dx} f(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$$
```

### `code_refs`

**Type**: `object` with `rust`, `python`, `nodejs` keys
**Usage**: Links to actual function implementations

```yaml
code_refs:
  rust: "mathhook_core::calculus::derivative"
  python: "mathhook.calculus.derivative"
  nodejs: "mathhook.calculus.derivative"
```

### `use_cases`

**Type**: `array` of `string`
**Usage**: Real-world applications

```yaml
use_cases:
  - "Physics: Computing velocity from position"
  - "Machine Learning: Backpropagation gradients"
  - "Economics: Marginal cost analysis"
```

### `related_topics`

**Type**: `array` of `string` (topic IDs)
**Usage**: Cross-references to related documentation

```yaml
related_topics:
  - "calculus.integral"
  - "calculus.chain_rule"
  - "calculus.limits"
```

### `performance`

**Type**: `object` with `complexity`, `typical_time`

```yaml
performance:
  complexity: "O(n)"
  typical_time: "0.5ms"
```

## Article Content

For long-form documentation:

### `article.introduction.hook`

Opening paragraph to engage readers:

```yaml
article:
  introduction:
    hook: |
      The derivative measures the instantaneous rate of change of a function.
      It's fundamental to calculus and has applications across science and engineering.
```

### `article.sections`

Main documentation content:

```yaml
article:
  sections:
    - title: "Theory"
      content: |
        The formal definition of a derivative...

        $$f'(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$$

    - title: "Applications"
      content: |
        Derivatives are used in physics to compute velocity...
```

### `article.sidebars`

Callout boxes for important information:

```yaml
article:
  sidebars:
    - type: "note"      # note, warning, info, performance
      title: "Remember"
      content: "The derivative of a constant is always zero."

    - type: "warning"
      title: "Common Mistake"
      content: "Don't forget the chain rule for composite functions."
```

## Output Hints

Customize generation for specific formats:

### Jupyter Hints

```yaml
outputs:
  jupyter:
    include_interactive_plots: true
    include_performance_section: false
    kernel: "python3"
```

### Vue Site Hints

```yaml
outputs:
  vue_site:
    include_live_demo: true
    seo_keywords: ["symbolic differentiation", "calculus"]
    og_image: "derivative-og.png"
```

### LLM-RAG Hints

```yaml
outputs:
  llm_rag:
    chunk_strategy: "by_example"  # by_example, by_section, fixed_size
    max_chunk_size: 512
    include_metadata: true
```

### mdBook Hints

```yaml
outputs:
  mdbook:
    include_mathematical_proof: true
    runnable_code: true
    hide_nodejs_examples: false
```

### LaTeX Hints

```yaml
outputs:
  latex:
    document_class: "article"
    include_proofs: true
    bibliography_style: "plain"
```

## Validation Rules

### Required Fields
- `topic`: Must be non-empty string
- `title`: Must be non-empty string
- `description`: Must be non-empty string
- `examples`: Must have at least 1 example

### Example Validation
- Each example must have `title`, `explanation`, `code`
- `code` must include all three languages: `python`, `rust`, `nodejs`
- Code should be syntactically valid

### Cross-Language Consistency
- Code examples should produce mathematically equivalent results
- Same input should yield same output across languages

### Topic ID Format
- Lowercase letters, numbers, underscores, dots only
- Hierarchy separated by dots
- Examples: `calculus.derivative`, `ode.first_order.linear`

## Complete Example

```yaml
topic: "calculus.derivative"
title: "Symbolic Differentiation"

description: |
  Computes the derivative of an expression with respect to a variable
  using symbolic differentiation rules (power rule, chain rule, etc.).

mathematical_definition: |
  $$\frac{d}{dx} f(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$$

code_refs:
  rust: "mathhook_core::calculus::derivative"
  python: "mathhook.calculus.derivative"
  nodejs: "mathhook.calculus.derivative"

examples:
  - title: "Power Rule"
    explanation: "Derivative of x^n is n*x^(n-1)"
    code:
      python: |
        from mathhook import symbol, derivative
        x = symbol('x')
        result = derivative(x**3, x)
        print(result)  # 3*x^2
      rust: |
        let x = symbol!(x);
        let f = expr!(x ^ 3);
        let df = f.derivative(&x, 1);
        println!("{}", df);  // 3*x^2
      nodejs: |
        const { symbol, derivative, expr } = require('mathhook');
        const x = symbol('x');
        const df = derivative(expr('x^3'), x);
        console.log(df.toString());  // 3*x^2
    expected_output: "3*x^2"

use_cases:
  - "Physics: Computing velocity from position"
  - "Machine Learning: Backpropagation gradients"

related_topics:
  - "calculus.integral"
  - "calculus.chain_rule"

performance:
  complexity: "O(n)"
  typical_time: "0.5ms"

article:
  introduction:
    hook: |
      **Symbolic differentiation** computes exact derivatives of mathematical
      expressions, preserving the symbolic form rather than using numerical
      approximations.
  sections:
    - title: "Differentiation Rules"
      content: |
        MathHook implements all standard differentiation rules:

        - **Power Rule**: $\frac{d}{dx} x^n = nx^{n-1}$
        - **Chain Rule**: $\frac{d}{dx} f(g(x)) = f'(g(x)) \cdot g'(x)$
        - **Product Rule**: $\frac{d}{dx} [f(x)g(x)] = f'(x)g(x) + f(x)g'(x)$
  sidebars:
    - type: "performance"
      title: "Speed Comparison"
      content: "MathHook is 50-100x faster than SymPy for polynomial derivatives."

outputs:
  jupyter:
    include_interactive_plots: true
  vue_site:
    seo_keywords: ["derivative", "calculus", "symbolic math"]
```

## Related Documentation

- [CLAUDE.md](../CLAUDE.md) - AI agent development guide
- [Architecture](./architecture.md) - System design
- [Generator Guide](./generator-guide.md) - Writing new generators
