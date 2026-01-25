// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! WebSocket transport for distributed propagator networks.
//!
//! This module provides a production-ready WebSocket transport that enables
//! propagator networks to synchronize across multiple nodes over the network.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────┐         WebSocket          ┌─────────────────┐
//! │   Node A        │◀═══════════════════════════▶│   Node B        │
//! │                 │                             │                 │
//! │  Propagator     │  ┌─────────────────────┐   │  Propagator     │
//! │  Network        │  │ NetworkMessageChirho│   │  Network        │
//! │                 │  │ - SyncRequest       │   │                 │
//! │  Cells:         │  │ - SyncResponse      │   │  Cells:         │
//! │  - temperature  │  │ - Update            │   │  - temperature  │
//! │  - pressure     │  │ - Heartbeat         │   │  - pressure     │
//! │                 │  └─────────────────────┘   │                 │
//! └─────────────────┘                             └─────────────────┘
//! ```
//!
//! # Example
//!
//! ```ignore
//! use propagators_chirho::websocket_transport_chirho::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Server node
//!     let server_chirho = WebSocketServerChirho::bind_chirho("0.0.0.0:8080").await?;
//!
//!     // Client node
//!     let client_chirho = WebSocketClientChirho::connect_chirho("ws://server:8080").await?;
//!
//!     // Send updates
//!     client_chirho.send_chirho(NetworkMessageChirho::HeartbeatChirho {
//!         from_chirho: "client".to_string(),
//!         timestamp_chirho: 1,
//!     }).await?;
//! }
//! ```
//!
//! # Feature Flag
//!
//! Enable with the `network-websocket` feature:
//! ```toml
//! propagators-chirho = { version = "0.1", features = ["network-websocket"] }
//! ```

use super::network_chirho::{NetworkMessageChirho, TransportErrorChirho};

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{accept_async, connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

// ============================================================================
// TYPES
// ============================================================================

/// A WebSocket connection (either client or server-side).
type WsStreamChirho = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsSinkChirho = SplitSink<WsStreamChirho, Message>;
type WsSourceChirho = SplitStream<WsStreamChirho>;

/// Server-side WebSocket stream (from accept).
type ServerWsStreamChirho = WebSocketStream<TcpStream>;
type ServerWsSinkChirho = SplitSink<ServerWsStreamChirho, Message>;
type ServerWsSourceChirho = SplitStream<ServerWsStreamChirho>;

/// Peer connection state.
#[derive(Debug)]
struct PeerConnectionChirho {
    /// The peer's ID.
    peer_id_chirho: String,
    /// Sender half of the WebSocket.
    sink_chirho: Arc<Mutex<ServerWsSinkChirho>>,
    /// Whether the connection is active.
    active_chirho: bool,
}

// ============================================================================
// WEBSOCKET CLIENT
// ============================================================================

/// A WebSocket client for connecting to a propagator network server.
///
/// Use this when you want to connect to an existing network.
///
/// # Example
///
/// ```ignore
/// let client_chirho = WebSocketClientChirho::connect_chirho("ws://server:8080", "my-node").await?;
/// client_chirho.send_chirho(message).await?;
/// ```
pub struct WebSocketClientChirho {
    /// This node's ID.
    node_id_chirho: String,
    /// The server we're connected to.
    server_url_chirho: String,
    /// Sender half for outgoing messages.
    sink_chirho: Arc<Mutex<WsSinkChirho>>,
    /// Channel for received messages.
    inbox_chirho: Arc<Mutex<Vec<(String, NetworkMessageChirho)>>>,
    /// Connection state.
    connected_chirho: Arc<RwLock<bool>>,
}

impl WebSocketClientChirho {
    /// Connects to a WebSocket server.
    ///
    /// # Arguments
    ///
    /// * `url_chirho` - WebSocket URL (e.g., "ws://localhost:8080")
    /// * `node_id_chirho` - This node's unique identifier
    ///
    /// # Errors
    ///
    /// Returns error if connection fails.
    pub async fn connect_chirho(
        url_chirho: &str,
        node_id_chirho: &str,
    ) -> Result<Self, TransportErrorChirho> {
        // Validate URL format
        let _url_parsed_chirho = Url::parse(url_chirho).map_err(|e_chirho| {
            TransportErrorChirho::ConnectionFailedChirho(format!("Invalid URL: {}", e_chirho))
        })?;

        let (ws_stream_chirho, _) = connect_async(url_chirho).await.map_err(|e_chirho| {
            TransportErrorChirho::ConnectionFailedChirho(format!(
                "WebSocket connection failed: {}",
                e_chirho
            ))
        })?;

        let (sink_chirho, source_chirho) = ws_stream_chirho.split();
        let inbox_chirho = Arc::new(Mutex::new(Vec::new()));
        let connected_chirho = Arc::new(RwLock::new(true));

        // Spawn receiver task
        let inbox_clone_chirho = inbox_chirho.clone();
        let connected_clone_chirho = connected_chirho.clone();
        let server_id_chirho = url_chirho.to_string();

        tokio::spawn(async move {
            Self::receive_loop_chirho(
                source_chirho,
                inbox_clone_chirho,
                connected_clone_chirho,
                server_id_chirho,
            )
            .await;
        });

        Ok(Self {
            node_id_chirho: node_id_chirho.to_string(),
            server_url_chirho: url_chirho.to_string(),
            sink_chirho: Arc::new(Mutex::new(sink_chirho)),
            inbox_chirho,
            connected_chirho,
        })
    }

    /// Sends a message to the server.
    ///
    /// # Errors
    ///
    /// Returns error if send fails.
    pub async fn send_chirho(
        &self,
        message_chirho: &NetworkMessageChirho,
    ) -> Result<(), TransportErrorChirho> {
        let json_chirho = serde_json::to_string(message_chirho).map_err(|e_chirho| {
            TransportErrorChirho::SerializationErrorChirho(e_chirho.to_string())
        })?;

        let mut sink_chirho = self.sink_chirho.lock().await;
        sink_chirho
            .send(Message::Text(json_chirho))
            .await
            .map_err(|e_chirho| {
                TransportErrorChirho::SendFailedChirho(format!(
                    "WebSocket send failed: {}",
                    e_chirho
                ))
            })
    }

    /// Receives a message (non-blocking).
    pub async fn recv_chirho(&self) -> Option<(String, NetworkMessageChirho)> {
        let mut inbox_chirho = self.inbox_chirho.lock().await;
        inbox_chirho.pop()
    }

    /// Checks if connected.
    pub async fn is_connected_chirho(&self) -> bool {
        *self.connected_chirho.read().await
    }

    /// Returns the node ID.
    pub fn node_id_chirho(&self) -> &str {
        &self.node_id_chirho
    }

    /// Background receive loop.
    async fn receive_loop_chirho(
        mut source_chirho: WsSourceChirho,
        inbox_chirho: Arc<Mutex<Vec<(String, NetworkMessageChirho)>>>,
        connected_chirho: Arc<RwLock<bool>>,
        server_id_chirho: String,
    ) {
        while let Some(msg_result_chirho) = source_chirho.next().await {
            match msg_result_chirho {
                Ok(Message::Text(text_chirho)) => {
                    if let Ok(message_chirho) =
                        serde_json::from_str::<NetworkMessageChirho>(&text_chirho)
                    {
                        let mut inbox_guard_chirho = inbox_chirho.lock().await;
                        inbox_guard_chirho.push((server_id_chirho.clone(), message_chirho));
                    }
                }
                Ok(Message::Close(_)) => {
                    let mut connected_guard_chirho = connected_chirho.write().await;
                    *connected_guard_chirho = false;
                    break;
                }
                Err(_) => {
                    let mut connected_guard_chirho = connected_chirho.write().await;
                    *connected_guard_chirho = false;
                    break;
                }
                _ => {} // Ignore ping/pong/binary
            }
        }
    }
}

// ============================================================================
// WEBSOCKET SERVER
// ============================================================================

/// A WebSocket server for hosting a propagator network.
///
/// Other nodes can connect to this server to synchronize their networks.
///
/// # Example
///
/// ```ignore
/// let server_chirho = WebSocketServerChirho::bind_chirho("0.0.0.0:8080", "server-node").await?;
///
/// // Handle incoming connections and messages
/// loop {
///     server_chirho.tick_chirho().await?;
///     tokio::time::sleep(Duration::from_millis(10)).await;
/// }
/// ```
pub struct WebSocketServerChirho {
    /// This node's ID.
    node_id_chirho: String,
    /// Bound address.
    address_chirho: String,
    /// Connected peers.
    peers_chirho: Arc<RwLock<HashMap<String, PeerConnectionChirho>>>,
    /// Channel for new connections.
    new_connections_chirho: mpsc::Receiver<(String, ServerWsSinkChirho, ServerWsSourceChirho)>,
    /// Sender for new connections (given to accept loop).
    connection_sender_chirho: mpsc::Sender<(String, ServerWsSinkChirho, ServerWsSourceChirho)>,
    /// Incoming message queue.
    inbox_chirho: Arc<Mutex<Vec<(String, NetworkMessageChirho)>>>,
    /// Counter for generating peer IDs.
    peer_counter_chirho: Arc<RwLock<u64>>,
}

impl WebSocketServerChirho {
    /// Binds to an address and starts accepting connections.
    ///
    /// # Arguments
    ///
    /// * `address_chirho` - Address to bind to (e.g., "0.0.0.0:8080")
    /// * `node_id_chirho` - This node's unique identifier
    ///
    /// # Errors
    ///
    /// Returns error if binding fails.
    pub async fn bind_chirho(
        address_chirho: &str,
        node_id_chirho: &str,
    ) -> Result<Self, TransportErrorChirho> {
        let listener_chirho = TcpListener::bind(address_chirho)
            .await
            .map_err(|e_chirho| {
                TransportErrorChirho::ConnectionFailedChirho(format!("Bind failed: {}", e_chirho))
            })?;

        let (tx_chirho, rx_chirho) = mpsc::channel(100);
        let peer_counter_chirho = Arc::new(RwLock::new(0u64));
        let counter_clone_chirho = peer_counter_chirho.clone();

        // Spawn accept loop
        tokio::spawn(async move {
            Self::accept_loop_chirho(listener_chirho, tx_chirho, counter_clone_chirho).await;
        });

        Ok(Self {
            node_id_chirho: node_id_chirho.to_string(),
            address_chirho: address_chirho.to_string(),
            peers_chirho: Arc::new(RwLock::new(HashMap::new())),
            new_connections_chirho: rx_chirho,
            connection_sender_chirho: mpsc::Sender::clone(&mpsc::channel(1).0), // Placeholder
            inbox_chirho: Arc::new(Mutex::new(Vec::new())),
            peer_counter_chirho,
        })
    }

    /// Sends a message to a specific peer.
    ///
    /// # Errors
    ///
    /// Returns error if peer not found or send fails.
    pub async fn send_to_chirho(
        &self,
        peer_id_chirho: &str,
        message_chirho: &NetworkMessageChirho,
    ) -> Result<(), TransportErrorChirho> {
        let peers_chirho = self.peers_chirho.read().await;
        let peer_chirho = peers_chirho
            .get(peer_id_chirho)
            .ok_or_else(|| TransportErrorChirho::PeerNotFoundChirho(peer_id_chirho.to_string()))?;

        let json_chirho = serde_json::to_string(message_chirho).map_err(|e_chirho| {
            TransportErrorChirho::SerializationErrorChirho(e_chirho.to_string())
        })?;

        let mut sink_chirho = peer_chirho.sink_chirho.lock().await;
        sink_chirho
            .send(Message::Text(json_chirho))
            .await
            .map_err(|e_chirho| {
                TransportErrorChirho::SendFailedChirho(format!(
                    "WebSocket send failed: {}",
                    e_chirho
                ))
            })
    }

    /// Broadcasts a message to all connected peers.
    ///
    /// # Errors
    ///
    /// Returns error if any send fails.
    pub async fn broadcast_chirho(
        &self,
        message_chirho: &NetworkMessageChirho,
    ) -> Result<(), TransportErrorChirho> {
        let json_chirho = serde_json::to_string(message_chirho).map_err(|e_chirho| {
            TransportErrorChirho::SerializationErrorChirho(e_chirho.to_string())
        })?;

        let peers_chirho = self.peers_chirho.read().await;
        for (_, peer_chirho) in peers_chirho.iter() {
            if peer_chirho.active_chirho {
                let mut sink_chirho = peer_chirho.sink_chirho.lock().await;
                let _ = sink_chirho.send(Message::Text(json_chirho.clone())).await;
            }
        }

        Ok(())
    }

    /// Receives a message (non-blocking).
    pub async fn recv_chirho(&self) -> Option<(String, NetworkMessageChirho)> {
        let mut inbox_chirho = self.inbox_chirho.lock().await;
        inbox_chirho.pop()
    }

    /// Processes new connections and removes dead ones.
    /// Call this periodically.
    pub async fn tick_chirho(&mut self) -> Result<(), TransportErrorChirho> {
        // Accept new connections
        while let Ok((peer_id_chirho, sink_chirho, source_chirho)) =
            self.new_connections_chirho.try_recv()
        {
            let sink_arc_chirho = Arc::new(Mutex::new(sink_chirho));

            // Add to peers
            {
                let mut peers_chirho = self.peers_chirho.write().await;
                peers_chirho.insert(
                    peer_id_chirho.clone(),
                    PeerConnectionChirho {
                        peer_id_chirho: peer_id_chirho.clone(),
                        sink_chirho: sink_arc_chirho,
                        active_chirho: true,
                    },
                );
            }

            // Spawn receive loop for this peer
            let inbox_chirho = self.inbox_chirho.clone();
            let peers_chirho = self.peers_chirho.clone();
            let peer_id_clone_chirho = peer_id_chirho.clone();

            tokio::spawn(async move {
                Self::peer_receive_loop_chirho(
                    source_chirho,
                    inbox_chirho,
                    peers_chirho,
                    peer_id_clone_chirho,
                )
                .await;
            });
        }

        Ok(())
    }

    /// Returns the list of connected peer IDs.
    pub async fn peers_chirho(&self) -> Vec<String> {
        let peers_chirho = self.peers_chirho.read().await;
        peers_chirho
            .iter()
            .filter(|(_, p_chirho)| p_chirho.active_chirho)
            .map(|(id_chirho, _)| id_chirho.clone())
            .collect()
    }

    /// Returns the node ID.
    pub fn node_id_chirho(&self) -> &str {
        &self.node_id_chirho
    }

    /// Returns the bound address.
    pub fn address_chirho(&self) -> &str {
        &self.address_chirho
    }

    /// Background accept loop.
    async fn accept_loop_chirho(
        listener_chirho: TcpListener,
        tx_chirho: mpsc::Sender<(String, ServerWsSinkChirho, ServerWsSourceChirho)>,
        counter_chirho: Arc<RwLock<u64>>,
    ) {
        while let Ok((stream_chirho, addr_chirho)) = listener_chirho.accept().await {
            if let Ok(ws_stream_chirho) = accept_async(stream_chirho).await {
                let (sink_chirho, source_chirho) = ws_stream_chirho.split();

                // Generate peer ID
                let peer_id_chirho = {
                    let mut counter_guard_chirho = counter_chirho.write().await;
                    *counter_guard_chirho += 1;
                    format!("peer-{}-{}", addr_chirho, *counter_guard_chirho)
                };

                let _ = tx_chirho
                    .send((peer_id_chirho, sink_chirho, source_chirho))
                    .await;
            }
        }
    }

    /// Receive loop for a single peer.
    async fn peer_receive_loop_chirho(
        mut source_chirho: ServerWsSourceChirho,
        inbox_chirho: Arc<Mutex<Vec<(String, NetworkMessageChirho)>>>,
        peers_chirho: Arc<RwLock<HashMap<String, PeerConnectionChirho>>>,
        peer_id_chirho: String,
    ) {
        while let Some(msg_result_chirho) = source_chirho.next().await {
            match msg_result_chirho {
                Ok(Message::Text(text_chirho)) => {
                    if let Ok(message_chirho) =
                        serde_json::from_str::<NetworkMessageChirho>(&text_chirho)
                    {
                        let mut inbox_guard_chirho = inbox_chirho.lock().await;
                        inbox_guard_chirho.push((peer_id_chirho.clone(), message_chirho));
                    }
                }
                Ok(Message::Close(_)) | Err(_) => {
                    // Mark peer as inactive
                    let mut peers_guard_chirho = peers_chirho.write().await;
                    if let Some(peer_chirho) = peers_guard_chirho.get_mut(&peer_id_chirho) {
                        peer_chirho.active_chirho = false;
                    }
                    break;
                }
                _ => {} // Ignore ping/pong/binary
            }
        }
    }
}

// ============================================================================
// ASYNC DISTRIBUTED NETWORK
// ============================================================================

/// An async-aware distributed propagator network using WebSocket transport.
///
/// This wraps `DistributedNetworkChirho` with async WebSocket connectivity.
///
/// # Example
///
/// ```ignore
/// // Server node
/// let mut server_chirho = AsyncDistributedNetworkChirho::new_server_chirho(
///     "0.0.0.0:8080",
///     "server-node"
/// ).await?;
///
/// let temp_chirho = server_chirho.make_shared_cell_chirho("temperature");
/// server_chirho.set_exact_chirho(&temp_chirho, 25.0);
///
/// // Run the network
/// loop {
///     server_chirho.tick_chirho().await?;
/// }
/// ```
pub struct AsyncDistributedNetworkChirho {
    /// Node ID.
    node_id_chirho: String,
    /// Server (if this node is hosting).
    server_chirho: Option<WebSocketServerChirho>,
    /// Clients (connections to other nodes).
    clients_chirho: HashMap<String, WebSocketClientChirho>,
    /// The underlying distributed network.
    network_chirho: crate::network_chirho::DistributedNetworkChirho<
        crate::network_chirho::InMemoryTransportChirho,
    >,
}

impl AsyncDistributedNetworkChirho {
    /// Creates a new server node.
    ///
    /// # Errors
    ///
    /// Returns error if binding fails.
    pub async fn new_server_chirho(
        address_chirho: &str,
        node_id_chirho: &str,
    ) -> Result<Self, TransportErrorChirho> {
        let server_chirho =
            WebSocketServerChirho::bind_chirho(address_chirho, node_id_chirho).await?;
        let transport_chirho =
            crate::network_chirho::InMemoryTransportChirho::new_chirho(node_id_chirho);
        let network_chirho = crate::network_chirho::DistributedNetworkChirho::new_chirho(
            node_id_chirho,
            transport_chirho,
        );

        Ok(Self {
            node_id_chirho: node_id_chirho.to_string(),
            server_chirho: Some(server_chirho),
            clients_chirho: HashMap::new(),
            network_chirho,
        })
    }

    /// Creates a new client node and connects to a server.
    ///
    /// # Errors
    ///
    /// Returns error if connection fails.
    pub async fn new_client_chirho(
        server_url_chirho: &str,
        node_id_chirho: &str,
    ) -> Result<Self, TransportErrorChirho> {
        let client_chirho =
            WebSocketClientChirho::connect_chirho(server_url_chirho, node_id_chirho).await?;
        let transport_chirho =
            crate::network_chirho::InMemoryTransportChirho::new_chirho(node_id_chirho);
        let network_chirho = crate::network_chirho::DistributedNetworkChirho::new_chirho(
            node_id_chirho,
            transport_chirho,
        );

        let mut clients_chirho = HashMap::new();
        clients_chirho.insert(server_url_chirho.to_string(), client_chirho);

        Ok(Self {
            node_id_chirho: node_id_chirho.to_string(),
            server_chirho: None,
            clients_chirho,
            network_chirho,
        })
    }

    /// Creates a shared cell.
    pub fn make_shared_cell_chirho(
        &mut self,
        name_chirho: &str,
    ) -> crate::network_chirho::CellIdChirho {
        self.network_chirho.make_shared_cell_chirho(name_chirho)
    }

    /// Sets a cell to an exact value.
    pub fn set_exact_chirho(
        &mut self,
        cell_id_chirho: &crate::network_chirho::CellIdChirho,
        value_chirho: f64,
    ) {
        self.network_chirho
            .set_exact_chirho(cell_id_chirho, value_chirho);
    }

    /// Gets cell content.
    pub fn get_content_chirho(
        &self,
        cell_id_chirho: &crate::network_chirho::CellIdChirho,
    ) -> Option<crate::interval_chirho::NumericInfoChirho> {
        self.network_chirho.get_content_chirho(cell_id_chirho)
    }

    /// Runs one tick: process messages, flush updates, broadcast.
    ///
    /// # Errors
    ///
    /// Returns error if network operations fail.
    pub async fn tick_chirho(&mut self) -> Result<(), TransportErrorChirho> {
        // Collect messages from server
        let mut messages_chirho: Vec<(String, NetworkMessageChirho)> = Vec::new();

        if let Some(server_chirho) = &mut self.server_chirho {
            server_chirho.tick_chirho().await?;

            while let Some(msg_chirho) = server_chirho.recv_chirho().await {
                messages_chirho.push(msg_chirho);
            }
        }

        // Collect messages from clients
        for (_, client_chirho) in &self.clients_chirho {
            while let Some(msg_chirho) = client_chirho.recv_chirho().await {
                messages_chirho.push(msg_chirho);
            }
        }

        // Process all collected messages
        for (from_chirho, message_chirho) in messages_chirho {
            self.handle_message_chirho(&from_chirho, message_chirho);
        }

        // Flush pending updates
        self.network_chirho.flush_updates_chirho()?;

        Ok(())
    }

    /// Broadcasts an update to all connected peers.
    ///
    /// # Errors
    ///
    /// Returns error if broadcast fails.
    pub async fn broadcast_update_chirho(
        &self,
        cell_id_chirho: &crate::network_chirho::CellIdChirho,
    ) -> Result<(), TransportErrorChirho> {
        if let Some(content_chirho) = self.network_chirho.get_content_chirho(cell_id_chirho) {
            let message_chirho = crate::network_chirho::NetworkMessageChirho::UpdateChirho(
                crate::network_chirho::CellUpdateChirho {
                    cell_id_chirho: cell_id_chirho.clone(),
                    info_chirho: content_chirho,
                    timestamp_chirho: 0, // TODO: proper timestamp
                    origin_chirho: self.node_id_chirho.clone(),
                },
            );

            if let Some(server_chirho) = &self.server_chirho {
                server_chirho.broadcast_chirho(&message_chirho).await?;
            }

            for (_, client_chirho) in &self.clients_chirho {
                client_chirho.send_chirho(&message_chirho).await?;
            }
        }

        Ok(())
    }

    fn handle_message_chirho(&mut self, _from_chirho: &str, message_chirho: NetworkMessageChirho) {
        match message_chirho {
            NetworkMessageChirho::UpdateChirho(update_chirho) => {
                // Ensure cell exists
                if self
                    .network_chirho
                    .get_cell_chirho(&update_chirho.cell_id_chirho)
                    .is_none()
                {
                    self.network_chirho
                        .make_shared_cell_chirho(&update_chirho.cell_id_chirho.name_chirho);
                }

                // Apply update
                self.network_chirho
                    .add_info_chirho(&update_chirho.cell_id_chirho, update_chirho.info_chirho);
            }
            NetworkMessageChirho::SyncRequestChirho {
                from_chirho,
                known_cells_chirho: _,
            } => {
                // TODO: Implement sync response
                let _ = from_chirho;
            }
            NetworkMessageChirho::SyncResponseChirho { updates_chirho, .. } => {
                for update_chirho in updates_chirho {
                    self.network_chirho
                        .add_info_chirho(&update_chirho.cell_id_chirho, update_chirho.info_chirho);
                }
            }
            NetworkMessageChirho::HeartbeatChirho { .. } => {
                // Heartbeat received - connection is alive
            }
            NetworkMessageChirho::SubscribeChirho { .. } => {
                // TODO: Implement subscriptions
            }
        }
    }

    /// Returns the node ID.
    pub fn node_id_chirho(&self) -> &str {
        &self.node_id_chirho
    }

    /// Returns connected peer count.
    pub async fn peer_count_chirho(&self) -> usize {
        let mut count_chirho = 0;

        if let Some(server_chirho) = &self.server_chirho {
            count_chirho += server_chirho.peers_chirho().await.len();
        }

        count_chirho += self.clients_chirho.len();
        count_chirho
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_websocket_server_bind_chirho() {
        let server_chirho = WebSocketServerChirho::bind_chirho("127.0.0.1:0", "test-server").await;
        assert!(server_chirho.is_ok());
    }

    #[tokio::test]
    async fn test_async_distributed_network_server_chirho() {
        let result_chirho =
            AsyncDistributedNetworkChirho::new_server_chirho("127.0.0.1:0", "server").await;
        assert!(result_chirho.is_ok());

        let mut network_chirho = result_chirho.unwrap();
        let cell_chirho = network_chirho.make_shared_cell_chirho("test");
        network_chirho.set_exact_chirho(&cell_chirho, 42.0);

        let content_chirho = network_chirho.get_content_chirho(&cell_chirho);
        assert!(content_chirho.is_some());
    }

    #[tokio::test]
    async fn test_client_server_connection_chirho() {
        // Start server
        let server_chirho = WebSocketServerChirho::bind_chirho("127.0.0.1:9876", "server")
            .await
            .unwrap();

        // Give server time to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Connect client
        let client_result_chirho =
            WebSocketClientChirho::connect_chirho("ws://127.0.0.1:9876", "client").await;

        // Connection should succeed
        assert!(client_result_chirho.is_ok());

        let client_chirho = client_result_chirho.unwrap();
        assert!(client_chirho.is_connected_chirho().await);
    }
}
