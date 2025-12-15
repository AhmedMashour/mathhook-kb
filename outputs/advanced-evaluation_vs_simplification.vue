<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Evaluation vs Simplification</h1>
      <p class="description">Understand the key differences between evaluation (computing numerical results)
and simplification (algebraic transformation) in MathHook's symbolic engine.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Evaluation Example</h3>
        <p></p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);

let evaluated = expr.substitute(&x, &expr!(3));
// Result: 16 (numerical)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = symbol('x')
expr_val = x**2 + 2*x + 1

evaluated = expr_val.subs(x, 3)
# Result: 16
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');
const expr = add(pow(x, 2), mul(2, x), 1);

const evaluated = expr.subs(x, 3);
// Result: 16
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Simplification Example</h3>
        <p></p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">let x = symbol!(x);
let expr = expr!(x + x + x);

let simplified = expr.simplify();
// Result: 3*x (still symbolic)
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">x = symbol('x')
expr_val = x + x + x

simplified = simplify(expr_val)
# Result: 3*x
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const x = symbol('x');
const expr = add(x, x, x);

const simplified = expr.simplify();
// Result: 3*x
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
const metaDescription = "Understand the key differences between evaluation (computing numerical results)
and simplification (algebraic transformation) in MathHook's symbolic engine.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Evaluation vs Simplification',
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
