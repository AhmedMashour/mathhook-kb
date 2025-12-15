# PDE Classification

Mathematical classification of partial differential equations into elliptic, parabolic,
and hyperbolic types using the discriminant formula. Different PDE types require
completely different solution methods and have distinct physical interpretations.


---
chunk_id: advanced_pde_classification::0
topic: advanced.pde.classification
title: Wave Equation Classification (Hyperbolic)
priority: medium
keywords:
  - classification
  - wave equation classification (hyperbolic)
languages: [rust, python, javascript]
chunk: 1/3
---

## Wave Equation Classification (Hyperbolic)

Wave equation has positive discriminant and is classified as hyperbolic

### Rust

```rust
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

// Wave equation structure
let equation = expr!(mul: x, t);
let pde = Pde::new(equation, u, vec![x, t]);

// Automatic classification
let pde_type = pde.pde_type();
assert_eq!(pde_type, Some(PdeType::Hyperbolic));

// Discriminant computation
let disc = pde.compute_discriminant();
assert!(disc > 0.0);
assert_eq!(disc, 4.0);

```

### Python

```python
u = symbol('u')
x = symbol('x')
t = symbol('t')

# Wave equation structure
equation = expr(mul=[x, t])
pde = Pde.new(equation, u, [x, t])

# Automatic classification
pde_type = pde.pde_type()
assert pde_type == PdeType.Hyperbolic

# Discriminant computation
disc = pde.compute_discriminant()
assert disc > 0.0
assert disc == 4.0

```

### JavaScript

```javascript
const u = symbol('u');
const x = symbol('x');
const t = symbol('t');

// Wave equation structure
const equation = expr({ mul: [x, t] });
const pde = Pde.new(equation, u, [x, t]);

// Automatic classification
const pdeType = pde.pdeType();
assert(pdeType === PdeType.Hyperbolic);

// Discriminant computation
const disc = pde.computeDiscriminant();
assert(disc > 0.0);
assert(disc === 4.0);

```



---
chunk_id: advanced_pde_classification::1
topic: advanced.pde.classification
title: Heat Equation Classification (Parabolic)
priority: medium
keywords:
  - classification
  - heat equation classification (parabolic)
languages: [rust, python, javascript]
chunk: 2/3
---

## Heat Equation Classification (Parabolic)

Heat equation has zero discriminant and is classified as parabolic

### Rust

```rust
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

// Heat equation structure
let equation = expr!(add: x, t);
let pde = Pde::new(equation, u, vec![x, t]);

// Automatic classification
let pde_type = pde.pde_type();
assert_eq!(pde_type, Some(PdeType::Parabolic));

// Discriminant
let disc = pde.compute_discriminant();
assert_eq!(disc.abs(), 0.0);

```

### Python

```python
u = symbol('u')
x = symbol('x')
t = symbol('t')

# Heat equation structure
equation = expr(add=[x, t])
pde = Pde.new(equation, u, [x, t])

# Automatic classification
pde_type = pde.pde_type()
assert pde_type == PdeType.Parabolic

# Discriminant
disc = pde.compute_discriminant()
assert abs(disc) == 0.0

```

### JavaScript

```javascript
const u = symbol('u');
const x = symbol('x');
const t = symbol('t');

// Heat equation structure
const equation = expr({ add: [x, t] });
const pde = Pde.new(equation, u, [x, t]);

// Automatic classification
const pdeType = pde.pdeType();
assert(pdeType === PdeType.Parabolic);

// Discriminant
const disc = pde.computeDiscriminant();
assert(Math.abs(disc) === 0.0);

```



---
chunk_id: advanced_pde_classification::2
topic: advanced.pde.classification
title: Laplace Equation Classification (Elliptic)
priority: medium
keywords:
  - classification
  - laplace equation classification (elliptic)
languages: [rust, python, javascript]
chunk: 3/3
---

## Laplace Equation Classification (Elliptic)

Laplace equation has negative discriminant and is classified as elliptic

### Rust

```rust
let u = symbol!(u);
let x = symbol!(x);
let y = symbol!(y);

// Laplace equation structure
let equation = expr!(add: x, y);
let pde = Pde::new(equation, u, vec![x, y]);

// Automatic classification
let pde_type = pde.pde_type();
assert_eq!(pde_type, Some(PdeType::Elliptic));

// Discriminant
let disc = pde.compute_discriminant();
assert!(disc < 0.0);
assert_eq!(disc, -4.0);

```

### Python

```python
u = symbol('u')
x = symbol('x')
y = symbol('y')

# Laplace equation structure
equation = expr(add=[x, y])
pde = Pde.new(equation, u, [x, y])

# Automatic classification
pde_type = pde.pde_type()
assert pde_type == PdeType.Elliptic

# Discriminant
disc = pde.compute_discriminant()
assert disc < 0.0
assert disc == -4.0

```

### JavaScript

```javascript
const u = symbol('u');
const x = symbol('x');
const y = symbol('y');

// Laplace equation structure
const equation = expr({ add: [x, y] });
const pde = Pde.new(equation, u, [x, y]);

// Automatic classification
const pdeType = pde.pdeType();
assert(pdeType === PdeType.Elliptic);

// Discriminant
const disc = pde.computeDiscriminant();
assert(disc < 0.0);
assert(disc === -4.0);

```



