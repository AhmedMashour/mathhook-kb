<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Educational Message Registry</h1>
      <p class="description">The message registry system provides organized, mappable, hashable educational content separated from code logic. Instead of hardcoding explanatory text throughout the codebase, MathHook maintains a centralized registry of educational messages that can be customized, internationalized, and adapted for different audiences.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Basic Message Usage</h3>
        <p>Create and use educational messages with template substitution</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let intro = MessageBuilder::new(
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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">intro = MessageBuilder(
    MessageCategory.LINEAR_EQUATION,
    MessageType.INTRODUCTION,
    0  # variant
) \
.with_substitution("equation", "2x + 3 = 7") \
.with_substitution("variable", "x") \
.build()

print(intro.description)
# Output: "We have a linear equation in the form ax + b = c. To solve for x, we'll isolate the variable."
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const intro = new MessageBuilder(
  MessageCategory.LINEAR_EQUATION,
  MessageType.INTRODUCTION,
  0  // variant
)
.withSubstitution('equation', '2x + 3 = 7')
.withSubstitution('variable', 'x')
.build();

console.log(intro.description);
// Output: "We have a linear equation in the form ax + b = c. To solve for x, we'll isolate the variable."
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Calculus Message with Power Rule</h3>
        <p>Generate educational message for derivative explanation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let derivative_msg = MessageBuilder::new(
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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">derivative_msg = MessageBuilder(
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
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const derivativeMsg = new MessageBuilder(
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
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Multiple Variants for Different Audiences</h3>
        <p>Use different message variants for beginner, intermediate, and advanced students</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Variant 0: Formal mathematical language
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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Variant 0: Formal mathematical language
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
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Variant 0: Formal mathematical language
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
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Generating Educational Step Sequences</h3>
        <p>Generate complete explanation sequences using message registry</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Generate complete explanation sequence
let steps = EducationalMessageGenerator::linear_equation_steps(
    "2x + 3 = 7",  // equation
    "x",           // variable
    "2"            // solution
);

for step in &steps {
    println!("{}", step.description);
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Generate complete explanation sequence
steps = EducationalMessageGenerator.linear_equation_steps(
    "2x + 3 = 7",  # equation
    "x",           # variable
    "2"            # solution
)

for step in steps:
    print(step.description)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Generate complete explanation sequence
const steps = EducationalMessageGenerator.linearEquationSteps(
  '2x + 3 = 7',  // equation
  'x',           // variable
  '2'            // solution
);

steps.forEach(step => {
  console.log(step.description);
});
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
const metaDescription = "The message registry system provides organized, mappable, hashable educational content separated from code logic. Instead of hardcoding explanatory text throughout the codebase, MathHook maintains a centralized registry of educational messages that can be customized, internationalized, and adapted for different audiences.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Educational Message Registry',
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
