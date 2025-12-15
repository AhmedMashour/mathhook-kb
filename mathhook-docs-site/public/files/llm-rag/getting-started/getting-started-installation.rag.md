# Installation

Complete installation guide for MathHook across Rust, Python, and Node.js platforms,
including platform-specific requirements, troubleshooting, and optional features.


---
chunk_id: getting-started_installation::0
topic: getting-started.installation
title: Rust Installation Verification
priority: medium
keywords:
  - installation
  - rust installation verification
languages: [rust, python, javascript]
chunk: 1/3
---

## Rust Installation Verification

Verify Rust installation with a simple test program

### Rust

```rust
use mathhook::prelude::*;

fn main() {
    let x = symbol!(x);
    let expr = expr!(x ^ 2);
    println!("Expression: {}", expr);
}

```

### Python

```python
from mathhook import Expression

x = Expression.symbol('x')
expr = x.pow(2)
print(f"Expression: {expr}")

```

### JavaScript

```javascript
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const expr = x.pow(2);
console.log(`Expression: ${expr.toString()}`);

```



---
chunk_id: getting-started_installation::1
topic: getting-started.installation
title: Python Virtual Environment Setup
priority: medium
keywords:
  - installation
  - python virtual environment setup
languages: [rust, python, javascript]
chunk: 2/3
---

## Python Virtual Environment Setup

Best practice for Python installation using virtual environments

### Rust

```rust
# Not applicable for Rust

```

### Python

```python
# Create and activate virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install mathhook

```

### JavaScript

```javascript
// Not applicable for Node.js
// Use npm or yarn directly

```



---
chunk_id: getting-started_installation::2
topic: getting-started.installation
title: Building from Source
priority: medium
keywords:
  - installation
  - building from source
languages: [rust, python, javascript]
chunk: 3/3
---

## Building from Source

Complete source build for all platforms

### Rust

```rust
# Clone and build Rust core
git clone https://github.com/AhmedMashour/mathhook.git
cd mathhook
cargo build --release
cargo test

```

### Python

```python
# Build Python bindings from source
cd crates/mathhook-python
pip install maturin
maturin develop --release

```

### JavaScript

```javascript
// Build Node.js bindings from source
cd crates/mathhook-node
npm install
npm run build

```



