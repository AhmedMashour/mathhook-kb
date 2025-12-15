# Method of Characteristics

The Method of Characteristics is the primary technique for solving first-order
partial differential equations (PDEs). It transforms the PDE into a system of
ordinary differential equations (ODEs) that can be solved along special curves
called characteristic curves.


---
chunk_id: advanced_pde_method_of_characteristics::0
topic: advanced.pde.method_of_characteristics
title: Transport Equation Solution
priority: medium
keywords:
  - method_of_characteristics
  - transport equation solution
languages: [rust, python, javascript]
chunk: 1/2
---

## Transport Equation Solution

Solving the transport equation using method of characteristics

### Rust

```rust
let u = symbol!(u);
let t = symbol!(t);
let x = symbol!(x);

// Transport equation PDE structure
let equation = expr!(u);
let pde = Pde::new(equation, u, vec![t, x]);

// Solve
let solution = method_of_characteristics(&pde).unwrap();
println!("Solution: u(x,t) = F(x - ct)");

// With initial condition u(x,0) = sin(x):
println!("Specific solution: u(x,t) = sin(x - ct)");

```

### Python

```python
u = symbol('u')
t = symbol('t')
x = symbol('x')

# Transport equation PDE structure
equation = expr(u)
pde = Pde.new(equation, u, [t, x])

# Solve
solution = method_of_characteristics(pde)
print("Solution: u(x,t) = F(x - ct)")

# With initial condition u(x,0) = sin(x):
print("Specific solution: u(x,t) = sin(x - ct)")

```

### JavaScript

```javascript
const u = symbol('u');
const t = symbol('t');
const x = symbol('x');

// Transport equation PDE structure
const equation = expr(u);
const pde = Pde.new(equation, u, [t, x]);

// Solve
const solution = methodOfCharacteristics(pde);
console.log("Solution: u(x,t) = F(x - ct)");

// With initial condition u(x,0) = sin(x):
console.log("Specific solution: u(x,t) = sin(x - ct)");

```



---
chunk_id: advanced_pde_method_of_characteristics::1
topic: advanced.pde.method_of_characteristics
title: General Usage Pattern
priority: medium
keywords:
  - method_of_characteristics
  - general usage pattern
languages: [rust, python, javascript]
chunk: 2/2
---

## General Usage Pattern

Standard pattern for using method of characteristics in MathHook

### Rust

```rust
// Define PDE
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

let equation = /* build PDE expression */;
let pde = Pde::new(equation, u, vec![x, t]);

// Solve
match method_of_characteristics(&pde) {
    Ok(solution) => {
        println!("Solution: {}", solution.solution);
        // Apply initial conditions as needed
    }
    Err(e) => println!("Error: {:?}", e),
}

```

### Python

```python
# Define PDE
u = symbol('u')
x = symbol('x')
t = symbol('t')

equation = # build PDE expression
pde = Pde.new(equation, u, [x, t])

# Solve
try:
    solution = method_of_characteristics(pde)
    print(f"Solution: {solution.solution}")
    # Apply initial conditions as needed
except Exception as e:
    print(f"Error: {e}")

```

### JavaScript

```javascript
// Define PDE
const u = symbol('u');
const x = symbol('x');
const t = symbol('t');

const equation = /* build PDE expression */;
const pde = Pde.new(equation, u, [x, t]);

// Solve
try {
    const solution = methodOfCharacteristics(pde);
    console.log(`Solution: ${solution.solution}`);
    // Apply initial conditions as needed
} catch (e) {
    console.log(`Error: ${e}`);
}

```



