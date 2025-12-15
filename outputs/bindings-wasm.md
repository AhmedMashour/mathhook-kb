---









---

# WebAssembly Bindings

> **Topic**: `bindings.wasm`

WebAssembly (WASM) bindings for MathHook, enabling browser-based symbolic mathematics
without requiring Node.js. Documentation is under development.





# WebAssembly Bindings

Language-specific documentation is under development.

For now, please refer to:
- Python: [mathhook-python README](../../crates/mathhook-python/README.md)
- Node.js: [mathhook-node README](../../crates/mathhook-node/README.md)
- [PyPI](https://pypi.org/project/mathhook/) for Python package
- [npm](https://www.npmjs.com/package/mathhook-node) for Node.js package

## Planned Features

The WebAssembly bindings will enable:

### Browser-Native Symbolic Math
- Run MathHook directly in browsers without server-side computation
- Zero dependencies on Node.js or Python runtimes
- Full symbolic computation in client-side JavaScript

### Use Cases
- **Interactive Calculators**: Build symbolic calculators that run entirely in the browser
- **Educational Tools**: Create math learning platforms with instant feedback
- **Offline Applications**: Enable offline symbolic computation in web apps
- **Mobile Web Apps**: Provide full CAS capabilities on mobile browsers

### Integration Targets
- Vanilla JavaScript (ES6+)
- React/Vue/Angular components
- Web Workers for background computation
- Progressive Web Apps (PWA)

### Performance Characteristics
- Near-native performance through WebAssembly
- Smaller bundle size compared to pure JavaScript CAS
- Efficient memory usage through Rust's ownership model
- SIMD support for numerical operations (where available)

## Current Status

**Status**: Under development

**Tracking Issue**: [GitHub Issue #XXX](https://github.com/AhmedMashour/mathhook/issues/XXX)

**Target Release**: Q2 2025

## Temporary Alternatives

While WASM bindings are under development, consider:

1. **Node.js Bindings** for server-side rendering:
   - Use Next.js/Nuxt.js server actions
   - Build REST APIs with Express/Fastify
   - See [Node.js API Guide](./nodejs.md)

2. **Python Bindings** for Jupyter/computational notebooks:
   - Use in Jupyter notebooks
   - Build Python-based web APIs (FastAPI, Flask)
   - See [Python API Guide](./python.md)

3. **Rust Direct** for maximum performance:
   - Build custom WASM modules using wasm-bindgen
   - Direct access to mathhook-core crate
   - See [mathhook-core documentation](../core/overview.md)

## Stay Updated

To receive updates on WASM bindings development:
- Watch the [GitHub repository](https://github.com/AhmedMashour/mathhook)
- Join the [Discord community](https://discord.gg/mathhook)
- Subscribe to the [newsletter](https://mathhook.dev/newsletter)












## Examples


### Planned Browser Usage (Future)

Example of how WASM bindings will be used in browsers (not yet available)

<details>
<summary><b>Rust</b></summary>

```rust
// Current Rust usage (direct)
use mathhook::{expr, symbol, simplify};

let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);
let simplified = simplify(expr);
// Result: (x + 1)^2

```
</details>

<details>
<summary><b>Python</b></summary>

```python

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
// Future WASM usage (not yet available)
import init, { Symbol, parse, simplify } from 'mathhook-wasm';

await init();

// Create symbols
const x = Symbol.create('x');

// Parse and simplify
const expr = parse('x^2 + 2*x + 1');
const simplified = simplify(expr);

console.log(simplified.toString());  // (x + 1)^2

```
</details>




### React Component Example (Future)

Planned integration with React components using WASM

<details>
<summary><b>Rust</b></summary>

```rust

```
</details>

<details>
<summary><b>Python</b></summary>

```python

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
// Future React integration (not yet available)
import React, { useState, useEffect } from 'react';
import init, { parse, simplify } from 'mathhook-wasm';

function Calculator() {
  const [initialized, setInitialized] = useState(false);
  const [input, setInput] = useState('');
  const [result, setResult] = useState('');

  useEffect(() => {
    init().then(() => setInitialized(true));
  }, []);

  const handleSimplify = () => {
    if (!initialized) return;

    try {
      const expr = parse(input);
      const simplified = simplify(expr);
      setResult(simplified.toString());
    } catch (error) {
      setResult(`Error: ${error.message}`);
    }
  };

  return (
    <div>
      <input
        value={input}
        onChange={(e) => setInput(e.target.value)}
        placeholder="Enter expression"
      />
      <button onClick={handleSimplify} disabled={!initialized}>
        Simplify
      </button>
      {result && <div>Result: {result}</div>}
    </div>
  );
}

```
</details>




### Web Worker Integration (Future)

Offload heavy symbolic computation to Web Workers

<details>
<summary><b>Rust</b></summary>

```rust

```
</details>

<details>
<summary><b>Python</b></summary>

```python

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
// Future Web Worker usage (not yet available)
// worker.js
import init, { parse, simplify, derivative } from 'mathhook-wasm';

await init();

self.onmessage = (e) => {
  const { operation, expression, variable } = e.data;

  try {
    const expr = parse(expression);
    let result;

    switch (operation) {
      case 'simplify':
        result = simplify(expr);
        break;
      case 'derivative':
        const x = Symbol.create(variable);
        result = derivative(expr, x);
        break;
    }

    self.postMessage({ success: true, result: result.toString() });
  } catch (error) {
    self.postMessage({ success: false, error: error.message });
  }
};

// main.js
const worker = new Worker('worker.js', { type: 'module' });

worker.postMessage({
  operation: 'simplify',
  expression: 'x^2 + 2*x + 1'
});

worker.onmessage = (e) => {
  console.log('Result:', e.data.result);
};

```
</details>







## API Reference

- **Rust**: `mathhook`
- **Python**: `mathhook`
- **JavaScript**: `mathhook`


## See Also


- [bindings.python](../bindings/python.md)

- [bindings.nodejs](../bindings/nodejs.md)

- [getting-started.installation](../getting-started/installation.md)

- [core.overview](../core/overview.md)


