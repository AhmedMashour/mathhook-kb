# Educational Message Registry

The message registry system provides organized, mappable, hashable educational content separated from code logic. Instead of hardcoding explanatory text throughout the codebase, MathHook maintains a centralized registry of educational messages that can be customized, internationalized, and adapted for different audiences.


---
chunk_id: educational_messages::0
topic: educational.messages
title: Basic Message Usage
priority: medium
keywords:
  - messages
  - basic message usage
languages: [rust, python, javascript]
chunk: 1/4
---

## Basic Message Usage

Create and use educational messages with template substitution

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: educational_messages::1
topic: educational.messages
title: Calculus Message with Power Rule
priority: medium
keywords:
  - messages
  - calculus message with power rule
languages: [rust, python, javascript]
chunk: 2/4
---

## Calculus Message with Power Rule

Generate educational message for derivative explanation

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: educational_messages::2
topic: educational.messages
title: Multiple Variants for Different Audiences
priority: medium
keywords:
  - messages
  - multiple variants for different audiences
languages: [rust, python, javascript]
chunk: 3/4
---

## Multiple Variants for Different Audiences

Use different message variants for beginner, intermediate, and advanced students

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: educational_messages::3
topic: educational.messages
title: Generating Educational Step Sequences
priority: medium
keywords:
  - messages
  - generating educational step sequences
languages: [rust, python, javascript]
chunk: 4/4
---

## Generating Educational Step Sequences

Generate complete explanation sequences using message registry

### Rust

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

### Python

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

### JavaScript

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



