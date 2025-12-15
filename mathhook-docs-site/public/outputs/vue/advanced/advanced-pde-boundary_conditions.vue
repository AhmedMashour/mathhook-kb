<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Boundary Conditions</h1>
      <p class="description">Boundary conditions (BCs) specify constraints on the PDE solution at domain boundaries.
They determine eigenvalues and influence solution behavior. Covers Dirichlet, Neumann,
Robin, and periodic boundary conditions with implementation details and eigenvalue computation.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">**Dirichlet (Fixed Value):**
$$u = g \quad \text{on } \partial\Omega$$

**Neumann (Fixed Derivative):**
$$\frac{\partial u}{\partial n} = h \quad \text{on } \partial\Omega$$

**Robin (Mixed):**
$$\alpha u + \beta \frac{\partial u}{\partial n} = g \quad \text{on } \partial\Omega$$

**Periodic:**
$$u(0,t) = u(L,t), \quad \frac{\partial u}{\partial x}(0,t) = \frac{\partial u}{\partial x}(L,t)$$

**Eigenvalues (1D Dirichlet):**
$$\lambda_n = \left(\frac{n\pi}{L}\right)^2, \quad n = 1, 2, 3, \ldots$$

**Eigenvalues (1D Neumann):**
$$\lambda_n = \left(\frac{n\pi}{L}\right)^2, \quad n = 0, 1, 2, \ldots$$
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Homogeneous Dirichlet Boundary Condition</h3>
        <p>Fixed value at both boundaries (u=0), commonly used for heat equation with fixed temperatures</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);

// Homogeneous: u(0,t) = 0
let bc_left = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: x.clone(), value: expr!(0) },
);

// Homogeneous: u(L,t) = 0
let bc_right = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: x, value: expr!(1) },
);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = symbol('x')

# Homogeneous: u(0,t) = 0
bc_left = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.Simple(variable=x, value=expr(0))
)

# Homogeneous: u(L,t) = 0
bc_right = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.Simple(variable=x, value=expr(1))
)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');

// Homogeneous: u(0,t) = 0
const bcLeft = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.Simple({ variable: x, value: expr(0) })
);

// Homogeneous: u(L,t) = 0
const bcRight = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.Simple({ variable: x, value: expr(1) })
);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Non-Homogeneous Dirichlet Boundary Condition</h3>
        <p>Fixed non-zero value at boundary, common in heat transfer with maintained temperatures</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);

// u(0,t) = 0
let bc_left = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: x.clone(), value: expr!(0) },
);

// Non-homogeneous: u(L,t) = 100
let bc_right = BoundaryCondition::dirichlet(
    expr!(100),
    BoundaryLocation::Simple { variable: x, value: expr!(1) },
);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = symbol('x')

# u(0,t) = 0
bc_left = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.Simple(variable=x, value=expr(0))
)

# Non-homogeneous: u(L,t) = 100
bc_right = BoundaryCondition.dirichlet(
    expr(100),
    BoundaryLocation.Simple(variable=x, value=expr(1))
)
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');

// u(0,t) = 0
const bcLeft = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.Simple({ variable: x, value: expr(0) })
);

// Non-homogeneous: u(L,t) = 100
const bcRight = BoundaryCondition.dirichlet(
    expr(100),
    BoundaryLocation.Simple({ variable: x, value: expr(1) })
);
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
const metaDescription = "Boundary conditions (BCs) specify constraints on the PDE solution at domain boundaries.
They determine eigenvalues and influence solution behavior. Covers Dirichlet, Neumann,
Robin, and periodic boundary conditions with implementation details and eigenvalue computation.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Boundary Conditions',
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
