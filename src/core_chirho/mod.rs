// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Core foundational types for propagators.
//!
//! This module contains the basic building blocks that have no internal dependencies:
//! - Algebraic traits (Semigroup, Monoid, etc.)
//! - Interval arithmetic types
//! - SIMD batch operations

pub mod algebra_chirho;
pub mod interval_chirho;
pub mod simd_chirho;

// Re-exports for convenience
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
