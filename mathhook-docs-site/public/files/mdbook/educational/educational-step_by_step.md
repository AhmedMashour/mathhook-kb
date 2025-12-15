---









---

# Step-by-Step Explanations

> **Topic**: `educational.step_by_step`

Educational explanations transform mathematical operations from black boxes into transparent learning experiences. The step-by-step system generates detailed walkthroughs showing exactly how MathHook arrives at solutions.





# Step-by-Step Explanations

> 📍 **Navigation:** [Educational API](./api.md) | [Message Registry](./messages.md) | [Operations](../operations/simplification.md)

Educational explanations transform mathematical operations from black boxes into transparent learning experiences. The step-by-step system generates detailed walkthroughs showing exactly how MathHook arrives at solutions.

## What Are Step-by-Step Explanations?

Step-by-step explanations provide detailed walkthroughs of mathematical operations, breaking down complex procedures into digestible steps. Each step includes:

- **Human-readable description** - Natural language explanation
- **Mathematical notation** - LaTeX and symbolic expressions
- **Rule applied** - The mathematical principle used
- **Current state** - Expression at this stage of solving

**Learning Journey:** This is your entry point for understanding MathHook's educational features. Once you master basic explanations, explore [message customization](./messages.md) and [programmatic integration](./api.md).

## Core Architecture

### StepByStepExplanation Structure

The core explanation type contains the complete journey from problem to solution:

```rust
pub struct StepByStepExplanation {
    pub initial_expression: Expression,
    pub final_expression: Expression,
    pub steps: Vec<Step>,
    pub total_steps: usize,
    pub rules_used: Vec<String>,
}
```

**Mathematical Formula for Steps:**

Each transformation follows the pattern:

$$
\text{Expression}_i \xrightarrow{\text{rule}} \text{Expression}_{i+1}
$$

Where the complete journey is:

$$
E_0 \xrightarrow{r_1} E_1 \xrightarrow{r_2} E_2 \xrightarrow{r_3} \cdots \xrightarrow{r_n} E_n
$$

### Step Structure

Each individual step captures one transformation:

```rust
pub struct Step {
    pub title: String,              // Brief step title
    pub description: String,        // Detailed explanation
    pub expression: Expression,     // Result after this step
    pub rule_applied: String,       // Mathematical rule name
    pub latex: Option<String>,      // LaTeX representation
}
```

### EnhancedStep: Dual-Format System

Enhanced steps provide both human and machine-consumable content:

```rust
pub struct EnhancedStep {
    pub step_id: String,
    pub title: String,
    pub human_message: String,      // Student-friendly text
    pub api_data: StepApiData,      // Machine-readable data
    pub message_key: MessageKey,    // Lookup for customization
    pub math_context: MathContext,  // Variables, progress, state
    pub presentation: PresentationHints,
}
```

**Design Philosophy:** Human messages teach students; API data enables external applications (LMS, mobile apps, assessment tools).












## Examples


### Simple Simplification Explanation

Generate and display step-by-step simplification


<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);
let expr = expr!(2*x + 3*x + 5);

// Generate step-by-step explanation
let explanation = expr.explain_simplification();

// Display for students
println!("Simplifying: {}", explanation.initial_expression);
for (i, step) in explanation.steps.iter().enumerate() {
    println!("\nStep {}: {}", i + 1, step.title);
    println!("  {}", step.description);
    println!("  Result: {}", step.expression);
    println!("  Rule: {}", step.rule_applied);
}
println!("\nFinal answer: {}", explanation.final_expression);

```
</details>



<details>
<summary><b>Python</b></summary>

```python
x = symbol('x')
expr = expr_parse('2*x + 3*x + 5')

# Generate step-by-step explanation
explanation = expr.explain_simplification()

# Display for students
print(f"Simplifying: {explanation.initial_expression}")
for i, step in enumerate(explanation.steps):
    print(f"\nStep {i + 1}: {step.title}")
    print(f"  {step.description}")
    print(f"  Result: {step.expression}")
    print(f"  Rule: {step.rule_applied}")
print(f"\nFinal answer: {explanation.final_expression}")

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');
const expr = parseExpr('2*x + 3*x + 5');

// Generate step-by-step explanation
const explanation = expr.explainSimplification();

// Display for students
console.log(`Simplifying: ${explanation.initialExpression}`);
explanation.steps.forEach((step, i) => {
  console.log(`\nStep ${i + 1}: ${step.title}`);
  console.log(`  ${step.description}`);
  console.log(`  Result: ${step.expression}`);
  console.log(`  Rule: ${step.ruleApplied}`);
});
console.log(`\nFinal answer: ${explanation.finalExpression}`);

```
</details>





### Expansion Explanation

Show polynomial expansion with educational context


<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);
let expr = expr!((x + 1) * (x - 1));

let explanation = expr.explain_expansion();

// Check if expansion occurred
if explanation.total_steps > 0 {
    println!("Expansion required {} steps", explanation.total_steps);
    println!("Rules used: {:?}", explanation.rules_used);
} else {
    println!("Already in expanded form");
}

```
</details>



<details>
<summary><b>Python</b></summary>

```python
x = symbol('x')
expr = expr_parse('(x + 1) * (x - 1)')

explanation = expr.explain_expansion()

# Check if expansion occurred
if explanation.total_steps > 0:
    print(f"Expansion required {explanation.total_steps} steps")
    print(f"Rules used: {explanation.rules_used}")
else:
    print("Already in expanded form")

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');
const expr = parseExpr('(x + 1) * (x - 1)');

const explanation = expr.explainExpansion();

// Check if expansion occurred
if (explanation.totalSteps > 0) {
  console.log(`Expansion required ${explanation.totalSteps} steps`);
  console.log(`Rules used: ${explanation.rulesUsed}`);
} else {
  console.log('Already in expanded form');
}

```
</details>





### Enhanced Steps with API Data

Create enhanced steps for external application integration


<details>
<summary><b>Rust</b></summary>

```rust
use std::collections::HashMap;

let mut inputs = HashMap::new();
inputs.insert("coefficient".to_string(), "2".to_string());
inputs.insert("variable".to_string(), "x".to_string());

let mut outputs = HashMap::new();
outputs.insert("solution".to_string(), "x = 3".to_string());

let step = EnhancedStepBuilder::new("step_1")
    .with_human_message(
        "Isolate Variable",
        "Divide both sides by the coefficient to isolate x"
    )
    .with_api_data("linear_equation", "isolation", "divide_coefficient")
    .with_input("coefficient", "2")
    .with_output("solution", "x = 3")
    .with_math_context("2x = 6", "x", 0.75)  // 75% progress
    .with_message_key("linear_equation", "isolation", 0)
    .build();

```
</details>



<details>
<summary><b>Python</b></summary>

```python
inputs = {
    "coefficient": "2",
    "variable": "x"
}

outputs = {
    "solution": "x = 3"
}

step = EnhancedStepBuilder("step_1") \
    .with_human_message(
        "Isolate Variable",
        "Divide both sides by the coefficient to isolate x"
    ) \
    .with_api_data("linear_equation", "isolation", "divide_coefficient") \
    .with_input("coefficient", "2") \
    .with_output("solution", "x = 3") \
    .with_math_context("2x = 6", "x", 0.75) \
    .with_message_key("linear_equation", "isolation", 0) \
    .build()

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const inputs = new Map([
  ['coefficient', '2'],
  ['variable', 'x']
]);

const outputs = new Map([
  ['solution', 'x = 3']
]);

const step = new EnhancedStepBuilder('step_1')
  .withHumanMessage(
    'Isolate Variable',
    'Divide both sides by the coefficient to isolate x'
  )
  .withApiData('linear_equation', 'isolation', 'divide_coefficient')
  .withInput('coefficient', '2')
  .withOutput('solution', 'x = 3')
  .withMathContext('2x = 6', 'x', 0.75)  // 75% progress
  .withMessageKey('linear_equation', 'isolation', 0)
  .build();

```
</details>





### JSON Export for External Applications

Export structured educational data for LMS integration


<details>
<summary><b>Rust</b></summary>

```rust
// Export structured data for LMS integration
let json = explanation.to_json()?;
// Send to learning management system, mobile app, etc.

```
</details>



<details>
<summary><b>Python</b></summary>

```python
# Export structured data for LMS integration
json_data = explanation.to_json()
# Send to learning management system, mobile app, etc.

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
// Export structured data for LMS integration
const json = explanation.toJson();
// Send to learning management system, mobile app, etc.

```
</details>








## API Reference

- **Rust**: `mathhook::educational::step_by_step`
- **Python**: `mathhook.educational.step_by_step`
- **JavaScript**: `mathhook.educational.stepByStep`


## See Also


- [educational.messages](../educational/messages.md)

- [educational.api](../educational/api.md)

- [operations.simplification](../operations/simplification.md)

- [operations.solving](../operations/solving.md)

- [operations.differentiation](../operations/differentiation.md)


