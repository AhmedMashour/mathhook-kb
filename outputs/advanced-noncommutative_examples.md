---









---

# Noncommutative Algebra Examples

> **Topic**: `advanced.noncommutative_examples`

Comprehensive examples of noncommutative algebra in MathHook covering
quantum mechanics, matrix algebra, and quaternion rotations.






## Examples


### Quantum Commutator

Position-momentum canonical commutation relation [x,p] = iℏ

<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x; operator);
let p = symbol!(p; operator);

// Commutator: [x, p] = xp - px
let xp = expr!(x * p);
let px = expr!(p * x);
let commutator = expr!(xp - px);

// LaTeX output
let latex = commutator.to_latex(None).unwrap();
// Output: \hat{x}\hat{p} - \hat{p}\hat{x}

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from sympy.physics.quantum import Operator, Commutator

x = Operator('x')
p = Operator('p')

# Commutator
comm = Commutator(x, p)
# Result: I*hbar (in quantum mechanics)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x', {type: 'operator'});
const p = symbol('p', {type: 'operator'});

// Commutator
const xp = x.mul(p);
const px = p.mul(x);
const comm = xp.sub(px);

```
</details>




### Matrix Equation Left Division

Solve A*X = B using left division X = A^(-1)*B

<details>
<summary><b>Rust</b></summary>

```rust
let solver = MatrixEquationSolver::new();
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Equation: A*X - B = 0
let equation = expr!((A * X) - B);

let result = solver.solve(&equation, &X);
// Returns: X = A^(-1)*B (left division)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from sympy import symbols, MatrixSymbol, solve, Eq

A = MatrixSymbol('A', n, n)
X = MatrixSymbol('X', n, n)
B = MatrixSymbol('B', n, n)

# Solve A*X = B
equation = Eq(A*X, B)
solution = solve(equation, X)
# Returns: X = A^(-1)*B

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const A = symbol('A', {type: 'matrix'});
const X = symbol('X', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

const equation = A.mul(X).sub(B);
const solution = solve(equation, X);
// Returns: X = A.inv().mul(B)

```
</details>




### Quaternion Multiplication

Noncommutative quaternion basis multiplication i*j = k, j*i = -k

<details>
<summary><b>Rust</b></summary>

```rust
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);

// i*j = k
let ij = expr!(i * j);

// j*i = -k (different!)
let ji = expr!(j * i);

// Order matters
assert_ne!(ij.to_string(), ji.to_string());

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from sympy.algebras.quaternion import Quaternion

i = Quaternion(0, 1, 0, 0)
j = Quaternion(0, 0, 1, 0)
k = Quaternion(0, 0, 0, 1)

# Verify: i*j = k
assert i * j == k

# Verify: j*i = -k (noncommutative!)
assert j * i == -k

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const i = symbol('i', {type: 'quaternion'});
const j = symbol('j', {type: 'quaternion'});

const ij = i.mul(j);  // k
const ji = j.mul(i);  // -k

```
</details>







## API Reference

- **Rust**: `mathhook_examples::noncommutative`
- **Python**: `mathhook.examples.noncommutative`
- **JavaScript**: `mathhook.examples.noncommutative`


## See Also


- [advanced.noncommutative_algebra](../advanced/noncommutative_algebra.md)

- [advanced.noncommutative_api_reference](../advanced/noncommutative_api_reference.md)

- [advanced.matrices](../advanced/matrices.md)

- [physics.quantum_mechanics](../physics/quantum_mechanics.md)


