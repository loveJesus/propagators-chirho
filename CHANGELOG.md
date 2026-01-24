<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
