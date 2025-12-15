# Educational API

The Educational API provides programmatic access to MathHook's educational features for external applications. Integrate step-by-step explanations, assessment tools, and adaptive learning systems into Learning Management Systems (LMS), mobile apps, and educational platforms.


---
chunk_id: educational_api::0
topic: educational.api
title: Basic Dual-Format Output
priority: medium
keywords:
  - api
  - basic dual-format output
languages: [rust, python, javascript]
chunk: 1/5
---

## Basic Dual-Format Output

Generate both human and machine-readable educational content

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: educational_api::1
topic: educational.api
title: SmartStepFactory Context-Aware Generation
priority: medium
keywords:
  - api
  - smartstepfactory context-aware generation
languages: [rust, python, javascript]
chunk: 2/5
---

## SmartStepFactory Context-Aware Generation

Generate educational steps based on operation context and difficulty

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: educational_api::2
topic: educational.api
title: Educational Operation Trait Implementation
priority: medium
keywords:
  - api
  - educational operation trait implementation
languages: [rust, python, javascript]
chunk: 3/5
---

## Educational Operation Trait Implementation

Add educational capabilities to mathematical operations

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: educational_api::3
topic: educational.api
title: LMS Integration with Progress Tracking
priority: medium
keywords:
  - api
  - lms integration with progress tracking
languages: [rust, python, javascript]
chunk: 4/5
---

## LMS Integration with Progress Tracking

Export educational content to Learning Management System

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: educational_api::4
topic: educational.api
title: Assessment and Verification
priority: medium
keywords:
  - api
  - assessment and verification
languages: [rust, python, javascript]
chunk: 5/5
---

## Assessment and Verification

Verify student answers and provide feedback

### Rust

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

### Python

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

### JavaScript

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



