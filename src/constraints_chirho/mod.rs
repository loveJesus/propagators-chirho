// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! High-level constraint API for propagator networks.
//!
//! This module contains:
//! - ConstraintSystemChirho: High-level constraint system builder
//! - FiniteDomainChirho: Finite domain constraints (AllDifferent, etc.)
//! - AmbChirho: Nondeterministic choice with backtracking

pub mod amb_chirho;
pub mod finite_domain_chirho;
pub mod system_chirho;

// Re-exports for convenience
pub use amb_chirho::{
    AmbChirho, BacktrackingSearchChirho, DependencyDirectedSearchChirho, DomWdegChirho,
    FirstFailChirho, ImpactBasedChirho, MaxValueChirho, MiddleOutChirho, MinValueChirho,
    RestartSearchChirho, SearchResultChirho, SearchStatsChirho, ValueOrderingChirho,
    VariableOrderingChirho,
};
pub use finite_domain_chirho::{
    AllDifferentChirho, CardinalityChirho, CircuitChirho, CumulativeChirho, ElementChirho,
    EqualsChirho, FiniteDomainChirho, LessThanChirho, NotEqualsChirho, TableChirho,
};
pub use system_chirho::{
    CellBuilderChirho, CellExplanationChirho, CheckpointChirho, ConstraintIdChirho,
    ConstraintInfoChirho, ConstraintSystemChirho, ConstraintTypeChirho, PropagationGraphChirho,
};
