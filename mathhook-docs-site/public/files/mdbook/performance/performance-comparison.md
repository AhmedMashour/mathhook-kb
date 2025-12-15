---









---

# Performance Comparison

> **Topic**: `performance.comparison`

MathHook is designed for high performance while maintaining mathematical correctness.
This page shows how MathHook compares to other computer algebra systems, particularly SymPy.





# Performance Comparison

MathHook is designed for high performance while maintaining mathematical correctness. This page shows how MathHook compares to other computer algebra systems.

## MathHook vs SymPy

SymPy is the most widely used Python computer algebra system. MathHook, written in Rust, provides significant performance improvements while maintaining compatibility.

### Derivative Performance

| Operation | SymPy | MathHook | Speedup |
|-----------|-------|----------|---------|
| d/dx(x^2) | 54.9ms | 0.035ms | **1580x** |
| d/dx(x^3) | 2.5ms | 0.019ms | **131x** |
| d/dx(x^4) | 3.3ms | 0.017ms | **189x** |
| d/dx(sin(x)) | 1.6ms | 0.048ms | **34x** |
| d/dx(cos(x)) | 1.6ms | 0.038ms | **42x** |
| d/dx(tan(x)) | 3.5ms | 0.035ms | **101x** |
| d/dx(exp(x)) | 1.8ms | 0.049ms | **36x** |
| d/dx(x*sin(x)) | 2.6ms | 0.079ms | **34x** |
| d/dx(sin(x)/x) | 7.3ms | 0.129ms | **56x** |
| d/dx(sin(x^2)) | 2.4ms | 0.042ms | **57x** |
| d/dx(x^2+2x+1) | 2.7ms | 0.014ms | **190x** |
| d/dx(x^2*exp(x)) | 3.3ms | 0.106ms | **31x** |
| d/dx(1/x) | 1.0ms | 0.006ms | **167x** |

### Summary Statistics

| Metric | Value |
|--------|-------|
| Minimum Speedup | 17x |
| Maximum Speedup | 1580x |
| Average Speedup | **179x** |
| Correctness Rate | 86.7% |

### Why MathHook is Faster

1. **Compiled Language**: Rust compiles to native machine code, eliminating interpreter overhead
2. **Zero-Cost Abstractions**: Rust's ownership system enables efficient memory management without garbage collection
3. **Cache-Optimized Structures**: Expression type is exactly 32 bytes, fitting two expressions per CPU cache line
4. **SIMD Operations**: Vectorized operations for numerical evaluation
5. **Lazy Evaluation**: Computations are deferred until results are needed

## Core Operation Performance

Based on Criterion benchmarks:

| Operation | Time | Target |
|-----------|------|--------|
| Expression Creation | ~159 ns | < 200 ns |
| Basic Simplification | ~8 ns | < 50 ns |
| Expression with Parsing | ~441 us | < 1 ms |

## Running Benchmarks

### Quick Benchmark

```bash
./scripts/bench.sh quick
```

### Full Benchmark Suite

```bash
./scripts/bench.sh run
```

### Specific Benchmark Groups

```bash
./scripts/bench.sh rust core_performance
./scripts/bench.sh rust calculus_benchmarks
./scripts/bench.sh rust polynomial_benchmarks
./scripts/bench.sh rust simplification_benchmarks
```

### Compare Against Baseline

```bash
# Save current as baseline
./scripts/bench.sh save my-baseline

# Run and compare
./scripts/bench.sh compare my-baseline
```

## Validation Against SymPy

MathHook includes validation scripts to verify mathematical correctness against SymPy:

```bash
./scripts/validate.sh           # Run all validations
./scripts/validate.sh simplify  # Simplification only
./scripts/validate.sh ode       # ODE solver only
```

## Continuous Integration

Performance is monitored in CI:
- Benchmarks run on every PR
- Regressions > 10% fail the build
- Results are posted as PR comments
- Baselines are updated on merge to main

See the [benchmarking guide](./benchmarking.md) for detailed information.












## Examples


### Simple Performance Test

Benchmark derivative computation performance


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use std::time::Instant;

let x = symbol!(x);
let expr = expr!(x ^ 2);

let start = Instant::now();
for _ in 0..1000 {
    let _ = expr.derivative(&x, 1);
}
let duration = start.elapsed();
println!("1000 derivatives: {:?}", duration);

```
</details>



<details>
<summary><b>Python</b></summary>

```python
import mathhook
import time

x = mathhook.symbol('x')
expr = mathhook.expr('x^2')

start = time.time()
for _ in range(1000):
    expr.derivative(x)
duration = time.time() - start
print(f"1000 derivatives: {duration:.3f}s")

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');
const expression = expr('x^2');

const start = Date.now();
for (let i = 0; i < 1000; i++) {
    expression.derivative(x);
}
const duration = Date.now() - start;
console.log(`1000 derivatives: ${duration}ms`);

```
</details>





### Compare Operations

Compare different calculus operation speeds


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;
use std::time::Instant;

let x = symbol!(x);
let expr = expr!((x ^ 2) * sin(x));

// Time derivative
let start = Instant::now();
let deriv = expr.derivative(&x, 1);
println!("Derivative time: {:?}", start.elapsed());

// Time simplification
let start = Instant::now();
let simplified = deriv.simplify();
println!("Simplify time: {:?}", start.elapsed());

```
</details>



<details>
<summary><b>Python</b></summary>

```python
import mathhook
import time

x = mathhook.symbol('x')
expr = mathhook.expr('x^2 * sin(x)')

# Time derivative
start = time.time()
deriv = expr.derivative(x)
print(f"Derivative time: {(time.time() - start)*1000:.3f}ms")

# Time simplification
start = time.time()
simplified = deriv.simplify()
print(f"Simplify time: {(time.time() - start)*1000:.3f}ms")

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');
const expression = expr('x^2 * sin(x)');

// Time derivative
let start = Date.now();
const deriv = expression.derivative(x);
console.log(`Derivative time: ${Date.now() - start}ms`);

// Time simplification
start = Date.now();
const simplified = deriv.simplify();
console.log(`Simplify time: ${Date.now() - start}ms`);

```
</details>








## API Reference

- **Rust**: `mathhook_core`
- **Python**: `mathhook`
- **JavaScript**: `mathhook-node`


## See Also


- [performance.benchmarking](../performance/benchmarking.md)

- [performance.architecture](../performance/architecture.md)

- [performance.simd](../performance/simd.md)

- [performance.caching](../performance/caching.md)


