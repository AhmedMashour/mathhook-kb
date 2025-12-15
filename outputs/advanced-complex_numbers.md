---









---

# Complex Number Operations

> **Topic**: `advanced.complex_numbers`

Work with complex numbers in MathHook including imaginary unit i, complex
arithmetic, polar form, Euler's formula, and operations like conjugate,
magnitude, and argument.



## Mathematical Definition

$$Complex number: $$z = a + bi$$ where $a, b \in \mathbb{R}$ and $i^2 = -1$

Polar form: $$z = r e^{i\theta} = r(\cos\theta + i\sin\theta)$$

where $r = |z| = \sqrt{a^2 + b^2}$ and $\theta = \arg(z) = \arctan(b/a)$
$$



# Complex Number Operations

MathHook provides comprehensive support for complex number arithmetic,
conversions between rectangular and polar forms, and complex functions.

## Creating Complex Numbers

```rust
use mathhook::Expression;

// Imaginary unit
let i = Expression::i();

// Complex number: 3 + 4i
let z = expr!(3 + 4*i);

// Pure imaginary: 5i
let w = expr!(5*i);
```

## Operations

### Addition/Subtraction
Component-wise: (a + bi) ± (c + di) = (a ± c) + (b ± d)i

### Multiplication
(a + bi)(c + di) = (ac - bd) + (ad + bc)i

### Division
Division by conjugate multiplication

### Conjugate
conj(a + bi) = a - bi

### Magnitude
|a + bi| = √(a² + b²)

### Argument
arg(a + bi) = arctan(b/a)












## Examples


### Basic Complex Arithmetic



<details>
<summary><b>Rust</b></summary>

```rust
let i = Expression::i();
let z1 = expr!(3 + 4*i);
let z2 = expr!(1 - 2*i);

let sum = expr!(z1 + z2);      // 4 + 2i
let product = expr!(z1 * z2);  // 11 - 2i

```
</details>

<details>
<summary><b>Python</b></summary>

```python
i = expr('I')
z1 = expr('3 + 4*I')
z2 = expr('1 - 2*I')

sum_z = z1 + z2       # 4 + 2*I
product = z1 * z2     # 11 - 2*I

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const i = expr('I');
const z1 = expr('3 + 4*I');
const z2 = expr('1 - 2*I');

const sum = z1.add(z2);      // 4 + 2*I
const product = z1.mul(z2);  // 11 - 2*I

```
</details>




### Euler's Formula



<details>
<summary><b>Rust</b></summary>

```rust
let theta = symbol!(theta);
let euler = expr!(exp(i * theta));

// Expands to: cos(theta) + i*sin(theta)
let expanded = euler.expand();

```
</details>

<details>
<summary><b>Python</b></summary>

```python
theta = symbol('theta')
euler = exp(I * theta)

# Expands to: cos(theta) + I*sin(theta)
expanded = expand(euler)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const theta = symbol('theta');
const euler = exp(mul(I, theta));

// Expands to: cos(theta) + I*sin(theta)
const expanded = euler.expand();

```
</details>




### Polar Form Conversion



<details>
<summary><b>Rust</b></summary>

```rust
let z = expr!(3 + 4*i);

let magnitude = expr!(abs(z));  // 5
let angle = expr!(arg(z));      // arctan(4/3)

// Polar form: r*exp(i*theta)
let polar = expr!(magnitude * exp(i * angle));

```
</details>

<details>
<summary><b>Python</b></summary>

```python
z = expr('3 + 4*I')

magnitude = abs(z)  # 5
angle = arg(z)      # atan(4/3)

# Polar form
polar = magnitude * exp(I * angle)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const z = expr('3 + 4*I');

const magnitude = abs(z);  // 5
const angle = arg(z);      // atan(4/3)

// Polar form
const polar = magnitude.mul(exp(I.mul(angle)));

```
</details>






## Performance

**Time Complexity**: O(1)


## API Reference

- **Rust**: `mathhook_core::complex`
- **Python**: `mathhook.complex`
- **JavaScript**: `mathhook.complex`


## See Also


- [core.numbers](../core/numbers.md)

- [core.functions](../core/functions.md)

- [operations.trigonometry](../operations/trigonometry.md)

- [advanced.special_functions](../advanced/special_functions.md)


