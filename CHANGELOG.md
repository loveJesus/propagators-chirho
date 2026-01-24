<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.5] - 2025-01-24

### Added

- **WebSocket Transport** (`network-websocket` feature)
  - `WebSocketClientChirho` - Async WebSocket client for connecting to peers
  - `WebSocketServerChirho` - Async WebSocket server for hosting networks
  - `AsyncDistributedNetworkChirho` - High-level async network wrapper
  - Full duplex messaging with automatic reconnection handling
  - Tokio-based async runtime integration

- **PropCoin Cryptocurrency** (`crypto` feature)
  - `PropCoinLedgerChirho` - Propagator-based distributed ledger
  - `AccountChirho` - Accounts with interval balance bounds
  - `TransactionChirho` - Transfer transactions with nonce protection
  - `BlockChirho` - Blockchain blocks with transaction batching
  - CRDT-like balance merging for conflict-free distributed sync
  - Interval balances enable partial knowledge and privacy proofs
  - 6 comprehensive tests for ledger operations

## [0.1.4] - 2025-01-24

### Added

- **Cloudflare Durable Objects Module** (`cloudflare` feature)
  - `DurableCellStateChirho` - Persistent cell state for Durable Object storage
  - `WsMessageChirho` - WebSocket message types for real-time subscriptions
  - `MultiDoConfigChirho` - Configuration for sharding across multiple DOs
  - `ShardingStrategyChirho` - Strategies: single, hash-based, prefix-based, explicit
  - `HibernationHintsChirho` - Hints for DO hibernation optimization
  - Storage key helpers: `cell_key_chirho`, `CELL_INDEX_KEY_CHIRHO`, `CLOCK_KEY_CHIRHO`
  - Comprehensive documentation with TypeScript DO example
  - Best practices for persistence, hibernation, and cost optimization

## [0.1.3] - 2025-01-24

### Added

- **Distributed Network Module** (`network` feature)
  - `DistributedNetworkChirho` - Multi-node propagator network with automatic sync
  - `DistributedCellChirho` - Thread-safe cells with Lamport timestamps
  - `TransportChirho` trait - Pluggable transport layer (TCP, WebSocket, etc.)
  - `InMemoryTransportChirho` - In-process transport for testing
  - `NetworkMessageChirho` - Sync, Update, Heartbeat, Subscribe messages
  - CRDT-like conflict-free merging (monotonic lattice joins)
  - Gossip-based synchronization protocol

- **Test Script** (`scripts_chirho/test_all_chirho.sh`)
  - Comprehensive feature combination testing
  - Tests 28 configurations: individual features, combinations, no-std, examples, docs, clippy

### Fixed

- `no-std` mode now correctly imports `alloc::vec` in test modules

## [0.1.2] - 2025-01-24

### Changed

- **DRY Improvements**
  - Added `comparison_propagator_chirho!` macro for Max/Min propagators (reduces ~70 lines)
  - Added `impl_add_ternary_chirho!` and `impl_add_binary_chirho!` macros for arena add_* methods
  - Extracted helper methods in arena_chirho.rs: `run_ternary_chirho`, `run_ternary_with_guard_chirho`,
    `run_binary_chirho`, `run_binary_with_guard_chirho`, `run_binary_absoluter_chirho`
  - Reduces match arm complexity in arena `run_propagator_chirho` from ~130 lines to ~30 lines

### Added

- **New ConstraintSystemChirho Methods**
  - `add_negater_chirho` - Negation: -a = b
  - `add_exp_chirho` - Exponential: e^a = b
  - `add_ln_chirho` - Natural logarithm: ln(a) = b
  - `add_power_chirho` - Integer power: a^n = b
  - `add_clamp_chirho` - Clamping: clamp(a, lo, hi) = b

- **New Arena Propagator Types**
  - `SubtractorChirho` - Bidirectional a - b = c
  - `DividerChirho` - Bidirectional a / b = c
  - `SqrterChirho` - Bidirectional √a = b
  - `AbsoluterChirho` - Bidirectional |a| = b
  - `NegaterChirho` - Bidirectional -a = b

- **New Tests**
  - 8 arena propagator tests (total: 13)
  - 7 ConstraintSystemChirho tests (total: 12)
  - Total tests: 245

## [0.1.1] - 2025-01-24

### Changed

- **DRY Improvements**: Added `ternary_propagator_chirho!` and `binary_propagator_chirho!` macros
  - Eliminates ~300 lines of boilerplate in propagator definitions
  - Zero runtime overhead (macros expand at compile time)
  - Applies to: `IntervalAdderChirho`, `IntervalSubtractorChirho`, `IntervalMultiplierChirho`,
    `IntervalDividerChirho`, `SquarerChirho`, `SqrterChirho`, `AbsoluterChirho`

### Added

- **Algebraic Traits (Kmett-style)**
  - `SemigroupChirho` - Associative binary operation with `combine_chirho`
  - `CommutativeSemigroupChirho` - Commutative semigroup
  - `IdempotentSemigroupChirho` - Idempotent semigroup
  - `MonoidChirho` - Semigroup with identity element
  - `JoinSemilatticeChirho` - Idempotent, commutative monoid
  - `BoundedJoinSemilatticeChirho` - Semilattice with top element

- **Enhanced Truth Maintenance System**
  - `JustificationChirho` - Track derivation chains with antecedents
  - `NogoodStoreChirho` - Manage contradictory premise sets with minimal nogood computation
  - `TmsNetworkChirho` - Full TMS-aware propagator networks
  - Feature flag `tms-full` for full justification tracking

- **Dependency-Directed Backtracking**
  - `DependencyDirectedSearchChirho` - Smart backtracking using nogood information
  - Skips irrelevant choices based on conflict analysis
  - Tracks jump statistics for efficiency measurement

- **Error Handling**
  - `PropagatorErrorChirho` enum with structured error variants
  - `PropagatorResultChirho<T>` type alias
  - `try_make_cell_chirho` and `try_get_cell_chirho` for fallible operations
  - Error variants: `ContradictionChirho`, `InvalidIntervalChirho`, `CellNameNotFoundChirho`, `CellAlreadyExistsChirho`

- **no_std Support**
  - Feature flag `no-std` for embedded/bare-metal environments
  - Core interval arithmetic works without std
  - Uses `libm` for math operations in no_std mode
  - Modules available: `IntervalChirho`, `NumericInfoChirho`, `algebra_chirho`, `simd_chirho`

- **Tracing Support**
  - Feature flag `tracing` for debug instrumentation
  - `SpanGuardChirho` for RAII-style span management
  - Logs propagator invocations, cell updates, and scheduling

- **Serde Serialization**
  - Feature flag `serde` for serialization support
  - Serialization for `IntervalChirho` and `NumericInfoChirho`
  - Roundtrip tests in CI

- **Additional Kani Proofs**
  - Formal verification of interval arithmetic laws
  - Proofs for semilattice properties
  - Proofs for merge monotonicity

- **Property-Based Tests**
  - Extended proptest coverage for all interval operations
  - Tests for algebraic trait laws
  - Tests for TMS consistency

### Changed

- CI workflow now tests serde serialization
- CI workflow includes no_std build verification

## [0.1.0] - 2025-01-23

### Added

- **Core Infrastructure**
  - `IntervalChirho` - Interval arithmetic with proper math operations
  - `NumericInfoChirho` - Partial information lattice (Nothing → Interval → Contradiction)
  - `CellChirho<T>` - Cells with monotonic merge and neighbor notification
  - `PropagatorChirho` trait - Interface for building custom propagators
  - `SchedulerChirho` - Runs propagators to fixpoint

- **Propagators**
  - `IntervalAdderChirho` - Bidirectional a + b = c
  - `IntervalSubtractorChirho` - Bidirectional a - b = c
  - `IntervalMultiplierChirho` - Bidirectional a × b = c
  - `IntervalDividerChirho` - Bidirectional a ÷ b = c
  - `SquarerChirho` - Bidirectional a² = b
  - `SqrterChirho` - Bidirectional √a = b
  - `AbsoluterChirho` - Bidirectional |a| = b
  - `MaxChirho` - max(a, b) = c
  - `MinChirho` - min(a, b) = c
  - `ConditionalChirho` - if p > 0 then a else b = c
  - `ConstantChirho` - Sets cell to constant value

- **Truth Maintenance System**
  - `SupportedChirho<T>` - Values with supporting premises
  - `BeliefChirho` - Beliefs with source tracking
  - `TmsCellChirho` - Cells with multiple beliefs
  - Nogood detection for contradictory premise sets

- **Worldviews**
  - `WorldviewChirho` - Set of active premises
  - Fork, assume, and retract operations
  - Hypothetical reasoning support

- **Amb & Search**
  - `AmbChirho` - Nondeterministic choice
  - `BacktrackingSearchChirho` - Constraint satisfaction search

- **High-Level API**
  - `ConstraintSystemChirho` - Builder-style interface
  - Compound constraints (linear, Pythagorean)

- **Examples**
  - Temperature conversion (bidirectional)
  - Pythagorean theorem (backward computation)
  - Electrical circuits (Ohm's law, power)
  - Sudoku solver (constraint satisfaction)

- **Testing**
  - Unit tests for all modules
  - Property-based tests with proptest
  - Tests for mathematical invariants

### References

This implementation is based on:

1. Radul, A., & Sussman, G. J. (2009). *The Art of the Propagator*.
   MIT CSAIL Technical Report.

2. Radul, A. (2009). *Propagation Networks: A Flexible and Expressive
   Substrate for Computation*. PhD Thesis, MIT.
