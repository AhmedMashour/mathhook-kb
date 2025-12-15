# WebAssembly Bindings

WebAssembly (WASM) bindings for MathHook, enabling browser-based symbolic mathematics
without requiring Node.js. Documentation is under development.


---
chunk_id: bindings_wasm::0
topic: bindings.wasm
title: Planned Browser Usage (Future)
priority: medium
keywords:
  - wasm
  - planned browser usage (future)
languages: [rust, python, javascript]
chunk: 1/3
---

## Planned Browser Usage (Future)

Example of how WASM bindings will be used in browsers (not yet available)

### Rust

```rust
// Current Rust usage (direct)
use mathhook::{expr, symbol, simplify};

let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);
let simplified = simplify(expr);
// Result: (x + 1)^2

```

### Python

```python

```

### JavaScript

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



---
chunk_id: bindings_wasm::1
topic: bindings.wasm
title: React Component Example (Future)
priority: medium
keywords:
  - wasm
  - react component example (future)
languages: [rust, python, javascript]
chunk: 2/3
---

## React Component Example (Future)

Planned integration with React components using WASM

### Rust

```rust

```

### Python

```python

```

### JavaScript

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



---
chunk_id: bindings_wasm::2
topic: bindings.wasm
title: Web Worker Integration (Future)
priority: medium
keywords:
  - wasm
  - web worker integration (future)
languages: [rust, python, javascript]
chunk: 3/3
---

## Web Worker Integration (Future)

Offload heavy symbolic computation to Web Workers

### Rust

```rust

```

### Python

```python

```

### JavaScript

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



