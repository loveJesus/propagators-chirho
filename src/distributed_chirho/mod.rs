// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Distributed propagator network support.
//!
//! This module contains:
//! - DistributedNetworkChirho: Multi-node propagation
//! - WebSocket transport for real network communication
//! - Cloudflare Durable Objects integration

#[cfg(feature = "network")]
pub mod network_chirho;

#[cfg(feature = "network-websocket")]
pub mod websocket_chirho;

#[cfg(feature = "cloudflare")]
pub mod cloudflare_chirho;

// Re-exports for convenience
#[cfg(feature = "network")]
pub use network_chirho::{
    CellIdChirho, CellUpdateChirho, DistributedCellChirho, DistributedNetworkChirho,
    InMemoryTransportChirho, NetworkMessageChirho, TransportChirho, TransportErrorChirho,
};
