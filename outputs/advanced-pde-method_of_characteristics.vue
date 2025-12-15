<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Method of Characteristics</h1>
      <p class="description">The Method of Characteristics is the primary technique for solving first-order
partial differential equations (PDEs). It transforms the PDE into a system of
ordinary differential equations (ODEs) that can be solved along special curves
called characteristic curves.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Quasi-linear PDE:**
$$a(x,y,u) \frac{\partial u}{\partial x} + b(x,y,u) \frac{\partial u}{\partial y} = c(x,y,u)$$

**Characteristic equations:**
$$\frac{dx}{ds} = a(x,y,u), \quad \frac{dy}{ds} = b(x,y,u), \quad \frac{du}{ds} = c(x,y,u)$$

where $s$ is a parameter along the characteristic curve.

**Transport Equation:**
$$\frac{\partial u}{\partial t} + c \cdot \frac{\partial u}{\partial x} = 0$$

**General solution:** $u(x,t) = f(x - ct)$ where $f$ is determined by initial conditions.

**Burgers' Equation (Nonlinear):**
$$\frac{\partial u}{\partial t} + u \frac{\partial u}{\partial x} = 0$$

**Implicit solution:** $u(x,t) = f(x - u(x,t) \cdot t)$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Transport Equation Solution</h3>
        <p>Solving the transport equation using method of characteristics</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let u = symbol!(u);
let t = symbol!(t);
let x = symbol!(x);

// Transport equation PDE structure
let equation = expr!(u);
let pde = Pde::new(equation, u, vec![t, x]);

// Solve
let solution = method_of_characteristics(&pde).unwrap();
println!("Solution: u(x,t) = F(x - ct)");

// With initial condition u(x,0) = sin(x):
println!("Specific solution: u(x,t) = sin(x - ct)");
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">u = symbol('u')
t = symbol('t')
x = symbol('x')

# Transport equation PDE structure
equation = expr(u)
pde = Pde.new(equation, u, [t, x])

# Solve
solution = method_of_characteristics(pde)
print("Solution: u(x,t) = F(x - ct)")

# With initial condition u(x,0) = sin(x):
print("Specific solution: u(x,t) = sin(x - ct)")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const u = symbol('u');
const t = symbol('t');
const x = symbol('x');

// Transport equation PDE structure
const equation = expr(u);
const pde = Pde.new(equation, u, [t, x]);

// Solve
const solution = methodOfCharacteristics(pde);
console.log("Solution: u(x,t) = F(x - ct)");

// With initial condition u(x,0) = sin(x):
console.log("Specific solution: u(x,t) = sin(x - ct)");
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>General Usage Pattern</h3>
        <p>Standard pattern for using method of characteristics in MathHook</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Define PDE
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

let equation = /* build PDE expression */;
let pde = Pde::new(equation, u, vec![x, t]);

// Solve
match method_of_characteristics(&pde) {
    Ok(solution) => {
        println!("Solution: {}", solution.solution);
        // Apply initial conditions as needed
    }
    Err(e) => println!("Error: {:?}", e),
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Define PDE
u = symbol('u')
x = symbol('x')
t = symbol('t')

equation = # build PDE expression
pde = Pde.new(equation, u, [x, t])

# Solve
try:
    solution = method_of_characteristics(pde)
    print(f"Solution: {solution.solution}")
    # Apply initial conditions as needed
except Exception as e:
    print(f"Error: {e}")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Define PDE
const u = symbol('u');
const x = symbol('x');
const t = symbol('t');

const equation = /* build PDE expression */;
const pde = Pde.new(equation, u, [x, t]);

// Solve
try {
    const solution = methodOfCharacteristics(pde);
    console.log(`Solution: ${solution.solution}`);
    // Apply initial conditions as needed
} catch (e) {
    console.log(`Error: ${e}`);
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
const metaDescription = "The Method of Characteristics is the primary technique for solving first-order
partial differential equations (PDEs). It transforms the PDE into a system of
ordinary differential equations (ODEs) that can be solved along special curves
called characteristic curves.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Method of Characteristics',
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
