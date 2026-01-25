// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Cells and scheduling for propagator networks.
//!
//! This module contains:
//! - CellChirho: The basic container for partial information
//! - GenericCellChirho: A generic cell implementation
//! - SchedulerChirho: Coordinates propagator execution

pub mod cell_chirho;
pub mod generic_cell_chirho;
pub mod scheduler_chirho;

// Re-exports for convenience
pub use cell_chirho::{CellChirho, MergeableChirho};
pub use generic_cell_chirho::{GenericCellChirho, GenericNetworkChirho};
pub use scheduler_chirho::SchedulerChirho;
