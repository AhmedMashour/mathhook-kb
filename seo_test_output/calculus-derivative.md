---
description: "Compute symbolic derivatives with MathHook's powerful CAS. Supports chain rule, product rule, power rule. Fast, accurate, and easy to use for Rust, Python, and JavaScript."
keywords: symbolic differentiation, derivative calculator, automatic differentiation, calculus library, mathematical derivatives
canonical_url: "https://docs.mathhook.org/calculus/derivative"
og:title: "Symbolic Differentiation - MathHook Computer Algebra System"
og:description: "Professional-grade symbolic differentiation library. Calculate derivatives symbolically with support for chain rule, product rule, and more. Available for Rust, Python, and JavaScript."
og:image: "https://docs.mathhook.org/images/derivatives-og.png"
twitter:card: "summary_large_image"
schema_type: "TechArticle"
language: "en"
---

# Symbolic Differentiation

> **Topic**: `calculus.derivative`

Computes the derivative of an expression with respect to a variable using symbolic mathematics


## Mathematical Definition

$$\frac{d}{dx}f(x)$$




## Examples


### Power Rule

Demonstrates differentiation of polynomial expressions using the power rule

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::expr;
let x = symbol!(x);
let f = expr!(x ^ 3);
let derivative = f.derivative(&x, 1);
assert_eq!(derivative, expr!(3 * (x ^ 2)));

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import expr, symbol
x = symbol('x')
f = expr('x^3')
derivative = f.derivative(x, 1)
assert derivative == expr('3*x^2')

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { expr, symbol } = require('mathhook');
const x = symbol('x');
const f = expr('x^3');
const derivative = f.derivative(x, 1);
console.log(derivative.toString()); // "3*x^2"

```
</details>


**Expected Output:**
```
3*x^2
```



### Chain Rule

Demonstrates differentiation of composite functions using the chain rule

<details>
<summary><b>Rust</b></summary>

```rust
let f = expr!(sin(x ^ 2));
let derivative = f.derivative(&x, 1);
// Returns: 2*x*cos(x^2)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
f = expr('sin(x^2)')
derivative = f.derivative(x, 1)
# Returns: 2*x*cos(x^2)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const f = expr('sin(x^2)');
const derivative = f.derivative(x, 1);
// Returns: 2*x*cos(x^2)

```
</details>


**Expected Output:**
```
2*x*cos(x^2)
```



### Product Rule

Demonstrates differentiation of products using the product rule

<details>
<summary><b>Rust</b></summary>

```rust
let f = expr!(x * sin(x));
let derivative = f.derivative(&x, 1);
// Returns: sin(x) + x*cos(x)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
f = expr('x*sin(x)')
derivative = f.derivative(x, 1)
# Returns: sin(x) + x*cos(x)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const f = expr('x*sin(x)');
const derivative = f.derivative(x, 1);
// Returns: sin(x) + x*cos(x)

```
</details>


**Expected Output:**
```
sin(x) + x*cos(x)
```





## Performance

**Time Complexity**: O(n) where n is the number of nodes in the expression tree


## API Reference

- **Rust**: `mathhook_core::calculus::derivative`
- **Python**: `mathhook.calculus.derivative`
- **JavaScript**: `mathhook.calculus.derivative`


## See Also


- [calculus.integral](../calculus/integral.md)

- [calculus.limit](../calculus/limit.md)

- [algebra.simplify](../algebra/simplify.md)

- [functions.trigonometric](../functions/trigonometric.md)


