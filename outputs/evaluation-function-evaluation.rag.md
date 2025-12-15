# Function Evaluation

MathHook provides a unified, intelligent function evaluation system that handles both symbolic
and numerical computation. The system uses the Universal Function Registry architecture to
dispatch function calls to specialized implementations while maintaining mathematical correctness.


---
chunk_id: evaluation_function-evaluation::0
topic: evaluation.function-evaluation
title: Elementary Functions
priority: medium
keywords:
  - function-evaluation
  - elementary functions
languages: [rust, python, javascript]
chunk: 1/5
---

## Elementary Functions

Evaluating basic trigonometric and exponential functions

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);

let sin_x = expr!(sin(x));
let cos_x = expr!(cos(x));
let exp_x = expr!(exp(x));
let log_x = expr!(log(x));

```

### Python

```python
from mathhook import symbol, expr

x = symbol('x')

sin_x = expr('sin(x)')
cos_x = expr('cos(x)')
exp_x = expr('exp(x)')
log_x = expr('log(x)')

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');

const sinX = expr('sin(x)');
const cosX = expr('cos(x)');
const expX = expr('exp(x)');
const logX = expr('log(x)');

```



---
chunk_id: evaluation_function-evaluation::1
topic: evaluation.function-evaluation
title: Special Value Evaluation
priority: medium
keywords:
  - function-evaluation
  - special value evaluation
languages: [rust, python, javascript]
chunk: 2/5
---

## Special Value Evaluation

Automatic simplification of known exact values

### Rust

```rust
use mathhook::prelude::*;

// Trigonometric special values
let sin_0 = expr!(sin(0));
assert_eq!(sin_0.simplify(), expr!(0));

let cos_0 = expr!(cos(0));
assert_eq!(cos_0.simplify(), expr!(1));

// Exponential and logarithmic
let exp_0 = expr!(exp(0));
assert_eq!(exp_0.simplify(), expr!(1));

let log_1 = expr!(log(1));
assert_eq!(log_1.simplify(), expr!(0));

```

### Python

```python
from mathhook import expr

# Trigonometric special values
sin_0 = expr('sin(0)')
assert sin_0.simplify() == expr('0')

cos_0 = expr('cos(0)')
assert cos_0.simplify() == expr('1')

# Exponential and logarithmic
exp_0 = expr('exp(0)')
assert exp_0.simplify() == expr('1')

log_1 = expr('log(1)')
assert log_1.simplify() == expr('0')

```

### JavaScript

```javascript
const { expr } = require('mathhook');

// Trigonometric special values
const sin0 = expr('sin(0)');
console.assert(sin0.simplify().equals(expr('0')));

const cos0 = expr('cos(0)');
console.assert(cos0.simplify().equals(expr('1')));

// Exponential and logarithmic
const exp0 = expr('exp(0)');
console.assert(exp0.simplify().equals(expr('1')));

const log1 = expr('log(1)');
console.assert(log1.simplify().equals(expr('0')));

```



---
chunk_id: evaluation_function-evaluation::2
topic: evaluation.function-evaluation
title: Composite Expression Evaluation
priority: medium
keywords:
  - function-evaluation
  - composite expression evaluation
languages: [rust, python, javascript]
chunk: 3/5
---

## Composite Expression Evaluation

Mixed symbolic and numeric evaluation

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// sqrt(4) evaluates to 2, symbolic parts preserved
let composite = expr!((sin((x^2) + 1) * cos(y)) - sqrt(4));
let result = composite.simplify();
// Result: sin(x^2 + 1) * cos(y) - 2

```

### Python

```python
from mathhook import symbol, expr

x = symbol('x')
y = symbol('y')

# sqrt(4) evaluates to 2, symbolic parts preserved
composite = expr('sin(x^2 + 1) * cos(y) - sqrt(4)')
result = composite.simplify()
# Result: sin(x^2 + 1) * cos(y) - 2

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');
const y = symbol('y');

// sqrt(4) evaluates to 2, symbolic parts preserved
const composite = expr('sin(x^2 + 1) * cos(y) - sqrt(4)');
const result = composite.simplify();
// Result: sin(x^2 + 1) * cos(y) - 2

```



---
chunk_id: evaluation_function-evaluation::3
topic: evaluation.function-evaluation
title: Function Composition
priority: medium
keywords:
  - function-evaluation
  - function composition
languages: [rust, python, javascript]
chunk: 4/5
---

## Function Composition

Nested and composed functions

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// sin(cos(x))
let nested = expr!(sin(cos(x)));

// exp(log(x)) simplifies to x
let exp_log = expr!(exp(log(x)));
let simplified = exp_log.simplify();
// Result: x (identity simplification)

```

### Python

```python
from mathhook import symbol, expr

x = symbol('x')

# sin(cos(x))
nested = expr('sin(cos(x))')

# exp(log(x)) simplifies to x
exp_log = expr('exp(log(x))')
simplified = exp_log.simplify()
# Result: x

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');

// sin(cos(x))
const nested = expr('sin(cos(x))');

// exp(log(x)) simplifies to x
const expLog = expr('exp(log(x))');
const simplified = expLog.simplify();
// Result: x

```



---
chunk_id: evaluation_function-evaluation::4
topic: evaluation.function-evaluation
title: Bulk Evaluation
priority: medium
keywords:
  - function-evaluation
  - bulk evaluation
languages: [rust, python, javascript]
chunk: 5/5
---

## Bulk Evaluation

Efficient numerical evaluation over multiple points

### Rust

```rust
use mathhook::functions::FunctionEvaluator;

let evaluator = FunctionEvaluator::new();
let points = vec![0.0, 0.5, 1.0, 1.5, 2.0];

// SIMD-optimized evaluation
if let Some(results) = evaluator.evaluate_bulk_f64("sin", &points) {
    println!("Results: {:?}", results);
}

```

### Python

```python
from mathhook.functions import FunctionEvaluator

evaluator = FunctionEvaluator()
points = [0.0, 0.5, 1.0, 1.5, 2.0]

# SIMD-optimized evaluation
results = evaluator.evaluate_bulk('sin', points)
print(f"Results: {results}")

```

### JavaScript

```javascript
const { FunctionEvaluator } = require('mathhook/functions');

const evaluator = new FunctionEvaluator();
const points = [0.0, 0.5, 1.0, 1.5, 2.0];

// SIMD-optimized evaluation
const results = evaluator.evaluateBulk('sin', points);
console.log(`Results: ${results}`);

```



