# Performance Comparison

MathHook is designed for high performance while maintaining mathematical correctness.
This page shows how MathHook compares to other computer algebra systems, particularly SymPy.


---
chunk_id: performance_comparison::0
topic: performance.comparison
title: Simple Performance Test
priority: medium
keywords:
  - comparison
  - simple performance test
languages: [rust, python, javascript]
chunk: 1/2
---

## Simple Performance Test

Benchmark derivative computation performance

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: performance_comparison::1
topic: performance.comparison
title: Compare Operations
priority: medium
keywords:
  - comparison
  - compare operations
languages: [rust, python, javascript]
chunk: 2/2
---

## Compare Operations

Compare different calculus operation speeds

### Rust

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

### Python

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

### JavaScript

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



