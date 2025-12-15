---









---

# Learning Paths

> **Topic**: `getting-started.learning-paths`

Choose your journey based on background and goals. Structured learning paths for
Python data scientists, Node.js developers, Rust programmers, mathematics educators,
and computational scientists with time estimates and outcomes.





# Learning Paths

Choose your journey based on your background and goals. Each path is designed to
get you productive with MathHook as quickly as possible.

## Path 1: Python Data Scientist

**Background**: Familiar with NumPy, SymPy, pandas
**Goal**: Use MathHook for faster symbolic computation in Python
**Time to Productivity**: 1-2 hours

Learn Python API, performance comparison with SymPy, integration with data science
stack, and when to use MathHook vs SymPy.

## Path 2: Node.js/TypeScript Developer

**Background**: JavaScript/TypeScript web development
**Goal**: Add symbolic math to web applications
**Time to Productivity**: 2-3 hours

Learn Node.js bindings, LaTeX parsing for web forms, web framework integration,
and V8 optimization.

## Path 3: Rust Systems Programmer

**Background**: Rust experience, need high-performance CAS
**Goal**: Embed MathHook in Rust application or contribute to core
**Time to Productivity**: 4-6 hours to mastery

Learn architecture, memory layout, SIMD optimization, and custom extensions.

## Path 4: Mathematics Student/Educator

**Background**: Calculus, linear algebra, abstract algebra knowledge
**Goal**: Understand CAS internals, use for teaching, contribute
**Time to Productivity**: 8-12 hours to contribution-ready

Learn symbolic computation theory, algorithm implementation, and educational features.

## Path 5: Computational Scientist

**Background**: MATLAB, Julia, scientific computing
**Goal**: Fast symbolic preprocessing for numerical simulations
**Time to Productivity**: 3-4 hours

Learn symbolic matrix algebra, system solving, hybrid symbolic-numerical workflows,
and code generation.

## Common Themes

Essential concepts for all users:
- Expressions are immutable (safe for concurrent use)
- Canonical forms (x + y equals y + x)
- Exact vs approximate arithmetic (rationals vs floats)
- Error handling (domain errors, undefined operations)












## Examples


### Python Data Scientist - SymPy Migration

Quick comparison of SymPy vs MathHook syntax

<details>
<summary><b>Rust</b></summary>

```rust
// Not applicable for Python path

```
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
// Not applicable for Python path

```
</details>




### Node.js Developer - Web Form Parsing

Parse user input LaTeX from web forms

<details>
<summary><b>Rust</b></summary>

```rust
// Not applicable for Node.js path

```
</details>

<details>
<summary><b>Python</b></summary>

```python
# Not applicable for Node.js path

```
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>




### Rust Programmer - Custom Extension

Extend Universal Function Registry with custom function

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

```python
# Not applicable for Rust path

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
// Not applicable for Rust path

```
</details>




### Mathematics Educator - Step-by-Step

Generate educational explanations for students

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!((x + 1) * (x - 1));

let explanation = expr.explain_simplification();
for step in &explanation.steps {
    println!("{}: {}", step.title, step.description);
}

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import Expression

x = Expression.symbol('x')
expr = (x.add(1)).mul(x.sub(1))

explanation = expr.explain_simplification()
for step in explanation.steps:
    print(f"{step.title}: {step.description}")

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const expr = x.add(1).mul(x.sub(1));

const explanation = expr.explainSimplification();
for (const step of explanation.steps) {
    console.log(`${step.title}: ${step.description}`);
}

```
</details>




### Computational Scientist - Symbolic Jacobian

Generate Jacobian matrix for nonlinear system

<details>
<summary><b>Rust</b></summary>

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
</details>

<details>
<summary><b>Python</b></summary>

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
</details>

<details>
<summary><b>JavaScript</b></summary>

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
</details>







## API Reference

- **Rust**: `mathhook::prelude`
- **Python**: `mathhook`
- **JavaScript**: `mathhook-node`


## See Also


- [getting-started.installation](../getting-started/installation.md)

- [getting-started.quick-start](../getting-started/quick-start.md)

- [bindings.python](../bindings/python.md)

- [bindings.nodejs](../bindings/nodejs.md)

- [architecture.principles](../architecture/principles.md)

- [educational.step-by-step](../educational/step-by-step.md)


