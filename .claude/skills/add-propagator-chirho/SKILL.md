---
name: add-propagator-chirho
description: Add a new propagator type to propagators-chirho
disable-model-invocation: true
allowed-tools: Bash, Read, Edit, Write, Grep, Glob
argument-hint: [propagator description, e.g., "modulo propagator for a % b = c"]
---

<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

Add a new propagator to propagators-chirho: $ARGUMENTS

## Instructions

1. Analyze the requested propagator type and its bidirectional semantics
2. Determine if it fits existing macros:
   - `ternary_propagator_chirho!` - for a op b = c patterns
   - `binary_propagator_chirho!` - for unary a = b patterns
   - `comparison_propagator_chirho!` - for max/min style patterns
3. Add the propagator to `src/propagator_chirho.rs` following existing patterns
4. Add corresponding method to `ConstraintSystemChirho` in `src/constraint_system_chirho.rs`
5. If arena feature is used, add to `src/arena_chirho.rs` as well
6. Add unit tests for the new propagator
7. Update CHANGELOG.md with the new propagator under [Unreleased]
8. Run tests to verify: `cargo test`

## Naming Convention

- Struct: `{Name}Chirho` (e.g., `ModuloChirho`)
- Install method: `install_chirho`
- ConstraintSystem method: `add_{name}_chirho` (e.g., `add_modulo_chirho`)
- All fields/variables use `_chirho` suffix per project convention
