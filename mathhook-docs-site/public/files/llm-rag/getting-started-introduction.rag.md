# Introduction to MathHook

MathHook is a high-performance educational computer algebra system (CAS) written in Rust,
designed to combine mathematical correctness with exceptional performance.


---
chunk_id: getting-started_introduction::0
topic: getting-started.introduction
title: Expression Building
priority: medium
keywords:
  - introduction
  - expression building
languages: [rust, python, javascript]
chunk: 1/3
---

## Expression Building

Create mathematical expressions using macros

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(add: (x ^ 2), (2 * x), 1);

```

### Python

```python
from mathhook import symbol, expr

x = symbol('x')
expression = expr('x^2 + 2*x + 1')

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');
const expression = expr('x^2 + 2*x + 1');

```



---
chunk_id: getting-started_introduction::1
topic: getting-started.introduction
title: Symbolic Computation
priority: medium
keywords:
  - introduction
  - symbolic computation
languages: [rust, python, javascript]
chunk: 2/3
---

## Symbolic Computation

Perform algebraic manipulations

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(add: (x ^ 2), (2 * x), 1);

let simplified = expr.simplify();
let expanded = expr.expand();
let factored = expr.factor();

```

### Python

```python
from mathhook import symbol, expr

x = symbol('x')
expression = expr('x^2 + 2*x + 1')

simplified = expression.simplify()
expanded = expression.expand()
factored = expression.factor()

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');
const expression = expr('x^2 + 2*x + 1');

const simplified = expression.simplify();
const expanded = expression.expand();
const factored = expression.factor();

```



---
chunk_id: getting-started_introduction::2
topic: getting-started.introduction
title: Calculus Operations
priority: medium
keywords:
  - introduction
  - calculus operations
languages: [rust, python, javascript]
chunk: 3/3
---

## Calculus Operations

Compute derivatives and integrals

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(add: (x ^ 2), (2 * x), 1);

let derivative = expr.derivative(x.clone());
let integral = expr.integrate(x, 0);

```

### Python

```python
from mathhook import symbol, expr

x = symbol('x')
expression = expr('x^2 + 2*x + 1')

derivative = expression.derivative(x)
integral = expression.integrate(x)

```

### JavaScript

```javascript
const { symbol, expr } = require('mathhook');

const x = symbol('x');
const expression = expr('x^2 + 2*x + 1');

const derivative = expression.derivative(x);
const integral = expression.integrate(x);

```



