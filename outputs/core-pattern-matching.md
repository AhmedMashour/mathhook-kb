---









---

# Pattern Matching

> **Topic**: `core.pattern-matching`

Pattern matching is a powerful technique in MathHook for identifying, transforming,
and simplifying mathematical expressions. MathHook combines Rust's native pattern
matching with specialized mathematical pattern recognition to enable sophisticated
symbolic manipulation including algebraic identities, calculus rules, and
trigonometric transformations.



## Mathematical Definition

$$**Common Mathematical Patterns:**

- **Difference of Squares**: $$a^2 - b^2 = (a + b)(a - b)$$
- **Perfect Square**: $$a^2 + 2ab + b^2 = (a + b)^2$$
- **Power Rule**: $$\frac{d}{dx}(x^n) = nx^{n-1}$$
- **Chain Rule**: $$\frac{d}{dx}f(g(x)) = f'(g(x)) \cdot g'(x)$$
- **Pythagorean Identity**: $$\sin^2(x) + \cos^2(x) = 1$$
$$




## Examples


### Rust Native Pattern Matching on Expressions

Using Rust's match statement to analyze expression structure

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

fn analyze_expression(expr: &Expression) -> String {
    match expr {
        Expression::Integer(n) => format!("Integer: {}", n),
        Expression::Symbol(s) => format!("Variable: {}", s.name()),
        Expression::Add(terms) => format!("Sum of {} terms", terms.len()),
        Expression::Mul(factors) => format!("Product of {} factors", factors.len()),
        Expression::Pow(base, exp) => format!("Power: ({})^({})", base, exp),
        Expression::Function { name, args } => {
            format!("Function {}: {} args", name, args.len())
        }
        _ => "Other expression type".to_string(),
    }
}

let expr = expr!(x^2 + 2*x + 1);
println!("{}", analyze_expression(&expr));

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import Expression, expr

def analyze_expression(e):
    if e.is_integer():
        return f"Integer: {e.value()}"
    elif e.is_symbol():
        return f"Variable: {e.name()}"
    elif e.is_add():
        return f"Sum of {len(e.terms())} terms"
    elif e.is_mul():
        return f"Product of {len(e.factors())} factors"
    elif e.is_pow():
        return f"Power: ({e.base()})^({e.exponent()})"
    elif e.is_function():
        return f"Function {e.name()}: {len(e.args())} args"
    else:
        return "Other expression type"

expr_val = expr('x^2 + 2*x + 1')
print(analyze_expression(expr_val))

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { Expression, expr } = require('mathhook-node');

function analyzeExpression(e) {
    if (e.isInteger()) {
        return `Integer: ${e.value()}`;
    } else if (e.isSymbol()) {
        return `Variable: ${e.name()}`;
    } else if (e.isAdd()) {
        return `Sum of ${e.terms().length} terms`;
    } else if (e.isMul()) {
        return `Product of ${e.factors().length} factors`;
    } else if (e.isPow()) {
        return `Power: (${e.base()})^(${e.exponent()})`;
    } else if (e.isFunction()) {
        return `Function ${e.name()}: ${e.args().length} args`;
    }
    return 'Other expression type';
}

const exprVal = expr('x^2 + 2*x + 1');
console.log(analyzeExpression(exprVal));

```
</details>




### Algebraic Pattern: Difference of Squares

Recognizing and factoring difference of squares pattern

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let a = symbol!(a);
let b = symbol!(b);

// Pattern: a² - b² = (a + b)(a - b)
let diff_squares = expr!(a^2 - b^2);
let factored = diff_squares.factor();
assert_eq!(factored, expr!((a + b) * (a - b)));

// Recognizes in complex forms
let x = symbol!(x);
let example = expr!(x^4 - 16);
let factored_example = example.factor();
// (x² + 4)(x² - 4)
assert_eq!(factored_example, expr!((x^2 + 4) * (x^2 - 4)));

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr

a = symbol('a')
b = symbol('b')

# Pattern: a² - b² = (a + b)(a - b)
diff_squares = expr('a^2 - b^2')
factored = diff_squares.factor()
assert factored == expr('(a + b) * (a - b)')

# Complex forms
x = symbol('x')
example = expr('x^4 - 16')
factored_example = example.factor()
assert factored_example == expr('(x^2 + 4) * (x^2 - 4)')

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook-node');

const a = symbol('a');
const b = symbol('b');

// Pattern: a² - b² = (a + b)(a - b)
const diffSquares = expr('a^2 - b^2');
const factored = diffSquares.factor();
console.assert(factored.equals(expr('(a + b) * (a - b)')));

// Complex forms
const x = symbol('x');
const example = expr('x^4 - 16');
const factoredExample = example.factor();
console.assert(factoredExample.equals(expr('(x^2 + 4) * (x^2 - 4)')));

```
</details>




### Calculus Pattern: Power Rule Derivative

Automatic pattern recognition for power rule differentiation

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Pattern: d/dx(x^n) = n*x^(n-1)
let f = expr!(x^5);
let df = f.derivative(&x, 1);
assert_eq!(df, expr!(5 * x^4));

// Works for any power
let sqrt_x = expr!(x^(1/2));
let d_sqrt = sqrt_x.derivative(&x, 1);
assert_eq!(d_sqrt, expr!((1/2) * x^(-1/2)));

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr

x = symbol('x')

# Pattern: d/dx(x^n) = n*x^(n-1)
f = expr('x^5')
df = f.derivative(x)
assert df == expr('5 * x^4')

# Works for any power
sqrt_x = expr('x^(1/2)')
d_sqrt = sqrt_x.derivative(x)
assert d_sqrt == expr('(1/2) * x^(-1/2)')

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook-node');

const x = symbol('x');

// Pattern: d/dx(x^n) = n*x^(n-1)
const f = expr('x^5');
const df = f.derivative(x);
console.assert(df.equals(expr('5 * x^4')));

// Works for any power
const sqrtX = expr('x^(1/2)');
const dSqrt = sqrtX.derivative(x);
console.assert(dSqrt.equals(expr('(1/2) * x^(-1/2)')));

```
</details>




### Calculus Pattern: Chain Rule

Automatic chain rule application for composite functions

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Pattern: d/dx f(g(x)) = f'(g(x)) * g'(x)
let f = expr!(sin(x^2));
// sin(u) where u = x²
// Derivative: cos(u) * du/dx = cos(x²) * 2x
let df = f.derivative(&x, 1);
assert_eq!(df, expr!(2*x*cos(x^2)));

// Nested composition
let nested = expr!(sin(cos(x)));
let d_nested = nested.derivative(&x, 1);
// cos(cos(x)) * (-sin(x))
assert_eq!(d_nested, expr!(-sin(x)*cos(cos(x))));

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr

x = symbol('x')

# Pattern: chain rule
f = expr('sin(x^2)')
df = f.derivative(x)
assert df == expr('2*x*cos(x^2)')

# Nested composition
nested = expr('sin(cos(x))')
d_nested = nested.derivative(x)
assert d_nested == expr('-sin(x)*cos(cos(x))')

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook-node');

const x = symbol('x');

// Chain rule
const f = expr('sin(x^2)');
const df = f.derivative(x);
console.assert(df.equals(expr('2*x*cos(x^2)')));

// Nested composition
const nested = expr('sin(cos(x))');
const dNested = nested.derivative(x);
console.assert(dNested.equals(expr('-sin(x)*cos(cos(x))')));

```
</details>




### Trigonometric Pattern: Pythagorean Identity

Automatic simplification using trigonometric identities

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let x = symbol!(x);

// Pattern: sin²(x) + cos²(x) = 1
let identity = expr!(sin(x)^2 + cos(x)^2);
assert_eq!(identity.simplify(), expr!(1));

// Pattern: 1 + tan²(x) = sec²(x)
let tan_identity = expr!(1 + tan(x)^2);
assert_eq!(tan_identity.simplify(), expr!(sec(x)^2));

// Pattern: 1 + cot²(x) = csc²(x)
let cot_identity = expr!(1 + cot(x)^2);
assert_eq!(cot_identity.simplify(), expr!(csc(x)^2));

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr

x = symbol('x')

# Pythagorean identities
identity = expr('sin(x)^2 + cos(x)^2')
assert identity.simplify() == 1

tan_identity = expr('1 + tan(x)^2')
assert tan_identity.simplify() == expr('sec(x)^2')

cot_identity = expr('1 + cot(x)^2')
assert cot_identity.simplify() == expr('csc(x)^2')

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook-node');

const x = symbol('x');

// Pythagorean identities
const identity = expr('sin(x)^2 + cos(x)^2');
console.assert(identity.simplify().equals(1));

const tanIdentity = expr('1 + tan(x)^2');
console.assert(tanIdentity.simplify().equals(expr('sec(x)^2')));

const cotIdentity = expr('1 + cot(x)^2');
console.assert(cotIdentity.simplify().equals(expr('csc(x)^2')));

```
</details>




### Logarithm Pattern: Log Laws

Automatic application of logarithm expansion and combination rules

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

let a = symbol!(a);
let b = symbol!(b);
let n = symbol!(n);

// Pattern: ln(a*b) = ln(a) + ln(b)
let log_product = expr!(ln(a*b));
assert_eq!(log_product.expand(), expr!(ln(a) + ln(b)));

// Pattern: ln(a/b) = ln(a) - ln(b)
let log_quotient = expr!(ln(a/b));
assert_eq!(log_quotient.expand(), expr!(ln(a) - ln(b)));

// Pattern: ln(a^n) = n*ln(a)
let log_power = expr!(ln(a^n));
assert_eq!(log_power.expand(), expr!(n*ln(a)));

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import symbol, expr

a = symbol('a')
b = symbol('b')
n = symbol('n')

# Log laws
log_product = expr('ln(a*b)')
assert log_product.expand() == expr('ln(a) + ln(b)')

log_quotient = expr('ln(a/b)')
assert log_quotient.expand() == expr('ln(a) - ln(b)')

log_power = expr('ln(a^n)')
assert log_power.expand() == expr('n*ln(a)')

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { symbol, expr } = require('mathhook-node');

const a = symbol('a');
const b = symbol('b');
const n = symbol('n');

// Log laws
const logProduct = expr('ln(a*b)');
console.assert(logProduct.expand().equals(expr('ln(a) + ln(b)')));

const logQuotient = expr('ln(a/b)');
console.assert(logQuotient.expand().equals(expr('ln(a) - ln(b)')));

const logPower = expr('ln(a^n)');
console.assert(logPower.expand().equals(expr('n*ln(a)')));

```
</details>




### Custom Pattern Matcher: Polynomial Detection

Implementing custom pattern recognition for polynomial expressions

<details>
<summary><b>Rust</b></summary>

```rust
use mathhook::prelude::*;

/// Pattern matcher for polynomial expressions
fn is_polynomial(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        // Constant term
        Expression::Integer(_) | Expression::Rational(_) => true,

        // Variable itself
        Expression::Symbol(s) if s == var => true,

        // Power of variable
        Expression::Pow(base, exp) => {
            matches!(**base, Expression::Symbol(ref s) if s == var) &&
            matches!(**exp, Expression::Integer(n) if n >= 0)
        }

        // Sum or product of polynomials
        Expression::Add(terms) | Expression::Mul(factors) => {
            terms.iter().all(|t| is_polynomial(t, var)) ||
            factors.iter().all(|f| is_polynomial(f, var))
        }

        _ => false,
    }
}

let x = symbol!(x);
assert!(is_polynomial(&expr!(x^2 + 3*x + 1), &x));
assert!(!is_polynomial(&expr!(sin(x)), &x));

```
</details>

<details>
<summary><b>Python</b></summary>

```python
from mathhook import Expression, symbol, expr

def is_polynomial(e, var):
    if e.is_integer() or e.is_rational():
        return True
    elif e.is_symbol():
        return e == var
    elif e.is_pow():
        base = e.base()
        exp = e.exponent()
        return (base.is_symbol() and base == var and
                exp.is_integer() and exp.value() >= 0)
    elif e.is_add() or e.is_mul():
        terms = e.terms() if e.is_add() else e.factors()
        return all(is_polynomial(t, var) for t in terms)
    return False

x = symbol('x')
assert is_polynomial(expr('x^2 + 3*x + 1'), x)
assert not is_polynomial(expr('sin(x)'), x)

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const { Expression, symbol, expr } = require('mathhook-node');

function isPolynomial(e, varSym) {
    if (e.isInteger() || e.isRational()) {
        return true;
    } else if (e.isSymbol()) {
        return e.equals(varSym);
    } else if (e.isPow()) {
        const base = e.base();
        const exp = e.exponent();
        return base.isSymbol() && base.equals(varSym) &&
               exp.isInteger() && exp.value() >= 0;
    } else if (e.isAdd() || e.isMul()) {
        const terms = e.isAdd() ? e.terms() : e.factors();
        return terms.every(t => isPolynomial(t, varSym));
    }
    return false;
}

const x = symbol('x');
console.assert(isPolynomial(expr('x^2 + 3*x + 1'), x));
console.assert(!isPolynomial(expr('sin(x)'), x));

```
</details>






## Performance

**Time Complexity**: O(n) for pattern matching on expression tree of size n


## API Reference

- **Rust**: `mathhook_core::pattern`
- **Python**: `mathhook.pattern`
- **JavaScript**: `mathhook-node.pattern`


## See Also


- [core.expressions](../core/expressions.md)

- [operations.simplification](../operations/simplification.md)

- [operations.differentiation](../operations/differentiation.md)

- [operations.integration](../operations/integration.md)

- [operations.expansion-factoring](../operations/expansion-factoring.md)


