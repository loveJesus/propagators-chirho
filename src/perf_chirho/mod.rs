// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Performance optimization features.
//!
//! This module contains:
//! - Arena-based allocation for reduced Rc overhead
//! - Parallel propagation using rayon
//! - Storage adapters for persistence

#[cfg(feature = "arena")]
pub mod arena_chirho;

#[cfg(feature = "parallel")]
pub mod parallel_chirho;

pub mod storage_chirho;

// Re-exports for convenience
#[cfg(feature = "arena")]
pub use arena_chirho::{ArenaNetworkChirho, CellIdChirho};

#[cfg(feature = "parallel")]
pub use parallel_chirho::{
    NumericParallelNetworkChirho, ParallelCellChirho, ParallelNetworkChirho,
};

pub use storage_chirho::{
    FileStorageChirho, InMemoryStorageChirho, NetworkStateChirho, StorageAdapterChirho,
    StorageErrorChirho, StorageResultChirho, CURRENT_SCHEMA_VERSION_CHIRHO,
};
