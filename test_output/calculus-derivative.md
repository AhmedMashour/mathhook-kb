# Symbolic Differentiation

> **Topic**: `calculus.derivative`

Computes the derivative of an expression with respect to a variable using
symbolic differentiation rules. Supports power rule, product rule, quotient
rule, chain rule, and derivatives of elementary functions.



## Mathematical Definition

$$$$\frac{d}{dx} f(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$$
$$



Have you ever wondered how fast things change? Whether it's the speed of a car,
the growth rate of a population, or the slope of a curve at a specific point,
derivatives are the mathematical tool that answers these questions.

In this guide, we'll explore symbolic differentiation with MathHook - a powerful
way to compute derivatives **exactly**, without numerical approximations. By the
end, you'll be computing derivatives of complex functions with just a few lines
of code!



## What is a Derivative?

At its core, a **derivative** measures how a function changes as its input changes.
Geometrically, it's the slope of the tangent line to the function's curve at a point.

Imagine you're driving a car and your position is given by the function f(t).
The derivative f'(t) tells you your **velocity** - how quickly your position is changing.
The derivative of velocity, f''(t), gives you **acceleration** - how quickly your
speed is changing!

### The Formal Definition

For a function f(x), the derivative at a point x is defined as:

$$f'(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$$

This says: "As we make h infinitesimally small, what's the ratio of the change in
f to the change in x?" That ratio is the instantaneous rate of change - the derivative.


## The Power Rule

The simplest and most fundamental rule is the **power rule**:

**If** f(x) = x^n, **then** f'(x) = n·x^(n-1)

This rule says: "Bring down the exponent, then reduce the exponent by one."

### Examples:
- f(x) = x² → f'(x) = 2x
- f(x) = x³ → f'(x) = 3x²
- f(x) = x^10 → f'(x) = 10x^9
- f(x) = 1/x = x^(-1) → f'(x) = -x^(-2) = -1/x²


## The Chain Rule

What if you have a **composition** of functions, like f(g(x))? That's where the
**chain rule** comes in!

**Chain Rule**: If y = f(g(x)), then dy/dx = f'(g(x)) · g'(x)

In words: "Derivative of the outer function (evaluated at the inner function)
times the derivative of the inner function."

### Example: sin(x²)

Here, the outer function is sin(u) and the inner function is u = x².
- Outer derivative: d/du[sin(u)] = cos(u)
- Inner derivative: d/dx[x²] = 2x
- Chain rule: d/dx[sin(x²)] = cos(x²) · 2x = 2x·cos(x²)


## The Product Rule

When you have a **product** of two functions u(x)·v(x), you can't just multiply
their derivatives. Instead, use the **product rule**:

**(uv)' = u'v + uv'**

In words: "Derivative of the first times the second, plus the first times
the derivative of the second."




---

## Deep Dives


### Implementation Details

MathHook implements differentiation using a recursive pattern-matching
approach. The `derivative()` method on `Expression` matches against each
expression type (Add, Mul, Pow, Function, etc.) and applies the appropriate
differentiation rule.

For example, the power rule is implemented as:

```rust
Expression::Pow(base, exp) => {
    // Power rule: d/dx[x^n] = n*x^(n-1)
    if exp.is_constant() {
        // Simple case: x^n
        return multiply(vec![
            exp.clone(),
            power(base.clone(), subtract(exp.clone(), one())),
            base.derivative(var, order)
        ]);
    }
    // Complex case: f(x)^g(x) requires logarithmic differentiation
    // ...
}
```


### Automatic Simplification

After computing a derivative, MathHook automatically simplifies the result
to produce cleaner expressions. This includes:
- Combining like terms (2x + 3x → 5x)
- Eliminating zero terms (f(x) + 0 → f(x))
- Canceling common factors
- Applying trigonometric identities where beneficial





---

## Implementation Notes

The differentiation engine is located in `crates/mathhook-core/src/calculus/derivatives/`.
It supports higher-order derivatives through the `order` parameter.




---

## Complexity Analysis

**Time Complexity**: O(n) where n is the number of nodes in the expression tree.
Each node is visited exactly once during differentiation.

**Space Complexity**: O(d) where d is the maximum depth of the expression tree
(for the recursion stack).





## Examples


### Power Rule

The derivative of x^n is n·x^(n-1). This is the most fundamental rule.


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

When differentiating a composition of functions f(g(x)), use the chain rule:
(f∘g)'(x) = f'(g(x)) · g'(x)


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

For the product of two functions u(x)·v(x), the derivative is u'(x)·v(x) + u(x)·v'(x)


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


