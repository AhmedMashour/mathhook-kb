# PDE Quick Reference Card

One-page cheat sheet for Method of Characteristics covering standard forms,
solution templates, common patterns, shock formation, and troubleshooting guide.
Includes code templates, decision trees, and physical applications.


---
chunk_id: advanced_pde_quick_reference::0
topic: advanced.pde.quick_reference
title: Quick Code Template
priority: medium
keywords:
  - quick_reference
  - quick code template
languages: [rust, python, javascript]
chunk: 1/1
---

## Quick Code Template

Standard template for solving PDEs with method of characteristics

### Rust

```rust
// Define symbols
let u = symbol!(u);
let t = symbol!(t);
let x = symbol!(x);

// Build PDE
let equation = expr!(u);
let pde = Pde::new(equation, u, vec![t, x]);

// Solve
let result = method_of_characteristics(&pde).unwrap();

// Apply IC and verify
let solution = expr!(f(x - c*t));  // Example for transport

```

### Python

```python
# Define symbols
u = symbol('u')
t = symbol('t')
x = symbol('x')

# Build PDE
equation = expr(u)
pde = Pde.new(equation, u, [t, x])

# Solve
result = method_of_characteristics(pde)

# Apply IC and verify
solution = expr(f(x - c*t))  # Example for transport

```

### JavaScript

```javascript
// Define symbols
const u = symbol('u');
const t = symbol('t');
const x = symbol('x');

// Build PDE
const equation = expr(u);
const pde = Pde.new(equation, u, [t, x]);

// Solve
const result = methodOfCharacteristics(pde);

// Apply IC and verify
const solution = expr(f(x - c*t));  // Example for transport

```



