<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

# propagators-chirho

[![Crates.io](https://img.shields.io/crates/v/propagators-chirho.svg)](https://crates.io/crates/propagators-chirho)
[![Documentation](https://docs.rs/propagators-chirho/badge.svg)](https://docs.rs/propagators-chirho)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Kani](https://img.shields.io/badge/Kani-Verified-green.svg)](https://model-checking.github.io/kani/)
[![Demo](https://img.shields.io/badge/Demo-Live-blue.svg)](https://propagators-chirho.lovejesus.software/)

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
- `JustificationChirho` — Track derivation chains (with `tms-full` feature)
- `NogoodStoreChirho` — Manage contradictory premise sets with minimal nogood computation
- `TmsNetworkChirho` — Full TMS-aware propagator networks

### Worldviews (Hypothetical Reasoning)

- `WorldviewChirho` — Set of active premises
- Fork and explore alternate assumptions
- Retract premises when they lead to contradictions

### Amb Operator (Nondeterministic Choice)

- `AmbChirho` — Choice among values
- `BacktrackingSearchChirho` — Constraint satisfaction with backtracking
- `DependencyDirectedSearchChirho` — Smart backtracking using nogood information (skips irrelevant choices)

### High-Level API

- `ConstraintSystemChirho` — Builder-style interface for constraint networks

### Algebraic Traits (Kmett-style)

- `SemigroupChirho` — Associative binary operation
- `MonoidChirho` — Semigroup with identity element
- `JoinSemilatticeChirho` — Idempotent, commutative monoid
- `BoundedJoinSemilatticeChirho` — Semilattice with top element
- `PropagatorErrorChirho` / `PropagatorResultChirho` — Proper error handling

### Lattice Abstractions

- `LatticeChirho` trait — Join semilattice with partial order
- `BoundedLatticeChirho` trait — Lattice with top (contradiction) and bottom (nothing)
- `PropagatorFnChirho` trait — Composable functional propagators
- `SupportedValueChirho<L>` — Values with provenance tracking

### Finite Domains (for CSP)

- `FiniteDomainChirho` — Set-based domains for discrete values
- `AllDifferentChirho` — Global constraint for Sudoku-like problems
- `LessThanChirho`, `EqualsChirho`, `NotEqualsChirho` — Relational constraints

### Global Constraints (v0.2+)

- `ElementChirho` — Array indexing constraint (result = array[index])
- `TableChirho` — Extensional constraint with explicit allowed tuples
- `CircuitChirho` — Hamiltonian circuit for TSP problems
- `CumulativeChirho` — Resource scheduling with time-table filtering
- `CardinalityChirho` — Bounds on value occurrence counts

### Search Heuristics (v0.2+)

- `FirstFailChirho` — Smallest domain first (most constrained variable)
- `DomWdegChirho` — Domain over weighted degree with failure recording
- `ImpactBasedChirho` — Impact measurement with running averages
- `MinValueChirho`, `MaxValueChirho`, `MiddleOutChirho` — Value ordering strategies
- `RestartSearchChirho` — Luby sequence restarts with nogood learning

### Unified Network Architecture (v0.3+)

- `UnifiedNetworkChirho<T, TmsMode, AllocMode, ExecMode>` — Composable network with phantom type configuration
- `StandardModeChirho` / `TmsModeChirho` — Optional TMS support
- `NetworkBuilderChirho` — Fluent construction of networks
- Type aliases: `NumericNetworkChirho`, `TmsNumericNetworkChirho`

```rust
use propagators_chirho::{UnifiedNetworkChirho, TmsModeChirho, NumericInfoChirho};

// Standard network (no TMS overhead)
let mut net_chirho = UnifiedNetworkChirho::<NumericInfoChirho>::new_chirho();

// TMS-enabled network (tracks beliefs and premises)
let mut tms_net_chirho = UnifiedNetworkChirho::<NumericInfoChirho, TmsModeChirho>::with_tms_chirho();
```

### Storage & Persistence (v0.2+)

- `StorageAdapterChirho` trait — Generic storage interface
- `InMemoryStorageChirho` — In-memory persistence for testing
- `FileStorageChirho` — JSON file-based persistence
- `NetworkStateChirho` — Serializable network snapshots with schema versioning

### Visualization & Debugging (v0.2+)

- `PropagationTraceChirho` — Record propagation events with timestamps
- `CellHistoryChirho` — Track value changes per cell
- `NetworkSnapshotChirho` — Complete state capture with JSON/DOT export
- `PropagationGraphChirho` — Visualize constraint networks (DOT format)

### Generic Cells

- `GenericCellChirho<L>` — Cells that work with any lattice type
- `GenericNetworkChirho<L>` — Networks for custom lattice types

### Parallel Propagation

- `ParallelNetworkChirho<L>` — Thread-safe parallel propagation with rayon
- Independent propagators run concurrently in batches

### SIMD Batch Operations

- `batch_add_chirho`, `batch_mul_chirho`, `batch_intersect_chirho` — Batch interval operations
- `IntervalVecChirho` — Structure-of-Arrays format for optimal vectorization
- Auto-vectorizes on modern compilers for 2-4x speedup on batch operations

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
propagators-chirho = "0.3"
```

## Cargo Features

| Feature | Description | Overhead |
|---------|-------------|----------|
| `arena` | High-performance arena-based cells | Faster, uses typed-arena |
| `parallel` | Parallel propagation with rayon | Thread-safe, uses rayon |
| `serde` | Serialization for intervals/cells | Adds serde dependency |
| `tracing` | Debug instrumentation | Runtime overhead when enabled |
| `tms-full` | Full TMS with justification tracking | Memory overhead |
| `backtrack` | Dependency-directed backtracking | Memory + CPU overhead |
| `no-std` | Embedded/bare-metal support | Limited modules |

### no_std Support

For embedded or bare-metal environments:

```toml
[dependencies]
propagators-chirho = { version = "0.3", default-features = false, features = ["no-std"] }
```

Available modules in no_std mode:
- `IntervalChirho` / `NumericInfoChirho` — Interval arithmetic
- `algebra_chirho` traits — `SemigroupChirho`, `MonoidChirho`, etc.
- `simd_chirho` — Batch SIMD operations

**Note**: `no-std` is mutually exclusive with other features.

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

### Unified Network (v0.3+)

```rust
use propagators_chirho::{UnifiedNetworkChirho, NumericNetworkChirho};

// The unified network provides a clean API for constraint networks
let mut network_chirho: NumericNetworkChirho = UnifiedNetworkChirho::new_chirho();

// Create cells and constraints
let a_chirho = network_chirho.make_exact_chirho("a", 3.0);
let b_chirho = network_chirho.make_exact_chirho("b", 4.0);
let c_chirho = network_chirho.make_cell_chirho("c");

// a + b = c (bidirectional!)
network_chirho.add_adder_chirho(a_chirho, b_chirho, c_chirho.clone());
network_chirho.run_chirho();

// c is now [7, 7]
```

## Running Examples

```bash
cargo run --example temperature_chirho        # Bidirectional temperature conversion
cargo run --example pythagorean_chirho        # Pythagorean theorem
cargo run --example electrical_chirho         # Electrical circuit analysis
cargo run --example sudoku_chirho             # Sudoku solver with backtracking
cargo run --example search_heuristics_chirho  # N-Queens with variable ordering heuristics
cargo run --example scheduling_chirho         # Resource scheduling with cumulative
```

## Performance

### Features

Enable high-performance arena mode for ~2x faster network operations:

```toml
[dependencies]
propagators-chirho = { version = "0.3", features = ["arena"] }
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

### IEEE 754-2019 Compliance: When Does It Matter?

**inari** provides IEEE 754-2019 compliant interval arithmetic. We don't. Here's when that matters:

| Use Case | Need IEEE? | Recommendation |
|----------|------------|----------------|
| Constraint solving / SAT | No | Use propagators-chirho |
| Bidirectional computation | No | Use propagators-chirho |
| Verified numerical proofs | **Yes** | Use inari |
| Safety-critical systems | **Yes** | Use inari |
| Financial calculations | **Yes** | Use inari |
| Game physics / graphics | No | Use propagators-chirho |
| Optimization / search | No | Use propagators-chirho |

**What IEEE compliance guarantees:**
- The true mathematical result is *always* within the computed interval
- Proper directed rounding (round toward ±∞)
- Correct handling of NaN, infinities, subnormals
- Reproducible results across platforms

**What we sacrifice for speed:**
- Use default round-to-nearest (may miss true value by ULP in edge cases)
- Simpler NaN/infinity handling
- Potentially tighter intervals that could theoretically exclude the true value

**Bottom line:** For constraint propagation and bidirectional computation (our primary use case), IEEE compliance is unnecessary overhead. If you need verified numerical computation, use inari and accept the 28-51x slowdown.

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
cargo test                                          # Run all tests
cargo test --features arena                         # Test arena mode
cargo test --features parallel                      # Test parallel mode
cargo test --features serde                         # Test serialization
cargo test --features "parallel,serde,arena,tracing,tms-full"  # All std features
cargo test --test property_tests_chirho             # Property-based tests
```

### Formal Verification (Kani)

```bash
cargo kani --features kani    # Run Kani proofs for interval arithmetic laws
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
