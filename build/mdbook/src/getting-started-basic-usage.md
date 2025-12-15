---









---

# Basic Usage

> **Topic**: `getting-started.basic-usage`

Comprehensive guide to using MathHook including expression creation with macros
and constructors, simplification, pattern matching, symbol manipulation, number
types, constants, and function expressions.





# Basic Usage

This chapter provides a guide to using MathHook in your projects.

## Expression Creation

### Using Macros

The recommended way to create expressions is using the `expr!` and `symbol!` macros
for clean, readable code.

### Using Constructors

For programmatic construction, use explicit constructors like `Expression::integer()`,
`Expression::rational()`, `Expression::float()`.

## Simplification

Simplification transforms expressions to their canonical form by combining like terms,
applying identities, and evaluating constants.

## Pattern Matching

Work with expression structure using Rust's pattern matching on `Expression` enum
variants: `Add`, `Mul`, `Pow`, etc.

## Working with Symbols

Symbols represent variables in expressions. Symbols with the same name are equal.

## Number Types

MathHook supports integers (exact, arbitrary precision), rationals (exact fractions),
floats (approximate), and complex numbers.

## Constants

Built-in mathematical constants: π, e, i (imaginary unit), φ (golden ratio),
γ (Euler-Mascheroni constant).

## Function Expressions

Create function calls using `expr!` macro or `function!` macro for elementary
functions: sin, cos, tan, log, etc.












## Examples


### Expression Creation - Macros

Create expressions using ergonomic macros


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Simple arithmetic
let expr1 = expr!(x + y);
let expr2 = expr!(2 * x);
let expr3 = expr!(x ^ 2);

// Complex expressions with explicit grouping
let expr4 = expr!((x + 1) * (x - 1));

// Multi-term expressions
let expr5 = expr!(add: (2*x), (3*y), (-5));

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import Expression

x = Expression.symbol('x')
y = Expression.symbol('y')

# Method chaining for expressions
expr1 = x.add(y)
expr2 = Expression.integer(2).mul(x)
expr3 = x.pow(2)

# Complex expressions
expr4 = x.add(1).mul(x.sub(1))

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const y = Expression.symbol('y');

// Method chaining for expressions
const expr1 = x.add(y);
const expr2 = Expression.integer(2).mul(x);
const expr3 = x.pow(2);

// Complex expressions
const expr4 = x.add(1).mul(x.sub(1));

```
</details>





### Expression Creation - Constructors

Programmatic construction with explicit API


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::Expression;

// Numbers
let int = Expression::integer(42);
let float = Expression::float(3.14);
let rational = Expression::rational(3, 4);  // 3/4

// Operations
let sum = expr!(1 + 2);
let product = expr!(2 * x);

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import Expression

# Numbers
int_val = Expression.integer(42)
float_val = Expression.float(3.14)
rational_val = Expression.rational(3, 4)  # 3/4

# Operations
sum_val = Expression.integer(1).add(Expression.integer(2))
product_val = Expression.integer(2).mul(x)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
import { Expression } from 'mathhook-node';

// Numbers
const intVal = Expression.integer(42);
const floatVal = Expression.float(3.14);
const rationalVal = Expression.rational(3, 4);  // 3/4

// Operations
const sumVal = Expression.integer(1).add(Expression.integer(2));
const productVal = Expression.integer(2).mul(x);

```
</details>





### Simplification

Transform expressions to canonical form


<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);

// Combine like terms
let expr = expr!(x + x);
let simplified = expr.simplify();
// Result: 2*x

// Apply identities
let expr = expr!(x * 1);
let simplified = expr.simplify();
// Result: x

// Evaluate constants
let expr = expr!(2 + 3);
let simplified = expr.simplify();
// Result: 5

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import Expression

x = Expression.symbol('x')

# Combine like terms
expr = x.add(x)
simplified = expr.simplify()
# Result: 2*x

# Apply identities
expr = x.mul(Expression.integer(1))
simplified = expr.simplify()
# Result: x

# Evaluate constants
expr = Expression.integer(2).add(Expression.integer(3))
simplified = expr.simplify()
# Result: 5

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');

// Combine like terms
const expr = x.add(x);
const simplified = expr.simplify();
// Result: 2*x

// Apply identities
const expr2 = x.mul(Expression.integer(1));
const simplified2 = expr2.simplify();
// Result: x

// Evaluate constants
const expr3 = Expression.integer(2).add(Expression.integer(3));
const simplified3 = expr3.simplify();
// Result: 5

```
</details>





### Pattern Matching (Rust)

Inspect expression structure with pattern matching


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::Expression;

let x = symbol!(x);
let y = symbol!(y);
let test_expr = expr!(x + y);

match test_expr {
    Expression::Add(terms) => {
        println!("Addition with {} terms", terms.len());
    }
    Expression::Mul(factors) => {
        println!("Multiplication with {} factors", factors.len());
    }
    Expression::Pow(base, exp) => {
        println!("Power: base={}, exp={}", base, exp);
    }
    _ => {}
}

```
</details>



<details>
<summary><b>Python</b></summary>

```python
# Python doesn't have Rust-style pattern matching
# Use type checking instead
from mathhook import Expression

x = Expression.symbol('x')
y = Expression.symbol('y')
test_expr = x.add(y)

# Check expression type
if test_expr.is_add():
    print("Addition expression")

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
// TypeScript/JavaScript doesn't have pattern matching
// Use type checking instead
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');
const y = Expression.symbol('y');
const testExpr = x.add(y);

// Check expression type
if (testExpr.isAdd()) {
    console.log("Addition expression");
}

```
</details>





### Number Types

Different number representations in MathHook


<details>
<summary><b>Rust</b></summary>

```rust
// Integers (exact, arbitrary precision)
let int = Expression::integer(123456789);

// Rationals (exact fractions)
let frac = Expression::rational(22, 7);  // 22/7 ≈ π

// Floats (approximate)
let float = Expression::float(3.14159265359);

// Complex numbers
let complex = Expression::complex(
    Expression::integer(3),
    Expression::integer(4)
);  // 3 + 4i

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import Expression

# Integers (exact)
int_val = Expression.integer(123456789)

# Rationals (exact fractions)
frac = Expression.rational(22, 7)  # 22/7 ≈ π

# Floats (approximate)
float_val = Expression.float(3.14159265359)

# Complex numbers
complex_val = Expression.complex(
    Expression.integer(3),
    Expression.integer(4)
)  # 3 + 4i

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
import { Expression } from 'mathhook-node';

// Integers (exact)
const intVal = Expression.integer(123456789);

// Rationals (exact fractions)
const frac = Expression.rational(22, 7);  // 22/7 ≈ π

// Floats (approximate)
const floatVal = Expression.float(3.14159265359);

// Complex numbers
const complexVal = Expression.complex(
    Expression.integer(3),
    Expression.integer(4)
);  // 3 + 4i

```
</details>





### Mathematical Constants

Built-in mathematical constants


<details>
<summary><b>Rust</b></summary>

```rust
let pi = Expression::pi();
let e = Expression::e();
let i = Expression::i();              // imaginary unit
let phi = Expression::golden_ratio();
let gamma = Expression::euler_gamma();

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import Expression

pi = Expression.pi()
e = Expression.e()
i = Expression.i()              # imaginary unit
phi = Expression.golden_ratio()
gamma = Expression.euler_gamma()

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
import { Expression } from 'mathhook-node';

const pi = Expression.pi();
const e = Expression.e();
const i = Expression.i();              // imaginary unit
const phi = Expression.goldenRatio();
const gamma = Expression.eulerGamma();

```
</details>





### Function Expressions

Create elementary function calls


<details>
<summary><b>Rust</b></summary>

```rust
// Elementary functions using expr! macro
let sin_x = expr!(sin(x));
let cos_x = expr!(cos(x));
let log_x = expr!(log(x));

// Or using function! macro
let tan_x = function!(tan, x);

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import Expression

x = Expression.symbol('x')

# Elementary functions
sin_x = Expression.function('sin', [x])
cos_x = Expression.function('cos', [x])
log_x = Expression.function('log', [x])

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
import { Expression } from 'mathhook-node';

const x = Expression.symbol('x');

// Elementary functions
const sinX = Expression.function('sin', [x]);
const cosX = Expression.function('cos', [x]);
const logX = Expression.function('log', [x]);

```
</details>








## API Reference

- **Rust**: `mathhook::prelude::Expression`
- **Python**: `mathhook.Expression`
- **JavaScript**: `mathhook-node.Expression`


## See Also


- [getting-started.quick-start](../getting-started/quick-start.md)

- [getting-started.common-patterns](../getting-started/common-patterns.md)

- [core.expressions](../core/expressions.md)

- [core.symbols-numbers](../core/symbols-numbers.md)

- [operations.simplification](../operations/simplification.md)


