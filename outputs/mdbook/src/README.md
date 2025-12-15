# MathHook Documentation

Welcome to the MathHook Computer Algebra System documentation!

MathHook is a high-performance symbolic mathematics library written in Rust, with bindings for Python and JavaScript.

## Features

- **Symbolic Differentiation & Integration**
- **ODE & PDE Solvers**
- **Matrix Operations**
- **Special Functions**
- **Expression Simplification**
- **LaTeX Parsing & Rendering**

## Quick Example

```rust
use mathhook_core::{symbol, expr};

let x = symbol!(x);
let f = expr!(x ^ 2 + 3 * x + 2);
let df = f.derivative(&x, 1);
// Result: 2*x + 3
```

## Getting Started

Check out the [Quick Start Guide](./getting-started/quick-start.md) to begin using MathHook.
