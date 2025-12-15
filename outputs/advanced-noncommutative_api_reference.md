---









---

# Noncommutative Algebra API Reference

> **Topic**: `advanced.noncommutative_api_reference`

Complete API reference for MathHook's noncommutative algebra support,
including symbol creation macros, type queries, and equation solving.






## Examples


### Symbol Creation Macros

Create symbols with different types

<details>
<summary><b>Rust</b></summary>

```rust
// Scalar (default)
let x = symbol!(x);

// Matrix (noncommutative)
let A = symbol!(A; matrix);

// Operator (noncommutative)
let p = symbol!(p; operator);

// Quaternion (noncommutative)
let i = symbol!(i; quaternion);

```
</details>

<details>
<summary><b>Python</b></summary>

```python
# Scalar
x = symbols('x')

# Matrix
A = MatrixSymbol('A', n, n)

# Operator (quantum mechanics)
p = Operator('p')

# Quaternion
from sympy.algebras.quaternion import Quaternion
i = Quaternion(0, 1, 0, 0)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
// Scalar
const x = symbol('x');

// Matrix
const A = symbol('A', {type: 'matrix'});

// Operator
const p = symbol('p', {type: 'operator'});

// Quaternion
const i = symbol('i', {type: 'quaternion'});

```
</details>




### Bulk Symbol Creation

Create multiple symbols at once

<details>
<summary><b>Rust</b></summary>

```rust
// Multiple scalars
let scalars = symbols![x, y, z];

// Multiple matrices
let matrices = symbols![A, B, C => matrix];

// Multiple operators
let operators = symbols![p, x, H => operator];

```
</details>

<details>
<summary><b>Python</b></summary>

```python
# Multiple scalars
x, y, z = symbols('x y z')

# Multiple matrices
A, B, C = symbols('A B C', cls=MatrixSymbol)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
// Multiple symbols
const [x, y, z] = symbols(['x', 'y', 'z']);

// Multiple matrices
const [A, B, C] = symbols(['A', 'B', 'C'], {type: 'matrix'});

```
</details>




### Type Queries

Check symbol type and commutativity

<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);
let A = symbol!(A; matrix);

// Type check
assert_eq!(x.symbol_type(), SymbolType::Scalar);
assert_eq!(A.symbol_type(), SymbolType::Matrix);

// Commutativity check
assert_eq!(x.commutativity(), Commutativity::Commutative);
assert_eq!(A.commutativity(), Commutativity::Noncommutative);

```
</details>

<details>
<summary><b>Python</b></summary>

```python
x = symbols('x')
A = MatrixSymbol('A', n, n)

# Type check
print(type(x))  # Symbol
print(type(A))  # MatrixSymbol

# Commutativity (implicit in type)
print(A.is_commutative)  # False

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');
const A = symbol('A', {type: 'matrix'});

// Type check
console.log(x.type);  // 'scalar'
console.log(A.type);  // 'matrix'

// Commutativity
console.log(x.is_commutative);  // true
console.log(A.is_commutative);  // false

```
</details>







## API Reference

- **Rust**: `mathhook_core::noncommutative::api`
- **Python**: `mathhook.noncommutative.api`
- **JavaScript**: `mathhook.noncommutative.api`


## See Also


- [advanced.noncommutative_algebra](../advanced/noncommutative_algebra.md)

- [advanced.noncommutative_examples](../advanced/noncommutative_examples.md)

- [core.symbols](../core/symbols.md)

- [core.macros](../core/macros.md)


