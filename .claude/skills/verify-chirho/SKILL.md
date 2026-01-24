---
name: verify-chirho
description: Run Kani formal verification proofs for propagators-chirho
disable-model-invocation: true
allowed-tools: Bash, Read
---

<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

Run Kani formal verification proofs for propagators-chirho.

## Instructions

1. Check if Kani is installed: `cargo kani --version`
2. If not installed, inform user how to install:
   ```bash
   cargo install --locked kani-verifier
   cargo kani setup
   ```
3. Run Kani proofs: `cargo kani --features kani`
4. Report verification results for each proof
5. If any proofs fail, analyze the counterexample and suggest fixes

## Key Proofs to Verify

- Interval arithmetic laws (associativity, commutativity)
- Merge monotonicity (information can only increase)
- Semilattice properties (idempotence, commutativity)
- No contradiction from valid interval operations
