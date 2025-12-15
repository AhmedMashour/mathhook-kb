<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>PDE Module Performance Report</h1>
      <p class="description">Comprehensive performance benchmarks for the MathHook PDE module, establishing baseline metrics
for regression detection and optimization efforts. Includes 8 benchmarks covering critical
operations from coefficient extraction to numerical integration, with detailed scalability
analysis and optimization recommendations.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">Performance characteristics of key operations:

**Coefficient Extraction**: $$O(1)$$ - constant-time for simplified coefficients

**ODE System Construction**: $$O(1)$$ - fixed three equations

**Numerical Integration**: $$O(n/h)$$ where $$n$$ = interval length, $$h$$ = step size

**Memory Overhead**: Expression size = 32 bytes, Number size = 16 bytes (hard constraints)
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Benchmark Execution</h3>
        <p>Run comprehensive benchmark suite</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">// Run all PDE benchmarks
cargo bench --bench pde_benchmarks

// Run specific benchmark
cargo bench --bench pde_benchmarks -- pde_coefficient_extraction

// Save baseline for future comparison
cargo bench --bench pde_benchmarks -- --save-baseline main
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python"># Run all PDE benchmarks
pytest benchmarks/test_pde_benchmarks.py --benchmark-only

# Run specific benchmark
pytest benchmarks/test_pde_benchmarks.py::test_coefficient_extraction --benchmark-only

# Save baseline for future comparison
pytest benchmarks/test_pde_benchmarks.py --benchmark-save=main
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">// Run all PDE benchmarks
npm run benchmark:pde

// Run specific benchmark
npm run benchmark:pde -- coefficient_extraction

// Save baseline for future comparison
npm run benchmark:pde -- --save-baseline main
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Memory Profiling</h3>
        <p>Profile memory allocations during PDE solving</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use dhat::{Dhat, DhatAlloc};

#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    let _dhat = Dhat::start_heap_profiling();

    // Your PDE solving code
    let pde = Pde::new(equation, u, vec![x, t]);
    let solution = method_of_characteristics(&pde);

    // Memory statistics printed on drop
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">from memory_profiler import profile

@profile
def profile_pde_solving():
    # Your PDE solving code
    pde = Pde(equation, u, [x, t])
    solution = method_of_characteristics(pde)

if __name__ == '__main__':
    profile_pde_solving()
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const memwatch = require('memwatch-next');

memwatch.on('stats', (stats) => {
    console.log('Memory usage:', stats);
});

// Your PDE solving code
const pde = new Pde(equation, u, [x, t]);
const solution = methodOfCharacteristics(pde);
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Performance Comparison</h3>
        <p>Compare MathHook performance against SymPy</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_mathhook_vs_sympy(c: &mut Criterion) {
    let mut group = c.benchmark_group("mathhook_vs_sympy");

    // MathHook benchmark
    group.bench_function("mathhook_transport", |b| {
        b.iter(|| {
            let pde = Pde::new(black_box(equation), u, vec![x, t]);
            method_of_characteristics(&pde)
        });
    });

    // SymPy benchmark (via Python binding)
    group.bench_function("sympy_transport", |b| {
        b.iter(|| {
            sympy_solve_transport(black_box(&equation))
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_mathhook_vs_sympy);
criterion_main!(benches);
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">import time
import sympy as sp
from mathhook import Pde, method_of_characteristics

def benchmark_comparison():
    # MathHook timing
    start = time.perf_counter()
    for _ in range(1000):
        pde = Pde(equation, u, [x, t])
        method_of_characteristics(pde)
    mathhook_time = time.perf_counter() - start

    # SymPy timing
    start = time.perf_counter()
    for _ in range(1000):
        sp.pdsolve(equation, u)
    sympy_time = time.perf_counter() - start

    print(f"MathHook: {mathhook_time:.4f}s")
    print(f"SymPy: {sympy_time:.4f}s")
    print(f"Speedup: {sympy_time/mathhook_time:.2f}x")
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">const { performance } = require('perf_hooks');
const { Pde, methodOfCharacteristics } = require('mathhook');

function benchmarkComparison() {
    // MathHook timing
    const startMathhook = performance.now();
    for (let i = 0; i < 1000; i++) {
        const pde = new Pde(equation, u, [x, t]);
        methodOfCharacteristics(pde);
    }
    const mathhookTime = performance.now() - startMathhook;

    // SymPy timing (via Python subprocess)
    const startSympy = performance.now();
    for (let i = 0; i < 1000; i++) {
        sympySolveTransport(equation);
    }
    const sympyTime = performance.now() - startSympy;

    console.log(`MathHook: ${mathhookTime.toFixed(4)}ms`);
    console.log(`SymPy: ${sympyTime.toFixed(4)}ms`);
    console.log(`Speedup: ${(sympyTime/mathhookTime).toFixed(2)}x`);
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
const metaDescription = "Comprehensive performance benchmarks for the MathHook PDE module, establishing baseline metrics
for regression detection and optimization efforts. Includes 8 benchmarks covering critical
operations from coefficient extraction to numerical integration, with detailed scalability
analysis and optimization recommendations.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'PDE Module Performance Report',
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
