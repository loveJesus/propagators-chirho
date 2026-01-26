// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Truth Maintenance System (TMS) for dependency-directed reasoning.
//!
//! This module contains:
//! - Beliefs and justifications
//! - Nogood store for tracking contradictions
//! - TMS-aware cells and networks
//! - Worldview for hypothetical reasoning

// Allow module_inception: tms_chirho module in tms_chirho/ directory is intentional
// for consistent naming with file structure
#[allow(clippy::module_inception)]
pub mod tms_chirho;
pub mod worldview_chirho;

// Re-exports for convenience
pub use tms_chirho::{
    BeliefChirho, JustificationChirho, NogoodStoreChirho, PremiseSetChirho, SupportedChirho,
    TmsCellChirho, TmsNetworkChirho,
};
pub use worldview_chirho::WorldviewChirho;
