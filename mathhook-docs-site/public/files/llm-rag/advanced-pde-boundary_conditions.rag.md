# Boundary Conditions

Boundary conditions (BCs) specify constraints on the PDE solution at domain boundaries.
They determine eigenvalues and influence solution behavior. Covers Dirichlet, Neumann,
Robin, and periodic boundary conditions with implementation details and eigenvalue computation.


---
chunk_id: advanced_pde_boundary_conditions::0
topic: advanced.pde.boundary_conditions
title: Homogeneous Dirichlet Boundary Condition
priority: medium
keywords:
  - boundary_conditions
  - homogeneous dirichlet boundary condition
languages: [rust, python, javascript]
chunk: 1/2
---

## Homogeneous Dirichlet Boundary Condition

Fixed value at both boundaries (u=0), commonly used for heat equation with fixed temperatures

### Rust

```rust
let x = symbol!(x);

// Homogeneous: u(0,t) = 0
let bc_left = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: x.clone(), value: expr!(0) },
);

// Homogeneous: u(L,t) = 0
let bc_right = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: x, value: expr!(1) },
);

```

### Python

```python
x = symbol('x')

# Homogeneous: u(0,t) = 0
bc_left = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.Simple(variable=x, value=expr(0))
)

# Homogeneous: u(L,t) = 0
bc_right = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.Simple(variable=x, value=expr(1))
)

```

### JavaScript

```javascript
const x = symbol('x');

// Homogeneous: u(0,t) = 0
const bcLeft = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.Simple({ variable: x, value: expr(0) })
);

// Homogeneous: u(L,t) = 0
const bcRight = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.Simple({ variable: x, value: expr(1) })
);

```



---
chunk_id: advanced_pde_boundary_conditions::1
topic: advanced.pde.boundary_conditions
title: Non-Homogeneous Dirichlet Boundary Condition
priority: medium
keywords:
  - boundary_conditions
  - non-homogeneous dirichlet boundary condition
languages: [rust, python, javascript]
chunk: 2/2
---

## Non-Homogeneous Dirichlet Boundary Condition

Fixed non-zero value at boundary, common in heat transfer with maintained temperatures

### Rust

```rust
let x = symbol!(x);

// u(0,t) = 0
let bc_left = BoundaryCondition::dirichlet(
    expr!(0),
    BoundaryLocation::Simple { variable: x.clone(), value: expr!(0) },
);

// Non-homogeneous: u(L,t) = 100
let bc_right = BoundaryCondition::dirichlet(
    expr!(100),
    BoundaryLocation::Simple { variable: x, value: expr!(1) },
);

```

### Python

```python
x = symbol('x')

# u(0,t) = 0
bc_left = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.Simple(variable=x, value=expr(0))
)

# Non-homogeneous: u(L,t) = 100
bc_right = BoundaryCondition.dirichlet(
    expr(100),
    BoundaryLocation.Simple(variable=x, value=expr(1))
)

```

### JavaScript

```javascript
const x = symbol('x');

// u(0,t) = 0
const bcLeft = BoundaryCondition.dirichlet(
    expr(0),
    BoundaryLocation.Simple({ variable: x, value: expr(0) })
);

// Non-homogeneous: u(L,t) = 100
const bcRight = BoundaryCondition.dirichlet(
    expr(100),
    BoundaryLocation.Simple({ variable: x, value: expr(1) })
);

```



