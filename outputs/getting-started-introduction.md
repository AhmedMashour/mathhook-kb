---









---

# Introduction to MathHook

> **Topic**: `getting-started.introduction`

MathHook is a high-performance educational computer algebra system (CAS) written in Rust,
designed to combine mathematical correctness with exceptional performance.





# Introduction

Welcome to the MathHook documentation! MathHook is a high-performance educational computer algebra system (CAS) written in Rust, designed to combine mathematical correctness with exceptional performance.

## What is MathHook?

MathHook is a symbolic mathematics engine that can:

- **Parse** mathematical expressions from multiple formats (LaTeX, Wolfram Language, standard notation)
- **Simplify** algebraic expressions using canonical forms and mathematical identities
- **Differentiate** and **integrate** expressions symbolically
- **Solve** equations and systems of equations
- **Manipulate** matrices with full linear algebra support
- **Explain** mathematical operations step-by-step for educational purposes

## Why MathHook?

### Performance-First Design

MathHook is built from the ground up for speed:

- **32-byte expression representation** fits perfectly in CPU cache lines
- **SIMD operations** for vectorized arithmetic (2-4x speedup)
- **Zero-copy parsing** directly constructs AST without intermediate allocations
- **Thread-safe immutable expressions** enable parallel processing
- **10-100x faster** than SymPy for common operations

### Mathematical Correctness

Every operation in MathHook is designed to be mathematically correct:

- Exact rational arithmetic (never loses precision)
- Proper domain handling (sqrt, log, division by zero)
- Canonical forms for reliable equality checking
- Validated against SymPy

### Educational Focus

MathHook provides step-by-step explanations for all mathematical operations, making it ideal for:

- Educational software
- Mathematics learning platforms
- Interactive mathematics tools
- Automated tutoring systems

### Multi-Language Support

MathHook provides first-class bindings for:

- **Rust** (native API with ergonomic macros)
- **Python** (via PyO3)
- **Node.js/TypeScript** (via NAPI-RS)
- **WebAssembly** (coming soon)

## Key Features

### Expression Building

Create mathematical expressions naturally using the `expr!` and `symbol!` macros.

### Symbolic Computation

Perform algebraic manipulations symbolically:
- Simplification
- Expansion
- Factorization

### Calculus Operations

Compute derivatives and integrals symbolically.

### Equation Solving

Solve equations and systems of equations.

### Matrix Operations

Full linear algebra support with symbolic and numeric matrices.

## Architecture

MathHook is organized as a multi-crate workspace:

- **mathhook-core**: Core mathematical engine (pure Rust)
- **mathhook**: High-level API with ergonomic macros
- **mathhook-python**: Python bindings
- **mathhook-node**: Node.js/TypeScript bindings
- **mathhook-benchmarks**: Performance benchmarking suite

## Design Principles

MathHook follows five core principles (in priority order):

1. **Mathematical Correctness First**: Every operation must be mathematically correct
2. **Performance**: Cache-friendly data structures, SIMD operations, parallel processing
3. **Ergonomic API**: Macros and operator overloading for natural expression
4. **Educational Value**: Step-by-step explanations for all operations
5. **Multi-Language**: First-class bindings for Python, Node.js, and WebAssembly












## Examples


### Expression Building

Create mathematical expressions using macros

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(add: (x ^ 2), (2 * x), 1);

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr

x = symbol('x')
expression = expr('x^2 + 2*x + 1')

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');
const expression = expr('x^2 + 2*x + 1');

```
</details>




### Symbolic Computation

Perform algebraic manipulations

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(add: (x ^ 2), (2 * x), 1);

let simplified = expr.simplify();
let expanded = expr.expand();
let factored = expr.factor();

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr

x = symbol('x')
expression = expr('x^2 + 2*x + 1')

simplified = expression.simplify()
expanded = expression.expand()
factored = expression.factor()

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');
const expression = expr('x^2 + 2*x + 1');

const simplified = expression.simplify();
const expanded = expression.expand();
const factored = expression.factor();

```
</details>




### Calculus Operations

Compute derivatives and integrals

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(add: (x ^ 2), (2 * x), 1);

let derivative = expr.derivative(x.clone());
let integral = expr.integrate(x, 0);

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr

x = symbol('x')
expression = expr('x^2 + 2*x + 1')

derivative = expression.derivative(x)
integral = expression.integrate(x)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');
const expression = expr('x^2 + 2*x + 1');

const derivative = expression.derivative(x);
const integral = expression.integrate(x);

```
</details>







## API Reference

- **Rust**: `mathhook`
- **Python**: `mathhook`
- **JavaScript**: `mathhook`


## See Also


- [getting-started.installation](../getting-started/installation.md)

- [getting-started.quick-start](../getting-started/quick-start.md)

- [getting-started.basic-usage](../getting-started/basic-usage.md)

- [architecture.design-principles](../architecture/design-principles.md)

- [performance.benchmarks](../performance/benchmarks.md)


