---









---

# Core Expression System

> **Topic**: `api.core.expressions`

The Expression type is the foundation of MathHook. Expressions are immutable,
32-byte cache-optimized structures representing mathematical constructs from
numbers to complex symbolic operations.





# Core Expression System

## Overview

The `Expression` type is MathHook's core data structure, designed for:
- **Immutability**: Thread-safe, predictable behavior
- **Performance**: 32-byte size for cache optimization (2 per cache line)
- **Canonical Forms**: Automatic normalization for equality checking

## Expression Structure

Expressions use Rust enums for type-safe mathematical constructs:
- **Numbers**: Integer, Rational, Float, Complex
- **Variables**: Symbol
- **Operations**: Add, Mul, Pow
- **Functions**: Function calls (sin, cos, log, etc.)
- **Constants**: π, e, i, φ, γ
- **Matrices**: Matrix (noncommutative)
- **Relations**: Equation, Inequality

## Design Decisions

### Why 32 Bytes?
- Modern CPUs have 64-byte cache lines
- Two expressions fit perfectly in one cache line
- 3-5x faster operations in hot loops
- Critical for CAS workloads with millions of expression traversals

### Why Immutable?
- Thread safety without locks
- No hidden mutation surprises
- Compiler optimizations
- Traceable expression history

### Why Canonical Forms?
- Structural equality: y + x → x + y
- Flattening: (a + b) + c → Add(a, b, c)
- Identity removal: x + 0 → x
- Rational reduction: 6/4 → 3/2












## Examples


### Creating Expressions with Macros

Use expr!() and symbol!() macros for ergonomic expression creation

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);

// Basic arithmetic
let sum = expr!(x + y);
let product = expr!(x * y);
let power = expr!(x ^ 2);

// Complex nested expressions
let complex = expr!(sin(x ^ 2) + cos(y ^ 2));

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr

x = symbol('x')
y = symbol('y')

# Basic arithmetic
sum_expr = x + y
product = x * y
power = x**2

# Complex nested expressions
from mathhook import sin, cos
complex_expr = sin(x**2) + cos(y**2)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { symbol, parse } from 'mathhook';

const x = symbol('x');
const y = symbol('y');

// Parse expressions
const sum = parse('x + y');
const product = parse('x * y');
const power = parse('x^2');

// Complex nested expressions
const complex = parse('sin(x^2) + cos(y^2)');

```
</details>




### Immutability and Operations

All operations return new expressions, original unchanged

<details>
<summary><b>Rust</b></summary>

```rust
let expr = expr!(x + 1);
let doubled = expr.mul(&expr!(2));  // Returns new expression
// `expr` is unchanged - still x + 1

// Safe to use in multiple threads
use std::sync::Arc;
let expr_arc = Arc::new(expr!(x ^ 2));
let clone = Arc::clone(&expr_arc);

```
</details>

<details>
<summary><b>Python</b></summary>

```python
expr = x + 1
doubled = expr * 2  # Returns new expression
# expr is unchanged - still x + 1

# Safe for concurrent use
import threading
shared_expr = x**2

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const expr = parse('x + 1');
const doubled = expr.mul(2);  // Returns new expression
// expr is unchanged - still x + 1

// Immutable - safe for concurrent access

```
</details>




### Canonical Forms and Equality

Automatic normalization ensures equivalent expressions are equal

<details>
<summary><b>Rust</b></summary>

```rust
let expr1 = expr!(x + y);
let expr2 = expr!(y + x);
assert_eq!(expr1, expr2);  // True - both normalized to x + y

// Flattening
let nested = expr!((x + y) + z);
// Automatically flattened to Add(x, y, z)

// Identity removal
let identity = expr!(x + 0);
assert_eq!(identity.simplify(), expr!(x));

```
</details>

<details>
<summary><b>Python</b></summary>

```python
expr1 = x + y
expr2 = y + x
assert expr1 == expr2  # True - both normalized to x + y

# Flattening and identity removal
nested = (x + y) + z
identity = x + 0
assert identity.simplify() == x

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const expr1 = parse('x + y');
const expr2 = parse('y + x');
// Both normalized to x + y

// Identity removal
const identity = parse('x + 0');
const simplified = identity.simplify();
// Result: x

```
</details>




### Pattern Matching and Structure

Work with expression structure using pattern matching

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::Expression;

match expr {
    Expression::Add(terms) => {
        println!("Sum with {} terms", terms.len());
    }
    Expression::Mul(factors) => {
        println!("Product with {} factors", factors.len());
    }
    Expression::Pow(base, exp) => {
        println!("Power: {} ^ {}", base, exp);
    }
    Expression::Function(name, args) => {
        println!("Function {} with {} args", name, args.len());
    }
    _ => {}
}

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import Expression

# Python uses method introspection
if expr.is_add():
    terms = expr.get_terms()
    print(f"Sum with {len(terms)} terms")
elif expr.is_mul():
    factors = expr.get_factors()
    print(f"Product with {len(factors)} factors")
elif expr.is_pow():
    base, exp = expr.get_base_exp()
    print(f"Power: {base} ^ {exp}")

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
import { Expression } from 'mathhook';

// Node.js uses type checking methods
if (expr.isAdd()) {
    const terms = expr.getTerms();
    console.log(`Sum with ${terms.length} terms`);
} else if (expr.isMul()) {
    const factors = expr.getFactors();
    console.log(`Product with ${factors.length} factors`);
} else if (expr.isPow()) {
    const [base, exp] = expr.getBaseExp();
    console.log(`Power: ${base} ^ ${exp}`);
}

```
</details>






## Performance

**Time Complexity**: O(1) construction, O(n) operations for n-node trees


## API Reference

- **Rust**: `mathhook_core::expression::Expression`
- **Python**: `mathhook.Expression`
- **JavaScript**: `mathhook.Expression`


## See Also


- [api.core.symbols_numbers](../api/core/symbols_numbers.md)

- [api.core.functions](../api/core/functions.md)

- [api.operations.simplification](../api/operations/simplification.md)

- [api.operations.substitution](../api/operations/substitution.md)


