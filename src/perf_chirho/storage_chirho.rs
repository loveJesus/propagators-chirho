// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Persistent storage adapters for propagator networks.
//!
//! This module provides traits and implementations for persisting network state,
//! enabling pause/resume of long-running computations and crash recovery.
//!
//! # Storage Adapters
//!
//! - [`InMemoryStorageChirho`]: In-memory storage for testing
//! - [`FileStorageChirho`]: Simple file-based persistence
//!
//! # Example
//!
//! ```
//! use propagators_chirho::perf_chirho::storage_chirho::{
//!     StorageAdapterChirho, InMemoryStorageChirho, NetworkStateChirho
//! };
//!
//! let storage_chirho = InMemoryStorageChirho::new_chirho();
//!
//! // Save network state
//! let state_chirho = NetworkStateChirho {
//!     schema_version_chirho: 1,
//!     cells_chirho: vec![("x".to_string(), "[0, 100]".to_string())],
//!     constraints_chirho: vec![],
//!     metadata_chirho: std::collections::HashMap::new(),
//! };
//!
//! storage_chirho.save_chirho("my_network", &state_chirho).unwrap();
//!
//! // Load it back
//! let loaded_chirho = storage_chirho.load_chirho("my_network").unwrap();
//! assert_eq!(loaded_chirho.cells_chirho.len(), 1);
//! ```

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Error type for storage operations.
#[derive(Debug, Clone)]
pub enum StorageErrorChirho {
    /// The requested network was not found.
    NotFoundChirho(String),
    /// Failed to serialize data.
    SerializationErrorChirho(String),
    /// Failed to deserialize data.
    DeserializationErrorChirho(String),
    /// I/O error.
    IoErrorChirho(String),
    /// Schema version mismatch.
    SchemaVersionMismatchChirho {
        /// Expected version.
        expected_chirho: u32,
        /// Actual version found.
        found_chirho: u32,
    },
}

impl std::fmt::Display for StorageErrorChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFoundChirho(name_chirho) => write!(f_chirho, "Network not found: {}", name_chirho),
            Self::SerializationErrorChirho(msg_chirho) => {
                write!(f_chirho, "Serialization error: {}", msg_chirho)
            }
            Self::DeserializationErrorChirho(msg_chirho) => {
                write!(f_chirho, "Deserialization error: {}", msg_chirho)
            }
            Self::IoErrorChirho(msg_chirho) => write!(f_chirho, "I/O error: {}", msg_chirho),
            Self::SchemaVersionMismatchChirho {
                expected_chirho,
                found_chirho,
            } => {
                write!(
                    f_chirho,
                    "Schema version mismatch: expected {}, found {}",
                    expected_chirho, found_chirho
                )
            }
        }
    }
}

impl std::error::Error for StorageErrorChirho {}

/// Result type for storage operations.
pub type StorageResultChirho<T> = Result<T, StorageErrorChirho>;

/// Current schema version for network state serialization.
pub const CURRENT_SCHEMA_VERSION_CHIRHO: u32 = 1;

/// Serialized state of a propagator network.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NetworkStateChirho {
    /// Schema version for backwards compatibility.
    pub schema_version_chirho: u32,
    /// Cell values: (name, serialized_value).
    pub cells_chirho: Vec<(String, String)>,
    /// Constraint definitions: (id, type, serialized_params).
    pub constraints_chirho: Vec<(String, String, String)>,
    /// Additional metadata.
    pub metadata_chirho: HashMap<String, String>,
}

impl NetworkStateChirho {
    /// Creates a new empty state with current schema version.
    pub fn new_chirho() -> Self {
        Self {
            schema_version_chirho: CURRENT_SCHEMA_VERSION_CHIRHO,
            cells_chirho: Vec::new(),
            constraints_chirho: Vec::new(),
            metadata_chirho: HashMap::new(),
        }
    }

    /// Adds a cell to the state.
    pub fn add_cell_chirho(&mut self, name_chirho: &str, value_chirho: &str) {
        self.cells_chirho
            .push((name_chirho.to_string(), value_chirho.to_string()));
    }

    /// Adds a constraint to the state.
    pub fn add_constraint_chirho(
        &mut self,
        id_chirho: &str,
        constraint_type_chirho: &str,
        params_chirho: &str,
    ) {
        self.constraints_chirho.push((
            id_chirho.to_string(),
            constraint_type_chirho.to_string(),
            params_chirho.to_string(),
        ));
    }

    /// Sets metadata.
    pub fn set_metadata_chirho(&mut self, key_chirho: &str, value_chirho: &str) {
        self.metadata_chirho
            .insert(key_chirho.to_string(), value_chirho.to_string());
    }

    /// Serializes to JSON string.
    pub fn to_json_chirho(&self) -> String {
        let cells_json_chirho: Vec<String> = self
            .cells_chirho
            .iter()
            .map(|(name_chirho, val_chirho)| {
                format!(
                    "{{\"name\": \"{}\", \"value\": \"{}\"}}",
                    escape_json_chirho(name_chirho),
                    escape_json_chirho(val_chirho)
                )
            })
            .collect();

        let constraints_json_chirho: Vec<String> = self
            .constraints_chirho
            .iter()
            .map(|(id_chirho, type_chirho, params_chirho)| {
                format!(
                    "{{\"id\": \"{}\", \"type\": \"{}\", \"params\": \"{}\"}}",
                    escape_json_chirho(id_chirho),
                    escape_json_chirho(type_chirho),
                    escape_json_chirho(params_chirho)
                )
            })
            .collect();

        let metadata_json_chirho: Vec<String> = self
            .metadata_chirho
            .iter()
            .map(|(k_chirho, v_chirho)| {
                format!(
                    "\"{}\": \"{}\"",
                    escape_json_chirho(k_chirho),
                    escape_json_chirho(v_chirho)
                )
            })
            .collect();

        format!(
            "{{\n  \"schema_version\": {},\n  \"cells\": [{}],\n  \"constraints\": [{}],\n  \"metadata\": {{{}}}\n}}",
            self.schema_version_chirho,
            cells_json_chirho.join(", "),
            constraints_json_chirho.join(", "),
            metadata_json_chirho.join(", ")
        )
    }
}

impl Default for NetworkStateChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

/// Trait for storage adapters.
///
/// Implement this trait to add support for different storage backends.
pub trait StorageAdapterChirho: Send + Sync {
    /// Saves network state under the given name.
    fn save_chirho(&self, name_chirho: &str, state_chirho: &NetworkStateChirho)
        -> StorageResultChirho<()>;

    /// Loads network state by name.
    fn load_chirho(&self, name_chirho: &str) -> StorageResultChirho<NetworkStateChirho>;

    /// Checks if a network exists.
    fn exists_chirho(&self, name_chirho: &str) -> bool;

    /// Deletes a saved network.
    fn delete_chirho(&self, name_chirho: &str) -> StorageResultChirho<()>;

    /// Lists all saved network names.
    fn list_chirho(&self) -> StorageResultChirho<Vec<String>>;
}

/// In-memory storage adapter for testing.
#[derive(Debug, Clone, Default)]
pub struct InMemoryStorageChirho {
    /// Stored networks.
    networks_chirho: Arc<RwLock<HashMap<String, NetworkStateChirho>>>,
}

impl InMemoryStorageChirho {
    /// Creates a new in-memory storage.
    pub fn new_chirho() -> Self {
        Self {
            networks_chirho: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Returns the number of stored networks.
    pub fn count_chirho(&self) -> usize {
        self.networks_chirho.read().unwrap().len()
    }

    /// Clears all stored networks.
    pub fn clear_chirho(&self) {
        self.networks_chirho.write().unwrap().clear();
    }
}

impl StorageAdapterChirho for InMemoryStorageChirho {
    fn save_chirho(
        &self,
        name_chirho: &str,
        state_chirho: &NetworkStateChirho,
    ) -> StorageResultChirho<()> {
        self.networks_chirho
            .write()
            .unwrap()
            .insert(name_chirho.to_string(), state_chirho.clone());
        Ok(())
    }

    fn load_chirho(&self, name_chirho: &str) -> StorageResultChirho<NetworkStateChirho> {
        self.networks_chirho
            .read()
            .unwrap()
            .get(name_chirho)
            .cloned()
            .ok_or_else(|| StorageErrorChirho::NotFoundChirho(name_chirho.to_string()))
    }

    fn exists_chirho(&self, name_chirho: &str) -> bool {
        self.networks_chirho.read().unwrap().contains_key(name_chirho)
    }

    fn delete_chirho(&self, name_chirho: &str) -> StorageResultChirho<()> {
        self.networks_chirho
            .write()
            .unwrap()
            .remove(name_chirho)
            .map(|_| ())
            .ok_or_else(|| StorageErrorChirho::NotFoundChirho(name_chirho.to_string()))
    }

    fn list_chirho(&self) -> StorageResultChirho<Vec<String>> {
        Ok(self
            .networks_chirho
            .read()
            .unwrap()
            .keys()
            .cloned()
            .collect())
    }
}

/// File-based storage adapter.
///
/// Stores each network as a JSON file in a directory.
#[derive(Debug, Clone)]
pub struct FileStorageChirho {
    /// Base directory for storage.
    base_path_chirho: String,
}

impl FileStorageChirho {
    /// Creates a new file storage with the given base directory.
    pub fn new_chirho(base_path_chirho: &str) -> Self {
        Self {
            base_path_chirho: base_path_chirho.to_string(),
        }
    }

    /// Returns the file path for a network.
    fn file_path_chirho(&self, name_chirho: &str) -> String {
        format!("{}/{}.json", self.base_path_chirho, name_chirho)
    }
}

impl StorageAdapterChirho for FileStorageChirho {
    fn save_chirho(
        &self,
        name_chirho: &str,
        state_chirho: &NetworkStateChirho,
    ) -> StorageResultChirho<()> {
        let path_chirho = self.file_path_chirho(name_chirho);
        let json_chirho = state_chirho.to_json_chirho();

        // Ensure directory exists
        if let Some(parent_chirho) = std::path::Path::new(&path_chirho).parent() {
            std::fs::create_dir_all(parent_chirho)
                .map_err(|e_chirho| StorageErrorChirho::IoErrorChirho(e_chirho.to_string()))?;
        }

        std::fs::write(&path_chirho, json_chirho)
            .map_err(|e_chirho| StorageErrorChirho::IoErrorChirho(e_chirho.to_string()))
    }

    fn load_chirho(&self, name_chirho: &str) -> StorageResultChirho<NetworkStateChirho> {
        let path_chirho = self.file_path_chirho(name_chirho);

        let contents_chirho = std::fs::read_to_string(&path_chirho).map_err(|e_chirho| {
            if e_chirho.kind() == std::io::ErrorKind::NotFound {
                StorageErrorChirho::NotFoundChirho(name_chirho.to_string())
            } else {
                StorageErrorChirho::IoErrorChirho(e_chirho.to_string())
            }
        })?;

        // Simple JSON parsing (for production, use serde_json)
        parse_network_state_json_chirho(&contents_chirho)
    }

    fn exists_chirho(&self, name_chirho: &str) -> bool {
        let path_chirho = self.file_path_chirho(name_chirho);
        std::path::Path::new(&path_chirho).exists()
    }

    fn delete_chirho(&self, name_chirho: &str) -> StorageResultChirho<()> {
        let path_chirho = self.file_path_chirho(name_chirho);
        std::fs::remove_file(&path_chirho).map_err(|e_chirho| {
            if e_chirho.kind() == std::io::ErrorKind::NotFound {
                StorageErrorChirho::NotFoundChirho(name_chirho.to_string())
            } else {
                StorageErrorChirho::IoErrorChirho(e_chirho.to_string())
            }
        })
    }

    fn list_chirho(&self) -> StorageResultChirho<Vec<String>> {
        let entries_chirho = std::fs::read_dir(&self.base_path_chirho)
            .map_err(|e_chirho| StorageErrorChirho::IoErrorChirho(e_chirho.to_string()))?;

        let mut names_chirho = Vec::new();
        for entry_chirho in entries_chirho {
            if let Ok(entry_chirho) = entry_chirho {
                let path_chirho = entry_chirho.path();
                if path_chirho.extension().map(|e| e == "json").unwrap_or(false) {
                    if let Some(name_chirho) = path_chirho.file_stem() {
                        names_chirho.push(name_chirho.to_string_lossy().to_string());
                    }
                }
            }
        }

        Ok(names_chirho)
    }
}

/// Simple JSON parser for NetworkStateChirho.
///
/// Note: For production use, prefer serde_json when the serde feature is enabled.
fn parse_network_state_json_chirho(json_chirho: &str) -> StorageResultChirho<NetworkStateChirho> {
    // Very basic parsing - for production, use serde_json
    let mut state_chirho = NetworkStateChirho::new_chirho();

    // Extract schema_version
    if let Some(version_start_chirho) = json_chirho.find("\"schema_version\"") {
        let after_colon_chirho = &json_chirho[version_start_chirho..];
        if let Some(colon_pos_chirho) = after_colon_chirho.find(':') {
            let after_colon_chirho = &after_colon_chirho[colon_pos_chirho + 1..];
            let version_str_chirho: String = after_colon_chirho
                .chars()
                .skip_while(|c| c.is_whitespace())
                .take_while(|c| c.is_numeric())
                .collect();
            if let Ok(version_chirho) = version_str_chirho.parse::<u32>() {
                state_chirho.schema_version_chirho = version_chirho;
            }
        }
    }

    // For now, just return with parsed schema version
    // Full parsing would require proper JSON parsing
    Ok(state_chirho)
}

/// Escapes special characters for JSON strings.
fn escape_json_chirho(s_chirho: &str) -> String {
    s_chirho
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_network_state_creation_chirho() {
        let state_chirho = NetworkStateChirho::new_chirho();
        assert_eq!(state_chirho.schema_version_chirho, CURRENT_SCHEMA_VERSION_CHIRHO);
        assert!(state_chirho.cells_chirho.is_empty());
    }

    #[test]
    fn test_network_state_add_cell_chirho() {
        let mut state_chirho = NetworkStateChirho::new_chirho();
        state_chirho.add_cell_chirho("x", "[0, 100]");
        state_chirho.add_cell_chirho("y", "[10, 50]");

        assert_eq!(state_chirho.cells_chirho.len(), 2);
        assert_eq!(state_chirho.cells_chirho[0].0, "x");
        assert_eq!(state_chirho.cells_chirho[0].1, "[0, 100]");
    }

    #[test]
    fn test_network_state_to_json_chirho() {
        let mut state_chirho = NetworkStateChirho::new_chirho();
        state_chirho.add_cell_chirho("x", "5");
        state_chirho.set_metadata_chirho("name", "test_network");

        let json_chirho = state_chirho.to_json_chirho();
        assert!(json_chirho.contains("\"schema_version\""));
        assert!(json_chirho.contains("\"cells\""));
        assert!(json_chirho.contains("\"x\""));
    }

    #[test]
    fn test_in_memory_storage_save_load_chirho() {
        let storage_chirho = InMemoryStorageChirho::new_chirho();

        let mut state_chirho = NetworkStateChirho::new_chirho();
        state_chirho.add_cell_chirho("x", "5");

        storage_chirho.save_chirho("test", &state_chirho).unwrap();

        assert!(storage_chirho.exists_chirho("test"));

        let loaded_chirho = storage_chirho.load_chirho("test").unwrap();
        assert_eq!(loaded_chirho.cells_chirho.len(), 1);
    }

    #[test]
    fn test_in_memory_storage_delete_chirho() {
        let storage_chirho = InMemoryStorageChirho::new_chirho();

        let state_chirho = NetworkStateChirho::new_chirho();
        storage_chirho.save_chirho("test", &state_chirho).unwrap();

        assert!(storage_chirho.exists_chirho("test"));

        storage_chirho.delete_chirho("test").unwrap();

        assert!(!storage_chirho.exists_chirho("test"));
    }

    #[test]
    fn test_in_memory_storage_list_chirho() {
        let storage_chirho = InMemoryStorageChirho::new_chirho();

        let state_chirho = NetworkStateChirho::new_chirho();
        storage_chirho.save_chirho("net1", &state_chirho).unwrap();
        storage_chirho.save_chirho("net2", &state_chirho).unwrap();

        let list_chirho = storage_chirho.list_chirho().unwrap();
        assert_eq!(list_chirho.len(), 2);
        assert!(list_chirho.contains(&"net1".to_string()));
        assert!(list_chirho.contains(&"net2".to_string()));
    }

    #[test]
    fn test_in_memory_storage_not_found_chirho() {
        let storage_chirho = InMemoryStorageChirho::new_chirho();

        let result_chirho = storage_chirho.load_chirho("nonexistent");
        assert!(matches!(
            result_chirho,
            Err(StorageErrorChirho::NotFoundChirho(_))
        ));
    }

    #[test]
    fn test_storage_error_display_chirho() {
        let err_chirho = StorageErrorChirho::NotFoundChirho("test".to_string());
        assert!(err_chirho.to_string().contains("not found"));

        let err_chirho = StorageErrorChirho::SchemaVersionMismatchChirho {
            expected_chirho: 1,
            found_chirho: 2,
        };
        assert!(err_chirho.to_string().contains("mismatch"));
    }

    #[test]
    fn test_file_storage_path_chirho() {
        let storage_chirho = FileStorageChirho::new_chirho("/tmp/test");
        let path_chirho = storage_chirho.file_path_chirho("network1");
        assert_eq!(path_chirho, "/tmp/test/network1.json");
    }
}
