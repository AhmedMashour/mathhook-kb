---









---

# SymPy Validation Workflow

> **Topic**: `advanced.pde.sympy_validation`

SymPy serves as the authoritative reference for validating MathHook PDE solvers. With 15+ years
of development, extensive test coverage, and academic validation, SymPy provides a reliable
baseline for comparing solution structures, eigenvalues, and boundary condition satisfaction.
This workflow is used internally for validation only; public documentation cites textbooks.



## Mathematical Definition

Validation criteria for PDE solutions:

1. **Solution Structure Match**: Both implementations produce equivalent forms
2. **Eigenvalue Formula**: $$\lambda_n = \left(\frac{n\pi}{L}\right)^2$$ matches numerically
3. **Boundary Conditions**: $$u(0,t) = 0, \quad u(L,t) = 0$$ satisfied
4. **Temporal Behavior**: $$\exp(-\lambda_n \alpha t)$$ matches across implementations




# SymPy Validation Workflow

## Why SymPy is the Reference Implementation

**SymPy** (`~/Documents/work/math/sympy/`) is the **authoritative reference** for validating MathHook PDE solvers for the following reasons:

1. **Mature and Battle-Tested**: SymPy's PDE solving has been developed and refined over 15+ years
2. **Extensive Test Suite**: Thousands of test cases covering edge cases
3. **Academic Validation**: Used in research and education worldwide
4. **Well-Documented**: Clear mathematical foundations and algorithms
5. **Python MCP Available**: Can be queried programmatically for validation

**Important**: SymPy is used **internally** for validation only. Public documentation cites textbooks and papers, NOT SymPy.

## Validation Workflow

### Step 1: Define Problem in Both Systems

Define the same PDE problem in both MathHook (Rust) and SymPy (Python) to enable comparison.

### Step 2: Compare Solution Structure

**Validation criteria**:
- ✅ Structure matches (sine modes, exponential decay)
- ✅ Eigenvalue formula matches: $\lambda_n = (n\pi/L)^2$
- ✅ Both use symbolic coefficients
- ✅ Temporal behavior matches: $\exp(-\lambda_n \alpha t)$

### Step 3: Verify Eigenvalues Numerically

Compare computed eigenvalues with high precision (typically < 1e-4 relative error).

### Step 4: Validate Boundary Condition Satisfaction

SymPy can verify BC satisfaction through symbolic substitution. MathHook returns symbolic
solutions that can be validated manually or programmatically.

## Validation Test Cases

### Heat Equation

**Test 1: Dirichlet BCs, constant IC**
- Verify eigenvalue: λ₁ = π² ≈ 9.8696
- Check solution structure matches SymPy

**Test 2: Different domain lengths**
- L = 1: λ₁ = π²
- L = 2: λ₁ = (π/2)² = π²/4
- L = 0.5: λ₁ = (π/0.5)² = 4π²

### Wave Equation

**Test 1: Standing wave frequencies**
- Compare angular frequencies: ω_n = n*π*c/L
- Verify relationship: ω_n = c*√λ_n

### Laplace Equation

**Test 1: Rectangular domain eigenvalues**
- Verify 2D eigenvalue formula: λₙ = (nπ/a)²

## Known Differences (Acceptable)

### 1. Coefficient Representation

**SymPy**: Uses `Sum()` with index notation
```python
Sum(C_n * sin(n*pi*x/L) * exp(-n²*pi²*alpha*t/L²), (n, 1, oo))
```

**MathHook**: Expands finite sum explicitly
```rust
A_1*sin(π*x)*exp(-π²*α*t) + A_2*sin(2π*x)*exp(-4π²*α*t) + ...
```

**Why acceptable**: Both representations are mathematically equivalent. MathHook finite sum is more practical for numerical evaluation.

### 2. Variable Naming

**SymPy**: Uses function notation `u(x,t)`

**MathHook**: Uses symbol `u` with independent variables as context

**Why acceptable**: Notational difference only; mathematical content identical.

### 3. Symbolic vs Numerical Coefficients

**Both return symbolic coefficients** for Fourier series. SymPy requires separate `fourier_series()` call; MathHook plans integration in Phase 2.

**Why acceptable**: Both defer coefficient computation to avoid integration challenges.

## Validation Checklist

Before claiming a PDE solver is correct:

1. ✅ **Solution structure** matches SymPy (sine/cosine modes, exp/sinh/cosh temporal)
2. ✅ **Eigenvalue formula** matches SymPy (verified numerically)
3. ✅ **Boundary conditions** satisfied when substituted
4. ✅ **Initial conditions** structure correct (even if coefficients symbolic)
5. ✅ **Edge cases** tested (different domain lengths, BCs)
6. ✅ **Known limitations** documented (Neumann BCs, non-homogeneous BCs, etc.)

## SymPy MCP Integration

**Available via MCP**: SymPy can be queried programmatically for validation.

**Example workflow**:
1. Agent implements new MathHook PDE solver
2. Agent queries SymPy MCP for reference solution
3. Agent compares eigenvalues, solution structure
4. Agent verifies BCs/ICs satisfied
5. Agent documents any acceptable differences
6. Agent adds regression tests

**Important**: SymPy MCP is for **internal validation**, NOT cited in public documentation.

## Mathematical References (For Public Documentation)

When documenting PDE solvers, cite these instead of SymPy:

1. **Strauss, Walter A.** *Partial Differential Equations: An Introduction*, 2nd ed.
2. **Evans, Lawrence C.** *Partial Differential Equations*, 2nd ed.
3. **Haberman, Richard** *Applied Partial Differential Equations*, 5th ed.

## Summary

**SymPy Validation Workflow**:
1. Implement solver in MathHook
2. Compare solution structure with SymPy
3. Verify eigenvalues numerically
4. Test BC/IC satisfaction
5. Document acceptable differences
6. Add regression tests

**Validation Criteria**:
- ✅ Structure matches
- ✅ Eigenvalues match (numerical verification)
- ✅ BCs/ICs satisfied
- ⚠️ Symbolic coefficients acceptable (both implementations)












## Examples


### Heat Equation Validation

Compare MathHook solution with SymPy reference for heat equation


<details>
<summary><b>Rust</b></summary>

```rust
#[test]
fn test_heat_vs_sympy_dirichlet() {
    // MathHook solution
    let result = solve_heat_1d(...)?;

    // SymPy reference (computed offline)
    let expected_lambda_1 = 9.8696;  // π²

    // Validate eigenvalue
    let lambda_1 = result.eigenvalues[0].evaluate()?;
    assert!((lambda_1 - expected_lambda_1).abs() < 1e-4);
}

```
</details>



<details>
<summary><b>Python</b></summary>

```python
def test_heat_vs_sympy_dirichlet():
    # MathHook solution (via Python bindings)
    result = solve_heat_1d(...)

    # SymPy reference
    expected_lambda_1 = 9.8696  # π²

    # Validate eigenvalue
    lambda_1 = result.eigenvalues[0].evaluate()
    assert abs(lambda_1 - expected_lambda_1) < 1e-4

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
test('heat equation matches SymPy', () => {
    // MathHook solution
    const result = solveHeat1d(...);

    // SymPy reference
    const expectedLambda1 = 9.8696;  // π²

    // Validate eigenvalue
    const lambda1 = result.eigenvalues[0].evaluate();
    expect(Math.abs(lambda1 - expectedLambda1)).toBeLessThan(1e-4);
});

```
</details>





### Eigenvalue Scaling Validation

Test eigenvalue scaling with different domain lengths


<details>
<summary><b>Rust</b></summary>

```rust
#[test]
fn test_heat_eigenvalues_scaling() {
    // L = 1: λ₁ = π²
    // L = 2: λ₁ = (π/2)² = π²/4
    // L = 0.5: λ₁ = (π/0.5)² = 4π²

    let L = 2.0;
    let result = solve_heat_1d_with_length(L)?;
    let expected = std::f64::consts::PI.powi(2) / 4.0;
    assert!((result.eigenvalues[0].evaluate()? - expected).abs() < 1e-4);
}

```
</details>



<details>
<summary><b>Python</b></summary>

```python
def test_heat_eigenvalues_scaling():
    # L = 1: λ₁ = π²
    # L = 2: λ₁ = (π/2)² = π²/4
    # L = 0.5: λ₁ = (π/0.5)² = 4π²

    L = 2.0
    result = solve_heat_1d_with_length(L)
    expected = (math.pi ** 2) / 4.0
    assert abs(result.eigenvalues[0].evaluate() - expected) < 1e-4

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
test('heat eigenvalues scale correctly', () => {
    // L = 1: λ₁ = π²
    // L = 2: λ₁ = (π/2)² = π²/4
    // L = 0.5: λ₁ = (π/0.5)² = 4π²

    const L = 2.0;
    const result = solveHeat1dWithLength(L);
    const expected = (Math.PI ** 2) / 4.0;
    expect(Math.abs(result.eigenvalues[0].evaluate() - expected)).toBeLessThan(1e-4);
});

```
</details>





### Wave Equation Frequency Validation

Verify wave equation frequencies against SymPy


<details>
<summary><b>Rust</b></summary>

```rust
#[test]
fn test_wave_frequencies_vs_sympy() {
    let c = 340.0;  // m/s (speed of sound)
    let L = 1.0;    // m

    let result = solve_wave_1d(...)?;

    // SymPy: ω_n = n*π*c/L
    let omega_1 = std::f64::consts::PI * c / L;
    let f_1 = omega_1 / (2.0 * std::f64::consts::PI);  // Frequency in Hz

    // MathHook eigenvalues: λ_n = (nπ/L)²
    // ω_n = c*√λ_n = c*nπ/L
    let lambda_1 = result.eigenvalues[0].evaluate()?;
    let omega_mathhook = c * lambda_1.sqrt();

    assert!((omega_mathhook - omega_1).abs() < 1e-6);
}

```
</details>



<details>
<summary><b>Python</b></summary>

```python
def test_wave_frequencies_vs_sympy():
    c = 340.0  # m/s (speed of sound)
    L = 1.0    # m

    result = solve_wave_1d(...)

    # SymPy: ω_n = n*π*c/L
    omega_1 = math.pi * c / L
    f_1 = omega_1 / (2.0 * math.pi)  # Frequency in Hz

    # MathHook eigenvalues: λ_n = (nπ/L)²
    # ω_n = c*√λ_n = c*nπ/L
    lambda_1 = result.eigenvalues[0].evaluate()
    omega_mathhook = c * math.sqrt(lambda_1)

    assert abs(omega_mathhook - omega_1) < 1e-6

```
</details>



<details>
<summary><b>JavaScript</b></summary>

```javascript
test('wave equation frequencies match SymPy', () => {
    const c = 340.0;  // m/s (speed of sound)
    const L = 1.0;    // m

    const result = solveWave1d(...);

    // SymPy: ω_n = n*π*c/L
    const omega1 = Math.PI * c / L;
    const f1 = omega1 / (2.0 * Math.PI);  // Frequency in Hz

    // MathHook eigenvalues: λ_n = (nπ/L)²
    // ω_n = c*√λ_n = c*nπ/L
    const lambda1 = result.eigenvalues[0].evaluate();
    const omegaMathhook = c * Math.sqrt(lambda1);

    expect(Math.abs(omegaMathhook - omega1)).toBeLessThan(1e-6);
});

```
</details>







## Performance

**Time Complexity**: O(n)


## API Reference

- **Rust**: `mathhook_core::pde::validation`
- **Python**: `mathhook.pde.validation`
- **JavaScript**: `mathhook.pde.validation`


## See Also


- [advanced.pde.registry](../advanced/pde/registry.md)

- [advanced.pde.performance](../advanced/pde/performance.md)

- [advanced.pde.technical_guide](../advanced/pde/technical_guide.md)

- [advanced.pde.user_guide](../advanced/pde/user_guide.md)


