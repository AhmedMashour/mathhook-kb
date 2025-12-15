# Symbolic Differentiation

Computes the derivative of an expression with respect to a variable using
symbolic differentiation rules. Supports power rule, product rule, quotient
rule, chain rule, and derivatives of elementary functions.


---
chunk_id: calculus_derivative::0
topic: calculus.derivative
title: Power Rule
priority: high
keywords:
  - derivative
  - power rule
languages: [rust, python, javascript]
chunk: 1/3
---

## Power Rule

The derivative of x^n is n*x^(n-1). This is one of the fundamental
rules of calculus and applies to any real exponent n.


### Rust

```rust
let x = symbol!(x);
let f = expr!(x ^ 3);
let df = f.derivative(&x, 1);
// Result: 3*x^2

```

### Python

```python
x = symbol('x')
f = expr('x^3')
df = f.derivative(x)
# Result: 3*x^2

```

### JavaScript

```javascript
const x = symbol('x');
const f = expr('x^3');
const df = f.derivative(x);
// Result: 3*x^2

```

### Expected Output

```
3*x^2
```



---
chunk_id: calculus_derivative::1
topic: calculus.derivative
title: Chain Rule
priority: high
keywords:
  - derivative
  - chain rule
languages: [rust, python, javascript]
chunk: 2/3
---

## Chain Rule

When differentiating a composition of functions f(g(x)), we use the
chain rule: (f∘g)'(x) = f'(g(x)) · g'(x)


### Rust

```rust
let x = symbol!(x);
let f = expr!(sin(x ^ 2));
let df = f.derivative(&x, 1);
// Result: 2*x*cos(x^2)

```

### Python

```python
x = symbol('x')
f = expr('sin(x^2)')
df = f.derivative(x)
# Result: 2*x*cos(x^2)

```

### JavaScript

```javascript
const x = symbol('x');
const f = expr('sin(x^2)');
const df = f.derivative(x);
// Result: 2*x*cos(x^2)

```

### Expected Output

```
2*x*cos(x^2)
```



---
chunk_id: calculus_derivative::2
topic: calculus.derivative
title: Product Rule
priority: high
keywords:
  - derivative
  - product rule
languages: [rust, python, javascript]
chunk: 3/3
---

## Product Rule

For the product of two functions u(x)·v(x), the derivative is
u'(x)·v(x) + u(x)·v'(x)


### Rust

```rust
let x = symbol!(x);
let f = expr!((x ^ 2) * sin(x));
let df = f.derivative(&x, 1);
// Result: 2*x*sin(x) + x^2*cos(x)

```

### Python

```python
x = symbol('x')
f = expr('x^2 * sin(x)')
df = f.derivative(x)
# Result: 2*x*sin(x) + x^2*cos(x)

```

### JavaScript

```javascript
const x = symbol('x');
const f = expr('x^2 * sin(x)');
const df = f.derivative(x);
// Result: 2*x*sin(x) + x^2*cos(x)

```

### Expected Output

```
2*x*sin(x) + x^2*cos(x)
```



