---









---

# Evaluation vs Simplification

> **Topic**: `advanced.evaluation_vs_simplification`

Understand the key differences between evaluation (computing numerical results)
and simplification (algebraic transformation) in MathHook's symbolic engine.





# Evaluation vs Simplification

MathHook provides two distinct operations for transforming expressions:

## Evaluation
- **Purpose**: Compute numerical values
- **Input**: Symbolic expression with concrete values
- **Output**: Numerical result (integer, float, rational)
- **Example**: eval(2 + 3) → 5

## Simplification
- **Purpose**: Algebraic transformation to simpler form
- **Input**: Symbolic expression
- **Output**: Equivalent expression (may still be symbolic)
- **Example**: simplify(x + x) → 2*x

## Key Differences

| Operation | Input | Output | Preserves Symbols |
|-----------|-------|--------|-------------------|
| Evaluate | Symbols → numbers | Numerical | No |
| Simplify | Symbols → symbols | Symbolic | Yes |

## When to Use Each

### Use Evaluation
- Final numerical answer needed
- Plotting or visualization
- Numerical comparison
- Performance-critical paths

### Use Simplification
- Algebraic manipulation
- Pattern matching
- Symbolic solving
- Maintaining exact form












## Examples


### Evaluation Example



<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);
let expr = expr!(x^2 + 2*x + 1);

let evaluated = expr.substitute(&x, &expr!(3));
// Result: 16 (numerical)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
x = symbol('x')
expr_val = x**2 + 2*x + 1

evaluated = expr_val.subs(x, 3)
# Result: 16

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');
const expr = add(pow(x, 2), mul(2, x), 1);

const evaluated = expr.subs(x, 3);
// Result: 16

```
</details>




### Simplification Example



<details>
<summary><b>Rust</b></summary>

```rust
let x = symbol!(x);
let expr = expr!(x + x + x);

let simplified = expr.simplify();
// Result: 3*x (still symbolic)

```
</details>

<details>
<summary><b>Python</b></summary>

```python
x = symbol('x')
expr_val = x + x + x

simplified = simplify(expr_val)
# Result: 3*x

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const x = symbol('x');
const expr = add(x, x, x);

const simplified = expr.simplify();
// Result: 3*x

```
</details>







## API Reference

- **Rust**: `mathhook_core::evaluation`
- **Python**: `mathhook.evaluation`
- **JavaScript**: `mathhook.evaluation`


## See Also


- [operations.simplification](../operations/simplification.md)

- [operations.substitution](../operations/substitution.md)

- [core.expression](../core/expression.md)


