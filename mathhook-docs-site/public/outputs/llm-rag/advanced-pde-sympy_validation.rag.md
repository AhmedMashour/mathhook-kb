# SymPy Validation Workflow

SymPy serves as the authoritative reference for validating MathHook PDE solvers. With 15+ years
of development, extensive test coverage, and academic validation, SymPy provides a reliable
baseline for comparing solution structures, eigenvalues, and boundary condition satisfaction.
This workflow is used internally for validation only; public documentation cites textbooks.


---
chunk_id: advanced_pde_sympy_validation::0
topic: advanced.pde.sympy_validation
title: Heat Equation Validation
priority: medium
keywords:
  - sympy_validation
  - heat equation validation
languages: [rust, python, javascript]
chunk: 1/3
---

## Heat Equation Validation

Compare MathHook solution with SymPy reference for heat equation

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: advanced_pde_sympy_validation::1
topic: advanced.pde.sympy_validation
title: Eigenvalue Scaling Validation
priority: medium
keywords:
  - sympy_validation
  - eigenvalue scaling validation
languages: [rust, python, javascript]
chunk: 2/3
---

## Eigenvalue Scaling Validation

Test eigenvalue scaling with different domain lengths

### Rust

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

### Python

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

### JavaScript

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



---
chunk_id: advanced_pde_sympy_validation::2
topic: advanced.pde.sympy_validation
title: Wave Equation Frequency Validation
priority: medium
keywords:
  - sympy_validation
  - wave equation frequency validation
languages: [rust, python, javascript]
chunk: 3/3
---

## Wave Equation Frequency Validation

Verify wave equation frequencies against SymPy

### Rust

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

### Python

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

### JavaScript

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



