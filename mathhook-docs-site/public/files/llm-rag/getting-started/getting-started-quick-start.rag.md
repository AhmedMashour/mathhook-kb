# Quick Start

Get up and running with MathHook in 5 minutes. Learn basic expression creation,
parsing, differentiation, and common operations across Rust, Python, and Node.js.


---
chunk_id: getting-started_quick-start::0
topic: getting-started.quick-start
title: First Expression - Quadratic
priority: medium
keywords:
  - quick-start
  - first expression - quadratic
languages: [rust, python, javascript]
chunk: 1/6
---

## First Expression - Quadratic

Build and simplify x^2 + 2x + 1

### Rust

```rust
use mathhook::prelude::*;

fn main() {
    let x = symbol!(x);
    let expr = expr!(add: (x ^ 2), (2 * x), 1);
    let simplified = expr.simplify();

    println!("Original: {}", expr);
    println!("Simplified: {}", simplified);
}

```

### Python

```python
from mathhook import Expression

x = Expression.symbol('x')
expr = x.pow(2).add(x.multiply(2)).add(1)
simplified = expr.simplify()

print(f"Original: {expr}")
print(f"Simplified: {simplified}")

```

### JavaScript

```javascript
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const expr = x.pow(2).add(x.multiply(2)).add(1);
const simplified = expr.simplify();

console.log(`Original: ${expr.toString()}`);
console.log(`Simplified: ${simplified.toString()}`);

```



---
chunk_id: getting-started_quick-start::1
topic: getting-started.quick-start
title: Parsing LaTeX
priority: medium
keywords:
  - quick-start
  - parsing latex
languages: [rust, python, javascript]
chunk: 2/6
---

## Parsing LaTeX

Parse LaTeX notation into symbolic expression

### Rust

```rust
let parser = Parser::new(ParserConfig::default());
let expr = parser.parse(r"\frac{x^2 + 1}{2}").unwrap();
println!("{}", expr);

```

### Python

```python
from mathhook import Parser, ParserConfig

parser = Parser(ParserConfig.default())
expr = parser.parse(r"\frac{x^2 + 1}{2}")
print(expr)

```

### JavaScript

```javascript
import { Parser, ParserConfig } from 'mathhook-node';

const parser = new Parser(ParserConfig.default());
const expr = parser.parse(String.raw`\frac{x^2 + 1}{2}`);
console.log(expr.toString());

```



---
chunk_id: getting-started_quick-start::2
topic: getting-started.quick-start
title: Computing Derivatives
priority: medium
keywords:
  - quick-start
  - computing derivatives
languages: [rust, python, javascript]
chunk: 3/6
---

## Computing Derivatives

Compute first and second derivatives of x^3

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(x ^ 3);

let derivative = expr.derivative(x.clone());
let second_derivative = expr.nth_derivative(x, 2);

println!("f(x) = {}", expr);
println!("f'(x) = {}", derivative);
println!("f''(x) = {}", second_derivative);

```

### Python

```python
from mathhook import Expression

x = Expression.symbol('x')
expr = x.pow(3)

derivative = expr.derivative(x)
second_derivative = expr.nth_derivative(x, 2)

print(f"f(x) = {expr}")
print(f"f'(x) = {derivative}")
print(f"f''(x) = {second_derivative}")

```

### JavaScript

```javascript
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const expr = x.pow(3);

const derivative = expr.derivative(x);
const secondDerivative = expr.nthDerivative(x, 2);

console.log(`f(x) = ${expr.toString()}`);
console.log(`f'(x) = ${derivative.toString()}`);
console.log(`f''(x) = ${secondDerivative.toString()}`);

```



---
chunk_id: getting-started_quick-start::3
topic: getting-started.quick-start
title: Solving Equations
priority: medium
keywords:
  - quick-start
  - solving equations
languages: [rust, python, javascript]
chunk: 4/6
---

## Solving Equations

Solve x^2 = 4 symbolically

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let mut solver = MathSolver::new();
let equation = Expression::equation(expr!(x ^ 2), expr!(4));
let solutions = solver.solve(&equation, &x);

println!("Solutions: {:?}", solutions);
// Output: [x = 2, x = -2]

```

### Python

```python
from mathhook import Expression, MathSolver

x = Expression.symbol('x')
solver = MathSolver()
equation = Expression.equation(x.pow(2), Expression.integer(4))
solutions = solver.solve(equation, x)

print(f"Solutions: {solutions}")

```

### JavaScript

```javascript
import { Expression, MathSolver } from 'mathhook-node';

const x = Expression.symbol('x');
const solver = new MathSolver();
const equation = Expression.equation(x.pow(2), Expression.integer(4));
const solutions = solver.solve(equation, x);

console.log(`Solutions: ${solutions}`);

```



---
chunk_id: getting-started_quick-start::4
topic: getting-started.quick-start
title: Substitution
priority: medium
keywords:
  - quick-start
  - substitution
languages: [rust, python, javascript]
chunk: 5/6
---

## Substitution

Substitute x = 3 into x^2 + 2x + 1

### Rust

```rust
use mathhook::prelude::*;
use std::collections::HashMap;

let x = symbol!(x);
let expr = expr!(add: (x ^ 2), (2 * x), 1);

let mut vars = HashMap::new();
vars.insert("x".to_string(), Expression::integer(3));
let result = expr.substitute(&vars);
println!("Result: {}", result);
// Output: 16

```

### Python

```python
from mathhook import Expression

x = Expression.symbol('x')
expr = x.pow(2).add(x.multiply(2)).add(1)

vars = {'x': Expression.integer(3)}
result = expr.substitute(vars)
print(f"Result: {result}")

```

### JavaScript

```javascript
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const expr = x.pow(2).add(x.multiply(2)).add(1);

const vars = new Map([['x', Expression.integer(3)]]);
const result = expr.substitute(vars);
console.log(`Result: ${result.toString()}`);

```



---
chunk_id: getting-started_quick-start::5
topic: getting-started.quick-start
title: Creating Expressions Programmatically
priority: medium
keywords:
  - quick-start
  - creating expressions programmatically
languages: [rust, python, javascript]
chunk: 6/6
---

## Creating Expressions Programmatically

Use macros for compile-time values, explicit API for runtime

### Rust

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Compile-time - use macros
let expr = expr!((x ^ 2) + 3);

// Runtime - use explicit API
let mut terms = Vec::new();
for i in 0..5 {
    terms.push(Expression::mul(vec![
        Expression::integer(i as i64),
        Expression::pow(x.clone().into(), Expression::integer(i as i64))
    ]));
}
let polynomial = Expression::add(terms);

```

### Python

```python
from mathhook import Expression

x = Expression.symbol('x')

# Direct creation
expr = x.pow(2).add(3)

# Runtime creation
terms = []
for i in range(5):
    terms.append(
        Expression.mul([
            Expression.integer(i),
            x.pow(Expression.integer(i))
        ])
    )
polynomial = Expression.add(terms)

```

### JavaScript

```javascript
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');

// Direct creation
const expr = x.pow(2).add(3);

// Runtime creation
const terms = [];
for (let i = 0; i < 5; i++) {
    terms.push(
        Expression.mul([
            Expression.integer(i),
            x.pow(Expression.integer(i))
        ])
    );
}
const polynomial = Expression.add(terms);

```



