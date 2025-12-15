---









---

# Type System

> **Topic**: `architecture.type-system`

MathHook's type system design and constraints. Covers Expression, Number, and Symbol types
with their memory layout and performance characteristics.





# Type System

This chapter covers internal implementation details of MathHook's type system.

## Core Types

- **Expression**: 32-byte AST node (cache-optimized)
- **Number**: 16-byte tagged union (integer/rational/float/complex)
- **Symbol**: String interning for O(1) equality

## Type Constraints

### Expression Constraint

Expressions are exactly 32 bytes to fit two expressions per 64-byte cache line.
This is a non-negotiable architectural constraint that provides:

- Efficient memory access patterns
- Improved CPU cache utilization
- 10-100x speedup over Python-based systems

### Number Constraint

Numbers are 16 bytes to fit within the Expression type. The tagged union supports:

- Integer (arbitrary precision via pointer to heap)
- Rational (numerator/denominator pair)
- Float (f64)
- Complex (two f64s)

### Symbol Interning

Symbols use string interning for O(1) equality comparisons:

- First occurrence allocates and stores in global table
- Subsequent uses return pointer to existing string
- Equality is pointer comparison (no string comparison needed)

## Type Safety

MathHook's type system provides:

- **Compile-time safety**: Types checked at compile time
- **Mathematical correctness**: Operations preserve mathematical properties
- **Domain enforcement**: Domain restrictions respected (sqrt, log, division by zero)












## Examples





## API Reference

- **Rust**: `mathhook_core::Expression`
- **Python**: ``
- **JavaScript**: ``


## See Also


- [architecture.principles](../architecture/principles.md)

- [architecture.memory-layout](../architecture/memory-layout.md)

- [architecture.crate-structure](../architecture/crate-structure.md)


