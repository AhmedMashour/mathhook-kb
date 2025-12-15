<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Educational API</h1>
      <p class="description">The Educational API provides programmatic access to MathHook's educational features for external applications. Integrate step-by-step explanations, assessment tools, and adaptive learning systems into Learning Management Systems (LMS), mobile apps, and educational platforms.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Dual-Format Output</h3>
        <p>Generate both human and machine-readable educational content</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = symbol('x')
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
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');
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
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>SmartStepFactory Context-Aware Generation</h3>
        <p>Generate educational steps based on operation context and difficulty</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Generate introduction step for linear equation
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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Generate introduction step for linear equation
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
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Generate introduction step for linear equation
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
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Educational Operation Trait Implementation</h3>
        <p>Add educational capabilities to mathematical operations</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">struct LinearEquationSolver {
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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">class LinearEquationSolver(EducationalOperation):
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
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">class LinearEquationSolver extends EducationalOperation {
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
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>LMS Integration with Progress Tracking</h3>
        <p>Export educational content to Learning Management System</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Generate explanation
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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Generate explanation
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
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Generate explanation
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
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Assessment and Verification</h3>
        <p>Verify student answers and provide feedback</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Verify student's answer against expected solution
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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">def verify_answer(student_answer, expected_solution, variable):
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
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">function verifyAnswer(studentAnswer, expectedSolution, variable) {
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
</code></pre>
        </div>

        
      </div>
      
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const activeTab = ref('python')

// SEO metadata
const metaDescription = "The Educational API provides programmatic access to MathHook's educational features for external applications. Integrate step-by-step explanations, assessment tools, and adaptive learning systems into Learning Management Systems (LMS), mobile apps, and educational platforms.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Educational API',
  description: metaDescription,
  keywords: keywords.join(', '),
  
})
</script>

<style scoped>
.doc-page {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}

.doc-header h1 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.description {
  font-size: 1.2rem;
  color: #666;
}

.math-definition {
  background: #f5f5f5;
  padding: 1.5rem;
  border-radius: 8px;
  margin: 2rem 0;
}

.example-card {
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 1.5rem;
  margin-bottom: 2rem;
}

.code-tabs {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.code-tabs button {
  padding: 0.5rem 1rem;
  border: none;
  background: #eee;
  cursor: pointer;
  border-radius: 4px;
}

.code-tabs button:hover {
  background: #ddd;
}

.code-block {
  background: #1e1e1e;
  color: #d4d4d4;
  padding: 1rem;
  border-radius: 4px;
  overflow-x: auto;
}

.output {
  margin-top: 1rem;
  padding: 1rem;
  background: #f9f9f9;
  border-left: 4px solid #42b883;
}
</style>
