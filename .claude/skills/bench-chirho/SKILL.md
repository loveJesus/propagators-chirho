---
name: bench-chirho
description: Run criterion benchmarks for propagators-chirho performance analysis
disable-model-invocation: true
allowed-tools: Bash, Read
---

<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

Run benchmarks for propagators-chirho.

## Instructions

1. Run the criterion benchmarks: `cargo bench`
2. Compare results with previous runs if available in `target/criterion/`
3. Report key metrics: interval operations, propagator throughput, scheduling overhead
4. Suggest optimizations if any regressions are detected
