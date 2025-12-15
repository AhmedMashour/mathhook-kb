<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>Benchmarking Guide</h1>
      <p class="description">Comprehensive guide to MathHook's performance benchmarking infrastructure across all supported
platforms (Rust, Python, Node.js), including benchmark usage, development workflow, and
contributing new benchmarks.
</p>
    </header>

    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Rust Criterion Benchmark Template</h3>
        <p>Template for adding new benchmarks in Rust</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use criterion::{criterion_group, criterion_main, Criterion};
use mathhook_core::{parse, symbol, Expression};
use std::hint::black_box;

fn bench_my_operation(c: &mut Criterion) {
    let mut group = c.benchmark_group("my_operation_group");

    let x = symbol!(x);

    // Without parsing - measures pure algorithm
    group.bench_function("operation_native", |b| {
        let expr = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::integer(1),
        ]);
        b.iter(|| black_box(expr.clone().my_operation()))
    });

    // With parsing - measures end-to-end user experience
    group.bench_function("operation_with_parsing", |b| {
        b.iter(|| {
            let expr = parse("x + 1").unwrap();
            black_box(expr.my_operation())
        })
    });

    group.finish();
}

criterion_group!(benches, bench_my_operation);
criterion_main!(benches);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"></code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript"></code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Python Benchmark Example</h3>
        <p>Adding benchmark to Python binding benchmarks</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust"></code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">def bench_gcd_my_new_case():
    """GCD of my new test case."""
    f = mathhook.parse("x^3 - 1")
    g = mathhook.parse("x^2 - 1")
    return mathhook.gcd(f, g)

# Add to benchmarks dict:
benchmarks = {
    # ... existing benchmarks ...
    'gcd_my_new_case': bench_gcd_my_new_case,
}
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript"></code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Node.js Benchmark Example</h3>
        <p>Adding benchmark to Node.js binding benchmarks</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust"></code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"></code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const benchmarks = {
    // ... existing benchmarks ...

    gcd_my_new_case: () => {
        const f = mathhook.parse("x^3 - 1");
        const g = mathhook.parse("x^2 - 1");
        return mathhook.gcd(f, g);
    },
};
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
const metaDescription = "Comprehensive guide to MathHook's performance benchmarking infrastructure across all supported
platforms (Rust, Python, Node.js), including benchmark usage, development workflow, and
contributing new benchmarks.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'Benchmarking Guide',
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
