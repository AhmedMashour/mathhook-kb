---









---

# Matrix Operations

> **Topic**: `advanced.matrices`

Work with matrices symbolically and numerically in MathHook, with full support
for noncommutative algebra where order matters. Create matrices, perform
operations, and solve matrix equations.



## Mathematical Definition

Matrix multiplication: For $A_{m \times n}$ and $B_{n \times p}$:
$$C_{ij} = \sum_{k=1}^{n} A_{ik} B_{kj}$$

Matrix inverse: $A \times A^{-1} = A^{-1} \times A = I$

Determinant (2×2): $$\det\begin{pmatrix} a & b \\ c & d \end{pmatrix} = ad - bc$$





## Examples


### Creating Matrices

Create matrix symbols and numeric matrices


<details>
<summary><b>Rust</b></summary>

```rust
// Matrix symbols (noncommutative)
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Numeric 2×2 matrix
let M = Expression::matrix(vec![
    vec![expr!(1), expr!(2)],
    vec![expr!(3), expr!(4)],
]);

```
</details>



<details>
<summary><b>Python</b></summary>

```python
# Matrix symbols
A = MatrixSymbol('A', n, m)
B = MatrixSymbol('B', m, p)

# Numeric matrix
M = Matrix([[1, 2], [3, 4]])

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
// Matrix symbols
const A = symbol('A', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

// Numeric matrix
const M = matrix([[1, 2], [3, 4]]);

```
</details>





### Matrix Multiplication (Noncommutative)

A*B ≠ B*A in general


<details>
<summary><b>Rust</b></summary>

```rust
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

let AB = expr!(A * B);  // A*B
let BA = expr!(B * A);  // B*A ≠ A*B

```
</details>



<details>
<summary><b>Python</b></summary>

```python
A = MatrixSymbol('A', n, n)
B = MatrixSymbol('B', n, n)

AB = A * B  # Matrix product
BA = B * A  # Different result!

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const A = symbol('A', {type: 'matrix'});
const B = symbol('B', {type: 'matrix'});

const AB = A.mul(B);  // A*B
const BA = B.mul(A);  // B*A ≠ A*B

```
</details>





### Solving Linear System Ax=b

Solve matrix equation using inverse


<details>
<summary><b>Rust</b></summary>

```rust
let A = Expression::matrix(vec![
    vec![expr!(2), expr!(1)],
    vec![expr!(1), expr!(-1)],
]);
let b = Expression::matrix(vec![
    vec![expr!(5)],
    vec![expr!(1)],
]);

// Solution: x = A^(-1)*b
let x = expr!(A^(-1) * b);
// Result: [[2], [1]]

```
</details>



<details>
<summary><b>Python</b></summary>

```python
A = Matrix([[2, 1], [1, -1]])
b = Matrix([[5], [1]])

# Solution
x = A.inv() * b
# Result: Matrix([[2], [1]])

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const A = matrix([[2, 1], [1, -1]]);
const b = matrix([[5], [1]]);

// Solution
const x = A.inv().mul(b);
// Result: [[2], [1]]

```
</details>





### Matrix Equation A*X=B (Left Division)

Solve for matrix unknown X


<details>
<summary><b>Rust</b></summary>

```rust
let solver = MatrixEquationSolver::new();
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

let equation = expr!((A * X) - B);
let solution = solver.solve(&equation, &X);
// Returns: X = A^(-1)*B (left division)

```
</details>



<details>
<summary><b>Python</b></summary>

```python
A, X, B = symbols('A X B', matrix=True)
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







## Performance

**Time Complexity**: O(n^3) for n×n matrix operations


## API Reference

- **Rust**: `mathhook_core::matrix`
- **Python**: `mathhook.matrix`
- **JavaScript**: `mathhook.matrix`


## See Also


- [advanced.noncommutative_algebra](../advanced/noncommutative_algebra.md)

- [advanced.system_solving](../advanced/system_solving.md)

- [core.symbols](../core/symbols.md)

- [operations.solving](../operations/solving.md)


