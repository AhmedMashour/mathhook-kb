---









---

# Symbolic Assumptions

> **Topic**: `advanced.assumptions`

Declare properties about symbols to enable better simplification, solving,
and mathematical operations. Assumptions like positivity, reality, integer
constraints improve symbolic manipulation accuracy.





# Symbolic Assumptions

Assumptions allow you to declare mathematical properties about symbols
(positive, real, integer, etc.) to guide MathHook's symbolic engine toward
more accurate simplifications and solutions.

## Why Assumptions Matter

Without assumptions, MathHook treats all symbols as arbitrary complex numbers,
leading to overly conservative results. With assumptions:
- √(x²) can simplify to |x| (real assumption) or x (positive assumption)
- Inequalities can be solved symbolically
- Domain restrictions are enforced

## Current Status

**Planned Feature**: Full assumption system is under development.

## Expected API (Future)

```rust
let x = symbol!(x).assume_positive();
let y = symbol!(y).assume_real();
let n = symbol!(n).assume_integer();

let expr = expr!(sqrt(x^2));
// With x > 0: simplifies to x
// Without: stays as sqrt(x^2) or becomes |x|
```

## Assumption Types (Planned)

- **Positivity**: x > 0, x ≥ 0, x < 0, x ≤ 0
- **Reality**: x ∈ ℝ (real numbers)
- **Integer**: n ∈ ℤ
- **Rational**: q ∈ ℚ
- **Natural**: n ∈ ℕ (positive integers)
- **Finiteness**: x is finite (not infinity)
- **Primality**: p is prime (for number theory)

## Use Cases

### Simplification
With assumptions, expressions simplify more aggressively:
- √(x²) → x (if x > 0)
- |x| → x (if x > 0)
- x/x → 1 (if x ≠ 0)

### Solving
Inequalities and conditional solutions:
- Solve x² > 4 given x > 0 → x > 2
- Solve log(x) = 2 → x = e² (with x > 0 automatic)

### Integration
Domain restrictions for improper integrals:
- ∫₀^∞ e^(-ax) dx requires a > 0

### Series Expansion
Convergence conditions:
- Taylor series valid for |x| < R

## Relation to SymPy/SageMath

MathHook's assumption system will follow similar patterns to SymPy's
`assume()` API but integrated directly into the symbol creation macros.












## Examples





## API Reference

- **Rust**: `mathhook_core::assumptions`
- **Python**: `mathhook.assumptions`
- **JavaScript**: `mathhook.assumptions`


## See Also


- [core.symbols](../core/symbols.md)

- [operations.simplification](../operations/simplification.md)

- [operations.solving](../operations/solving.md)

- [advanced.piecewise](../advanced/piecewise.md)


