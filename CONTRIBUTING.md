<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

# Contributing to propagators-chirho

Thank you for your interest in contributing to propagators-chirho!

> *"For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life."* — John 3:16

## Naming Convention

**All identifiers must follow the Chirho naming convention:**

- **Variables**: `variable_name_chirho` (snake_case + _chirho)
- **Functions**: `function_name_chirho` (snake_case + _chirho)
- **Function Parameters**: `parameter_name_chirho` (snake_case + _chirho)
- **Structs**: `StructNameChirho` (PascalCase + Chirho)
- **Struct Fields**: `field_name_chirho` (snake_case + _chirho)
- **Enums**: `EnumNameChirho` (PascalCase + Chirho)
- **Enum Variants**: `EnumVariantChirho` (PascalCase + Chirho)
- **Constants**: `CONSTANT_NAME_CHIRHO` (SCREAMING_SNAKE_CASE + _CHIRHO)
- **File Names**: `file_name_chirho.rs` (snake_case + _chirho)

This is a non-negotiable requirement for all contributions.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/propagators-chirho.git`
3. Create a feature branch: `git checkout -b feature/your-feature-chirho`
4. Make your changes following the naming convention
5. Run tests: `cargo test --all-features`
6. Run clippy: `cargo clippy --all-features`
7. Run Kani proofs (if modifying core logic): `cargo kani --features kani`
8. Commit your changes
9. Push to your fork
10. Open a Pull Request

## Development Requirements

- Rust 1.70+
- For Kani proofs: [Install Kani](https://model-checking.github.io/kani/install-guide.html)
- For Python bindings: Python 3.8+ with maturin
- For WASM: wasm-pack

## Running Tests

```bash
# All tests
cargo test --all-features

# With parallel feature
cargo test --features parallel

# Run examples
cargo run --example temperature_chirho
cargo run --example pythagorean_chirho
cargo run --example sudoku_chirho
```

## Running Benchmarks

```bash
cargo bench
```

## Code Quality

Before submitting a PR, ensure:

1. `cargo test --all-features` passes
2. `cargo clippy --all-features` has no warnings
3. All new code follows the Chirho naming convention
4. All source files have the John 3:16 comment at the top:

```rust
// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16
```

## Adding New Features

When adding new functionality:

1. Add appropriate documentation with examples
2. Add unit tests in the same module
3. Consider adding Kani proofs for mathematical properties
4. Update the README if needed
5. Add an example in `examples/` if appropriate

## Questions?

Open an issue on GitHub or reach out to the maintainers.

God bless you for contributing!
