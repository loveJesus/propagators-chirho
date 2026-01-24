<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

# propagators-chirho

[![Crates.io](https://img.shields.io/crates/v/propagators-chirho.svg)](https://crates.io/crates/propagators-chirho)
[![Documentation](https://docs.rs/propagators-chirho/badge.svg)](https://docs.rs/propagators-chirho)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> *"For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life."* — John 3:16

A Rust implementation of **propagator networks** for constraint propagation and bidirectional computation, based on the seminal work by Radul and Sussman.

## What Are Propagators?

Propagators are a programming paradigm where:

- **Cells** hold partial information that can only grow monotonically
- **Propagators** are autonomous agents that watch cells and update other cells
- **Information flows in all directions** — constraints are bidirectional

This enables powerful constraint-based programming where you can run computations "backwards."

```rust
use propagators_chirho::prelude_chirho::*;

// Temperature conversion: F = C × 9/5 + 32
// Works BOTH ways!

let mut system_chirho = ConstraintSystemChirho::new_chirho();

let celsius_chirho = system_chirho.make_cell_chirho("celsius");
let fahrenheit_chirho = system_chirho.make_cell_chirho("fahrenheit");
// ... set up constraints ...

// Forward: given Celsius, compute Fahrenheit
system_chirho.set_exact_chirho(&celsius_chirho, 100.0);
system_chirho.run_chirho();
// fahrenheit is now 212.0

// Backward: given Fahrenheit, compute Celsius
system_chirho.set_exact_chirho(&fahrenheit_chirho, 32.0);
system_chirho.run_chirho();
// celsius is now 0.0
```

## Features

### Core Infrastructure

- `CellChirho<T>` — Cells with monotonic merge and neighbor notification
- `PropagatorChirho` trait — Interface for building custom propagators
- `SchedulerChirho` — Runs propagators to fixpoint

### Interval Arithmetic

- `IntervalChirho` — Partial numeric information as `[lo, hi]` bounds
- Full interval arithmetic: `+`, `-`, `*`, `/`, `sqrt`, `square`, `abs`
- Intervals intersect on merge, detecting contradictions

### Propagators

| Propagator | Constraint | Directions |
|------------|------------|------------|
| `IntervalAdderChirho` | a + b = c | All 3 |
| `IntervalSubtractorChirho` | a - b = c | All 3 |
| `IntervalMultiplierChirho` | a × b = c | All 3 |
| `IntervalDividerChirho` | a ÷ b = c | All 3 |
| `SquarerChirho` | a² = b | Both |
| `SqrterChirho` | √a = b | Both |
| `AbsoluterChirho` | \|a\| = b | Both |
| `MaxChirho` | max(a,b) = c | All 3 |
| `MinChirho` | min(a,b) = c | All 3 |
| `ConditionalChirho` | if p > 0 then a else b | Forward |

### Truth Maintenance System (TMS)

- `BeliefChirho` — Values with supporting premises
- `TmsCellChirho` — Cells that track multiple beliefs
- Nogood detection — Find which premises cause contradictions

### Worldviews (Hypothetical Reasoning)

- `WorldviewChirho` — Set of active premises
- Fork and explore alternate assumptions
- Retract premises when they lead to contradictions

### Amb Operator (Nondeterministic Choice)

- `AmbChirho` — Choice among values
- `BacktrackingSearchChirho` — Constraint satisfaction with backtracking

### High-Level API

- `ConstraintSystemChirho` — Builder-style interface for constraint networks

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
propagators-chirho = "0.1"
```

## Examples

### Bidirectional Computation

```rust
use propagators_chirho::prelude_chirho::*;

let mut system_chirho = ConstraintSystemChirho::new_chirho();

let a_chirho = system_chirho.make_cell_chirho("a");
let b_chirho = system_chirho.make_cell_chirho("b");
let c_chirho = system_chirho.make_cell_chirho("c");

// Constraint: a + b = c
system_chirho.add_adder_chirho(&a_chirho, &b_chirho, &c_chirho);

// Forward: 3 + 4 = ?
system_chirho.set_exact_chirho(&a_chirho, 3.0);
system_chirho.set_exact_chirho(&b_chirho, 4.0);
system_chirho.run_chirho();
// c = 7

// Backward: 3 + ? = 7
system_chirho.set_exact_chirho(&a_chirho, 3.0);
system_chirho.set_exact_chirho(&c_chirho, 7.0);
system_chirho.run_chirho();
// b = 4
```

### Interval Propagation

```rust
use propagators_chirho::{IntervalChirho, NumericInfoChirho};

// "The value is between 20 and 30"
let partial_chirho = NumericInfoChirho::interval_chirho(20.0, 30.0);

// "Actually, it's between 25 and 35"
let more_info_chirho = NumericInfoChirho::interval_chirho(25.0, 35.0);

// Merge: intersection is [25, 30]
let refined_chirho = partial_chirho.merge_chirho(&more_info_chirho);
```

### Pythagorean Theorem (Backwards!)

```rust
use propagators_chirho::prelude_chirho::*;

let mut system_chirho = ConstraintSystemChirho::new_chirho();

let a_chirho = system_chirho.make_cell_chirho("a");
let b_chirho = system_chirho.make_cell_chirho("b");
let c_chirho = system_chirho.make_cell_chirho("c");

// a² + b² = c²
system_chirho.add_pythagorean_chirho(&a_chirho, &b_chirho, &c_chirho);

// Given c=13 and a=5, find b
system_chirho.set_exact_chirho(&c_chirho, 13.0);
system_chirho.set_exact_chirho(&a_chirho, 5.0);
system_chirho.set_interval_chirho(&b_chirho, 0.0, f64::INFINITY);  // b > 0
system_chirho.run_chirho();
// b = 12 (since 5² + 12² = 13²)
```

### Truth Maintenance

```rust
use propagators_chirho::{BeliefChirho, TmsCellChirho, WorldviewChirho, NumericInfoChirho};

let cell_chirho = TmsCellChirho::new_chirho("temperature");

// Add beliefs from different sources
cell_chirho.add_belief_chirho(BeliefChirho::with_premises_chirho(
    NumericInfoChirho::exact_chirho(25.0),
    vec!["sensor_a".to_string()].into_iter().collect(),
    "sensor A reading",
));

cell_chirho.add_belief_chirho(BeliefChirho::with_premises_chirho(
    NumericInfoChirho::exact_chirho(26.0),
    vec!["sensor_b".to_string()].into_iter().collect(),
    "sensor B reading",
));

// Query under different worldviews
let wv_a_chirho = WorldviewChirho::new_chirho().assume_chirho("sensor_a".to_string());
let value_a_chirho = cell_chirho.content_in_worldview_chirho(&wv_a_chirho);
// value_a = 25.0
```

## Running Examples

```bash
cargo run --example temperature_chirho   # Bidirectional temperature conversion
cargo run --example pythagorean_chirho   # Pythagorean theorem
cargo run --example electrical_chirho    # Electrical circuit analysis
cargo run --example sudoku_chirho        # Sudoku solver
```

## Performance

### Features

Enable high-performance arena mode for ~2x faster network operations:

```toml
[dependencies]
propagators-chirho = { version = "0.1", features = ["arena"] }
```

```rust
use propagators_chirho::arena_chirho::ArenaNetworkChirho;

let mut net_chirho = ArenaNetworkChirho::new_chirho();
let a_chirho = net_chirho.make_cell_chirho();
// ... faster due to contiguous storage and no Rc overhead
```

### Benchmarks

#### Interval Primitives: propagators-chirho vs inari

| Operation | propagators-chirho | inari (IEEE 754) | Speedup |
|-----------|-------------------|------------------|---------|
| add | **0.42 ns** | 0.69 ns | 1.6x |
| mul | **1.02 ns** | 28.8 ns | **28x** |
| intersect | **0.42 ns** | 1.04 ns | 2.5x |
| square | **0.43 ns** | 22.2 ns | **51x** |
| sqrt | **0.43 ns** | 0.68 ns | 1.6x |

*Note: inari provides IEEE 754-2019 compliance with rounding mode guarantees. Our implementation prioritizes speed over strict IEEE compliance.*

#### Network Propagation

| Operation | Default Mode | Arena Mode | Speedup |
|-----------|-------------|------------|---------|
| Simple add (3 cells) | 265 ns | **186 ns** | 1.4x |
| Temperature (5 cells) | 899 ns | **348 ns** | 2.6x |

#### Comparison to Other Systems

| System | Type | Typical Time | Notes |
|--------|------|--------------|-------|
| **propagators-chirho (arena)** | Rust | **182-348 ns** | This library |
| **Gecode** | C++ | ~100-500 ns | Full CP solver |
| **Chuffed** | C++ | ~50-200 ns | Lazy clause gen |
| **MiniZinc** | Various | ~1-10 µs | Modeling layer |

### Running Benchmarks

```bash
# Basic benchmarks
cargo bench

# With arena mode
cargo bench --features arena

# Comparison against other libraries (requires tmp-chirho/)
cd tmp-chirho/comparison-bench-chirho && cargo bench
```

## Testing

```bash
cargo test                    # Run all tests
cargo test --features arena   # Test arena mode
cargo test --test property    # Property-based tests
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Constraint System                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   ┌──────────┐      ┌──────────────┐      ┌──────────┐         │
│   │ Cell A   │─────▶│  Propagator  │─────▶│ Cell C   │         │
│   │ [3, 5]   │      │   a + b = c  │      │ [7, 12]  │         │
│   └──────────┘      └──────────────┘      └──────────┘         │
│        ▲                   ▲                   │                │
│        │                   │                   │                │
│   ┌──────────┐             │                   │                │
│   │ Cell B   │─────────────┘                   │                │
│   │ [4, 7]   │◀────────────────────────────────┘                │
│   └──────────┘         (bidirectional!)                         │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│                         Scheduler                                │
│   • Maintains queue of alerted propagators                      │
│   • Runs until fixpoint (no more changes)                       │
└─────────────────────────────────────────────────────────────────┘
```

## Mathematical Foundations

### Partial Information Lattice

```
           Contradiction (⊤)
                 ↑
      ┌─────────┴─────────┐
      ↑                   ↑
 Interval [a,b]    Interval [c,d]  ...
      ↑                   ↑
      └─────────┬─────────┘
                ↑
           Nothing (⊥)
```

### Key Properties

- **Monotonicity**: Information can only increase (intervals narrow)
- **Confluence**: Order of propagator execution doesn't affect final result
- **Soundness**: Propagated intervals always contain the true value

## References

1. Radul, A., & Sussman, G. J. (2009). *The Art of the Propagator*.
   MIT CSAIL Technical Report.
   [https://dspace.mit.edu/handle/1721.1/44215](https://dspace.mit.edu/handle/1721.1/44215)

2. Radul, A. (2009). *Propagation Networks: A Flexible and Expressive
   Substrate for Computation*. PhD Thesis, MIT.
   [https://dspace.mit.edu/handle/1721.1/54635](https://dspace.mit.edu/handle/1721.1/54635)

3. Stallman, R. M., & Sussman, G. J. (1977). *Forward Reasoning and
   Dependency-Directed Backtracking*. Artificial Intelligence, 9(2).

4. de Kleer, J. (1986). *An Assumption-based TMS*. Artificial Intelligence, 28(2).

5. Moore, R. E. (1966). *Interval Analysis*. Prentice-Hall.

## License

MIT License

---

*Gloria in excelsis Deo* 🕊️
