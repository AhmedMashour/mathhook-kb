# Learning Paths

Choose your journey based on background and goals. Structured learning paths for
Python data scientists, Node.js developers, Rust programmers, mathematics educators,
and computational scientists with time estimates and outcomes.


---
chunk_id: getting-started_learning-paths::0
topic: getting-started.learning-paths
title: Python Data Scientist - SymPy Migration
priority: medium
keywords:
  - learning-paths
  - python data scientist - sympy migration
languages: [rust, python, javascript]
chunk: 1/5
---

## Python Data Scientist - SymPy Migration

Quick comparison of SymPy vs MathHook syntax

### Rust

```rust
// Not applicable for Python path

```

### Python

```python
# SymPy syntax (familiar to data scientists)
# from sympy import symbols, simplify
# x, y = symbols('x y')
# expr = (x + y)**2

# MathHook syntax (similar but faster)
from mathhook import Expression

x = Expression.symbol('x')
y = Expression.symbol('y')
expr = (x.add(y)).pow(2)
simplified = expr.simplify()

```

### JavaScript

```javascript
// Not applicable for Python path

```



---
chunk_id: getting-started_learning-paths::1
topic: getting-started.learning-paths
title: Node.js Developer - Web Form Parsing
priority: medium
keywords:
  - learning-paths
  - node.js developer - web form parsing
languages: [rust, python, javascript]
chunk: 2/5
---

## Node.js Developer - Web Form Parsing

Parse user input LaTeX from web forms

### Rust

```rust
// Not applicable for Node.js path

```

### Python

```python
# Not applicable for Node.js path

```

### JavaScript

```javascript
import { Parser, ParserConfig } from 'mathhook-node';

// Parse LaTeX from web form input
const userInput = req.body.equation; // e.g., "x^2 + 2x + 1"
const parser = new Parser(ParserConfig.default());
const expr = parser.parse(userInput);

// Render LaTeX output for display
const latex = expr.toLatex();
res.json({ latex });

```



---
chunk_id: getting-started_learning-paths::2
topic: getting-started.learning-paths
title: Rust Programmer - Custom Extension
priority: medium
keywords:
  - learning-paths
  - rust programmer - custom extension
languages: [rust, python, javascript]
chunk: 3/5
---

## Rust Programmer - Custom Extension

Extend Universal Function Registry with custom function

### Rust

```rust
use mathhook::prelude::*;

// Implement custom simplification rule
fn custom_simplify(expr: &Expression) -> Expression {
    // Custom logic here
    expr.clone()
}

// Register custom function
// (Actual API may vary - check documentation)
let x = symbol!(x);
let expr = expr!(x ^ 2);

```

### Python

```python
# Not applicable for Rust path

```

### JavaScript

```javascript
// Not applicable for Rust path

```



---
chunk_id: getting-started_learning-paths::3
topic: getting-started.learning-paths
title: Mathematics Educator - Step-by-Step
priority: medium
keywords:
  - learning-paths
  - mathematics educator - step-by-step
languages: [rust, python, javascript]
chunk: 4/5
---

## Mathematics Educator - Step-by-Step

Generate educational explanations for students

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!((x + 1) * (x - 1));

let explanation = expr.explain_simplification();
for step in &explanation.steps {
    println!("{}: {}", step.title, step.description);
}

```

### Python

```python
from mathhook import Expression

x = Expression.symbol('x')
expr = (x.add(1)).mul(x.sub(1))

explanation = expr.explain_simplification()
for step in explanation.steps:
    print(f"{step.title}: {step.description}")

```

### JavaScript

```javascript
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const expr = x.add(1).mul(x.sub(1));

const explanation = expr.explainSimplification();
for (const step of explanation.steps) {
    console.log(`${step.title}: ${step.description}`);
}

```



---
chunk_id: getting-started_learning-paths::4
topic: getting-started.learning-paths
title: Computational Scientist - Symbolic Jacobian
priority: medium
keywords:
  - learning-paths
  - computational scientist - symbolic jacobian
languages: [rust, python, javascript]
chunk: 5/5
---

## Computational Scientist - Symbolic Jacobian

Generate Jacobian matrix for nonlinear system

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// System of equations
let f1 = expr!(add: (x ^ 2), y);
let f2 = expr!(x * y);

// Compute Jacobian symbolically
let df1_dx = f1.derivative(x.clone());
let df1_dy = f1.derivative(y.clone());
let df2_dx = f2.derivative(x.clone());
let df2_dy = f2.derivative(y.clone());

let jacobian = Expression::matrix(vec![
    vec![df1_dx, df1_dy],
    vec![df2_dx, df2_dy],
]);

```

### Python

```python
from mathhook import Expression

x = Expression.symbol('x')
y = Expression.symbol('y')

# System of equations
f1 = x.pow(2).add(y)
f2 = x.mul(y)

# Compute Jacobian symbolically
df1_dx = f1.derivative(x)
df1_dy = f1.derivative(y)
df2_dx = f2.derivative(x)
df2_dy = f2.derivative(y)

jacobian = Expression.matrix([
    [df1_dx, df1_dy],
    [df2_dx, df2_dy]
])

```

### JavaScript

```javascript
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const y = Expression.symbol('y');

// System of equations
const f1 = x.pow(2).add(y);
const f2 = x.mul(y);

// Compute Jacobian symbolically
const df1_dx = f1.derivative(x);
const df1_dy = f1.derivative(y);
const df2_dx = f2.derivative(x);
const df2_dy = f2.derivative(y);

const jacobian = Expression.matrix([
    [df1_dx, df1_dy],
    [df2_dx, df2_dy]
]);

```



