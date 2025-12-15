# PDE Module Performance Report

Comprehensive performance benchmarks for the MathHook PDE module, establishing baseline metrics
for regression detection and optimization efforts. Includes 8 benchmarks covering critical
operations from coefficient extraction to numerical integration, with detailed scalability
analysis and optimization recommendations.


---
chunk_id: advanced_pde_performance::0
topic: advanced.pde.performance
title: Benchmark Execution
priority: medium
keywords:
  - performance
  - benchmark execution
languages: [rust, python, javascript]
chunk: 1/3
---

## Benchmark Execution

Run comprehensive benchmark suite

### Rust

```rust
// Run all PDE benchmarks
cargo bench --bench pde_benchmarks

// Run specific benchmark
cargo bench --bench pde_benchmarks -- pde_coefficient_extraction

// Save baseline for future comparison
cargo bench --bench pde_benchmarks -- --save-baseline main

```

### Python

```python
# Run all PDE benchmarks
pytest benchmarks/test_pde_benchmarks.py --benchmark-only

# Run specific benchmark
pytest benchmarks/test_pde_benchmarks.py::test_coefficient_extraction --benchmark-only

# Save baseline for future comparison
pytest benchmarks/test_pde_benchmarks.py --benchmark-save=main

```

### JavaScript

```javascript
// Run all PDE benchmarks
npm run benchmark:pde

// Run specific benchmark
npm run benchmark:pde -- coefficient_extraction

// Save baseline for future comparison
npm run benchmark:pde -- --save-baseline main

```



---
chunk_id: advanced_pde_performance::1
topic: advanced.pde.performance
title: Memory Profiling
priority: medium
keywords:
  - performance
  - memory profiling
languages: [rust, python, javascript]
chunk: 2/3
---

## Memory Profiling

Profile memory allocations during PDE solving

### Rust

```rust
use dhat::{Dhat, DhatAlloc};

#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    let _dhat = Dhat::start_heap_profiling();

    // Your PDE solving code
    let pde = Pde::new(equation, u, vec![x, t]);
    let solution = method_of_characteristics(&pde);

    // Memory statistics printed on drop
}

```

### Python

```python
from memory_profiler import profile

@profile
def profile_pde_solving():
    # Your PDE solving code
    pde = Pde(equation, u, [x, t])
    solution = method_of_characteristics(pde)

if __name__ == '__main__':
    profile_pde_solving()

```

### JavaScript

```javascript
const memwatch = require('memwatch-next');

memwatch.on('stats', (stats) => {
    console.log('Memory usage:', stats);
});

// Your PDE solving code
const pde = new Pde(equation, u, [x, t]);
const solution = methodOfCharacteristics(pde);

```



---
chunk_id: advanced_pde_performance::2
topic: advanced.pde.performance
title: Performance Comparison
priority: medium
keywords:
  - performance
  - performance comparison
languages: [rust, python, javascript]
chunk: 3/3
---

## Performance Comparison

Compare MathHook performance against SymPy

### Rust

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

```

### Python

```python
import time
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

```

### JavaScript

```javascript
const { performance } = require('perf_hooks');
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

```



