---









---

# PDE Classification

> **Topic**: `advanced.pde.classification`

Mathematical classification of partial differential equations into elliptic, parabolic,
and hyperbolic types using the discriminant formula. Different PDE types require
completely different solution methods and have distinct physical interpretations.



## Mathematical Definition

$$For a general second-order PDE:
$$A \frac{\partial^2 u}{\partial x^2} + B \frac{\partial^2 u}{\partial x \partial y} + C \frac{\partial^2 u}{\partial y^2} + \text{lower order terms} = 0$$

The **discriminant** is:
$$\Delta = B^2 - 4AC$$

**Classification:**
- $\Delta < 0$ → **Elliptic** (e.g., Laplace: $u_{xx} + u_{yy} = 0$)
- $\Delta = 0$ → **Parabolic** (e.g., Heat: $u_t = u_{xx}$)
- $\Delta > 0$ → **Hyperbolic** (e.g., Wave: $u_{tt} = c^2 u_{xx}$)
$$



# PDE Classification

## Why Classification Matters

Different PDE types require completely different solution methods:
- **Elliptic**: Boundary value problems, steady-state
- **Parabolic**: Initial value + boundary, diffusion
- **Hyperbolic**: Initial value + boundary, wave propagation

Using the wrong method **WILL FAIL** or produce nonsense results.

## Mathematical Classification Theory

### The Discriminant Formula

For a general second-order PDE:

$$A \frac{\partial^2 u}{\partial x^2} + B \frac{\partial^2 u}{\partial x \partial y} + C \frac{\partial^2 u}{\partial y^2} + \text{lower order terms} = 0$$

The **discriminant** is:

$$\Delta = B^2 - 4AC$$

### Classification Categories

| Discriminant | Type | Canonical Form | Prototype |
|--------------|------|----------------|-----------|
| $\Delta < 0$ | **Elliptic** | $u_{xx} + u_{yy} = 0$ | Laplace |
| $\Delta = 0$ | **Parabolic** | $u_t = u_{xx}$ | Heat |
| $\Delta > 0$ | **Hyperbolic** | $u_{tt} = c^2 u_{xx}$ | Wave |

### Physical Interpretation

#### Elliptic ($\Delta < 0$)

**Characteristics**: No real characteristics (complex)

**Physical Meaning**: Equilibrium states, no time evolution

**Properties**:
- Smooth solutions (infinitely differentiable if coefficients are smooth)
- Maximum principle: solution maximum on boundary
- Propagation speed: **infinite** (disturbance felt everywhere instantly)

**Examples**:
- Laplace's equation: $\nabla^2 u = 0$ (electrostatics, steady heat)
- Poisson's equation: $\nabla^2 u = f$ (gravity, charged regions)
- Minimal surface equation: $(1 + u_y^2) u_{xx} - 2u_x u_y u_{xy} + (1 + u_x^2) u_{yy} = 0$

#### Parabolic ($\Delta = 0$)

**Characteristics**: One family of real characteristics

**Physical Meaning**: Diffusion processes, irreversible evolution

**Properties**:
- Smoothing effect (rough initial data becomes smooth)
- Infinite propagation speed (finite but small amplitude)
- Irreversible in time (cannot reverse diffusion without external forcing)

**Examples**:
- Heat equation: $u_t = \alpha u_{xx}$ (thermal diffusion)
- Black-Scholes equation: $\frac{\partial V}{\partial t} + \frac{1}{2}\sigma^2 S^2 \frac{\partial^2 V}{\partial S^2} + rS\frac{\partial V}{\partial S} - rV = 0$ (option pricing)
- Fokker-Planck equation: $\frac{\partial p}{\partial t} = \frac{\partial^2}{\partial x^2}(D p) - \frac{\partial}{\partial x}(\mu p)$ (stochastic processes)

#### Hyperbolic ($\Delta > 0$)

**Characteristics**: Two families of real characteristics

**Physical Meaning**: Wave propagation, reversible evolution

**Properties**:
- Finite propagation speed $c$ (disturbances travel along characteristics)
- Preservation of discontinuities (shocks can form)
- Reversible in time (wave equation is time-symmetric)

**Examples**:
- Wave equation: $u_{tt} = c^2 u_{xx}$ (vibrations, sound)
- Telegraph equation: $u_{tt} + 2\alpha u_t = c^2 u_{xx}$ (damped waves)
- Beam equation: $u_{tt} + \gamma u_{xxxx} = 0$ (elastic beam vibrations)

## Discriminant Computation in MathHook

### Current Implementation Limitations

MathHook currently uses **pattern matching** instead of symbolic differentiation for coefficient extraction.

**Why Pattern Matching?**:
- Symbolic differentiation of coefficients requires extracting $A$, $B$, $C$ from PDE
- This requires: $A = \frac{\partial^2}{\partial x^2}(\text{equation})$ evaluated symbolically
- MathHook's differentiation module focuses on expressions, not PDE coefficient extraction
- Pattern matching works for standard equations (heat, wave, Laplace)

**Future Enhancement**:
Phase 2 will implement full symbolic coefficient extraction.

## Variable Naming Heuristics

MathHook infers PDE type from variable names:

### Time-Space PDEs

Variables named 't' or 'time' → considered temporal
Variables named 'x', 'y', 'z', or 'space' → considered spatial

**Heat/Wave Equation Detection**:
- Requires exactly 2 variables
- One temporal (`t` or `time`)
- One spatial (`x` or `space`)
- Heat: Additive structure (first-order time derivative)
- Wave: Multiplicative structure (second-order time derivative)

### Spatial-Only PDEs

Variables named 'x' and 'y' → spatial coordinates

**Laplace Equation Detection**:
- Requires 2+ spatial variables
- No temporal variable
- Additive structure

### Custom Variable Names

**⚠️ Warning**: Non-standard variable names may not classify correctly.

## Classification Edge Cases

### Mixed-Type PDEs

Some PDEs change type based on region:

**Tricomi Equation**: $y u_{xx} + u_{yy} = 0$

- $y > 0$: Elliptic ($\Delta = -4y < 0$)
- $y = 0$: Parabolic ($\Delta = 0$)
- $y < 0$: Hyperbolic ($\Delta = -4y > 0$)

**⚠️ MathHook does NOT handle mixed-type PDEs currently**.

### Degenerate Cases

**Equation**: $u_{xx} = 0$

This is technically a **degenerate elliptic** PDE (only one second derivative):
- Discriminant: $\Delta = 0 - 4(1)(0) = 0$ (parabolic by formula)
- But no time evolution (elliptic by behavior)

**MathHook Classification**: Depends on variable names. Use with caution.












## Examples


### Wave Equation Classification (Hyperbolic)

Wave equation has positive discriminant and is classified as hyperbolic

<details>
<summary><b>Rust</b></summary>

```rust
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

// Wave equation structure
let equation = expr!(mul: x, t);
let pde = Pde::new(equation, u, vec![x, t]);

// Automatic classification
let pde_type = pde.pde_type();
assert_eq!(pde_type, Some(PdeType::Hyperbolic));

// Discriminant computation
let disc = pde.compute_discriminant();
assert!(disc > 0.0);
assert_eq!(disc, 4.0);

```
</details>

<details>
<summary><b>Python</b></summary>

```python
u = symbol('u')
x = symbol('x')
t = symbol('t')

# Wave equation structure
equation = expr(mul=[x, t])
pde = Pde.new(equation, u, [x, t])

# Automatic classification
pde_type = pde.pde_type()
assert pde_type == PdeType.Hyperbolic

# Discriminant computation
disc = pde.compute_discriminant()
assert disc > 0.0
assert disc == 4.0

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const u = symbol('u');
const x = symbol('x');
const t = symbol('t');

// Wave equation structure
const equation = expr({ mul: [x, t] });
const pde = Pde.new(equation, u, [x, t]);

// Automatic classification
const pdeType = pde.pdeType();
assert(pdeType === PdeType.Hyperbolic);

// Discriminant computation
const disc = pde.computeDiscriminant();
assert(disc > 0.0);
assert(disc === 4.0);

```
</details>




### Heat Equation Classification (Parabolic)

Heat equation has zero discriminant and is classified as parabolic

<details>
<summary><b>Rust</b></summary>

```rust
let u = symbol!(u);
let x = symbol!(x);
let t = symbol!(t);

// Heat equation structure
let equation = expr!(add: x, t);
let pde = Pde::new(equation, u, vec![x, t]);

// Automatic classification
let pde_type = pde.pde_type();
assert_eq!(pde_type, Some(PdeType::Parabolic));

// Discriminant
let disc = pde.compute_discriminant();
assert_eq!(disc.abs(), 0.0);

```
</details>

<details>
<summary><b>Python</b></summary>

```python
u = symbol('u')
x = symbol('x')
t = symbol('t')

# Heat equation structure
equation = expr(add=[x, t])
pde = Pde.new(equation, u, [x, t])

# Automatic classification
pde_type = pde.pde_type()
assert pde_type == PdeType.Parabolic

# Discriminant
disc = pde.compute_discriminant()
assert abs(disc) == 0.0

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const u = symbol('u');
const x = symbol('x');
const t = symbol('t');

// Heat equation structure
const equation = expr({ add: [x, t] });
const pde = Pde.new(equation, u, [x, t]);

// Automatic classification
const pdeType = pde.pdeType();
assert(pdeType === PdeType.Parabolic);

// Discriminant
const disc = pde.computeDiscriminant();
assert(Math.abs(disc) === 0.0);

```
</details>




### Laplace Equation Classification (Elliptic)

Laplace equation has negative discriminant and is classified as elliptic

<details>
<summary><b>Rust</b></summary>

```rust
let u = symbol!(u);
let x = symbol!(x);
let y = symbol!(y);

// Laplace equation structure
let equation = expr!(add: x, y);
let pde = Pde::new(equation, u, vec![x, y]);

// Automatic classification
let pde_type = pde.pde_type();
assert_eq!(pde_type, Some(PdeType::Elliptic));

// Discriminant
let disc = pde.compute_discriminant();
assert!(disc < 0.0);
assert_eq!(disc, -4.0);

```
</details>

<details>
<summary><b>Python</b></summary>

```python
u = symbol('u')
x = symbol('x')
y = symbol('y')

# Laplace equation structure
equation = expr(add=[x, y])
pde = Pde.new(equation, u, [x, y])

# Automatic classification
pde_type = pde.pde_type()
assert pde_type == PdeType.Elliptic

# Discriminant
disc = pde.compute_discriminant()
assert disc < 0.0
assert disc == -4.0

```
</details>

<details>
<summary><b>JavaScript</b></summary>

```javascript
const u = symbol('u');
const x = symbol('x');
const y = symbol('y');

// Laplace equation structure
const equation = expr({ add: [x, y] });
const pde = Pde.new(equation, u, [x, y]);

// Automatic classification
const pdeType = pde.pdeType();
assert(pdeType === PdeType.Elliptic);

// Discriminant
const disc = pde.computeDiscriminant();
assert(disc < 0.0);
assert(disc === -4.0);

```
</details>






## Performance

**Time Complexity**: O(1)


## API Reference

- **Rust**: `mathhook_core::pde::classification`
- **Python**: `mathhook.pde.classification`
- **JavaScript**: `mathhook.pde.classification`


## See Also


- [advanced.pde.boundary_conditions](../advanced/pde/boundary_conditions.md)

- [advanced.pde.method_of_characteristics](../advanced/pde/method_of_characteristics.md)

- [calculus.heat_equation](../calculus/heat_equation.md)

- [calculus.wave_equation](../calculus/wave_equation.md)

- [calculus.laplace_equation](../calculus/laplace_equation.md)


