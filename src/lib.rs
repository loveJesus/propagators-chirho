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

// =============================================================================
// Module Structure
// =============================================================================

// Core modules (always available, work in no_std with alloc)
pub mod core_chirho;

// Modules requiring std
#[cfg(not(feature = "no-std"))]
pub mod cells_chirho;

#[cfg(not(feature = "no-std"))]
pub mod propagators_chirho;

#[cfg(not(feature = "no-std"))]
pub mod constraints_chirho;

#[cfg(not(feature = "no-std"))]
pub mod tms_chirho;

#[cfg(not(feature = "no-std"))]
pub mod lattice_chirho;

// Feature-gated modules
#[cfg(not(feature = "no-std"))]
pub mod distributed_chirho;

#[cfg(not(feature = "no-std"))]
pub mod perf_chirho;

#[cfg(not(feature = "no-std"))]
pub mod bindings_chirho;

#[cfg(not(feature = "no-std"))]
pub mod debug_chirho;

// =============================================================================
// Re-exports for backward compatibility
// =============================================================================

// Core exports (always available)
pub use core_chirho::algebra_chirho;
pub use core_chirho::interval_chirho;
pub use core_chirho::simd_chirho;

pub use core_chirho::{
    batch_add_chirho, batch_intersect_chirho, batch_mul_chirho, batch_sqrt_chirho,
    batch_square_chirho, batch_sub_chirho, BoundedJoinSemilatticeChirho,
    CommutativeSemigroupChirho, IdempotentSemigroupChirho, IntervalChirho, IntervalVecChirho,
    JoinSemilatticeChirho, MonoidChirho, NumericInfoChirho, PropagatorErrorChirho,
    PropagatorResultChirho, SemigroupChirho,
};

// Exports requiring std
#[cfg(not(feature = "no-std"))]
pub use cells_chirho::{
    CellChirho, CellDeltaChirho, ConstraintWeightsChirho, GenericCellChirho, GenericNetworkChirho,
    IncrementalSchedulerChirho, IncrementalStatsChirho, MergeableChirho, PrioritySchedulerChirho,
    PrioritySchedulerStatsChirho, SchedulerChirho, SchedulerStatsChirho, WeightedConstraintChirho,
};

#[cfg(not(feature = "no-std"))]
pub use propagators_chirho::{
    AbsoluterChirho, ClampChirho, ConditionalChirho, ConstantChirho, ExpChirho,
    IntervalAdderChirho, IntervalDividerChirho, IntervalMultiplierChirho, IntervalSubtractorChirho,
    LnChirho, MaxChirho, MinChirho, NegaterChirho, PowerChirho, PropagatorChirho, SqrterChirho,
    SquarerChirho, DEFAULT_PRIORITY_CHIRHO, HIGH_PRIORITY_CHIRHO, LOW_PRIORITY_CHIRHO,
};

#[cfg(not(feature = "no-std"))]
pub use constraints_chirho::{
    AllDifferentChirho, AmbChirho, BacktrackingSearchChirho, CardinalityChirho, CellBuilderChirho,
    CellExplanationChirho, CheckpointChirho, CircuitChirho, ConstraintIdChirho, ConstraintInfoChirho,
    ConstraintSystemChirho, ConstraintTypeChirho, CumulativeChirho, DependencyDirectedSearchChirho,
    DomWdegChirho, ElementChirho, EqualsChirho, FiniteDomainChirho, FirstFailChirho,
    ImpactBasedChirho, LessThanChirho, MaxValueChirho, MiddleOutChirho, MinValueChirho,
    NotEqualsChirho, PropagationGraphChirho, RestartSearchChirho, SearchResultChirho,
    SearchStatsChirho, TableChirho, ValueOrderingChirho, VariableOrderingChirho,
};

#[cfg(not(feature = "no-std"))]
pub use tms_chirho::{
    BeliefChirho, JustificationChirho, NogoodStoreChirho, PremiseSetChirho, SupportedChirho,
    TmsCellChirho, TmsNetworkChirho, WorldviewChirho,
};

#[cfg(not(feature = "no-std"))]
pub use lattice_chirho::{
    AddPropagatorChirho, BoundedLatticeChirho, LatticeChirho, MulPropagatorChirho,
    PropagatorComposeChirho, PropagatorFnChirho, SquarePropagatorChirho, SupportedValueChirho,
};

// Feature-gated re-exports
#[cfg(all(feature = "arena", not(feature = "no-std")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "arena", not(feature = "no-std")))))]
pub use perf_chirho::arena_chirho::{ArenaNetworkChirho, CellIdChirho};

#[cfg(all(feature = "parallel", not(feature = "no-std")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "parallel", not(feature = "no-std")))))]
pub use perf_chirho::parallel_chirho::{
    NumericParallelNetworkChirho, ParallelCellChirho, ParallelNetworkChirho,
};

#[cfg(all(feature = "network", not(feature = "no-std")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "network", not(feature = "no-std")))))]
pub use distributed_chirho::network_chirho::{
    CellIdChirho as NetworkCellIdChirho, CellUpdateChirho, DistributedCellChirho,
    DistributedNetworkChirho, InMemoryTransportChirho, NetworkMessageChirho, TransportChirho,
    TransportErrorChirho,
};

#[cfg(all(feature = "tracing", not(feature = "no-std")))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "tracing", not(feature = "no-std")))))]
pub use debug_chirho::tracing_chirho;

// Storage exports (always available when not no-std)
#[cfg(not(feature = "no-std"))]
pub use perf_chirho::storage_chirho::{
    FileStorageChirho, InMemoryStorageChirho, NetworkStateChirho, StorageAdapterChirho,
    StorageErrorChirho, StorageResultChirho, CURRENT_SCHEMA_VERSION_CHIRHO,
};

// Note: kani_chirho module is only compiled when running under the kani verifier
// (it has #![cfg(kani)] at the module level), so we only re-export when kani is active
#[cfg(kani)]
pub use debug_chirho::kani_chirho;

// =============================================================================
// Prelude
// =============================================================================

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

    pub use crate::cells_chirho::{CellChirho, MergeableChirho, SchedulerChirho};
    pub use crate::constraints_chirho::{
        AmbChirho, BacktrackingSearchChirho, ConstraintSystemChirho, SearchResultChirho,
    };
    pub use crate::core_chirho::{
        BoundedJoinSemilatticeChirho, IntervalChirho, JoinSemilatticeChirho, MonoidChirho,
        NumericInfoChirho, PropagatorErrorChirho, PropagatorResultChirho, SemigroupChirho,
    };
    pub use crate::propagators_chirho::{
        AbsoluterChirho, ConditionalChirho, ConstantChirho, IntervalAdderChirho,
        IntervalDividerChirho, IntervalMultiplierChirho, IntervalSubtractorChirho, MaxChirho,
        MinChirho, PropagatorChirho, SqrterChirho, SquarerChirho,
    };
    pub use crate::tms_chirho::{
        BeliefChirho, PremiseSetChirho, SupportedChirho, TmsCellChirho, WorldviewChirho,
    };
}

/// Minimal prelude for no_std mode.
#[cfg(feature = "no-std")]
pub mod prelude_chirho {
    //! Minimal re-exports for no_std mode.
    //!
    //! Only interval arithmetic and algebraic traits are available.

    pub use crate::core_chirho::{
        BoundedJoinSemilatticeChirho, IntervalChirho, JoinSemilatticeChirho, MonoidChirho,
        NumericInfoChirho, PropagatorErrorChirho, PropagatorResultChirho, SemigroupChirho,
    };
}

// =============================================================================
// Tests
// =============================================================================

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
