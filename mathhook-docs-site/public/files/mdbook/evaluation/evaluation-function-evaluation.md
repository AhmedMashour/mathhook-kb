---









---

# Function Evaluation

> **Topic**: `evaluation.function-evaluation`

MathHook provides a unified, intelligent function evaluation system that handles both symbolic
and numerical computation. The system uses the Universal Function Registry architecture to
dispatch function calls to specialized implementations while maintaining mathematical correctness.





# Function Evaluation

MathHook provides a unified, intelligent function evaluation system that handles both symbolic and numerical computation. The system uses the **Universal Function Registry** architecture to dispatch function calls to specialized implementations while maintaining mathematical correctness.

## Overview

Function evaluation in MathHook supports:

- **Elementary functions**: sin, cos, tan, exp, log, sqrt, abs, and their inverses
- **Hyperbolic functions**: sinh, cosh, tanh, and their inverses
- **Special functions**: gamma, zeta, bessel functions
- **Number theory functions**: factorial, binomial coefficients
- **Symbolic evaluation**: Returns exact symbolic results when possible
- **Numerical evaluation**: High-performance numerical approximations
- **Special value recognition**: Automatically simplifies known exact values

## Evaluation Architecture

### Function Intelligence System

Every function in MathHook has associated **intelligence properties** that define:

1. **Domain and Range**: Where the function is defined and what values it can produce
2. **Special Values**: Known exact values (e.g., sin(0) = 0, gamma(1) = 1)
3. **Evaluation Strategy**: How to compute the function symbolically and numerically
4. **Mathematical Properties**: Symmetry, periodicity, derivative rules, etc.

### Evaluation Flow

```
User Expression
      ↓
Function Name + Arguments
      ↓
Universal Registry Lookup
      ↓
Function Properties Dispatch
      ↓
┌─────────────────┬──────────────────┐
│ Special Value?  │ Symbolic Input?  │ Numerical Input?
│ → Exact Result  │ → Keep Symbolic  │ → Numerical Eval
└─────────────────┴──────────────────┘
```

## Performance Characteristics

The function evaluation system is designed for high performance:

- **Registry lookup**: O(1) constant time using hash maps
- **Special value detection**: <50ns for known values
- **Numerical evaluation**: <100ns for elementary functions
- **Total dispatch overhead**: <10ns
- **Bulk evaluation**: SIMD-optimized for arrays of values

## Mathematical Correctness Guarantees

MathHook's function evaluation system provides:

1. **Exact symbolic computation**: Special values return exact results (not floating-point approximations)
2. **Domain checking**: Functions respect their mathematical domains (e.g., log requires positive inputs)
3. **SymPy validation**: All implementations validated against SymPy reference
4. **Numerical stability**: Algorithms chosen for numerical accuracy












## Examples


### Elementary Functions

Evaluating basic trigonometric and exponential functions


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);

let sin_x = expr!(sin(x));
let cos_x = expr!(cos(x));
let exp_x = expr!(exp(x));
let log_x = expr!(log(x));

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr

x = symbol('x')

sin_x = expr('sin(x)')
cos_x = expr('cos(x)')
exp_x = expr('exp(x)')
log_x = expr('log(x)')

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');

const sinX = expr('sin(x)');
const cosX = expr('cos(x)');
const expX = expr('exp(x)');
const logX = expr('log(x)');

```
</details>





### Special Value Evaluation

Automatic simplification of known exact values


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

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
</details>



<details>
<summary><b>JavaScript</b></summary>

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
</details>





### Composite Expression Evaluation

Mixed symbolic and numeric evaluation


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// sqrt(4) evaluates to 2, symbolic parts preserved
let composite = expr!((sin((x^2) + 1) * cos(y)) - sqrt(4));
let result = composite.simplify();
// Result: sin(x^2 + 1) * cos(y) - 2

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr

x = symbol('x')
y = symbol('y')

# sqrt(4) evaluates to 2, symbolic parts preserved
composite = expr('sin(x^2 + 1) * cos(y) - sqrt(4)')
result = composite.simplify()
# Result: sin(x^2 + 1) * cos(y) - 2

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');
const y = symbol('y');

// sqrt(4) evaluates to 2, symbolic parts preserved
const composite = expr('sin(x^2 + 1) * cos(y) - sqrt(4)');
const result = composite.simplify();
// Result: sin(x^2 + 1) * cos(y) - 2

```
</details>





### Function Composition

Nested and composed functions


<details>
<summary><b>Rust</b></summary>

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
</details>



<details>
<summary><b>Python</b></summary>

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
</details>



<details>
<summary><b>JavaScript</b></summary>

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
</details>





### Bulk Evaluation

Efficient numerical evaluation over multiple points


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::functions::FunctionEvaluator;

let evaluator = FunctionEvaluator::new();
let points = vec![0.0, 0.5, 1.0, 1.5, 2.0];

// SIMD-optimized evaluation
if let Some(results) = evaluator.evaluate_bulk_f64("sin", &points) {
    println!("Results: {:?}", results);
}

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook.functions import FunctionEvaluator

evaluator = FunctionEvaluator()
points = [0.0, 0.5, 1.0, 1.5, 2.0]

# SIMD-optimized evaluation
results = evaluator.evaluate_bulk('sin', points)
print(f"Results: {results}")

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { FunctionEvaluator } = require('mathhook/functions');

const evaluator = new FunctionEvaluator();
const points = [0.0, 0.5, 1.0, 1.5, 2.0];

// SIMD-optimized evaluation
const results = evaluator.evaluateBulk('sin', points);
console.log(`Results: ${results}`);

```
</details>








## API Reference

- **Rust**: `mathhook_core::functions::evaluation`
- **Python**: `mathhook.functions.evaluation`
- **JavaScript**: `mathhook.functions.evaluation`


## See Also


- [architecture.function-intelligence](../architecture/function-intelligence.md)

- [advanced.special-functions](../advanced/special-functions.md)

- [performance.benchmarking](../performance/benchmarking.md)

- [calculus.derivatives](../calculus/derivatives.md)

- [simplification.algebraic](../simplification/algebraic.md)


