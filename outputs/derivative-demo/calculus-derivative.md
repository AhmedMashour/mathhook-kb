---









---

# Symbolic Differentiation

> **Topic**: `calculus.derivative`

Computes the derivative of an expression with respect to a variable using
symbolic differentiation rules. Supports power rule, product rule, quotient
rule, chain rule, and derivatives of elementary functions.



## Mathematical Definition

$$$$\frac{d}{dx} f(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$$
$$




## Examples


### Power Rule

The derivative of x^n is n*x^(n-1). This is one of the fundamental
rules of calculus and applies to any real exponent n.


<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);
let f = expr!(x ^ 3);
let df = f.derivative(&x, 1);
// Result: 3*x^2

```
</details>

<details>
<summary><b>Python</b></summary>

```python
x = symbol('x')
f = expr('x^3')
df = f.derivative(x)
# Result: 3*x^2

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');
const f = expr('x^3');
const df = f.derivative(x);
// Result: 3*x^2

```
</details>


**Expected Output:**
```
3*x^2
```



### Chain Rule

When differentiating a composition of functions f(g(x)), we use the
chain rule: (f∘g)'(x) = f'(g(x)) · g'(x)


<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);
let f = expr!(sin(x ^ 2));
let df = f.derivative(&x, 1);
// Result: 2*x*cos(x^2)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
x = symbol('x')
f = expr('sin(x^2)')
df = f.derivative(x)
# Result: 2*x*cos(x^2)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');
const f = expr('sin(x^2)');
const df = f.derivative(x);
// Result: 2*x*cos(x^2)

```
</details>


**Expected Output:**
```
2*x*cos(x^2)
```



### Product Rule

For the product of two functions u(x)·v(x), the derivative is
u'(x)·v(x) + u(x)·v'(x)


<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);
let f = expr!((x ^ 2) * sin(x));
let df = f.derivative(&x, 1);
// Result: 2*x*sin(x) + x^2*cos(x)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
x = symbol('x')
f = expr('x^2 * sin(x)')
df = f.derivative(x)
# Result: 2*x*sin(x) + x^2*cos(x)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');
const f = expr('x^2 * sin(x)');
const df = f.derivative(x);
// Result: 2*x*sin(x) + x^2*cos(x)

```
</details>


**Expected Output:**
```
2*x*sin(x) + x^2*cos(x)
```





## Performance

**Time Complexity**: O(n)


## API Reference

- **Rust**: `mathhook_core::calculus::derivative`
- **Python**: `mathhook.calculus.derivative`
- **JavaScript**: `mathhook.calculus.derivative`


## See Also


- [calculus.integral](../calculus/integral.md)

- [calculus.chain_rule](../calculus/chain_rule.md)

- [calculus.partial_derivative](../calculus/partial_derivative.md)

- [optimization.gradient_descent](../optimization/gradient_descent.md)

- [calculus.taylor_series](../calculus/taylor_series.md)


