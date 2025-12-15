<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Custom Parsers and Extensions</h1>
      <p class="description">Extend MathHook's parser for domain-specific mathematical notation.
Add custom functions, operators, preprocessors, and grammar modifications.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Adding Custom Functions</h3>
        <p>Register domain-specific functions</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::parser::ParserBuilder;

let parser = ParserBuilder::new()
    .add_function("erf", "error_function")
    .add_function("Si", "sine_integral")
    .add_function("Ci", "cosine_integral")
    .build();

let expr = parser.parse("erf(x) + Si(x)")?;
// Parsed as: error_function(x) + sine_integral(x)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.parser import ParserBuilder

parser = (ParserBuilder()
    .add_function("erf", "error_function")
    .add_function("Si", "sine_integral")
    .add_function("Ci", "cosine_integral")
    .build())

expr = parser.parse("erf(x) + Si(x)")
# Parsed as: error_function(x) + sine_integral(x)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { ParserBuilder } = require('mathhook');

const parser = new ParserBuilder()
    .addFunction("erf", "error_function")
    .addFunction("Si", "sine_integral")
    .addFunction("Ci", "cosine_integral")
    .build();

const expr = parser.parse("erf(x) + Si(x)");
// Parsed as: error_function(x) + sine_integral(x)
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Adding Custom Operators</h3>
        <p>Define new infix operators with precedence</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::parser::ParserBuilder;
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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.parser import ParserBuilder, Precedence

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
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { ParserBuilder, Precedence } = require('mathhook');

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
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Preprocessor Transformations</h3>
        <p>Transform input before parsing</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::parser::ParserBuilder;

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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.parser import ParserBuilder

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
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { ParserBuilder } = require('mathhook');

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
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Domain-Specific Parser (Chemistry)</h3>
        <p>Complete chemistry equation parser</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::parser::ParserBuilder;

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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.parser import ParserBuilder

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
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { ParserBuilder } = require('mathhook');

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
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Custom LaTeX Macros</h3>
        <p>Expand LaTeX macros before parsing</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::parser::LatexParserBuilder;

let parser = LatexParserBuilder::new()
    .add_macro(r"\RR", r"\mathbb{R}")    // Real numbers
    .add_macro(r"\CC", r"\mathbb{C}")    // Complex numbers
    .add_macro(r"\NN", r"\mathbb{N}")    // Natural numbers
    .add_macro(r"\dd", r"\mathrm{d}")    // Differential d
    .build();

let expr = parser.parse(r"f: \RR \to \CC")?;
// Expands to: f: \mathbb{R} \to \mathbb{C}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook.parser import LatexParserBuilder

parser = (LatexParserBuilder()
    .add_macro(r"\RR", r"\mathbb{R}")
    .add_macro(r"\CC", r"\mathbb{C}")
    .add_macro(r"\NN", r"\mathbb{N}")
    .add_macro(r"\dd", r"\mathrm{d}")
    .build())

expr = parser.parse(r"f: \RR \to \CC")
# Expands to: f: \mathbb{R} \to \mathbb{C}
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { LatexParserBuilder } = require('mathhook');

const parser = new LatexParserBuilder()
    .addMacro(String.raw`\RR`, String.raw`\mathbb{R}`)
    .addMacro(String.raw`\CC`, String.raw`\mathbb{C}`)
    .addMacro(String.raw`\NN`, String.raw`\mathbb{N}`)
    .addMacro(String.raw`\dd`, String.raw`\mathrm{d}`)
    .build();

const expr = parser.parse(String.raw`f: \RR \to \CC`);
// Expands to: f: \mathbb{R} \to \mathbb{C}
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
const metaDescription = "Extend MathHook's parser for domain-specific mathematical notation.
Add custom functions, operators, preprocessors, and grammar modifications.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Custom Parsers and Extensions',
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
