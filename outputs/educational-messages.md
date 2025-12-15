---









---

# Educational Message Registry

> **Topic**: `educational.messages`

The message registry system provides organized, mappable, hashable educational content separated from code logic. Instead of hardcoding explanatory text throughout the codebase, MathHook maintains a centralized registry of educational messages that can be customized, internationalized, and adapted for different audiences.





# Educational Message Registry

> 📍 **Navigation:** [Step-by-Step](./step-by-step.md) | [Educational API](./api.md) | [Previous: Getting Started](../getting-started/learning-paths.md)

The message registry system provides organized, mappable, hashable educational content separated from code logic. Instead of hardcoding explanatory text throughout the codebase, MathHook maintains a centralized registry of educational messages that can be customized, internationalized, and adapted for different audiences.

## What is the Message Registry?

**Problem:** Hardcoding educational text in code makes it difficult to:
- Customize explanations for different student levels
- Translate content to other languages
- Maintain consistent educational messaging
- A/B test different explanations
- Update content without code changes

**Solution:** The message registry provides a centralized, indexed system where:
- Messages are stored separately from code logic
- Each message has a unique hash for fast lookup
- Templates support dynamic substitution
- Content can be customized per audience without touching code

**Learning Journey:** After understanding [step-by-step explanations](./step-by-step.md), learn how to customize the language. Then explore [programmatic API integration](./api.md).

## Architecture

### Core Components

```rust
use mathhook::educational::message_registry::{
    MessageCategory,
    MessageType,
    MessageKey,
    MessageHashSystem,
    MessageBuilder,
};
```

### Message Key Structure

Every message is uniquely identified by a composite key:

```rust
pub struct MessageKey {
    pub category: String,        // Domain: "linear_equation", "calculus", etc.
    pub message_type: String,    // Type: "introduction", "strategy", "result"
    pub variant: u32,            // Alternative phrasing (0, 1, 2, ...)
    pub hash: u64,               // Fast lookup hash
    pub template_params: Vec<String>,  // Required substitutions
}
```

**Hash System for Performance:**

Messages are hashed for O(1) lookup:

$$
\text{hash} = \text{fnv1a}(\text{category} \oplus \text{type} \oplus \text{variant})
$$

This allows instant message retrieval without string comparison.

## Message Categories

### Algebra Messages

Messages for algebraic operations including linear equations, quadratic equations, polynomial equations, and general algebraic simplification.

### Calculus Messages

Messages for calculus operations including derivatives (power rule, chain rule), integration (by parts, substitution), and limits.

### Solver Messages

Messages for equation solving including system equations (substitution, elimination), matrix methods, and solution verification.

### ODE Messages

Messages for ordinary differential equations including separable equations, linear first-order equations, and exact equations.

**Separable ODE Form:**

$$
\frac{dy}{dx} = g(x) \cdot h(y) \implies \frac{dy}{h(y)} = g(x) \, dx
$$

### PDE Messages

Messages for partial differential equations including heat equation, wave equation, and Laplace equation.

**Heat Equation:**

$$
\frac{\partial u}{\partial t} = \alpha \frac{\partial^2 u}{\partial x^2}
$$

### Noncommutative Algebra Messages

Messages for matrix and operator algebra where order matters. Critical property: For matrices, $AB \neq BA$ in general, so left and right division are different.












## Examples


### Basic Message Usage

Create and use educational messages with template substitution

<details>
<summary><b>Rust</b></summary>

```rust
let intro = MessageBuilder::new(
    MessageCategory::LinearEquation,
    MessageType::Introduction,
    0  // variant
)
.with_substitution("equation", "2x + 3 = 7")
.with_substitution("variable", "x")
.build()
.unwrap();

println!("{}", intro.description);
// Output: "We have a linear equation in the form ax + b = c. To solve for x, we'll isolate the variable."

```
</details>

<details>
<summary><b>Python</b></summary>

```python
intro = MessageBuilder(
    MessageCategory.LINEAR_EQUATION,
    MessageType.INTRODUCTION,
    0  # variant
) \
.with_substitution("equation", "2x + 3 = 7") \
.with_substitution("variable", "x") \
.build()

print(intro.description)
# Output: "We have a linear equation in the form ax + b = c. To solve for x, we'll isolate the variable."

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const intro = new MessageBuilder(
  MessageCategory.LINEAR_EQUATION,
  MessageType.INTRODUCTION,
  0  // variant
)
.withSubstitution('equation', '2x + 3 = 7')
.withSubstitution('variable', 'x')
.build();

console.log(intro.description);
// Output: "We have a linear equation in the form ax + b = c. To solve for x, we'll isolate the variable."

```
</details>




### Calculus Message with Power Rule

Generate educational message for derivative explanation

<details>
<summary><b>Rust</b></summary>

```rust
let derivative_msg = MessageBuilder::new(
    MessageCategory::Calculus,
    MessageType::DerivativePowerRule,
    0
)
.with_substitution("expression", "x^3")
.with_substitution("exponent", "3")
.with_substitution("result", "3x^2")
.build()
.unwrap();

println!("{}", derivative_msg.description);
// Output: "Apply the power rule: d/dx(x^3) = 3·x^(3-1) = 3x^2"

```
</details>

<details>
<summary><b>Python</b></summary>

```python
derivative_msg = MessageBuilder(
    MessageCategory.CALCULUS,
    MessageType.DERIVATIVE_POWER_RULE,
    0
) \
.with_substitution("expression", "x^3") \
.with_substitution("exponent", "3") \
.with_substitution("result", "3x^2") \
.build()

print(derivative_msg.description)
# Output: "Apply the power rule: d/dx(x^3) = 3·x^(3-1) = 3x^2"

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const derivativeMsg = new MessageBuilder(
  MessageCategory.CALCULUS,
  MessageType.DERIVATIVE_POWER_RULE,
  0
)
.withSubstitution('expression', 'x^3')
.withSubstitution('exponent', '3')
.withSubstitution('result', '3x^2')
.build();

console.log(derivativeMsg.description);
// Output: "Apply the power rule: d/dx(x^3) = 3·x^(3-1) = 3x^2"

```
</details>




### Multiple Variants for Different Audiences

Use different message variants for beginner, intermediate, and advanced students

<details>
<summary><b>Rust</b></summary>

```rust
// Variant 0: Formal mathematical language
let formal = MessageBuilder::new(
    MessageCategory::LinearEquation,
    MessageType::Strategy,
    0  // variant 0
)
.build()
.unwrap();

// Variant 1: Conversational tone
let casual = MessageBuilder::new(
    MessageCategory::LinearEquation,
    MessageType::Strategy,
    1  // variant 1
)
.build()
.unwrap();

// Variant 2: Step-by-step procedural
let procedural = MessageBuilder::new(
    MessageCategory::LinearEquation,
    MessageType::Strategy,
    2  // variant 2
)
.build()
.unwrap();

```
</details>

<details>
<summary><b>Python</b></summary>

```python
# Variant 0: Formal mathematical language
formal = MessageBuilder(
    MessageCategory.LINEAR_EQUATION,
    MessageType.STRATEGY,
    0  # variant 0
).build()

# Variant 1: Conversational tone
casual = MessageBuilder(
    MessageCategory.LINEAR_EQUATION,
    MessageType.STRATEGY,
    1  # variant 1
).build()

# Variant 2: Step-by-step procedural
procedural = MessageBuilder(
    MessageCategory.LINEAR_EQUATION,
    MessageType.STRATEGY,
    2  # variant 2
).build()

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
// Variant 0: Formal mathematical language
const formal = new MessageBuilder(
  MessageCategory.LINEAR_EQUATION,
  MessageType.STRATEGY,
  0  // variant 0
).build();

// Variant 1: Conversational tone
const casual = new MessageBuilder(
  MessageCategory.LINEAR_EQUATION,
  MessageType.STRATEGY,
  1  // variant 1
).build();

// Variant 2: Step-by-step procedural
const procedural = new MessageBuilder(
  MessageCategory.LINEAR_EQUATION,
  MessageType.STRATEGY,
  2  // variant 2
).build();

```
</details>




### Generating Educational Step Sequences

Generate complete explanation sequences using message registry

<details>
<summary><b>Rust</b></summary>

```rust
// Generate complete explanation sequence
let steps = EducationalMessageGenerator::linear_equation_steps(
    "2x + 3 = 7",  // equation
    "x",           // variable
    "2"            // solution
);

for step in &steps {
    println!("{}", step.description);
}

```
</details>

<details>
<summary><b>Python</b></summary>

```python
# Generate complete explanation sequence
steps = EducationalMessageGenerator.linear_equation_steps(
    "2x + 3 = 7",  # equation
    "x",           # variable
    "2"            # solution
)

for step in steps:
    print(step.description)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
// Generate complete explanation sequence
const steps = EducationalMessageGenerator.linearEquationSteps(
  '2x + 3 = 7',  // equation
  'x',           // variable
  '2'            // solution
);

steps.forEach(step => {
  console.log(step.description);
});

```
</details>






## Performance

**Time Complexity**: O(1)


## API Reference

- **Rust**: `mathhook::educational::message_registry`
- **Python**: `mathhook.educational.message_registry`
- **JavaScript**: `mathhook.educational.messageRegistry`


## See Also


- [educational.step_by_step](../educational/step_by_step.md)

- [educational.api](../educational/api.md)

- [architecture.design_principles](../architecture/design_principles.md)

- [contributing.documentation](../contributing/documentation.md)


