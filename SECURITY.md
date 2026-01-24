<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

# Security Policy

> *"For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life."* — John 3:16

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability in propagators-chirho, please report it responsibly:

1. **Do NOT open a public issue**
2. Email the maintainers at: loveJesus@loveJesus.software
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will respond within 48 hours and work with you to address the issue.

## Security Measures

This library implements several security measures:

### Formal Verification

Key mathematical properties are formally verified using [Kani](https://model-checking.github.io/kani/):

- Interval intersection commutativity and idempotence
- Lattice merge properties (commutativity, identity, monotonicity)
- Contradiction absorption
- Bounds preservation

### Safe Rust

- Written in 100% safe Rust (no `unsafe` blocks in core library)
- Extensive use of Rust's type system for correctness
- No raw pointer manipulation

### Floating-Point Safety

- Finite value assumptions in critical paths
- Proper handling of NaN and infinity
- No undefined behavior from floating-point operations

### Dependencies

We carefully audit all dependencies:

- Minimal dependency tree
- Only well-maintained crates from trusted sources
- Regular dependency updates via Dependabot

## Best Practices for Users

1. **Validate inputs** when using the library in security-sensitive contexts
2. **Handle contradictions** properly (they indicate conflicting constraints)
3. **Use finite values** - the library handles infinities but may produce unexpected results
4. **Check for overflow** when using very large interval bounds

## Acknowledgments

Thank you to all security researchers who responsibly disclose vulnerabilities.

God bless you for helping keep this project secure!
