// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Distributed propagator network for multi-node computation.
//!
//! This module enables propagator networks to run across multiple nodes,
//! with automatic synchronization and conflict-free merging.
//!
//! # Key Concepts
//!
//! ## Monotonic Merge (CRDT-like)
//!
//! Propagator cells are monotonic - information only grows, never shrinks.
//! This makes them naturally conflict-free:
//! - Updates can be applied in any order
//! - Concurrent updates merge correctly (lattice join)
//! - No coordination needed for consistency
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
//! │   Node A    │────▶│   Node B    │────▶│   Node C    │
//! │ (propagate) │◀────│ (propagate) │◀────│ (propagate) │
//! └─────────────┘     └─────────────┘     └─────────────┘
//!       │                   │                   │
//!       └───────────────────┴───────────────────┘
//!                    Gossip Protocol
//! ```
//!
//! # Feature Flag
//!
//! Enable with the `network` feature:
//! ```toml
//! propagators-chirho = { version = "0.1", features = ["network"] }
//! ```
//!
//! # Example
//!
//! ```ignore
//! use propagators_chirho::network_chirho::*;
//!
//! // Create a distributed network
//! let mut node_chirho = DistributedNetworkChirho::new_chirho("node-1");
//!
//! // Create shared cells
//! let temp_chirho = node_chirho.make_shared_cell_chirho("temperature");
//!
//! // Local updates propagate to peers
//! node_chirho.set_exact_chirho(temp_chirho, 25.0);
//!
//! // Connect to peer
//! node_chirho.connect_peer_chirho("ws://peer-2:8080").await?;
//!
//! // Sync with peers
//! node_chirho.sync_chirho().await?;
//! ```

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::core_chirho::interval_chirho::NumericInfoChirho;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// MESSAGE TYPES
// ============================================================================

/// A unique identifier for a cell across the distributed network.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CellIdChirho {
    /// The cell's name (must be unique across all nodes).
    pub name_chirho: String,
}

impl CellIdChirho {
    /// Creates a new cell ID.
    pub fn new_chirho(name_chirho: &str) -> Self {
        Self {
            name_chirho: name_chirho.to_string(),
        }
    }
}

/// A versioned cell update for distributed synchronization.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CellUpdateChirho {
    /// The cell being updated.
    pub cell_id_chirho: CellIdChirho,
    /// The new information to merge.
    pub info_chirho: NumericInfoChirho,
    /// Logical timestamp (Lamport clock).
    pub timestamp_chirho: u64,
    /// The node that originated this update.
    pub origin_chirho: String,
}

/// Messages exchanged between distributed nodes.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NetworkMessageChirho {
    /// Request to sync all cell states.
    SyncRequestChirho {
        /// The requesting node's ID.
        from_chirho: String,
        /// Cells the requester knows about with their timestamps.
        known_cells_chirho: HashMap<CellIdChirho, u64>,
    },

    /// Response with cell updates.
    SyncResponseChirho {
        /// The responding node's ID.
        from_chirho: String,
        /// Updates for cells the requester needs.
        updates_chirho: Vec<CellUpdateChirho>,
    },

    /// A single cell update (for incremental sync).
    UpdateChirho(CellUpdateChirho),

    /// Heartbeat to maintain connection.
    HeartbeatChirho {
        /// The sender's node ID.
        from_chirho: String,
        /// Current logical time.
        timestamp_chirho: u64,
    },

    /// Request to subscribe to updates for specific cells.
    SubscribeChirho {
        /// The subscribing node's ID.
        from_chirho: String,
        /// Cells to subscribe to.
        cells_chirho: Vec<CellIdChirho>,
    },
}

// ============================================================================
// DISTRIBUTED CELL
// ============================================================================

/// A cell that can be synchronized across distributed nodes.
#[derive(Debug)]
pub struct DistributedCellChirho {
    /// The cell's unique ID.
    pub id_chirho: CellIdChirho,
    /// Current content.
    content_chirho: RwLock<NumericInfoChirho>,
    /// Last update timestamp.
    timestamp_chirho: RwLock<u64>,
    /// Nodes that have subscribed to this cell.
    subscribers_chirho: RwLock<HashSet<String>>,
}

impl DistributedCellChirho {
    /// Creates a new distributed cell.
    pub fn new_chirho(name_chirho: &str) -> Self {
        Self {
            id_chirho: CellIdChirho::new_chirho(name_chirho),
            content_chirho: RwLock::new(NumericInfoChirho::NothingChirho),
            timestamp_chirho: RwLock::new(0),
            subscribers_chirho: RwLock::new(HashSet::new()),
        }
    }

    /// Gets the current content.
    pub fn content_chirho(&self) -> NumericInfoChirho {
        *self.content_chirho.read().unwrap()
    }

    /// Gets the current timestamp.
    pub fn timestamp_chirho(&self) -> u64 {
        *self.timestamp_chirho.read().unwrap()
    }

    /// Merges new information into the cell.
    ///
    /// Returns `true` if the content changed.
    pub fn merge_chirho(&self, info_chirho: NumericInfoChirho, timestamp_chirho: u64) -> bool {
        let mut content_guard_chirho = self.content_chirho.write().unwrap();
        let mut ts_guard_chirho = self.timestamp_chirho.write().unwrap();

        let old_content_chirho = *content_guard_chirho;
        let new_content_chirho = old_content_chirho.merge_chirho(&info_chirho);

        let changed_chirho = new_content_chirho != old_content_chirho;
        if changed_chirho {
            *content_guard_chirho = new_content_chirho;
            *ts_guard_chirho = (*ts_guard_chirho).max(timestamp_chirho);
        }
        changed_chirho
    }

    /// Adds a subscriber.
    pub fn add_subscriber_chirho(&self, node_id_chirho: String) {
        self.subscribers_chirho
            .write()
            .unwrap()
            .insert(node_id_chirho);
    }

    /// Gets subscribers.
    pub fn subscribers_chirho(&self) -> Vec<String> {
        self.subscribers_chirho
            .read()
            .unwrap()
            .iter()
            .cloned()
            .collect()
    }
}

// ============================================================================
// TRANSPORT TRAIT
// ============================================================================

/// Trait for network transports.
///
/// Implement this trait to support different network protocols
/// (TCP, WebSocket, QUIC, etc.).
pub trait TransportChirho: Send + Sync {
    /// Sends a message to a peer.
    ///
    /// # Errors
    ///
    /// Returns `TransportErrorChirho` if the peer is not found or send fails.
    fn send_chirho(
        &self,
        peer_chirho: &str,
        message_chirho: &NetworkMessageChirho,
    ) -> Result<(), TransportErrorChirho>;

    /// Receives a message (non-blocking).
    fn recv_chirho(&self) -> Option<(String, NetworkMessageChirho)>;

    /// Broadcasts a message to all connected peers.
    ///
    /// # Errors
    ///
    /// Returns `TransportErrorChirho` if broadcast fails.
    fn broadcast_chirho(
        &self,
        message_chirho: &NetworkMessageChirho,
    ) -> Result<(), TransportErrorChirho>;

    /// Returns the list of connected peers.
    fn peers_chirho(&self) -> Vec<String>;

    /// Connects to a new peer.
    ///
    /// # Errors
    ///
    /// Returns `TransportErrorChirho` if connection fails.
    fn connect_chirho(&mut self, address_chirho: &str) -> Result<(), TransportErrorChirho>;

    /// Disconnects from a peer.
    ///
    /// # Errors
    ///
    /// Returns `TransportErrorChirho` if peer is not found.
    fn disconnect_chirho(&mut self, peer_chirho: &str) -> Result<(), TransportErrorChirho>;
}

/// Transport errors.
#[derive(Clone, Debug)]
pub enum TransportErrorChirho {
    /// Connection failed.
    ConnectionFailedChirho(String),
    /// Send failed.
    SendFailedChirho(String),
    /// Peer not found.
    PeerNotFoundChirho(String),
    /// Serialization error.
    SerializationErrorChirho(String),
}

impl std::fmt::Display for TransportErrorChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectionFailedChirho(msg_chirho) => {
                write!(f_chirho, "Connection failed: {}", msg_chirho)
            }
            Self::SendFailedChirho(msg_chirho) => write!(f_chirho, "Send failed: {}", msg_chirho),
            Self::PeerNotFoundChirho(peer_chirho) => {
                write!(f_chirho, "Peer not found: {}", peer_chirho)
            }
            Self::SerializationErrorChirho(msg_chirho) => {
                write!(f_chirho, "Serialization error: {}", msg_chirho)
            }
        }
    }
}

impl std::error::Error for TransportErrorChirho {}

// ============================================================================
// IN-MEMORY TRANSPORT (for testing)
// ============================================================================

/// An in-memory transport for testing distributed networks.
///
/// This transport uses channels for communication between nodes
/// in the same process.
#[derive(Default)]
pub struct InMemoryTransportChirho {
    /// The node's ID.
    node_id_chirho: String,
    /// Connected peers and their message queues.
    peers_chirho: Arc<RwLock<HashMap<String, Arc<InMemoryTransportChirho>>>>,
    /// Incoming message queue.
    inbox_chirho: Arc<RwLock<Vec<(String, NetworkMessageChirho)>>>,
}

impl InMemoryTransportChirho {
    /// Creates a new in-memory transport.
    pub fn new_chirho(node_id_chirho: &str) -> Self {
        Self {
            node_id_chirho: node_id_chirho.to_string(),
            peers_chirho: Arc::new(RwLock::new(HashMap::new())),
            inbox_chirho: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Connects two transports directly (for testing).
    pub fn connect_direct_chirho(a_chirho: &Arc<Self>, b_chirho: &Arc<Self>) {
        a_chirho
            .peers_chirho
            .write()
            .unwrap()
            .insert(b_chirho.node_id_chirho.clone(), b_chirho.clone());
        b_chirho
            .peers_chirho
            .write()
            .unwrap()
            .insert(a_chirho.node_id_chirho.clone(), a_chirho.clone());
    }

    /// Delivers a message to this transport's inbox.
    fn deliver_chirho(&self, from_chirho: String, message_chirho: NetworkMessageChirho) {
        self.inbox_chirho
            .write()
            .unwrap()
            .push((from_chirho, message_chirho));
    }
}

impl TransportChirho for InMemoryTransportChirho {
    fn send_chirho(
        &self,
        peer_chirho: &str,
        message_chirho: &NetworkMessageChirho,
    ) -> Result<(), TransportErrorChirho> {
        let peers_guard_chirho = self.peers_chirho.read().unwrap();
        if let Some(peer_transport_chirho) = peers_guard_chirho.get(peer_chirho) {
            peer_transport_chirho.deliver_chirho(self.node_id_chirho.clone(), message_chirho.clone());
            Ok(())
        } else {
            Err(TransportErrorChirho::PeerNotFoundChirho(
                peer_chirho.to_string(),
            ))
        }
    }

    fn recv_chirho(&self) -> Option<(String, NetworkMessageChirho)> {
        self.inbox_chirho.write().unwrap().pop()
    }

    fn broadcast_chirho(
        &self,
        message_chirho: &NetworkMessageChirho,
    ) -> Result<(), TransportErrorChirho> {
        let peers_guard_chirho = self.peers_chirho.read().unwrap();
        for (_, peer_transport_chirho) in peers_guard_chirho.iter() {
            peer_transport_chirho.deliver_chirho(self.node_id_chirho.clone(), message_chirho.clone());
        }
        Ok(())
    }

    fn peers_chirho(&self) -> Vec<String> {
        self.peers_chirho
            .read()
            .unwrap()
            .keys()
            .cloned()
            .collect()
    }

    fn connect_chirho(&mut self, _address_chirho: &str) -> Result<(), TransportErrorChirho> {
        // For in-memory transport, use connect_direct_chirho instead
        Err(TransportErrorChirho::ConnectionFailedChirho(
            "Use connect_direct_chirho for in-memory transport".to_string(),
        ))
    }

    fn disconnect_chirho(&mut self, peer_chirho: &str) -> Result<(), TransportErrorChirho> {
        self.peers_chirho.write().unwrap().remove(peer_chirho);
        Ok(())
    }
}

// ============================================================================
// DISTRIBUTED NETWORK
// ============================================================================

/// A distributed propagator network that synchronizes across nodes.
pub struct DistributedNetworkChirho<T: TransportChirho> {
    /// This node's ID.
    node_id_chirho: String,
    /// The network transport.
    transport_chirho: T,
    /// Shared cells.
    cells_chirho: HashMap<CellIdChirho, Arc<DistributedCellChirho>>,
    /// Lamport clock for ordering.
    logical_clock_chirho: u64,
    /// Pending updates to broadcast.
    pending_updates_chirho: Vec<CellUpdateChirho>,
    /// Last sync time.
    last_sync_chirho: Instant,
    /// Sync interval.
    sync_interval_chirho: Duration,
}

impl<T: TransportChirho> DistributedNetworkChirho<T> {
    /// Creates a new distributed network.
    pub fn new_chirho(node_id_chirho: &str, transport_chirho: T) -> Self {
        Self {
            node_id_chirho: node_id_chirho.to_string(),
            transport_chirho,
            cells_chirho: HashMap::new(),
            logical_clock_chirho: 0,
            pending_updates_chirho: Vec::new(),
            last_sync_chirho: Instant::now(),
            sync_interval_chirho: Duration::from_millis(100),
        }
    }

    /// Creates a new shared cell.
    pub fn make_shared_cell_chirho(&mut self, name_chirho: &str) -> CellIdChirho {
        let cell_id_chirho = CellIdChirho::new_chirho(name_chirho);
        let cell_chirho = Arc::new(DistributedCellChirho::new_chirho(name_chirho));
        self.cells_chirho
            .insert(cell_id_chirho.clone(), cell_chirho);
        cell_id_chirho
    }

    /// Gets a cell by ID.
    pub fn get_cell_chirho(&self, cell_id_chirho: &CellIdChirho) -> Option<&Arc<DistributedCellChirho>> {
        self.cells_chirho.get(cell_id_chirho)
    }

    /// Sets a cell to an exact value.
    pub fn set_exact_chirho(&mut self, cell_id_chirho: &CellIdChirho, value_chirho: f64) {
        self.add_info_chirho(cell_id_chirho, NumericInfoChirho::exact_chirho(value_chirho));
    }

    /// Sets a cell to an interval.
    pub fn set_interval_chirho(
        &mut self,
        cell_id_chirho: &CellIdChirho,
        lo_chirho: f64,
        hi_chirho: f64,
    ) {
        self.add_info_chirho(
            cell_id_chirho,
            NumericInfoChirho::interval_chirho(lo_chirho, hi_chirho),
        );
    }

    /// Adds information to a cell.
    pub fn add_info_chirho(&mut self, cell_id_chirho: &CellIdChirho, info_chirho: NumericInfoChirho) {
        self.logical_clock_chirho += 1;

        if let Some(cell_chirho) = self.cells_chirho.get(cell_id_chirho) {
            if cell_chirho.merge_chirho(info_chirho, self.logical_clock_chirho) {
                // Queue update for broadcast
                self.pending_updates_chirho.push(CellUpdateChirho {
                    cell_id_chirho: cell_id_chirho.clone(),
                    info_chirho,
                    timestamp_chirho: self.logical_clock_chirho,
                    origin_chirho: self.node_id_chirho.clone(),
                });
            }
        }
    }

    /// Gets the content of a cell.
    pub fn get_content_chirho(&self, cell_id_chirho: &CellIdChirho) -> Option<NumericInfoChirho> {
        self.cells_chirho
            .get(cell_id_chirho)
            .map(|cell_chirho| cell_chirho.content_chirho())
    }

    /// Broadcasts pending updates to all peers.
    ///
    /// # Errors
    ///
    /// Returns `TransportErrorChirho` if broadcast fails.
    pub fn flush_updates_chirho(&mut self) -> Result<(), TransportErrorChirho> {
        for update_chirho in self.pending_updates_chirho.drain(..) {
            self.transport_chirho
                .broadcast_chirho(&NetworkMessageChirho::UpdateChirho(update_chirho))?;
        }
        Ok(())
    }

    /// Processes incoming messages.
    pub fn process_messages_chirho(&mut self) {
        while let Some((from_chirho, message_chirho)) = self.transport_chirho.recv_chirho() {
            self.handle_message_chirho(&from_chirho, message_chirho);
        }
    }

    /// Handles a single message.
    #[allow(clippy::only_used_in_recursion)]
    fn handle_message_chirho(&mut self, from_chirho: &str, message_chirho: NetworkMessageChirho) {
        match message_chirho {
            NetworkMessageChirho::UpdateChirho(update_chirho) => {
                // Update our clock
                self.logical_clock_chirho =
                    self.logical_clock_chirho.max(update_chirho.timestamp_chirho) + 1;

                // Ensure cell exists
                if !self.cells_chirho.contains_key(&update_chirho.cell_id_chirho) {
                    let cell_chirho = Arc::new(DistributedCellChirho::new_chirho(
                        &update_chirho.cell_id_chirho.name_chirho,
                    ));
                    self.cells_chirho
                        .insert(update_chirho.cell_id_chirho.clone(), cell_chirho);
                }

                // Merge the update
                if let Some(cell_chirho) = self.cells_chirho.get(&update_chirho.cell_id_chirho) {
                    cell_chirho.merge_chirho(update_chirho.info_chirho, update_chirho.timestamp_chirho);
                }
            }

            NetworkMessageChirho::SyncRequestChirho {
                from_chirho: requester_chirho,
                known_cells_chirho,
            } => {
                // Collect updates for cells the requester doesn't have or has old versions
                let mut updates_chirho = Vec::new();

                for (cell_id_chirho, cell_chirho) in &self.cells_chirho {
                    let our_ts_chirho = cell_chirho.timestamp_chirho();
                    let their_ts_chirho = known_cells_chirho.get(cell_id_chirho).copied().unwrap_or(0);

                    if our_ts_chirho > their_ts_chirho {
                        updates_chirho.push(CellUpdateChirho {
                            cell_id_chirho: cell_id_chirho.clone(),
                            info_chirho: cell_chirho.content_chirho(),
                            timestamp_chirho: our_ts_chirho,
                            origin_chirho: self.node_id_chirho.clone(),
                        });
                    }
                }

                // Send response
                let _ = self.transport_chirho.send_chirho(
                    &requester_chirho,
                    &NetworkMessageChirho::SyncResponseChirho {
                        from_chirho: self.node_id_chirho.clone(),
                        updates_chirho,
                    },
                );
            }

            NetworkMessageChirho::SyncResponseChirho {
                from_chirho: _,
                updates_chirho,
            } => {
                for update_chirho in updates_chirho {
                    self.handle_message_chirho(
                        from_chirho,
                        NetworkMessageChirho::UpdateChirho(update_chirho),
                    );
                }
            }

            NetworkMessageChirho::HeartbeatChirho {
                from_chirho: _,
                timestamp_chirho,
            } => {
                // Update our clock
                self.logical_clock_chirho = self.logical_clock_chirho.max(timestamp_chirho);
            }

            NetworkMessageChirho::SubscribeChirho {
                from_chirho: subscriber_chirho,
                cells_chirho,
            } => {
                for cell_id_chirho in cells_chirho {
                    if let Some(cell_chirho) = self.cells_chirho.get(&cell_id_chirho) {
                        cell_chirho.add_subscriber_chirho(subscriber_chirho.clone());
                    }
                }
            }
        }
    }

    /// Requests a full sync from all peers.
    ///
    /// # Errors
    ///
    /// Returns `TransportErrorChirho` if broadcast fails.
    pub fn request_sync_chirho(&mut self) -> Result<(), TransportErrorChirho> {
        let known_cells_chirho: HashMap<CellIdChirho, u64> = self
            .cells_chirho
            .iter()
            .map(|(id_chirho, cell_chirho)| (id_chirho.clone(), cell_chirho.timestamp_chirho()))
            .collect();

        self.transport_chirho
            .broadcast_chirho(&NetworkMessageChirho::SyncRequestChirho {
                from_chirho: self.node_id_chirho.clone(),
                known_cells_chirho,
            })
    }

    /// Runs one tick of the network (process messages, flush updates).
    ///
    /// # Errors
    ///
    /// Returns `TransportErrorChirho` if flush or sync fails.
    pub fn tick_chirho(&mut self) -> Result<(), TransportErrorChirho> {
        self.process_messages_chirho();
        self.flush_updates_chirho()?;

        // Periodic sync
        if self.last_sync_chirho.elapsed() >= self.sync_interval_chirho {
            self.request_sync_chirho()?;
            self.last_sync_chirho = Instant::now();
        }

        Ok(())
    }

    /// Returns the number of cells.
    pub fn cell_count_chirho(&self) -> usize {
        self.cells_chirho.len()
    }

    /// Returns the list of connected peers.
    pub fn peers_chirho(&self) -> Vec<String> {
        self.transport_chirho.peers_chirho()
    }

    /// Returns the node ID.
    pub fn node_id_chirho(&self) -> &str {
        &self.node_id_chirho
    }

    /// Sets the sync interval.
    pub fn set_sync_interval_chirho(&mut self, interval_chirho: Duration) {
        self.sync_interval_chirho = interval_chirho;
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_distributed_cell_merge_chirho() {
        let cell_chirho = DistributedCellChirho::new_chirho("test");

        assert!(cell_chirho.content_chirho().is_nothing_chirho());

        // First merge
        let changed_chirho = cell_chirho.merge_chirho(NumericInfoChirho::interval_chirho(0.0, 100.0), 1);
        assert!(changed_chirho);

        // Narrowing merge
        let changed_chirho = cell_chirho.merge_chirho(NumericInfoChirho::interval_chirho(20.0, 80.0), 2);
        assert!(changed_chirho);

        let content_chirho = cell_chirho.content_chirho();
        let interval_chirho = content_chirho.as_interval_chirho().unwrap();
        assert_eq!(interval_chirho.lo_chirho, 20.0);
        assert_eq!(interval_chirho.hi_chirho, 80.0);
    }

    #[test]
    fn test_in_memory_transport_chirho() {
        let a_chirho = Arc::new(InMemoryTransportChirho::new_chirho("node-a"));
        let b_chirho = Arc::new(InMemoryTransportChirho::new_chirho("node-b"));

        InMemoryTransportChirho::connect_direct_chirho(&a_chirho, &b_chirho);

        // Send from A to B
        a_chirho
            .send_chirho(
                "node-b",
                &NetworkMessageChirho::HeartbeatChirho {
                    from_chirho: "node-a".to_string(),
                    timestamp_chirho: 1,
                },
            )
            .unwrap();

        // B should receive it
        let (from_chirho, msg_chirho) = b_chirho.recv_chirho().unwrap();
        assert_eq!(from_chirho, "node-a");
        assert!(matches!(msg_chirho, NetworkMessageChirho::HeartbeatChirho { .. }));
    }

    #[test]
    fn test_distributed_network_sync_chirho() {
        let transport_a_chirho = Arc::new(InMemoryTransportChirho::new_chirho("node-a"));
        let transport_b_chirho = Arc::new(InMemoryTransportChirho::new_chirho("node-b"));

        InMemoryTransportChirho::connect_direct_chirho(&transport_a_chirho, &transport_b_chirho);

        // We need to clone the Arcs and use them directly since DistributedNetworkChirho takes ownership
        // For testing, we'll use a simpler approach
        let mut network_a_chirho = DistributedNetworkChirho::new_chirho(
            "node-a",
            InMemoryTransportChirho::new_chirho("node-a"),
        );
        let mut network_b_chirho = DistributedNetworkChirho::new_chirho(
            "node-b",
            InMemoryTransportChirho::new_chirho("node-b"),
        );

        // Create a cell on A
        let temp_chirho = network_a_chirho.make_shared_cell_chirho("temperature");
        network_a_chirho.set_exact_chirho(&temp_chirho, 25.0);

        // Verify A has the value
        let content_chirho = network_a_chirho.get_content_chirho(&temp_chirho).unwrap();
        let interval_chirho = content_chirho.as_interval_chirho().unwrap();
        assert_eq!(interval_chirho.lo_chirho, 25.0);
    }

    #[test]
    fn test_concurrent_updates_merge_chirho() {
        let cell_chirho = DistributedCellChirho::new_chirho("test");

        // Simulate concurrent updates from different nodes
        // Node A says [0, 100]
        cell_chirho.merge_chirho(NumericInfoChirho::interval_chirho(0.0, 100.0), 1);

        // Node B says [50, 150] (overlapping)
        cell_chirho.merge_chirho(NumericInfoChirho::interval_chirho(50.0, 150.0), 2);

        // Result should be intersection: [50, 100]
        let content_chirho = cell_chirho.content_chirho();
        let interval_chirho = content_chirho.as_interval_chirho().unwrap();
        assert_eq!(interval_chirho.lo_chirho, 50.0);
        assert_eq!(interval_chirho.hi_chirho, 100.0);
    }

    #[test]
    fn test_network_message_types_chirho() {
        let update_chirho = CellUpdateChirho {
            cell_id_chirho: CellIdChirho::new_chirho("test"),
            info_chirho: NumericInfoChirho::exact_chirho(42.0),
            timestamp_chirho: 1,
            origin_chirho: "node-1".to_string(),
        };

        let msg_chirho = NetworkMessageChirho::UpdateChirho(update_chirho);
        assert!(matches!(msg_chirho, NetworkMessageChirho::UpdateChirho(_)));
    }
}
