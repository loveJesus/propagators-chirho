// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Debug and verification features.
//!
//! This module contains:
//! - Tracing instrumentation for debugging propagation
//! - Kani formal verification proofs

#[cfg(not(feature = "no-std"))]
pub mod tracing_chirho;

#[cfg(feature = "kani")]
pub mod kani_chirho;
