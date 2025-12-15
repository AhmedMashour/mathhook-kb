---









---

# Piecewise Functions

> **Topic**: `advanced.piecewise`

Define functions with different formulas in different regions, essential for
modeling discontinuous behavior, conditional logic, step functions, and
threshold-based systems.



## Mathematical Definition

Piecewise function:
$$f(x) = \begin{cases}
f_1(x) & \text{if } C_1(x) \\
f_2(x) & \text{if } C_2(x) \\
\vdots & \\
f_n(x) & \text{if } C_n(x) \\
f_{\text{default}} & \text{otherwise}
\end{cases}$$





## Examples


### Absolute Value Function

|x| = { x if x ≥ 0, -x if x < 0 }


<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);

let abs_x = Expression::piecewise(
    vec![
        (expr!(x), expr!(x >= 0)),
        (expr!(-x), expr!(x < 0)),
    ],
    None,
);

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from sympy import symbols, Piecewise

x = symbols('x')
abs_x = Piecewise((x, x >= 0), (-x, x < 0))

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');

const abs_x = piecewise([
    [x, ge(x, 0)],
    [neg(x), lt(x, 0)]
]);

```
</details>





### Heaviside Step Function

H(x) = { 0 if x < 0, 1 if x ≥ 0 }


<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);

let heaviside = Expression::piecewise(
    vec![
        (expr!(0), expr!(x < 0)),
        (expr!(1), expr!(x >= 0)),
    ],
    None,
);

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from sympy import symbols, Heaviside

x = symbols('x')
H = Heaviside(x)  # Built-in Heaviside function

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');

const H = piecewise([
    [0, lt(x, 0)],
    [1, ge(x, 0)]
]);

```
</details>





### Tax Bracket Example

Progressive tax with income thresholds


<details>
<summary><b>Rust</b></summary>

```rust
let income = symbol!(income);

// 10% on first $10k, 12% on next $30k, 22% on remainder
let tax = Expression::piecewise(
    vec![
        (expr!(0.10 * income), expr!(income <= 10000)),
        (expr!(1000 + 0.12 * (income - 10000)), expr!(income <= 40000)),
    ],
    Some(expr!(4600 + 0.22 * (income - 40000))),
);

// Calculate tax for $50,000
let tax_owed = tax.substitute(&income, &expr!(50000));
// Result: 4600 + 0.22 * 10000 = $6,800

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from sympy import symbols, Piecewise

income = symbols('income')

tax = Piecewise(
    (0.10 * income, income <= 10000),
    (1000 + 0.12 * (income - 10000), income <= 40000),
    (4600 + 0.22 * (income - 40000), True)
)

tax_owed = tax.subs(income, 50000)
# Result: 6800

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const income = symbol('income');

const tax = piecewise([
    [mul(0.10, income), le(income, 10000)],
    [add(1000, mul(0.12, sub(income, 10000))), le(income, 40000)],
    [add(4600, mul(0.22, sub(income, 40000))), true]
]);

const tax_owed = tax.subs(income, 50000);

```
</details>





### Differentiation of Piecewise

Derivative computed piece-by-piece


<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);

// f(x) = { x^2 if x ≥ 0, -x^2 if x < 0 }
let f = Expression::piecewise(
    vec![
        (expr!(x^2), expr!(x >= 0)),
        (expr!(-x^2), expr!(x < 0)),
    ],
    None,
);

// Derivative
let df = f.derivative(&x, 1);
// Result: { 2x if x ≥ 0, -2x if x < 0 }

```
</details>



<details>
<summary><b>Python</b></summary>

```python
from sympy import symbols, Piecewise, diff

x = symbols('x')
f = Piecewise((x**2, x >= 0), (-x**2, x < 0))

df = diff(f, x)
# Result: Piecewise((2*x, x > 0), (-2*x, x < 0))

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');

const f = piecewise([
    [pow(x, 2), ge(x, 0)],
    [neg(pow(x, 2)), lt(x, 0)]
]);

const df = diff(f, x);

```
</details>








## API Reference

- **Rust**: `mathhook_core::piecewise`
- **Python**: `mathhook.piecewise`
- **JavaScript**: `mathhook.piecewise`


## See Also


- [core.functions](../core/functions.md)

- [operations.solving](../operations/solving.md)

- [operations.differentiation](../operations/differentiation.md)

- [operations.integration](../operations/integration.md)

- [advanced.assumptions](../advanced/assumptions.md)


