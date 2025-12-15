---









---

# Educational API

> **Topic**: `educational.api`

The Educational API provides programmatic access to MathHook's educational features for external applications. Integrate step-by-step explanations, assessment tools, and adaptive learning systems into Learning Management Systems (LMS), mobile apps, and educational platforms.





# Educational API

> 📍 **Navigation:** [Step-by-Step](./step-by-step.md) | [Message Registry](./messages.md) | [Advanced Features](../advanced/complex-numbers.md)

The Educational API provides programmatic access to MathHook's educational features for external applications. Integrate step-by-step explanations, assessment tools, and adaptive learning systems into Learning Management Systems (LMS), mobile apps, and educational platforms.

## What is the Educational API?

**Learning Journey:** This is the advanced topic after mastering [step-by-step explanations](./step-by-step.md) and [message customization](./messages.md). Here you'll learn programmatic integration for external applications.

**Purpose:** Enable external applications to:
- Generate educational content programmatically
- Export structured data for machine consumption
- Integrate with Learning Management Systems
- Build adaptive learning applications
- Create assessment and verification tools
- Track student progress systematically

**Design Philosophy:** Dual-format output—human-readable explanations for students AND machine-consumable data for applications.

## API Architecture

### Core Components

```rust
use mathhook::educational::{
    traits::{EducationalOperation, OperationContext},
    enhanced_steps::{EnhancedStep, EnhancedStepExplanation, EnhancedStepBuilder},
    step_by_step::{StepByStepExplanation, Step},
};
```

### Layer Architecture

```
┌─────────────────────────────────────────────┐
│         External Application                 │
│    (LMS, Mobile App, Web Frontend)          │
└─────────────────┬───────────────────────────┘
                  │
                  │ JSON/REST API
                  │
┌─────────────────▼───────────────────────────┐
│      Educational API Layer                   │
│  (EnhancedStep, EducationalOperation)       │
└─────────────────┬───────────────────────────┘
                  │
                  │ Internal API
                  │
┌─────────────────▼───────────────────────────┐
│    Message Registry + Step Generation       │
│  (Templates, Substitution, SmartStepFactory)│
└─────────────────┬───────────────────────────┘
                  │
                  │ Core Operations
                  │
┌─────────────────▼───────────────────────────┐
│      Mathematical Engine                     │
│  (Solving, Simplification, Differentiation) │
└─────────────────────────────────────────────┘
```

## Dual-Format Output

### EnhancedStep Structure

The core dual-format type:

```rust
pub struct EnhancedStep {
    pub step_id: String,
    pub title: String,
    pub human_message: String,      // For students
    pub api_data: StepApiData,      // For machines
    pub message_key: MessageKey,    // For customization
    pub math_context: MathContext,  // Mathematical state
    pub presentation: PresentationHints,
}
```

### Machine-Readable Format

JSON Structure for external applications with metadata, summary, and detailed step information.












## Examples


### Basic Dual-Format Output

Generate both human and machine-readable educational content

<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);
let equation = expr!(2*x + 3);

// Get enhanced explanation
let explanation = EnhancedStepExplanation::new(steps);

// Display for students
for step in &explanation.steps {
    println!("{}", step.human_message);
}

// Export structured data
let json = explanation.to_json()?;
// Parse in external application
let data: serde_json::Value = serde_json::from_str(&json)?;

```
</details>

<details>
<summary><b>Python</b></summary>

```python
x = symbol('x')
equation = expr_parse('2*x + 3')

# Get enhanced explanation
explanation = EnhancedStepExplanation(steps)

# Display for students
for step in explanation.steps:
    print(step.human_message)

# Export structured data
json_data = explanation.to_json()
# Parse in external application
data = json.loads(json_data)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');
const equation = parseExpr('2*x + 3');

// Get enhanced explanation
const explanation = new EnhancedStepExplanation(steps);

// Display for students
explanation.steps.forEach(step => {
  console.log(step.humanMessage);
});

// Export structured data
const json = explanation.toJson();
// Parse in external application
const data = JSON.parse(json);

```
</details>




### SmartStepFactory Context-Aware Generation

Generate educational steps based on operation context and difficulty

<details>
<summary><b>Rust</b></summary>

```rust
// Generate introduction step for linear equation
let intro_step = EnhancedStepBuilder::new("step_1")
    .with_human_message(
        "Identify Equation Type",
        "We have a linear equation in one variable"
    )
    .with_api_data("linear_equation", "identification", "classify")
    .with_input("equation", "2x + 3 = 0")
    .with_input("variable", "x")
    .with_output("equation_type", "linear")
    .with_output("degree", "1")
    .with_math_context("2x + 3 = 0", "x", 0.25)
    .with_message_key("linear_equation", "introduction", 0)
    .build();

```
</details>

<details>
<summary><b>Python</b></summary>

```python
# Generate introduction step for linear equation
intro_step = EnhancedStepBuilder("step_1") \
    .with_human_message(
        "Identify Equation Type",
        "We have a linear equation in one variable"
    ) \
    .with_api_data("linear_equation", "identification", "classify") \
    .with_input("equation", "2x + 3 = 0") \
    .with_input("variable", "x") \
    .with_output("equation_type", "linear") \
    .with_output("degree", "1") \
    .with_math_context("2x + 3 = 0", "x", 0.25) \
    .with_message_key("linear_equation", "introduction", 0) \
    .build()

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
// Generate introduction step for linear equation
const introStep = new EnhancedStepBuilder('step_1')
  .withHumanMessage(
    'Identify Equation Type',
    'We have a linear equation in one variable'
  )
  .withApiData('linear_equation', 'identification', 'classify')
  .withInput('equation', '2x + 3 = 0')
  .withInput('variable', 'x')
  .withOutput('equation_type', 'linear')
  .withOutput('degree', '1')
  .withMathContext('2x + 3 = 0', 'x', 0.25)
  .withMessageKey('linear_equation', 'introduction', 0)
  .build();

```
</details>




### Educational Operation Trait Implementation

Add educational capabilities to mathematical operations

<details>
<summary><b>Rust</b></summary>

```rust
struct LinearEquationSolver {
    equation: Expression,
    variable: Symbol,
}

impl EducationalOperation for LinearEquationSolver {
    type Output = Vec<Expression>;

    fn execute_with_steps(&self) -> (Self::Output, StepByStepExplanation) {
        let mut steps = Vec::new();

        // Step 1: Identify equation type
        steps.push(Step::new(
            "Identify Equation Type",
            "This is a linear equation ax + b = 0"
        ));

        // Step 2: Isolate variable term
        steps.push(Step::new(
            "Isolate Variable Term",
            "Subtract constant from both sides"
        ));

        // Step 3: Solve for variable
        steps.push(Step::new(
            "Solve for Variable",
            "Divide both sides by coefficient"
        ));

        // Perform actual solving
        let solution = self.solve_internal();

        let explanation = StepByStepExplanation::new(steps);
        (solution, explanation)
    }

    fn educational_context(&self) -> OperationContext {
        OperationContext::equation_solving(3)  // difficulty level 3
    }

    fn execute_fast(&self) -> Self::Output {
        // Optimized path without explanation generation
        self.solve_internal()
    }

    fn estimated_steps(&self) -> Option<usize> {
        Some(3)  // Known step count for linear equations
    }
}

```
</details>

<details>
<summary><b>Python</b></summary>

```python
class LinearEquationSolver(EducationalOperation):
    def __init__(self, equation, variable):
        self.equation = equation
        self.variable = variable

    def execute_with_steps(self):
        steps = []

        # Step 1: Identify equation type
        steps.append(Step(
            "Identify Equation Type",
            "This is a linear equation ax + b = 0"
        ))

        # Step 2: Isolate variable term
        steps.append(Step(
            "Isolate Variable Term",
            "Subtract constant from both sides"
        ))

        # Step 3: Solve for variable
        steps.append(Step(
            "Solve for Variable",
            "Divide both sides by coefficient"
        ))

        # Perform actual solving
        solution = self.solve_internal()

        explanation = StepByStepExplanation(steps)
        return (solution, explanation)

    def educational_context(self):
        return OperationContext.equation_solving(3)

    def execute_fast(self):
        return self.solve_internal()

    def estimated_steps(self):
        return 3

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
class LinearEquationSolver extends EducationalOperation {
  constructor(equation, variable) {
    super();
    this.equation = equation;
    this.variable = variable;
  }

  executeWithSteps() {
    const steps = [];

    // Step 1: Identify equation type
    steps.push(new Step(
      'Identify Equation Type',
      'This is a linear equation ax + b = 0'
    ));

    // Step 2: Isolate variable term
    steps.push(new Step(
      'Isolate Variable Term',
      'Subtract constant from both sides'
    ));

    // Step 3: Solve for variable
    steps.push(new Step(
      'Solve for Variable',
      'Divide both sides by coefficient'
    ));

    // Perform actual solving
    const solution = this.solveInternal();

    const explanation = new StepByStepExplanation(steps);
    return [solution, explanation];
  }

  educationalContext() {
    return OperationContext.equationSolving(3);
  }

  executeFast() {
    return this.solveInternal();
  }

  estimatedSteps() {
    return 3;
  }
}

```
</details>




### LMS Integration with Progress Tracking

Export educational content to Learning Management System

<details>
<summary><b>Rust</b></summary>

```rust
// Generate explanation
let explanation = EnhancedStepExplanation::new(steps);

// Export to JSON
let json = explanation.to_json()?;

// Send to LMS via REST API
let client = reqwest::Client::new();
let response = client
    .post("https://lms.example.com/api/lessons")
    .json(&serde_json::from_str::<serde_json::Value>(&json)?)
    .send()
    .await?;

// Track which steps student has viewed
for step in &explanation.steps {
    lms_api.mark_step_viewed(
        student_id,
        lesson_id,
        &step.step_id
    ).await?;
}

```
</details>

<details>
<summary><b>Python</b></summary>

```python
# Generate explanation
explanation = EnhancedStepExplanation(steps)

# Export to JSON
json_data = explanation.to_json()

# Send to LMS via REST API
response = requests.post(
    "https://lms.example.com/api/lessons",
    json=json.loads(json_data)
)

# Track which steps student has viewed
for step in explanation.steps:
    lms_api.mark_step_viewed(
        student_id,
        lesson_id,
        step.step_id
    )

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
// Generate explanation
const explanation = new EnhancedStepExplanation(steps);

// Export to JSON
const json = explanation.toJson();

// Send to LMS via REST API
const response = await fetch('https://lms.example.com/api/lessons', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: json
});

// Track which steps student has viewed
for (const step of explanation.steps) {
  await lmsApi.markStepViewed(
    studentId,
    lessonId,
    step.stepId
  );
}

```
</details>




### Assessment and Verification

Verify student answers and provide feedback

<details>
<summary><b>Rust</b></summary>

```rust
// Verify student's answer against expected solution
fn verify_answer(
    student_answer: &str,
    expected_solution: &Expression,
    variable: &Symbol
) -> VerificationResult {
    let student_expr = parse_latex(student_answer)?;

    // Substitute student's answer into original equation
    let substituted = original_equation.substitute(variable, &student_expr);
    let simplified = substituted.simplify();

    VerificationResult {
        correct: simplified == Expression::integer(0),
        student_expression: student_expr,
        substituted_form: substituted,
        explanation: generate_verification_explanation(&simplified),
    }
}

```
</details>

<details>
<summary><b>Python</b></summary>

```python
def verify_answer(student_answer, expected_solution, variable):
    student_expr = parse_latex(student_answer)

    # Substitute student's answer into original equation
    substituted = original_equation.substitute(variable, student_expr)
    simplified = substituted.simplify()

    return VerificationResult(
        correct=(simplified == Expression.integer(0)),
        student_expression=student_expr,
        substituted_form=substituted,
        explanation=generate_verification_explanation(simplified)
    )

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
function verifyAnswer(studentAnswer, expectedSolution, variable) {
  const studentExpr = parseLatex(studentAnswer);

  // Substitute student's answer into original equation
  const substituted = originalEquation.substitute(variable, studentExpr);
  const simplified = substituted.simplify();

  return new VerificationResult({
    correct: simplified.equals(Expression.integer(0)),
    studentExpression: studentExpr,
    substitutedForm: substituted,
    explanation: generateVerificationExplanation(simplified)
  });
}

```
</details>







## API Reference

- **Rust**: `mathhook::educational::traits`
- **Python**: `mathhook.educational.traits`
- **JavaScript**: `mathhook.educational.traits`


## See Also


- [educational.step_by_step](../educational/step_by_step.md)

- [educational.messages](../educational/messages.md)

- [architecture.design_principles](../architecture/design_principles.md)

- [bindings.python](../bindings/python.md)

- [bindings.nodejs](../bindings/nodejs.md)


