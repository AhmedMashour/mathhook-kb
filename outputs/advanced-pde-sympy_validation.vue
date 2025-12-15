<template>
  <div class="doc-page">
    <header class="doc-header">
      <h1>SymPy Validation Workflow</h1>
      <p class="description">SymPy serves as the authoritative reference for validating MathHook PDE solvers. With 15+ years
of development, extensive test coverage, and academic validation, SymPy provides a reliable
baseline for comparing solution structures, eigenvalues, and boundary condition satisfaction.
This workflow is used internally for validation only; public documentation cites textbooks.
</p>
    </header>

    
    <section class="math-definition">
      <h2>Mathematical Definition</h2>
      <div class="katex-block">Validation criteria for PDE solutions:

1. **Solution Structure Match**: Both implementations produce equivalent forms
2. **Eigenvalue Formula**: $$\lambda_n = \left(\frac{n\pi}{L}\right)^2$$ matches numerically
3. **Boundary Conditions**: $$u(0,t) = 0, \quad u(L,t) = 0$$ satisfied
4. **Temporal Behavior**: $$\exp(-\lambda_n \alpha t)$$ matches across implementations
</div>
    </section>
    

    <section class="examples">
      <h2>Interactive Examples</h2>
      
      <div class="example-card">
        <h3>Heat Equation Validation</h3>
        <p>Compare MathHook solution with SymPy reference for heat equation</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">#[test]
fn test_heat_vs_sympy_dirichlet() {
    // MathHook solution
    let result = solve_heat_1d(...)?;

    // SymPy reference (computed offline)
    let expected_lambda_1 = 9.8696;  // π²

    // Validate eigenvalue
    let lambda_1 = result.eigenvalues[0].evaluate()?;
    assert!((lambda_1 - expected_lambda_1).abs() < 1e-4);
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">def test_heat_vs_sympy_dirichlet():
    # MathHook solution (via Python bindings)
    result = solve_heat_1d(...)

    # SymPy reference
    expected_lambda_1 = 9.8696  # π²

    # Validate eigenvalue
    lambda_1 = result.eigenvalues[0].evaluate()
    assert abs(lambda_1 - expected_lambda_1) < 1e-4
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">test('heat equation matches SymPy', () => {
    // MathHook solution
    const result = solveHeat1d(...);

    // SymPy reference
    const expectedLambda1 = 9.8696;  // π²

    // Validate eigenvalue
    const lambda1 = result.eigenvalues[0].evaluate();
    expect(Math.abs(lambda1 - expectedLambda1)).toBeLessThan(1e-4);
});
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Eigenvalue Scaling Validation</h3>
        <p>Test eigenvalue scaling with different domain lengths</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">#[test]
fn test_heat_eigenvalues_scaling() {
    // L = 1: λ₁ = π²
    // L = 2: λ₁ = (π/2)² = π²/4
    // L = 0.5: λ₁ = (π/0.5)² = 4π²

    let L = 2.0;
    let result = solve_heat_1d_with_length(L)?;
    let expected = std::f64::consts::PI.powi(2) / 4.0;
    assert!((result.eigenvalues[0].evaluate()? - expected).abs() < 1e-4);
}
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">def test_heat_eigenvalues_scaling():
    # L = 1: λ₁ = π²
    # L = 2: λ₁ = (π/2)² = π²/4
    # L = 0.5: λ₁ = (π/0.5)² = 4π²

    L = 2.0
    result = solve_heat_1d_with_length(L)
    expected = (math.pi ** 2) / 4.0
    assert abs(result.eigenvalues[0].evaluate() - expected) < 1e-4
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">test('heat eigenvalues scale correctly', () => {
    // L = 1: λ₁ = π²
    // L = 2: λ₁ = (π/2)² = π²/4
    // L = 0.5: λ₁ = (π/0.5)² = 4π²

    const L = 2.0;
    const result = solveHeat1dWithLength(L);
    const expected = (Math.PI ** 2) / 4.0;
    expect(Math.abs(result.eigenvalues[0].evaluate() - expected)).toBeLessThan(1e-4);
});
</code></pre>
        </div>

        
      </div>
      
      <div class="example-card">
        <h3>Wave Equation Frequency Validation</h3>
        <p>Verify wave equation frequencies against SymPy</p>
        
        <div class="code-tabs">
          <button @click="activeTab = 'rust'">Rust</button>
          <button @click="activeTab = 'python'">Python</button>
          <button @click="activeTab = 'javascript'">JavaScript</button>
        </div>

        <div v-show="activeTab === 'rust'" class="code-block">
          <pre><code class="language-rust">#[test]
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
</code></pre>
        </div>
        <div v-show="activeTab === 'python'" class="code-block">
          <pre><code class="language-python">def test_wave_frequencies_vs_sympy():
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
</code></pre>
        </div>
        <div v-show="activeTab === 'javascript'" class="code-block">
          <pre><code class="language-javascript">test('wave equation frequencies match SymPy', () => {
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
</code></pre>
        </div>

        
      </div>
      
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const activeTab = ref('python')

// SEO metadata
const metaDescription = "SymPy serves as the authoritative reference for validating MathHook PDE solvers. With 15+ years
of development, extensive test coverage, and academic validation, SymPy provides a reliable
baseline for comparing solution structures, eigenvalues, and boundary condition satisfaction.
This workflow is used internally for validation only; public documentation cites textbooks.
"
const keywords = []

// Define page metadata
definePageMeta({
  title: 'SymPy Validation Workflow',
  description: metaDescription,
  keywords: keywords.join(', '),
  
})
</script>

<style scoped>
.doc-page {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}

.doc-header h1 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.description {
  font-size: 1.2rem;
  color: #666;
}

.math-definition {
  background: #f5f5f5;
  padding: 1.5rem;
  border-radius: 8px;
  margin: 2rem 0;
}

.example-card {
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 1.5rem;
  margin-bottom: 2rem;
}

.code-tabs {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.code-tabs button {
  padding: 0.5rem 1rem;
  border: none;
  background: #eee;
  cursor: pointer;
  border-radius: 4px;
}

.code-tabs button:hover {
  background: #ddd;
}

.code-block {
  background: #1e1e1e;
  color: #d4d4d4;
  padding: 1rem;
  border-radius: 4px;
  overflow-x: auto;
}

.output {
  margin-top: 1rem;
  padding: 1rem;
  background: #f9f9f9;
  border-left: 4px solid #42b883;
}
</style>
