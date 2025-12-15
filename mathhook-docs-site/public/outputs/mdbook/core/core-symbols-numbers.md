---









---

# Symbols and Numbers

> **Topic**: `core.symbols-numbers`

Symbols represent mathematical variables (x, y, θ, etc.) using efficient string interning.
Numbers support integers, rationals, floats, and complex numbers with exact symbolic representation
for precise mathematical computation.





# Symbols and Numbers

This chapter covers the two fundamental building blocks of expressions: symbols (variables) and numbers.

## Symbols

Symbols represent mathematical variables like $$x$$, $$y$$, $$\theta$$, etc.

### Creating Symbols

```rust
use mathhook::prelude::*;

let x = symbol!(x);
let y = symbol!(y);
let theta = symbol!(theta);
```

### Symbol Equality

Symbols with the same name are considered equal:

```rust
assert_eq!(symbol!(x), symbol!(x));
assert_ne!(symbol!(x), symbol!(y));
```

### String Interning

MathHook uses string interning for symbols, making equality checks O(1) pointer comparisons.

## Why This Design?

### Why String Interning for Symbols?

**Design Decision**: Symbol names are stored in a global intern table, with symbols holding only a reference.

**Why?**
- **Fast equality**: Comparing two symbols is a single pointer comparison (O(1))
- **Memory efficiency**: Symbol name "x" stored once, shared by all `symbol!(x)` instances
- **Cache-friendly**: Symbols are just pointers (8 bytes on 64-bit systems)

**Without Interning**: Every `symbol!(x)` would store its own copy of "x" and require string comparison (O(n))

**Trade-off**: Global mutable state for intern table
- Thread-safe using locks or lock-free data structures
- One-time cost on first use of each symbol name
- Benefit far outweighs cost (10-100x faster symbol comparison)

**Example**:
```rust
let x1 = symbol!(x);
let x2 = symbol!(x);
// Same pointer internally - O(1) comparison
assert_eq!(x1, x2);
```

**When This Matters**:
- Pattern matching with many symbol comparisons
- Substitution operations
- Expression equality checking
- Hash table lookups

---

## Numbers

MathHook supports multiple number types for different use cases.

### Integers

Arbitrary precision integers for exact computation:

```rust
let n = Expression::integer(123456789);
let large = Expression::integer(9999999999999999999); // Arbitrary precision
```

### Rationals

Exact representation of fractions:

```rust
let frac = Expression::rational(22, 7);  // 22/7 ≈ π
let half = Expression::rational(1, 2);   // 1/2

// Always in reduced form
let six_fourths = Expression::rational(6, 4);  // Automatically becomes 3/2
```

### Floats

Floating-point numbers for approximate computation:

```rust
let pi_approx = Expression::float(3.14159265359);
let e_approx = Expression::float(2.71828182846);
```

**Warning**: Use floats only when approximation is acceptable. Prefer rationals for exact arithmetic.

## Why Rational Numbers Over Floats?

### Design Decision: Exact Rational Arithmetic

**Why MathHook Uses Rationals for Symbolic Math**:

**The Problem with Floats**:
```rust
// Using floats (WRONG for symbolic math)
let third = 0.33333333;
let result = 3.0 * third;
// Result: 0.99999999 (imprecise)

// Using rationals (CORRECT for symbolic math)
let third = Expression::rational(1, 3);
let result = expr!(3 * third);
// Result: 1 (exact)
```

**Why?**
- **Mathematical correctness**: `1/3` is exactly `1/3`, not an approximation
- **Symbolic operations**: Algebra requires exactness (cannot lose precision)
- **Accumulation prevention**: No rounding error buildup
- **Comparison reliability**: Exact equality testing

**When We Use Floats**:
- Only for numerical approximation (explicit `.evalf()`)
- Only when exact representation is impossible (e.g., transcendental results)
- **NEVER** in symbolic operations

**Real-World Example**:
```rust
// Solving x^2 - 2 = 0 symbolically
let x = symbol!(x);
let eq = expr!((x ^ 2) - 2);
let solutions = eq.solve(&x);
// Solutions: [-√2, √2] (exact symbolic form)
// NOT: [-1.414213562, 1.414213562] (approximate floats)
```

**Alternative Considered**: Always use floats (like numerical libraries)
- **Pros**: Simpler implementation, predictable memory usage
- **Cons**: Catastrophic for symbolic algebra (precision loss, equality breaks)
- **Decision**: Exact arithmetic is non-negotiable for CAS

**Why This Matters**:
- Computer algebra requires exactness by definition
- SymPy and Mathematica use same approach
- Prevents subtle bugs from rounding errors
- Enables reliable symbolic simplification

**Performance Impact**:
- Rational arithmetic is slower than float (2-10x)
- Acceptable trade-off for correctness
- Use `.evalf()` when you need speed and can tolerate approximation

---

### Why 16-Byte Number Type?

**Design Decision**: The `Number` type is exactly 16 bytes.

**Why?**
- **Cache efficiency**: Two numbers fit in a 32-byte expression
- **Tagged union**: Discriminant + data in 16 bytes
- **Balance**: Small enough for cache, large enough for pointer + metadata

**Structure**:
```
[1 byte: type tag] [15 bytes: data]
- Integer: pointer to BigInt (8 bytes) + padding
- Rational: two pointers to BigInt numerator/denominator (need clever packing)
- Float: f64 (8 bytes) + padding
- Complex: pointer to ComplexData (8 bytes) + padding
```

**Trade-off**: Arbitrary precision requires heap allocation
- Small integers (i64) could fit inline, but design consistency favors uniform handling
- Large integers/rationals use `BigInt` on heap (pointer stored in 16 bytes)

**Alternative Considered**: Variable-size numbers
- **Pros**: i64 could be inline (faster)
- **Cons**: Variable size breaks expression size constraint
- **Decision**: Uniform 16-byte size maintains expression size guarantee

---

### Complex Numbers

Complex numbers with real and imaginary parts:

```rust
// 3 + 4i
let z = Expression::complex(
    Expression::integer(3),
    Expression::integer(4)
);

// Or using addition
let z = expr!(3 + (4 * Expression::i()));
```

## Number Operations

### Arithmetic

```rust
let a = Expression::integer(2);
let b = Expression::integer(3);

let sum = expr!(a + b);      // 5
let product = expr!(a * b);  // 6
let power = Expression::pow(a.clone(), b.clone());          // 8
```

### Exact vs Approximate

```rust
// Exact: Use rationals
let exact = Expression::rational(1, 3);
let tripled = expr!(exact * 3);
// Result: 1 (exact)

// Approximate: Use floats
let approx = Expression::float(0.333333);
let tripled_approx = expr!(approx * 3.0);
// Result: 0.999999 (approximate)
```

## Type Conversions

### To Float

```rust
let rational = Expression::rational(1, 3);
let as_float = rational.to_float();  // 0.333...
```

### To Rational

```rust
let float = Expression::float(0.5);
let as_rational = float.to_rational();  // 1/2 (if representable)
```

## Mathematical Constants

Pre-defined constants are available:

```rust
let pi = Expression::pi();           // π
let e = Expression::e();             // e
let i = Expression::i();             // i (imaginary unit)
let phi = Expression::golden_ratio(); // φ = (1 + √5) / 2
let gamma = Expression::euler_gamma(); // γ (Euler-Mascheroni constant)
```

## Next Steps

- [Functions](./functions.md)
- [Constants](./constants.md)
- [Mathematical Operations](../operations/simplification.md)












## Examples


### Symbol Creation and Equality

Creating symbols with string interning for O(1) equality checks


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x1 = symbol!(x);
let x2 = symbol!(x);
let y = symbol!(y);

// O(1) pointer comparison
assert_eq!(x1, x2);
assert_ne!(x1, y);

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol

x1 = symbol('x')
x2 = symbol('x')
y = symbol('y')

# Fast equality check
assert x1 == x2
assert x1 != y

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol } = require('mathhook-node');

const x1 = symbol('x');
const x2 = symbol('x');
const y = symbol('y');

// Fast equality check
console.assert(x1.equals(x2));
console.assert(!x1.equals(y));

```
</details>





### Exact Rational Arithmetic

Using rationals for exact fractional computation


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

// Exact: 1/3
let third = Expression::rational(1, 3);
let result = expr!(3 * third);
assert_eq!(result, Expression::integer(1));

// Auto-reduction: 6/4 = 3/2
let frac = Expression::rational(6, 4);
assert_eq!(frac, Expression::rational(3, 2));

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import Expression, expr

# Exact: 1/3
third = Expression.rational(1, 3)
result = expr('3 * third')
assert result == Expression.integer(1)

# Auto-reduction: 6/4 = 3/2
frac = Expression.rational(6, 4)
assert frac == Expression.rational(3, 2)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { Expression, expr } = require('mathhook-node');

// Exact: 1/3
const third = Expression.rational(1, 3);
const result = expr('3 * third');
console.assert(result.equals(Expression.integer(1)));

// Auto-reduction: 6/4 = 3/2
const frac = Expression.rational(6, 4);
console.assert(frac.equals(Expression.rational(3, 2)));

```
</details>





### Complex Numbers

Working with complex numbers and imaginary unit


<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

// 3 + 4i
let z = Expression::complex(
    Expression::integer(3),
    Expression::integer(4)
);

// Magnitude: |z| = sqrt(3^2 + 4^2) = 5
let magnitude = expr!(sqrt((3^2) + (4^2)));
assert_eq!(magnitude.simplify(), Expression::integer(5));

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from mathhook import Expression, expr

# 3 + 4i
z = Expression.complex(3, 4)

# Magnitude: |z| = 5
magnitude = expr('sqrt(3^2 + 4^2)')
assert magnitude.simplify() == Expression.integer(5)

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const { Expression, expr } = require('mathhook-node');

// 3 + 4i
const z = Expression.complex(3, 4);

// Magnitude: |z| = 5
const magnitude = expr('sqrt(3^2 + 4^2)');
console.assert(magnitude.simplify().equals(Expression.integer(5)));

```
</details>







## Performance

**Time Complexity**: O(1) for symbol creation/comparison, O(1) for number operations


## API Reference

- **Rust**: `mathhook_core::expression::Symbol, mathhook_core::number::Number`
- **Python**: `mathhook.Symbol, mathhook.Number`
- **JavaScript**: `mathhook-node.Symbol, mathhook-node.Number`


## See Also


- [core.expressions](../core/expressions.md)

- [core.constants](../core/constants.md)

- [core.functions](../core/functions.md)


