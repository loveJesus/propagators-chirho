// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Cloudflare Durable Objects integration for propagator networks.
//!
//! This module provides patterns for running propagator networks as
//! long-lived Durable Objects with automatic state persistence.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    Cloudflare Edge                          │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
//! │  │   Worker    │  │   Worker    │  │   Worker    │         │
//! │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘         │
//! │         │                │                │                 │
//! │         └────────────────┼────────────────┘                 │
//! │                          ▼                                  │
//! │              ┌───────────────────────┐                      │
//! │              │    Durable Object     │                      │
//! │              │  ┌─────────────────┐  │                      │
//! │              │  │  Propagator     │  │                      │
//! │              │  │  Network        │  │◀──── Long-lived     │
//! │              │  │  (cells, props) │  │      Stateful       │
//! │              │  └─────────────────┘  │                      │
//! │              │          │            │                      │
//! │              │    ┌─────▼─────┐      │                      │
//! │              │    │  Storage  │      │◀──── Persistent     │
//! │              │    │  (K/V)    │      │      State          │
//! │              │    └───────────┘      │                      │
//! │              └───────────────────────┘                      │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Why Propagators Work Well with Durable Objects
//!
//! 1. **Monotonic State**: Propagator cells only grow more precise,
//!    never lose information. This maps perfectly to DO's transactional
//!    storage - updates are additive.
//!
//! 2. **CRDT-like Merging**: When multiple Workers send updates to the
//!    same DO, the propagator network merges them conflict-free.
//!
//! 3. **Lazy Evaluation**: Propagators only compute when inputs change.
//!    Combined with DO's hibernation, this is very cost-efficient.
//!
//! 4. **Bidirectional Computation**: Set any variable and the network
//!    computes all dependent values automatically.
//!
//! # Persistence Strategy
//!
//! Cells are persisted on every update to durable storage:
//!
//! ```text
//! cell:{cell_name} -> { content: NumericInfo, timestamp: u64 }
//! ```
//!
//! On DO wake-up, cells are lazily loaded from storage.
//!
//! # WebSocket Subscriptions
//!
//! Clients can subscribe to cell updates via WebSocket:
//!
//! ```text
//! Client ──WS──▶ DO
//!                 │
//!                 ├── Subscribe to "temperature"
//!                 │
//!                 ◀── Push updates when cell changes
//! ```
//!
//! # Example Durable Object (TypeScript)
//!
//! ```typescript
//! // This shows the JS/TS side - Rust propagator logic runs via WASM
//!
//! export class PropagatorDO implements DurableObject {
//!   state: DurableObjectState;
//!   network: PropagatorNetwork; // WASM binding
//!   subscribers: Map<string, Set<WebSocket>>;
//!
//!   constructor(state: DurableObjectState) {
//!     this.state = state;
//!     this.subscribers = new Map();
//!
//!     // Restore propagator network from storage
//!     this.state.blockConcurrencyWhile(async () => {
//!       const stored = await this.state.storage.get("network");
//!       this.network = stored
//!         ? PropagatorNetwork.deserialize(stored)
//!         : new PropagatorNetwork();
//!     });
//!   }
//!
//!   async fetch(request: Request): Promise<Response> {
//!     const url = new URL(request.url);
//!
//!     // WebSocket upgrade for real-time subscriptions
//!     if (request.headers.get("Upgrade") === "websocket") {
//!       const { 0: client, 1: server } = new WebSocketPair();
//!       this.handleWebSocket(server, url);
//!       return new Response(null, { status: 101, webSocket: client });
//!     }
//!
//!     // REST API for cell operations
//!     if (url.pathname.startsWith("/cell/")) {
//!       const cellName = url.pathname.split("/")[2];
//!
//!       if (request.method === "GET") {
//!         const content = this.network.getContent(cellName);
//!         return Response.json({ cell: cellName, content });
//!       }
//!
//!       if (request.method === "PUT") {
//!         const { value, lo, hi } = await request.json();
//!         if (value !== undefined) {
//!           this.network.setExact(cellName, value);
//!         } else {
//!           this.network.setInterval(cellName, lo, hi);
//!         }
//!
//!         // Persist and notify
//!         await this.persist();
//!         this.notifySubscribers(cellName);
//!
//!         return Response.json({ ok: true });
//!       }
//!     }
//!
//!     return new Response("Not found", { status: 404 });
//!   }
//!
//!   async persist() {
//!     await this.state.storage.put("network", this.network.serialize());
//!   }
//!
//!   notifySubscribers(cellName: string) {
//!     const subs = this.subscribers.get(cellName);
//!     if (subs) {
//!       const content = this.network.getContent(cellName);
//!       const msg = JSON.stringify({ cell: cellName, content });
//!       subs.forEach(ws => ws.send(msg));
//!     }
//!   }
//! }
//! ```
//!
//! # Feature Flag
//!
//! Enable with the `cloudflare` feature:
//! ```toml
//! propagators-chirho = { version = "0.1", features = ["cloudflare", "wasm", "serde"] }
//! ```

use crate::core_chirho::interval_chirho::NumericInfoChirho;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// DURABLE CELL STATE
// ============================================================================

/// State of a cell for persistence in Durable Object storage.
///
/// This is designed to be serialized to JSON and stored in
/// Cloudflare's key-value storage.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DurableCellStateChirho {
    /// The cell's content (interval or exact value).
    pub content_chirho: NumericInfoChirho,
    /// Lamport timestamp for ordering.
    pub timestamp_chirho: u64,
    /// Last modification time (Unix millis).
    pub modified_at_chirho: u64,
    /// Origin node (for distributed setups).
    pub origin_chirho: Option<String>,
}

impl DurableCellStateChirho {
    /// Creates a new durable cell state.
    pub fn new_chirho(content_chirho: NumericInfoChirho, timestamp_chirho: u64) -> Self {
        Self {
            content_chirho,
            timestamp_chirho,
            modified_at_chirho: 0, // Set by JS side with Date.now()
            origin_chirho: None,
        }
    }

    /// Merges with another state, keeping the more precise value.
    ///
    /// Returns `true` if the state changed.
    pub fn merge_chirho(&mut self, other_chirho: &Self) -> bool {
        let old_content_chirho = self.content_chirho;
        self.content_chirho = self
            .content_chirho
            .merge_chirho(&other_chirho.content_chirho);
        self.timestamp_chirho = self.timestamp_chirho.max(other_chirho.timestamp_chirho);

        self.content_chirho != old_content_chirho
    }
}

// ============================================================================
// STORAGE KEYS
// ============================================================================

/// Key format for cell storage.
///
/// Cells are stored with prefix `cell:` followed by the cell name.
pub fn cell_key_chirho(name_chirho: &str) -> String {
    format!("cell:{}", name_chirho)
}

/// Key for storing the list of all cell names.
pub const CELL_INDEX_KEY_CHIRHO: &str = "meta:cells";

/// Key for storing the network's Lamport clock.
pub const CLOCK_KEY_CHIRHO: &str = "meta:clock";

// ============================================================================
// HIBERNATION SUPPORT
// ============================================================================

/// Hints for Durable Object hibernation.
///
/// Propagator networks can hibernate efficiently because:
/// - State is fully persisted to storage
/// - No in-memory caches needed
/// - Cells can be lazily loaded on access
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HibernationHintsChirho {
    /// Cells that were accessed recently (for prefetching).
    pub hot_cells_chirho: Vec<String>,
    /// Last activity timestamp.
    pub last_activity_chirho: u64,
    /// Number of active WebSocket subscriptions.
    pub subscriber_count_chirho: usize,
}

// ============================================================================
// WEBSOCKET MESSAGE TYPES
// ============================================================================

/// Messages for WebSocket communication.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum WsMessageChirho {
    /// Subscribe to cell updates.
    SubscribeChirho {
        /// Cells to subscribe to.
        cells_chirho: Vec<String>,
    },

    /// Unsubscribe from cell updates.
    UnsubscribeChirho {
        /// Cells to unsubscribe from.
        cells_chirho: Vec<String>,
    },

    /// Set a cell to an exact value.
    SetExactChirho {
        /// Cell name.
        cell_chirho: String,
        /// Exact value.
        value_chirho: f64,
    },

    /// Set a cell to an interval.
    SetIntervalChirho {
        /// Cell name.
        cell_chirho: String,
        /// Lower bound.
        lo_chirho: f64,
        /// Upper bound.
        hi_chirho: f64,
    },

    /// Cell update notification (server -> client).
    UpdateChirho {
        /// Cell name.
        cell_chirho: String,
        /// New content.
        content_chirho: NumericInfoChirho,
        /// Timestamp.
        timestamp_chirho: u64,
    },

    /// Error message.
    ErrorChirho {
        /// Error message.
        message_chirho: String,
    },
}

// ============================================================================
// MULTI-DO COORDINATION
// ============================================================================

/// Configuration for coordinating multiple Durable Objects.
///
/// For large systems, you may want to shard cells across multiple DOs.
/// This configuration helps with routing and synchronization.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MultiDoConfigChirho {
    /// How to shard cells across DOs.
    pub sharding_chirho: ShardingStrategyChirho,
    /// Sync interval for cross-DO propagation (milliseconds).
    pub sync_interval_ms_chirho: u64,
    /// Maximum cells per DO before splitting.
    pub max_cells_per_do_chirho: usize,
}

impl Default for MultiDoConfigChirho {
    fn default() -> Self {
        Self {
            sharding_chirho: ShardingStrategyChirho::SingleDoChirho,
            sync_interval_ms_chirho: 1000,
            max_cells_per_do_chirho: 10000,
        }
    }
}

/// Strategy for sharding cells across Durable Objects.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ShardingStrategyChirho {
    /// All cells in a single DO (simplest, for small systems).
    SingleDoChirho,
    /// Hash cell names to determine DO (consistent hashing).
    HashBasedChirho {
        /// Number of shards.
        shard_count_chirho: usize,
    },
    /// Cells with the same prefix go to the same DO.
    PrefixBasedChirho {
        /// Delimiter for prefix extraction.
        delimiter_chirho: String,
    },
    /// Explicit cell-to-DO mapping.
    ExplicitChirho {
        /// Mapping from cell patterns to DO names.
        mapping_chirho: Vec<(String, String)>,
    },
}

// ============================================================================
// BEST PRACTICES
// ============================================================================

/// Best practices for using propagators with Durable Objects.
///
/// # Cell Naming
///
/// Use hierarchical names for logical grouping:
/// ```text
/// sensor:room1:temperature
/// sensor:room1:humidity
/// derived:room1:comfort_index
/// ```
///
/// # Propagator Constraints
///
/// Define constraints when the DO initializes:
/// ```ignore
/// // Define bidirectional temperature conversion
/// system.add_linear_chirho(&["celsius"], &[1.8], "fahrenheit_base", 0.0);
/// system.add_adder_chirho("fahrenheit_base", "offset32", "fahrenheit");
/// system.set_exact_chirho("offset32", 32.0);
///
/// // Now setting celsius automatically computes fahrenheit and vice versa
/// ```
///
/// # Persistence Granularity
///
/// - **Cell-level**: Persist each cell independently (simpler, more I/O)
/// - **Batch**: Persist all changed cells together (fewer writes, larger payloads)
/// - **Snapshot**: Periodically serialize entire network (simpler recovery)
///
/// Recommendation: Use batch persistence with a short debounce (10-50ms).
///
/// # Handling Hibernation
///
/// DOs can hibernate when idle. To handle this:
/// 1. Persist all state to storage before hibernation
/// 2. On wake, restore state lazily (load cells on first access)
/// 3. Re-establish propagator connections from stored constraint definitions
///
/// # Cost Optimization
///
/// - Use intervals instead of exact values when precision isn't needed
/// - Batch multiple updates into single requests
/// - Use WebSocket for frequent updates instead of REST
/// - Let the DO hibernate during idle periods
#[doc(hidden)]
pub struct _BestPracticesDocChirho;

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_durable_cell_state_merge_chirho() {
        let mut state_a_chirho =
            DurableCellStateChirho::new_chirho(NumericInfoChirho::interval_chirho(0.0, 100.0), 1);

        let state_b_chirho =
            DurableCellStateChirho::new_chirho(NumericInfoChirho::interval_chirho(25.0, 75.0), 2);

        let changed_chirho = state_a_chirho.merge_chirho(&state_b_chirho);
        assert!(changed_chirho);

        let interval_chirho = state_a_chirho.content_chirho.as_interval_chirho().unwrap();
        assert_eq!(interval_chirho.lo_chirho, 25.0);
        assert_eq!(interval_chirho.hi_chirho, 75.0);
        assert_eq!(state_a_chirho.timestamp_chirho, 2);
    }

    #[test]
    fn test_cell_key_chirho() {
        assert_eq!(cell_key_chirho("temperature"), "cell:temperature");
        assert_eq!(
            cell_key_chirho("sensor:room1:temp"),
            "cell:sensor:room1:temp"
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_ws_message_serialization_chirho() {
        let msg_chirho = WsMessageChirho::SetExactChirho {
            cell_chirho: "temperature".to_string(),
            value_chirho: 25.0,
        };

        let json_chirho = serde_json::to_string(&msg_chirho).unwrap();
        assert!(json_chirho.contains("SetExactChirho"));
        assert!(json_chirho.contains("temperature"));
    }
}
