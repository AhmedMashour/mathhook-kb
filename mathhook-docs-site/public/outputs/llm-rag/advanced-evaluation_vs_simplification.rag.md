# Evaluation vs Simplification

Understand the key differences between evaluation (computing numerical results)
and simplification (algebraic transformation) in MathHook's symbolic engine.


---
chunk_id: advanced_evaluation_vs_simplification::0
topic: advanced.evaluation_vs_simplification
title: Evaluation Example
priority: medium
keywords:
  - evaluation_vs_simplification
  - evaluation example
languages: [rust, python, javascript]
chunk: 1/2
---

## Evaluation Example



### Rust

```rust
let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);

let evaluated = expr.substitute(&x, &expr!(3));
// Result: 16 (numerical)

```

### Python

```python
x = symbol('x')
expr_val = x**2 + 2*x + 1

evaluated = expr_val.subs(x, 3)
# Result: 16

```

### JavaScript

```javascript
const x = symbol('x');
const expr = add(pow(x, 2), mul(2, x), 1);

const evaluated = expr.subs(x, 3);
// Result: 16

```



---
chunk_id: advanced_evaluation_vs_simplification::1
topic: advanced.evaluation_vs_simplification
title: Simplification Example
priority: medium
keywords:
  - evaluation_vs_simplification
  - simplification example
languages: [rust, python, javascript]
chunk: 2/2
---

## Simplification Example



### Rust

```rust
let x = symbol!(x);
let expr = expr!(x + x + x);

let simplified = expr.simplify();
// Result: 3*x (still symbolic)

```

### Python

```python
x = symbol('x')
expr_val = x + x + x

simplified = simplify(expr_val)
# Result: 3*x

```

### JavaScript

```javascript
const x = symbol('x');
const expr = add(x, x, x);

const simplified = expr.simplify();
// Result: 3*x

```



