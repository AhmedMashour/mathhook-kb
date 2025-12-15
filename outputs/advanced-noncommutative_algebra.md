---









---

# Noncommutative Algebra

> **Topic**: `advanced.noncommutative_algebra`

Support for noncommutative algebra in MathHook with four symbol types:
Scalar (commutative), Matrix, Operator, and Quaternion (all noncommutative).
Essential for quantum mechanics, linear algebra, and 3D rotations.



## Mathematical Definition

$$Noncommutative multiplication: $$A \times B \neq B \times A$$

Symbol types:
- Scalar: $x \cdot y = y \cdot x$ (commutative)
- Matrix: $A \cdot B \neq B \cdot A$ (noncommutative)
- Operator: $[\hat{x}, \hat{p}] = \hat{x}\hat{p} - \hat{p}\hat{x} \neq 0$
- Quaternion: $i \cdot j = k$, but $j \cdot i = -k$
$$




## Examples


### Matrix Symbols

Create noncommutative matrix symbols

<details>
<summary><b>Rust</b></summary>

```rust
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Order matters!
let AB = expr!(A * B);  // A*B
let BA = expr!(B * A);  // B*A ≠ A*B

```
</details>

<details>
<summary><b>Python</b></summary>

```python
A = MatrixSymbol('A', n, n)
B = MatrixSymbol('B', n, n)

AB = A * B  # Noncommutative
BA = B * A  # Different!

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const A = symbol('A', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

const AB = A.mul(B);  // Noncommutative
const BA = B.mul(A);  // Different!

```
</details>




### Quantum Operators

Position and momentum operators (canonical commutation relation)

<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x; operator);
let p = symbol!(p; operator);

// Commutator: [x, p] = xp - px
let commutator = expr!((x * p) - (p * x));
// Physical result: [x, p] = iℏ

```
</details>

<details>
<summary><b>Python</b></summary>

```python
x = Operator('x')
p = Operator('p')

# Commutator
commutator = Commutator(x, p)
# Result: I*hbar

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x', {type: 'operator'});
const p = symbol('p', {type: 'operator'});

// Commutator
const comm = x.mul(p).sub(p.mul(x));

```
</details>




### Quaternions

3D rotation with quaternion multiplication

<details>
<summary><b>Rust</b></summary>

```rust
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);

let ij = expr!(i * j);  // i*j = k
let ji = expr!(j * i);  // j*i = -k (different!)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from sympy.algebras.quaternion import Quaternion

i = Quaternion(0, 1, 0, 0)
j = Quaternion(0, 0, 1, 0)

ij = i * j  # k
ji = j * i  # -k

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




### LaTeX Type Inference

Parser infers types from LaTeX notation

<details>
<summary><b>Rust</b></summary>

```rust
let parser = Parser::new(ParserConfig::default());

// Bold notation → Matrix type
let eq1 = parser.parse(r"\mathbf{A}\mathbf{X} = \mathbf{B}").unwrap();

// Hat notation → Operator type
let eq2 = parser.parse(r"\hat{H}\hat{\psi} = E\hat{\psi}").unwrap();

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from sympy.parsing.latex import parse_latex

# Bold → Matrix
eq1 = parse_latex(r'\mathbf{A}\mathbf{X} = \mathbf{B}')

# Hat → Operator
eq2 = parse_latex(r'\hat{H}\hat{\psi} = E\hat{\psi}')

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const parser = new Parser();

// Bold → Matrix
const eq1 = parser.parse('\\mathbf{A}\\mathbf{X} = \\mathbf{B}');

// Hat → Operator
const eq2 = parser.parse('\\hat{H}\\hat{\\psi} = E\\hat{\\psi}');

```
</details>







## API Reference

- **Rust**: `mathhook_core::noncommutative`
- **Python**: `mathhook.noncommutative`
- **JavaScript**: `mathhook.noncommutative`


## See Also


- [advanced.matrices](../advanced/matrices.md)

- [advanced.noncommutative_api_reference](../advanced/noncommutative_api_reference.md)

- [advanced.noncommutative_examples](../advanced/noncommutative_examples.md)

- [core.symbols](../core/symbols.md)

- [parser.latex](../parser/latex.md)


