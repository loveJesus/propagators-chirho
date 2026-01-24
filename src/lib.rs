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

pub mod amb_chirho;
pub mod cell_chirho;
pub mod constraint_system_chirho;
pub mod finite_domain_chirho;
pub mod generic_cell_chirho;
pub mod interval_chirho;
pub mod lattice_chirho;
pub mod propagator_chirho;
pub mod scheduler_chirho;
pub mod tms_chirho;
pub mod worldview_chirho;

/// High-performance arena-based implementation.
///
/// Enable with the `arena` feature for reduced allocation overhead.
#[cfg(feature = "arena")]
#[cfg_attr(docsrs, doc(cfg(feature = "arena")))]
pub mod arena_chirho;

/// Parallel propagation using rayon.
///
/// Enable with the `parallel` feature for concurrent propagation.
#[cfg(feature = "parallel")]
#[cfg_attr(docsrs, doc(cfg(feature = "parallel")))]
pub mod parallel_chirho;

/// WebAssembly bindings for browser/Node.js usage.
///
/// Enable with the `wasm` and `arena` features.
#[cfg(all(feature = "wasm", feature = "arena"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "wasm", feature = "arena"))))]
pub mod wasm_chirho;

/// Python bindings via PyO3.
///
/// Enable with the `python` feature. Build with maturin.
#[cfg(feature = "python")]
#[cfg_attr(docsrs, doc(cfg(feature = "python")))]
pub mod python_chirho;

// Re-exports for convenience
pub use amb_chirho::{AmbChirho, BacktrackingSearchChirho, SearchResultChirho};
pub use cell_chirho::{CellChirho, MergeableChirho};
pub use constraint_system_chirho::ConstraintSystemChirho;
pub use interval_chirho::{IntervalChirho, NumericInfoChirho};
pub use propagator_chirho::{
    AbsoluterChirho, ConditionalChirho, ConstantChirho, IntervalAdderChirho, IntervalDividerChirho,
    IntervalMultiplierChirho, IntervalSubtractorChirho, MaxChirho, MinChirho, PropagatorChirho,
    SqrterChirho, SquarerChirho,
};
pub use scheduler_chirho::SchedulerChirho;
pub use tms_chirho::{BeliefChirho, PremiseSetChirho, SupportedChirho, TmsCellChirho};
pub use worldview_chirho::WorldviewChirho;
pub use lattice_chirho::{
    LatticeChirho, BoundedLatticeChirho, PropagatorFnChirho, PropagatorComposeChirho,
    SupportedValueChirho, AddPropagatorChirho, MulPropagatorChirho, SquarePropagatorChirho,
};
pub use finite_domain_chirho::{
    FiniteDomainChirho, AllDifferentChirho, LessThanChirho, EqualsChirho, NotEqualsChirho,
};
pub use generic_cell_chirho::{GenericCellChirho, GenericNetworkChirho};

// Feature-gated re-exports
#[cfg(feature = "arena")]
pub use arena_chirho::{ArenaNetworkChirho, CellIdChirho};

#[cfg(feature = "parallel")]
pub use parallel_chirho::{ParallelCellChirho, ParallelNetworkChirho, NumericParallelNetworkChirho};

/// Prelude module for convenient imports.
///
/// # Example
///
/// ```
/// use propagators_chirho::prelude_chirho::*;
/// ```
pub mod prelude_chirho {
    //! Convenient re-exports for common usage.

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

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_library_compiles_chirho() {
        // Basic smoke test
        let _interval_chirho = IntervalChirho::exact_chirho(42.0);
        let _scheduler_chirho = SchedulerChirho::new_chirho();
    }
}
