// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! # Propagators Chirho
//!
//! A Rust implementation of propagator networks for constraint propagation
//! and bidirectional computation.
//!
//! ## Overview
//!
//! Propagators are a programming paradigm where autonomous agents (propagators)
//! watch cells containing partial information and update other cells when they
//! learn something new. Information flows in all directions, enabling powerful
//! constraint-based programming.
//!
//! This implementation is based on the seminal work:
//!
//! > Radul, A., & Sussman, G. J. (2009). *The Art of the Propagator*.
//! > MIT Computer Science and Artificial Intelligence Laboratory Technical Report.
//! > <https://dspace.mit.edu/handle/1721.1/44215>
//!
//! ## Key Concepts
//!
//! ### Partial Information
//!
//! Cells hold partial information that can only grow monotonically. For numeric
//! values, we use intervals `[lo, hi]` that narrow as we learn more:
//!
//! ```
//! use propagators_chirho::{IntervalChirho, NumericInfoChirho};
//!
//! // "I know the value is between 0 and 100"
//! let partial_chirho = NumericInfoChirho::interval_chirho(0.0, 100.0);
//!
//! // "Actually, it's between 20 and 50" - more precise!
//! let refined_chirho = partial_chirho.merge_chirho(&NumericInfoChirho::interval_chirho(20.0, 50.0));
//! ```
//!
//! ### Bidirectional Constraints
//!
//! Unlike traditional programming where data flows one way, propagators
//! enforce constraints in all directions:
//!
//! ```
//! use propagators_chirho::prelude_chirho::*;
//!
//! // Create a constraint system
//! let mut system_chirho = ConstraintSystemChirho::new_chirho();
//!
//! // Create cells for Fahrenheit and Celsius
//! let f_chirho = system_chirho.make_cell_chirho("fahrenheit");
//! let c_chirho = system_chirho.make_cell_chirho("celsius");
//!
//! // F = C * 9/5 + 32 works BOTH ways!
//! // (Implementation would use adder/multiplier propagators)
//! ```
//!
//! ### Truth Maintenance
//!
//! Track beliefs with their supporting premises, enabling hypothetical reasoning:
//!
//! ```
//! use propagators_chirho::{BeliefChirho, WorldviewChirho, NumericInfoChirho};
//!
//! // Create a belief supported by premise "sensor_a"
//! let belief_chirho = BeliefChirho::with_premises_chirho(
//!     NumericInfoChirho::exact_chirho(25.0),
//!     vec!["sensor_a".to_string()].into_iter().collect(),
//!     "temperature reading",
//! );
//!
//! // Query under different worldviews
//! let worldview_chirho = WorldviewChirho::new_chirho();
//! let with_sensor_chirho = worldview_chirho.assume_chirho("sensor_a".to_string());
//! ```
//!
//! ## Features
//!
//! - **Interval Arithmetic**: Proper interval math with `+`, `-`, `*`, `/`, `sqrt`, `square`
//! - **Bidirectional Propagators**: Constraints work in all directions
//! - **Truth Maintenance System (TMS)**: Track beliefs and their justifications
//! - **Worldviews**: Hypothetical reasoning with fork/assume/retract
//! - **Amb Operator**: Nondeterministic choice with backtracking
//! - **Scheduler**: Runs propagators to fixpoint
//!
//! ## Cargo Features
//!
//! - `arena`: High-performance arena-based cells (reduces Rc overhead)
//! - `parallel`: Parallel propagation using rayon
//! - `serde`: Serialization support for intervals and cells
//! - `tracing`: Debugging instrumentation (adds overhead)
//! - `tms-full`: Full TMS with justification tracking (memory overhead)
//! - `no-std`: Embedded/no_std support (see below)
//!
//! ## no_std Support
//!
//! Enable the `no-std` feature for embedded or bare-metal environments:
//!
//! ```toml
//! [dependencies]
//! propagators-chirho = { version = "0.1", default-features = false, features = ["no-std"] }
//! ```
//!
//! In no_std mode, only core modules are available:
//! - `IntervalChirho` and `NumericInfoChirho` for interval arithmetic
//! - `algebra_chirho` traits (`SemigroupChirho`, `MonoidChirho`, etc.)
//! - `simd_chirho` for batch SIMD operations
//!
//! The full propagator network (cells, schedulers, TMS) requires `std`.
//!
//! **Note**: The `no-std` feature is mutually exclusive with other features.
//!
//! ## References
//!
//! 1. Radul, A., & Sussman, G. J. (2009). *The Art of the Propagator*.
//!    MIT CSAIL Technical Report. <https://dspace.mit.edu/handle/1721.1/44215>
//!
//! 2. Radul, A. (2009). *Propagation Networks: A Flexible and Expressive
//!    Substrate for Computation*. PhD Thesis, MIT.
//!    <https://dspace.mit.edu/handle/1721.1/54635>
//!
//! 3. Stallman, R. M., & Sussman, G. J. (1977). *Forward Reasoning and
//!    Dependency-Directed Backtracking in a System for Computer-Aided
//!    Circuit Analysis*. Artificial Intelligence, 9(2), 135-196.
//!
//! 4. de Kleer, J. (1986). *An Assumption-based TMS*. Artificial Intelligence,
//!    28(2), 127-162.
//!
//! 5. Moore, R. E. (1966). *Interval Analysis*. Prentice-Hall.
//!
//! ## Example: Temperature Conversion
//!
//! ```
//! use propagators_chirho::prelude_chirho::*;
//!
//! // Create scheduler and cells
//! let scheduler_chirho = SchedulerChirho::new_chirho();
//! let celsius_chirho: std::rc::Rc<CellChirho<NumericInfoChirho>> =
//!     CellChirho::new_chirho("celsius");
//!
//! // Add a known Celsius value
//! celsius_chirho.add_content_chirho(
//!     NumericInfoChirho::exact_chirho(100.0),
//!     &scheduler_chirho
//! );
//!
//! // With proper propagators set up, fahrenheit would become 212.0
//! ```

// no_std support: use core + alloc when std is not available
// Note: Most modules require alloc for Vec, HashMap, etc.
// Only the core interval arithmetic in interval_chirho works with just core.
//
// To use in a no_std environment, enable the "no-std" feature and provide
// an alloc crate. Full propagator networks require alloc; only interval
// arithmetic can work with just core.
#![cfg_attr(feature = "no-std", no_std)]
#![doc(html_root_url = "https://docs.rs/propagators-chirho/0.1.0")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::similar_names)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::missing_fields_in_debug)]
#![allow(clippy::mixed_attributes_style)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::manual_assert)]
#![allow(clippy::unnecessary_literal_bound)]

// Provide alloc crate when in no_std mode
#[cfg(feature = "no-std")]
extern crate alloc;

// Core modules that work in no_std (with alloc)
pub mod algebra_chirho;
pub mod interval_chirho;
pub mod simd_chirho;

// Modules requiring std (cells, scheduling, propagators, etc.)
#[cfg(not(feature = "no-std"))]
pub mod amb_chirho;
#[cfg(not(feature = "no-std"))]
pub mod cell_chirho;
#[cfg(not(feature = "no-std"))]
pub mod constraint_system_chirho;
#[cfg(not(feature = "no-std"))]
pub mod finite_domain_chirho;
#[cfg(not(feature = "no-std"))]
pub mod generic_cell_chirho;
#[cfg(not(feature = "no-std"))]
pub mod lattice_chirho;
#[cfg(not(feature = "no-std"))]
pub mod propagator_chirho;
#[cfg(not(feature = "no-std"))]
pub mod scheduler_chirho;
#[cfg(not(feature = "no-std"))]
pub mod tms_chirho;
#[cfg(not(feature = "no-std"))]
pub mod tracing_chirho;
#[cfg(not(feature = "no-std"))]
pub mod worldview_chirho;

/// High-performance arena-based implementation.
///
/// Enable with the `arena` feature for reduced allocation overhead.
/// Not available in no_std mode.
#[cfg(all(feature = "arena", not(feature = "no-std")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "arena", not(feature = "no-std")))))]
pub mod arena_chirho;

/// Parallel propagation using rayon.
///
/// Enable with the `parallel` feature for concurrent propagation.
/// Not available in no_std mode.
#[cfg(all(feature = "parallel", not(feature = "no-std")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "parallel", not(feature = "no-std")))))]
pub mod parallel_chirho;

/// Distributed propagator network for multi-node computation.
///
/// Enable with the `network` feature for distributed propagation.
/// Provides CRDT-like conflict-free merging across nodes.
/// Not available in no_std mode.
#[cfg(all(feature = "network", not(feature = "no-std")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "network", not(feature = "no-std")))))]
pub mod network_chirho;

/// Cloudflare Durable Objects integration for long-lived propagator networks.
///
/// Enable with the `cloudflare` feature for patterns and utilities
/// for running propagators as stateful Durable Objects.
/// Not available in no_std mode.
#[cfg(all(feature = "cloudflare", not(feature = "no-std")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "cloudflare", not(feature = "no-std")))))]
pub mod cloudflare_chirho;

/// WebAssembly bindings for browser/Node.js usage.
///
/// Enable with the `wasm` and `arena` features.
/// Not available in no_std mode.
#[cfg(all(feature = "wasm", feature = "arena", not(feature = "no-std")))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "wasm", feature = "arena", not(feature = "no-std"))))
)]
pub mod wasm_chirho;

/// Python bindings via PyO3.
///
/// Enable with the `python` feature. Build with maturin.
/// Not available in no_std mode.
#[cfg(all(feature = "python", not(feature = "no-std")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "python", not(feature = "no-std")))))]
pub mod python_chirho;

/// Kani formal verification proofs.
///
/// Enable with the `kani` feature and run with `cargo kani`.
#[cfg(feature = "kani")]
#[cfg_attr(docsrs, doc(cfg(feature = "kani")))]
pub mod kani_proofs_chirho;

// Re-exports for convenience
// Core exports (always available)
pub use algebra_chirho::{
    BoundedJoinSemilatticeChirho, CommutativeSemigroupChirho, IdempotentSemigroupChirho,
    JoinSemilatticeChirho, MonoidChirho, PropagatorErrorChirho, PropagatorResultChirho,
    SemigroupChirho,
};
pub use interval_chirho::{IntervalChirho, NumericInfoChirho};
pub use simd_chirho::{
    batch_add_chirho, batch_intersect_chirho, batch_mul_chirho, batch_sqrt_chirho,
    batch_square_chirho, batch_sub_chirho, IntervalVecChirho,
};

// Exports requiring std (not available in no_std mode)
#[cfg(not(feature = "no-std"))]
pub use amb_chirho::{
    AmbChirho, BacktrackingSearchChirho, DependencyDirectedSearchChirho, SearchResultChirho,
};
#[cfg(not(feature = "no-std"))]
pub use cell_chirho::{CellChirho, MergeableChirho};
#[cfg(not(feature = "no-std"))]
pub use constraint_system_chirho::ConstraintSystemChirho;
#[cfg(not(feature = "no-std"))]
pub use finite_domain_chirho::{
    AllDifferentChirho, EqualsChirho, FiniteDomainChirho, LessThanChirho, NotEqualsChirho,
};
#[cfg(not(feature = "no-std"))]
pub use generic_cell_chirho::{GenericCellChirho, GenericNetworkChirho};
#[cfg(not(feature = "no-std"))]
pub use lattice_chirho::{
    AddPropagatorChirho, BoundedLatticeChirho, LatticeChirho, MulPropagatorChirho,
    PropagatorComposeChirho, PropagatorFnChirho, SquarePropagatorChirho, SupportedValueChirho,
};
#[cfg(not(feature = "no-std"))]
pub use propagator_chirho::{
    AbsoluterChirho, ClampChirho, ConditionalChirho, ConstantChirho, ExpChirho,
    IntervalAdderChirho, IntervalDividerChirho, IntervalMultiplierChirho, IntervalSubtractorChirho,
    LnChirho, MaxChirho, MinChirho, NegaterChirho, PowerChirho, PropagatorChirho, SqrterChirho,
    SquarerChirho,
};
#[cfg(not(feature = "no-std"))]
pub use scheduler_chirho::SchedulerChirho;
#[cfg(not(feature = "no-std"))]
pub use tms_chirho::{
    BeliefChirho, JustificationChirho, NogoodStoreChirho, PremiseSetChirho, SupportedChirho,
    TmsCellChirho, TmsNetworkChirho,
};
#[cfg(not(feature = "no-std"))]
pub use worldview_chirho::WorldviewChirho;

// Feature-gated re-exports (requires std)
#[cfg(all(feature = "arena", not(feature = "no-std")))]
pub use arena_chirho::{ArenaNetworkChirho, CellIdChirho};

#[cfg(all(feature = "parallel", not(feature = "no-std")))]
pub use parallel_chirho::{
    NumericParallelNetworkChirho, ParallelCellChirho, ParallelNetworkChirho,
};

#[cfg(all(feature = "network", not(feature = "no-std")))]
pub use network_chirho::{
    CellIdChirho as NetworkCellIdChirho, CellUpdateChirho, DistributedCellChirho,
    DistributedNetworkChirho, InMemoryTransportChirho, NetworkMessageChirho, TransportChirho,
    TransportErrorChirho,
};

/// Prelude module for convenient imports.
///
/// In no_std mode, only interval-related exports are available.
///
/// # Example
///
/// ```
/// use propagators_chirho::prelude_chirho::*;
/// ```
#[cfg(not(feature = "no-std"))]
pub mod prelude_chirho {
    //! Convenient re-exports for common usage.

    pub use crate::algebra_chirho::{
        BoundedJoinSemilatticeChirho, JoinSemilatticeChirho, MonoidChirho, PropagatorErrorChirho,
        PropagatorResultChirho, SemigroupChirho,
    };
    pub use crate::amb_chirho::{AmbChirho, BacktrackingSearchChirho, SearchResultChirho};
    pub use crate::cell_chirho::{CellChirho, MergeableChirho};
    pub use crate::constraint_system_chirho::ConstraintSystemChirho;
    pub use crate::interval_chirho::{IntervalChirho, NumericInfoChirho};
    pub use crate::propagator_chirho::{
        AbsoluterChirho, ConditionalChirho, ConstantChirho, IntervalAdderChirho,
        IntervalDividerChirho, IntervalMultiplierChirho, IntervalSubtractorChirho, MaxChirho,
        MinChirho, PropagatorChirho, SqrterChirho, SquarerChirho,
    };
    pub use crate::scheduler_chirho::SchedulerChirho;
    pub use crate::tms_chirho::{BeliefChirho, PremiseSetChirho, SupportedChirho, TmsCellChirho};
    pub use crate::worldview_chirho::WorldviewChirho;
}

/// Minimal prelude for no_std mode.
#[cfg(feature = "no-std")]
pub mod prelude_chirho {
    //! Minimal re-exports for no_std mode.
    //!
    //! Only interval arithmetic and algebraic traits are available.

    pub use crate::algebra_chirho::{
        BoundedJoinSemilatticeChirho, JoinSemilatticeChirho, MonoidChirho, PropagatorErrorChirho,
        PropagatorResultChirho, SemigroupChirho,
    };
    pub use crate::interval_chirho::{IntervalChirho, NumericInfoChirho};
}

#[cfg(test)]
#[cfg(not(feature = "no-std"))]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_library_compiles_chirho() {
        // Basic smoke test
        let _interval_chirho = IntervalChirho::exact_chirho(42.0);
        let _scheduler_chirho = SchedulerChirho::new_chirho();
    }
}
