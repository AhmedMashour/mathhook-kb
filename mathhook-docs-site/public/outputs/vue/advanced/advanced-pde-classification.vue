<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>PDE Classification</h1>
      <p class="description">Mathematical classification of partial differential equations into elliptic, parabolic,
and hyperbolic types using the discriminant formula. Different PDE types require
completely different solution methods and have distinct physical interpretations.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">For a general second-order PDE:
$$A \frac{\partial^2 u}{\partial x^2} + B \frac{\partial^2 u}{\partial x \partial y} + C \frac{\partial^2 u}{\partial y^2} + \text{lower order terms} = 0$$

The **discriminant** is:
$$\Delta = B^2 - 4AC$$

**Classification:**
- $\Delta < 0$ → **Elliptic** (e.g., Laplace: $u_{xx} + u_{yy} = 0$)
- $\Delta = 0$ → **Parabolic** (e.g., Heat: $u_t = u_{xx}$)
- $\Delta > 0$ → **Hyperbolic** (e.g., Wave: $u_{tt} = c^2 u_{xx}$)
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Wave Equation Classification (Hyperbolic)</h3>
        <p>Wave equation has positive discriminant and is classified as hyperbolic</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

// Wave equation structure
let equation = expr!(mul: x, t);
let pde = Pde::new(equation, u, vec![x, t]);

// Automatic classification
let pde_type = pde.pde_type();
assert_eq!(pde_type, Some(PdeType::Hyperbolic));

// Discriminant computation
let disc = pde.compute_discriminant();
assert!(disc > 0.0);
assert_eq!(disc, 4.0);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">u = symbol('u')
x = symbol('x')
t = symbol('t')

# Wave equation structure
equation = expr(mul=[x, t])
pde = Pde.new(equation, u, [x, t])

# Automatic classification
pde_type = pde.pde_type()
assert pde_type == PdeType.Hyperbolic

# Discriminant computation
disc = pde.compute_discriminant()
assert disc > 0.0
assert disc == 4.0
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const u = symbol('u');
const x = symbol('x');
const t = symbol('t');

// Wave equation structure
const equation = expr({ mul: [x, t] });
const pde = Pde.new(equation, u, [x, t]);

// Automatic classification
const pdeType = pde.pdeType();
assert(pdeType === PdeType.Hyperbolic);

// Discriminant computation
const disc = pde.computeDiscriminant();
assert(disc > 0.0);
assert(disc === 4.0);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Heat Equation Classification (Parabolic)</h3>
        <p>Heat equation has zero discriminant and is classified as parabolic</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

// Heat equation structure
let equation = expr!(add: x, t);
let pde = Pde::new(equation, u, vec![x, t]);

// Automatic classification
let pde_type = pde.pde_type();
assert_eq!(pde_type, Some(PdeType::Parabolic));

// Discriminant
let disc = pde.compute_discriminant();
assert_eq!(disc.abs(), 0.0);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">u = symbol('u')
x = symbol('x')
t = symbol('t')

# Heat equation structure
equation = expr(add=[x, t])
pde = Pde.new(equation, u, [x, t])

# Automatic classification
pde_type = pde.pde_type()
assert pde_type == PdeType.Parabolic

# Discriminant
disc = pde.compute_discriminant()
assert abs(disc) == 0.0
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const u = symbol('u');
const x = symbol('x');
const t = symbol('t');

// Heat equation structure
const equation = expr({ add: [x, t] });
const pde = Pde.new(equation, u, [x, t]);

// Automatic classification
const pdeType = pde.pdeType();
assert(pdeType === PdeType.Parabolic);

// Discriminant
const disc = pde.computeDiscriminant();
assert(Math.abs(disc) === 0.0);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Laplace Equation Classification (Elliptic)</h3>
        <p>Laplace equation has negative discriminant and is classified as elliptic</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let u = symbol!(u);
let x = symbol!(x);
let y = symbol!(y);

// Laplace equation structure
let equation = expr!(add: x, y);
let pde = Pde::new(equation, u, vec![x, y]);

// Automatic classification
let pde_type = pde.pde_type();
assert_eq!(pde_type, Some(PdeType::Elliptic));

// Discriminant
let disc = pde.compute_discriminant();
assert!(disc < 0.0);
assert_eq!(disc, -4.0);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">u = symbol('u')
x = symbol('x')
y = symbol('y')

# Laplace equation structure
equation = expr(add=[x, y])
pde = Pde.new(equation, u, [x, y])

# Automatic classification
pde_type = pde.pde_type()
assert pde_type == PdeType.Elliptic

# Discriminant
disc = pde.compute_discriminant()
assert disc < 0.0
assert disc == -4.0
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const u = symbol('u');
const x = symbol('x');
const y = symbol('y');

// Laplace equation structure
const equation = expr({ add: [x, y] });
const pde = Pde.new(equation, u, [x, y]);

// Automatic classification
const pdeType = pde.pdeType();
assert(pdeType === PdeType.Elliptic);

// Discriminant
const disc = pde.computeDiscriminant();
assert(disc < 0.0);
assert(disc === -4.0);
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
const metaDescription = "Mathematical classification of partial differential equations into elliptic, parabolic,
and hyperbolic types using the discriminant formula. Different PDE types require
completely different solution methods and have distinct physical interpretations.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'PDE Classification',
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
