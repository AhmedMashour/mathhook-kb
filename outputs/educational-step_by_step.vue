<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Step-by-Step Explanations</h1>
      <p class="description">Educational explanations transform mathematical operations from black boxes into transparent learning experiences. The step-by-step system generates detailed walkthroughs showing exactly how MathHook arrives at solutions.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Simple Simplification Explanation</h3>
        <p>Generate and display step-by-step simplification</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = symbol('x')
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
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');
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
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Expansion Explanation</h3>
        <p>Show polynomial expansion with educational context</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
let expr = expr!((x + 1) * (x - 1));

let explanation = expr.explain_expansion();

// Check if expansion occurred
if explanation.total_steps > 0 {
    println!("Expansion required {} steps", explanation.total_steps);
    println!("Rules used: {:?}", explanation.rules_used);
} else {
    println!("Already in expanded form");
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = symbol('x')
expr = expr_parse('(x + 1) * (x - 1)')

explanation = expr.explain_expansion()

# Check if expansion occurred
if explanation.total_steps > 0:
    print(f"Expansion required {explanation.total_steps} steps")
    print(f"Rules used: {explanation.rules_used}")
else:
    print("Already in expanded form")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');
const expr = parseExpr('(x + 1) * (x - 1)');

const explanation = expr.explainExpansion();

// Check if expansion occurred
if (explanation.totalSteps > 0) {
  console.log(`Expansion required ${explanation.totalSteps} steps`);
  console.log(`Rules used: ${explanation.rulesUsed}`);
} else {
  console.log('Already in expanded form');
}
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Enhanced Steps with API Data</h3>
        <p>Create enhanced steps for external application integration</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use std::collections::HashMap;

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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">inputs = {
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
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const inputs = new Map([
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
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>JSON Export for External Applications</h3>
        <p>Export structured educational data for LMS integration</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Export structured data for LMS integration
let json = explanation.to_json()?;
// Send to learning management system, mobile app, etc.
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Export structured data for LMS integration
json_data = explanation.to_json()
# Send to learning management system, mobile app, etc.
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Export structured data for LMS integration
const json = explanation.toJson();
// Send to learning management system, mobile app, etc.
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
const metaDescription = "Educational explanations transform mathematical operations from black boxes into transparent learning experiences. The step-by-step system generates detailed walkthroughs showing exactly how MathHook arrives at solutions.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Step-by-Step Explanations',
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
