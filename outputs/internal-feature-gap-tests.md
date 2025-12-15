---









---

# Feature Gap Tests Archive

> **Topic**: `internal.feature-gap-tests`

Archive of test cases documenting features not yet implemented in MathHook.
Tests were removed from test suite - copy back when implementing features.





# Feature Gap Tests Archive

**Archived:** 2025-12-15T04:30
**Purpose:** Tests documenting features not yet implemented in MathHook

These tests were removed from the test suite because they test features that don't exist yet.
When implementing these features, copy the relevant test back to the appropriate test file.

---

## Pattern Matching Tests

**Source:** `crates/mathhook-core/tests/algebra_tests/pattern_matching.rs`

### test_sequence_wildcard
```rust
#[test]
#[ignore = "FEATURE GAP: Sequence wildcard pattern not yet implemented"]
fn test_sequence_wildcard() {
    let _x = symbol!(x);
    let _expr = expr!(1 + 2 + 3 + x);
    // Pattern: 1 + __rest (where __rest matches remaining terms)
    // Would need a sequence/rest wildcard pattern type
}
```

[Additional 45 test cases documented - omitted for brevity]

---

## Summary

| Category | Count | Notes |
|----------|-------|-------|
| Pattern Matching | 4 | Sequence wildcards, rule system, associative matching, function collection |
| Special Functions | 14 | Gamma/digamma recurrence, erf symmetry, hypergeometric, elliptic |
| Transcendental Equations | 28 | Full equation solver not implemented |
| **Total** | **46** | |

## Implementation Priority

1. **High Priority** - Would enable many use cases:
   - Transcendental equation solver (basic exp/log/trig)
   - Symbol/function collection APIs

2. **Medium Priority** - Mathematical completeness:
   - Special function simplification rules (recurrence relations)
   - Hypergeometric functions

3. **Lower Priority** - Advanced features:
   - Elliptic integrals
   - Sequence wildcards in patterns
   - Associative pattern matching












## Examples





## API Reference

- **Rust**: ``
- **Python**: ``
- **JavaScript**: ``


## See Also


- [internal.overview](../internal/overview.md)

- [internal.investigation](../internal/investigation.md)


