---









---

# Installation

> **Topic**: `getting-started.installation`

Complete installation guide for MathHook across Rust, Python, and Node.js platforms,
including platform-specific requirements, troubleshooting, and optional features.





# Installation

This guide covers installation of MathHook for Rust, Python, and Node.js.

## Rust

### Requirements

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Adding to Your Project

Add MathHook to your `Cargo.toml`:

```toml
[dependencies]
mathhook-core = "0.1"
```

For the high-level API with ergonomic macros:

```toml
[dependencies]
mathhook = "0.1"
```

### Verifying Installation

Create a simple test program and run with `cargo run`.

## Python

### Requirements

- Python 3.8 or higher
- pip

### Installing via pip

```bash
pip install mathhook
```

### Installing from Source

For the latest development version, clone the repository and use maturin.

### Virtual Environments

We recommend using a virtual environment for Python installations.

## Node.js/TypeScript

### Requirements

- Node.js 18 or higher
- npm or yarn

### Installing via npm

```bash
npm install mathhook-node
```

Or with yarn:

```bash
yarn add mathhook-node
```

## Building from Source

### Prerequisites

- Rust toolchain (rustup recommended)
- Git
- For Python bindings: Python 3.8+, maturin
- For Node.js bindings: Node.js 18+, npm

### Platform-Specific Notes

- **Windows**: Requires Visual Studio Build Tools
- **macOS**: Requires XCode Command Line Tools
- **Linux**: Requires GCC/Clang and python3-dev for Python bindings

## Optional Dependencies

- **SIMD Support**: Auto-detected, or enable with `features = ["simd"]`
- **Parallel Processing**: Enable with `features = ["parallel"]`

## Troubleshooting

- **Rust**: LALRPOP errors → `cargo install lalrpop && cargo clean && cargo build`
- **Python**: Import errors → `pip install --force-reinstall mathhook`
- **Node.js**: Native module errors → `npm rebuild mathhook-node`












## Examples


### Rust Installation Verification

Verify Rust installation with a simple test program


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

fn main() {
    let x = symbol!(x);
    let expr = expr!(x ^ 2);
    println!("Expression: {}", expr);
}

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import Expression

x = Expression.symbol('x')
expr = x.pow(2)
print(f"Expression: {expr}")

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const expr = x.pow(2);
console.log(`Expression: ${expr.toString()}`);

```
</details>





### Python Virtual Environment Setup

Best practice for Python installation using virtual environments


<details>
<summary><b>Rust</b></summary>

```rust
# Not applicable for Rust

```
</details>



<details>
<summary><b>Python</b></summary>

```python
# Create and activate virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install mathhook

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
// Not applicable for Node.js
// Use npm or yarn directly

```
</details>





### Building from Source

Complete source build for all platforms


<details>
<summary><b>Rust</b></summary>

```rust
# Clone and build Rust core
git clone https://github.com/AhmedMashour/mathhook.git
cd mathhook
cargo build --release
cargo test

```
</details>



<details>
<summary><b>Python</b></summary>

```python
# Build Python bindings from source
cd crates/mathhook-python
pip install maturin
maturin develop --release

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
// Build Node.js bindings from source
cd crates/mathhook-node
npm install
npm run build

```
</details>








## API Reference

- **Rust**: `mathhook::prelude`
- **Python**: `mathhook`
- **JavaScript**: `mathhook-node`


## See Also


- [getting-started.quick-start](../getting-started/quick-start.md)

- [getting-started.basic-usage](../getting-started/basic-usage.md)

- [bindings.python](../bindings/python.md)

- [bindings.nodejs](../bindings/nodejs.md)


