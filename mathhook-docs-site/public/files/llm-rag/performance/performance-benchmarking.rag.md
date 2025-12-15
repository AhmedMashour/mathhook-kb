# Benchmarking Guide

Comprehensive guide to MathHook's performance benchmarking infrastructure across all supported
platforms (Rust, Python, Node.js), including benchmark usage, development workflow, and
contributing new benchmarks.


---
chunk_id: performance_benchmarking::0
topic: performance.benchmarking
title: Rust Criterion Benchmark Template
priority: medium
keywords:
  - benchmarking
  - rust criterion benchmark template
languages: [rust, python, javascript]
chunk: 1/3
---

## Rust Criterion Benchmark Template

Template for adding new benchmarks in Rust

### Rust

```rust
use criterion::{criterion_group, criterion_main, Criterion};
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

```

### Python

```python

```

### JavaScript

```javascript

```



---
chunk_id: performance_benchmarking::1
topic: performance.benchmarking
title: Python Benchmark Example
priority: medium
keywords:
  - benchmarking
  - python benchmark example
languages: [rust, python, javascript]
chunk: 2/3
---

## Python Benchmark Example

Adding benchmark to Python binding benchmarks

### Rust

```rust

```

### Python

```python
def bench_gcd_my_new_case():
    """GCD of my new test case."""
    f = mathhook.parse("x^3 - 1")
    g = mathhook.parse("x^2 - 1")
    return mathhook.gcd(f, g)

# Add to benchmarks dict:
benchmarks = {
    # ... existing benchmarks ...
    'gcd_my_new_case': bench_gcd_my_new_case,
}

```

### JavaScript

```javascript

```



---
chunk_id: performance_benchmarking::2
topic: performance.benchmarking
title: Node.js Benchmark Example
priority: medium
keywords:
  - benchmarking
  - node.js benchmark example
languages: [rust, python, javascript]
chunk: 3/3
---

## Node.js Benchmark Example

Adding benchmark to Node.js binding benchmarks

### Rust

```rust

```

### Python

```python

```

### JavaScript

```javascript
const benchmarks = {
    // ... existing benchmarks ...

    gcd_my_new_case: () => {
        const f = mathhook.parse("x^3 - 1");
        const g = mathhook.parse("x^2 - 1");
        return mathhook.gcd(f, g);
    },
};

```



