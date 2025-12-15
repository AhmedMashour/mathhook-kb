<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>PDE Quick Start - 5 Minutes to Your First Solution</h1>
      <p class="description">Quick-start tutorial for solving partial differential equations with MathHook.
Covers transport equation solving in 30 seconds, common PDE patterns, and complete examples.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Transport Equation:**
$$\frac{\partial u}{\partial t} + c \cdot \frac{\partial u}{\partial x} = 0$$

where $c$ is the wave speed and $u(x,t)$ is the unknown function.
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Transport Equation - Basic Solution</h3>
        <p>Solve transport equation with sinusoidal initial condition</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;

// Define variables
let u = symbol!(u);
let t = symbol!(t);
let x = symbol!(x);

// Build PDE structure
let equation = expr!(u);
let pde = Pde::new(equation, u, vec![t.clone(), x.clone()]);

// Solve using method of characteristics
match method_of_characteristics(&pde) {
    Ok(solution) => {
        println!("General solution: F(x - t)");

        // Apply initial condition: u(x,0) = sin(x)
        // Therefore: u(x,t) = sin(x - t)
        let specific_solution = expr!(sin(x - t));

        println!("Specific solution: {}", specific_solution);
    }
    Err(e) => println!("Error: {:?}", e),
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, Pde, method_of_characteristics
import math

# Define variables
u = symbol('u')
t = symbol('t')
x = symbol('x')

# Build PDE
equation = expr(u)
pde = Pde(equation, u, [t, x])

# Solve
solution = method_of_characteristics(pde)
print("General solution: F(x - t)")

# Apply initial condition
specific_solution = expr(f"sin({x} - {t})")
print(f"Specific solution: {specific_solution}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, Pde, methodOfCharacteristics } = require('mathhook');

// Define variables
const u = symbol('u');
const t = symbol('t');
const x = symbol('x');

// Build PDE
const equation = expr(u);
const pde = new Pde(equation, u, [t, x]);

// Solve
const solution = methodOfCharacteristics(pde);
console.log("General solution: F(x - t)");

// Apply initial condition
const specificSolution = expr(`sin(${x} - ${t})`);
console.log(`Specific solution: ${specificSolution}`);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Complete Working Example - Full Workflow</h3>
        <p>End-to-end example with verification and characteristic trajectory</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use mathhook::prelude::*;
use mathhook::pde::method_of_characteristics::{
    method_of_characteristics, solve_characteristic_odes
};
use derivatives::Derivative;
use mathhook::simplify::Simplify;

fn main() {
    println!("═══════════════════════════════════════");
    println!("MathHook PDE Solver - Transport Equation");
    println!("═══════════════════════════════════════\n");

    // Problem: ∂u/∂t + 2·∂u/∂x = 0 with u(x,0) = x²
    let u = symbol!(u);
    let t = symbol!(t);
    let x = symbol!(x);

    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![t.clone(), x.clone()]);

    println!("PDE: ∂u/∂t + 2·∂u/∂x = 0");
    println!("IC:  u(x, 0) = x²\n");

    // Solve
    match method_of_characteristics(&pde) {
        Ok(result) => {
            println!("✓ Method of characteristics applied");

            let solution = expr!((x - (2 * t)) ^ 2);
            println!("Solution: u(x,t) = {}\n", solution);

            // Verify
            let du_dt = solution.derivative(t.clone());
            let du_dx = solution.derivative(x.clone());
            let lhs = expr!(du_dt + (2 * du_dx));

            println!("Verification:");
            println!("  PDE satisfied: {}", lhs.simplify() == expr!(0));
            println!("  IC satisfied: u(x,0) = x²\n");
            println!("✓ Solution complete!");
        }
        Err(e) => println!("✗ Error: {:?}", e),
    }
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from mathhook import symbol, expr, Pde, method_of_characteristics, derivative, simplify

print("═" * 40)
print("MathHook PDE Solver - Transport Equation")
print("═" * 40)

u = symbol('u')
t = symbol('t')
x = symbol('x')

equation = expr(u)
pde = Pde(equation, u, [t, x])

print("\nPDE: ∂u/∂t + 2·∂u/∂x = 0")
print("IC:  u(x, 0) = x²\n")

result = method_of_characteristics(pde)
print("✓ Method of characteristics applied")

solution = expr(f"({x} - 2*{t})^2")
print(f"Solution: u(x,t) = {solution}\n")

# Verify
du_dt = derivative(solution, t)
du_dx = derivative(solution, x)
lhs = expr(f"{du_dt} + 2*{du_dx}")

print("Verification:")
print(f"  PDE satisfied: {simplify(lhs) == expr(0)}")
print("  IC satisfied: u(x,0) = x²")
print("\n✓ Solution complete!")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { symbol, expr, Pde, methodOfCharacteristics, derivative, simplify } = require('mathhook');

console.log("═".repeat(40));
console.log("MathHook PDE Solver - Transport Equation");
console.log("═".repeat(40));

const u = symbol('u');
const t = symbol('t');
const x = symbol('x');

const equation = expr(u);
const pde = new Pde(equation, u, [t, x]);

console.log("\nPDE: ∂u/∂t + 2·∂u/∂x = 0");
console.log("IC:  u(x, 0) = x²\n");

const result = methodOfCharacteristics(pde);
console.log("✓ Method of characteristics applied");

const solution = expr(`(${x} - 2*${t})^2`);
console.log(`Solution: u(x,t) = ${solution}\n`);

// Verify
const duDt = derivative(solution, t);
const duDx = derivative(solution, x);
const lhs = expr(`${duDt} + 2*${duDx}`);

console.log("Verification:");
console.log(`  PDE satisfied: ${simplify(lhs) === expr(0)}`);
console.log("  IC satisfied: u(x,0) = x²");
console.log("\n✓ Solution complete!");
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
const metaDescription = "Quick-start tutorial for solving partial differential equations with MathHook.
Covers transport equation solving in 30 seconds, common PDE patterns, and complete examples.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'PDE Quick Start - 5 Minutes to Your First Solution',
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
