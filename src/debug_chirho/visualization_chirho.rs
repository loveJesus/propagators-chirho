// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Visualization and debugging tools for propagator networks.
//!
//! This module provides tools for visualizing and debugging constraint networks:
//!
//! - [`PropagationTraceChirho`]: Records propagation events for replay/analysis
//! - [`CellHistoryChirho`]: Tracks value changes for individual cells
//! - [`NetworkSnapshotChirho`]: Captures complete network state
//! - JSON export for web-based visualization
//!
//! # Example
//!
//! ```
//! use propagators_chirho::debug_chirho::visualization_chirho::{
//!     PropagationTraceChirho, TraceEventChirho
//! };
//!
//! let mut trace_chirho = PropagationTraceChirho::new_chirho();
//!
//! // Record events during propagation
//! trace_chirho.record_cell_update_chirho("x", "5.0", "propagator_1");
//! trace_chirho.record_propagator_run_chirho("propagator_1", vec!["x"], vec!["y"]);
//!
//! // Export to JSON for visualization
//! let json_chirho = trace_chirho.to_json_chirho();
//! ```

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A single event in the propagation trace.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TraceEventChirho {
    /// A cell's value was updated.
    CellUpdateChirho {
        /// Name of the cell.
        cell_name_chirho: String,
        /// New value (as string representation).
        new_value_chirho: String,
        /// Propagator that caused the update (if any).
        source_propagator_chirho: Option<String>,
        /// Timestamp relative to trace start.
        timestamp_micros_chirho: u64,
    },
    /// A propagator was executed.
    PropagatorRunChirho {
        /// Name of the propagator.
        propagator_name_chirho: String,
        /// Input cells read.
        inputs_chirho: Vec<String>,
        /// Output cells written.
        outputs_chirho: Vec<String>,
        /// Timestamp relative to trace start.
        timestamp_micros_chirho: u64,
    },
    /// A contradiction was detected.
    ContradictionChirho {
        /// Cell where contradiction occurred.
        cell_name_chirho: String,
        /// Description of the contradiction.
        description_chirho: String,
        /// Timestamp relative to trace start.
        timestamp_micros_chirho: u64,
    },
    /// Fixpoint was reached.
    FixpointChirho {
        /// Total propagator runs to reach fixpoint.
        total_runs_chirho: usize,
        /// Timestamp relative to trace start.
        timestamp_micros_chirho: u64,
    },
}

/// Records propagation events for debugging and visualization.
///
/// The trace can be exported to JSON for use with web-based visualization tools.
#[derive(Debug, Clone)]
pub struct PropagationTraceChirho {
    /// All recorded events.
    events_chirho: Vec<TraceEventChirho>,
    /// When the trace started.
    start_time_chirho: Instant,
    /// Maximum events to store (prevents memory exhaustion).
    max_events_chirho: usize,
}

impl PropagationTraceChirho {
    /// Creates a new empty trace.
    pub fn new_chirho() -> Self {
        Self {
            events_chirho: Vec::new(),
            start_time_chirho: Instant::now(),
            max_events_chirho: 100_000,
        }
    }

    /// Creates a trace with a custom event limit.
    pub fn with_limit_chirho(max_events_chirho: usize) -> Self {
        Self {
            events_chirho: Vec::new(),
            start_time_chirho: Instant::now(),
            max_events_chirho,
        }
    }

    /// Records a cell update event.
    pub fn record_cell_update_chirho(
        &mut self,
        cell_name_chirho: &str,
        new_value_chirho: &str,
        source_propagator_chirho: &str,
    ) {
        if self.events_chirho.len() >= self.max_events_chirho {
            return;
        }
        self.events_chirho.push(TraceEventChirho::CellUpdateChirho {
            cell_name_chirho: cell_name_chirho.to_string(),
            new_value_chirho: new_value_chirho.to_string(),
            source_propagator_chirho: if source_propagator_chirho.is_empty() {
                None
            } else {
                Some(source_propagator_chirho.to_string())
            },
            timestamp_micros_chirho: self.elapsed_micros_chirho(),
        });
    }

    /// Records a propagator run event.
    pub fn record_propagator_run_chirho(
        &mut self,
        propagator_name_chirho: &str,
        inputs_chirho: Vec<&str>,
        outputs_chirho: Vec<&str>,
    ) {
        if self.events_chirho.len() >= self.max_events_chirho {
            return;
        }
        self.events_chirho
            .push(TraceEventChirho::PropagatorRunChirho {
                propagator_name_chirho: propagator_name_chirho.to_string(),
                inputs_chirho: inputs_chirho.iter().map(|s| s.to_string()).collect(),
                outputs_chirho: outputs_chirho.iter().map(|s| s.to_string()).collect(),
                timestamp_micros_chirho: self.elapsed_micros_chirho(),
            });
    }

    /// Records a contradiction event.
    pub fn record_contradiction_chirho(&mut self, cell_name_chirho: &str, description_chirho: &str) {
        if self.events_chirho.len() >= self.max_events_chirho {
            return;
        }
        self.events_chirho
            .push(TraceEventChirho::ContradictionChirho {
                cell_name_chirho: cell_name_chirho.to_string(),
                description_chirho: description_chirho.to_string(),
                timestamp_micros_chirho: self.elapsed_micros_chirho(),
            });
    }

    /// Records that fixpoint was reached.
    pub fn record_fixpoint_chirho(&mut self, total_runs_chirho: usize) {
        if self.events_chirho.len() >= self.max_events_chirho {
            return;
        }
        self.events_chirho.push(TraceEventChirho::FixpointChirho {
            total_runs_chirho,
            timestamp_micros_chirho: self.elapsed_micros_chirho(),
        });
    }

    /// Returns elapsed microseconds since trace start.
    fn elapsed_micros_chirho(&self) -> u64 {
        self.start_time_chirho.elapsed().as_micros() as u64
    }

    /// Returns all recorded events.
    pub fn events_chirho(&self) -> &[TraceEventChirho] {
        &self.events_chirho
    }

    /// Returns the number of recorded events.
    pub fn event_count_chirho(&self) -> usize {
        self.events_chirho.len()
    }

    /// Clears all events and resets the start time.
    pub fn clear_chirho(&mut self) {
        self.events_chirho.clear();
        self.start_time_chirho = Instant::now();
    }

    /// Exports the trace to JSON format for web visualization.
    ///
    /// # Format
    ///
    /// ```json
    /// {
    ///   "events": [
    ///     {"type": "cell_update", "cell": "x", "value": "5.0", "source": "adder_1", "time_us": 100},
    ///     {"type": "propagator_run", "name": "adder_1", "inputs": ["a", "b"], "outputs": ["c"], "time_us": 150}
    ///   ],
    ///   "total_events": 2,
    ///   "duration_us": 200
    /// }
    /// ```
    pub fn to_json_chirho(&self) -> String {
        let mut json_chirho = String::from("{\n  \"events\": [\n");

        for (i_chirho, event_chirho) in self.events_chirho.iter().enumerate() {
            if i_chirho > 0 {
                json_chirho.push_str(",\n");
            }
            json_chirho.push_str("    ");
            json_chirho.push_str(&event_to_json_chirho(event_chirho));
        }

        json_chirho.push_str("\n  ],\n");
        json_chirho.push_str(&format!(
            "  \"total_events\": {},\n",
            self.events_chirho.len()
        ));
        json_chirho.push_str(&format!(
            "  \"duration_us\": {}\n",
            self.elapsed_micros_chirho()
        ));
        json_chirho.push_str("}");

        json_chirho
    }

    /// Returns trace statistics.
    pub fn stats_chirho(&self) -> TraceStatsChirho {
        let mut cell_updates_chirho = 0;
        let mut propagator_runs_chirho = 0;
        let mut contradictions_chirho = 0;

        for event_chirho in &self.events_chirho {
            match event_chirho {
                TraceEventChirho::CellUpdateChirho { .. } => cell_updates_chirho += 1,
                TraceEventChirho::PropagatorRunChirho { .. } => propagator_runs_chirho += 1,
                TraceEventChirho::ContradictionChirho { .. } => contradictions_chirho += 1,
                TraceEventChirho::FixpointChirho { .. } => {}
            }
        }

        TraceStatsChirho {
            cell_updates_chirho,
            propagator_runs_chirho,
            contradictions_chirho,
            total_events_chirho: self.events_chirho.len(),
            duration_chirho: self.start_time_chirho.elapsed(),
        }
    }
}

impl Default for PropagationTraceChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

/// Statistics about a propagation trace.
#[derive(Debug, Clone)]
pub struct TraceStatsChirho {
    /// Number of cell update events.
    pub cell_updates_chirho: usize,
    /// Number of propagator run events.
    pub propagator_runs_chirho: usize,
    /// Number of contradiction events.
    pub contradictions_chirho: usize,
    /// Total number of events.
    pub total_events_chirho: usize,
    /// Total duration of the trace.
    pub duration_chirho: Duration,
}

/// Converts a trace event to JSON string.
fn event_to_json_chirho(event_chirho: &TraceEventChirho) -> String {
    match event_chirho {
        TraceEventChirho::CellUpdateChirho {
            cell_name_chirho,
            new_value_chirho,
            source_propagator_chirho,
            timestamp_micros_chirho,
        } => {
            let source_chirho = source_propagator_chirho
                .as_ref()
                .map(|s| format!("\"{}\"", s))
                .unwrap_or_else(|| "null".to_string());
            format!(
                "{{\"type\": \"cell_update\", \"cell\": \"{}\", \"value\": \"{}\", \"source\": {}, \"time_us\": {}}}",
                escape_json_chirho(cell_name_chirho),
                escape_json_chirho(new_value_chirho),
                source_chirho,
                timestamp_micros_chirho
            )
        }
        TraceEventChirho::PropagatorRunChirho {
            propagator_name_chirho,
            inputs_chirho,
            outputs_chirho,
            timestamp_micros_chirho,
        } => {
            let inputs_json_chirho: Vec<String> = inputs_chirho
                .iter()
                .map(|s| format!("\"{}\"", escape_json_chirho(s)))
                .collect();
            let outputs_json_chirho: Vec<String> = outputs_chirho
                .iter()
                .map(|s| format!("\"{}\"", escape_json_chirho(s)))
                .collect();
            format!(
                "{{\"type\": \"propagator_run\", \"name\": \"{}\", \"inputs\": [{}], \"outputs\": [{}], \"time_us\": {}}}",
                escape_json_chirho(propagator_name_chirho),
                inputs_json_chirho.join(", "),
                outputs_json_chirho.join(", "),
                timestamp_micros_chirho
            )
        }
        TraceEventChirho::ContradictionChirho {
            cell_name_chirho,
            description_chirho,
            timestamp_micros_chirho,
        } => {
            format!(
                "{{\"type\": \"contradiction\", \"cell\": \"{}\", \"description\": \"{}\", \"time_us\": {}}}",
                escape_json_chirho(cell_name_chirho),
                escape_json_chirho(description_chirho),
                timestamp_micros_chirho
            )
        }
        TraceEventChirho::FixpointChirho {
            total_runs_chirho,
            timestamp_micros_chirho,
        } => {
            format!(
                "{{\"type\": \"fixpoint\", \"total_runs\": {}, \"time_us\": {}}}",
                total_runs_chirho, timestamp_micros_chirho
            )
        }
    }
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

/// Tracks value history for a single cell.
#[derive(Debug, Clone)]
pub struct CellHistoryChirho {
    /// Cell name.
    cell_name_chirho: String,
    /// Value changes: (timestamp_micros, value_string).
    history_chirho: Vec<(u64, String)>,
    /// Maximum history entries.
    max_entries_chirho: usize,
    /// Start time for timestamps.
    start_time_chirho: Instant,
}

impl CellHistoryChirho {
    /// Creates a new cell history tracker.
    pub fn new_chirho(cell_name_chirho: &str) -> Self {
        Self {
            cell_name_chirho: cell_name_chirho.to_string(),
            history_chirho: Vec::new(),
            max_entries_chirho: 1000,
            start_time_chirho: Instant::now(),
        }
    }

    /// Records a value change.
    pub fn record_value_chirho(&mut self, value_chirho: &str) {
        if self.history_chirho.len() >= self.max_entries_chirho {
            self.history_chirho.remove(0); // Remove oldest
        }
        let timestamp_chirho = self.start_time_chirho.elapsed().as_micros() as u64;
        self.history_chirho
            .push((timestamp_chirho, value_chirho.to_string()));
    }

    /// Returns the cell name.
    pub fn cell_name_chirho(&self) -> &str {
        &self.cell_name_chirho
    }

    /// Returns the value history.
    pub fn history_chirho(&self) -> &[(u64, String)] {
        &self.history_chirho
    }

    /// Returns the most recent value.
    pub fn current_value_chirho(&self) -> Option<&str> {
        self.history_chirho.last().map(|(_, v)| v.as_str())
    }

    /// Returns the number of recorded changes.
    pub fn change_count_chirho(&self) -> usize {
        self.history_chirho.len()
    }

    /// Exports to JSON format.
    pub fn to_json_chirho(&self) -> String {
        let entries_chirho: Vec<String> = self
            .history_chirho
            .iter()
            .map(|(ts_chirho, val_chirho)| {
                format!(
                    "{{\"time_us\": {}, \"value\": \"{}\"}}",
                    ts_chirho,
                    escape_json_chirho(val_chirho)
                )
            })
            .collect();

        format!(
            "{{\"cell\": \"{}\", \"history\": [{}]}}",
            escape_json_chirho(&self.cell_name_chirho),
            entries_chirho.join(", ")
        )
    }
}

/// Complete snapshot of network state for visualization.
#[derive(Debug, Clone)]
pub struct NetworkSnapshotChirho {
    /// Cell values at snapshot time.
    pub cells_chirho: HashMap<String, String>,
    /// Active constraints (id -> description).
    pub constraints_chirho: HashMap<String, String>,
    /// Edges: (from_cell, to_cell, constraint_id).
    pub edges_chirho: Vec<(String, String, String)>,
    /// Snapshot timestamp.
    pub timestamp_chirho: u64,
}

impl NetworkSnapshotChirho {
    /// Creates an empty snapshot.
    pub fn new_chirho() -> Self {
        Self {
            cells_chirho: HashMap::new(),
            constraints_chirho: HashMap::new(),
            edges_chirho: Vec::new(),
            timestamp_chirho: 0,
        }
    }

    /// Exports to JSON format for web visualization.
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
            .map(|(id_chirho, desc_chirho)| {
                format!(
                    "{{\"id\": \"{}\", \"description\": \"{}\"}}",
                    escape_json_chirho(id_chirho),
                    escape_json_chirho(desc_chirho)
                )
            })
            .collect();

        let edges_json_chirho: Vec<String> = self
            .edges_chirho
            .iter()
            .map(|(from_chirho, to_chirho, cid_chirho)| {
                format!(
                    "{{\"from\": \"{}\", \"to\": \"{}\", \"constraint\": \"{}\"}}",
                    escape_json_chirho(from_chirho),
                    escape_json_chirho(to_chirho),
                    escape_json_chirho(cid_chirho)
                )
            })
            .collect();

        format!(
            "{{\n  \"cells\": [{}],\n  \"constraints\": [{}],\n  \"edges\": [{}],\n  \"timestamp_us\": {}\n}}",
            cells_json_chirho.join(", "),
            constraints_json_chirho.join(", "),
            edges_json_chirho.join(", "),
            self.timestamp_chirho
        )
    }

    /// Exports to DOT/Graphviz format.
    pub fn to_dot_chirho(&self) -> String {
        let mut dot_chirho = String::from("digraph PropagatorNetwork {\n");
        dot_chirho.push_str("  rankdir=LR;\n");
        dot_chirho.push_str("  node [shape=ellipse];\n\n");

        // Cells as nodes
        for (name_chirho, value_chirho) in &self.cells_chirho {
            let label_chirho = format!("{}\\n{}", name_chirho, value_chirho);
            dot_chirho.push_str(&format!(
                "  \"{}\" [label=\"{}\"];\n",
                name_chirho, label_chirho
            ));
        }

        dot_chirho.push_str("\n");

        // Edges
        for (from_chirho, to_chirho, constraint_chirho) in &self.edges_chirho {
            dot_chirho.push_str(&format!(
                "  \"{}\" -> \"{}\" [label=\"{}\"];\n",
                from_chirho, to_chirho, constraint_chirho
            ));
        }

        dot_chirho.push_str("}\n");
        dot_chirho
    }
}

impl Default for NetworkSnapshotChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_trace_creation_chirho() {
        let trace_chirho = PropagationTraceChirho::new_chirho();
        assert_eq!(trace_chirho.event_count_chirho(), 0);
    }

    #[test]
    fn test_trace_record_events_chirho() {
        let mut trace_chirho = PropagationTraceChirho::new_chirho();

        trace_chirho.record_cell_update_chirho("x", "5.0", "adder_1");
        trace_chirho.record_propagator_run_chirho("adder_1", vec!["a", "b"], vec!["c"]);
        trace_chirho.record_contradiction_chirho("y", "Empty interval");
        trace_chirho.record_fixpoint_chirho(10);

        assert_eq!(trace_chirho.event_count_chirho(), 4);
    }

    #[test]
    fn test_trace_to_json_chirho() {
        let mut trace_chirho = PropagationTraceChirho::new_chirho();
        trace_chirho.record_cell_update_chirho("x", "5.0", "");

        let json_chirho = trace_chirho.to_json_chirho();
        assert!(json_chirho.contains("\"events\""));
        assert!(json_chirho.contains("cell_update"));
        assert!(json_chirho.contains("\"x\""));
    }

    #[test]
    fn test_trace_stats_chirho() {
        let mut trace_chirho = PropagationTraceChirho::new_chirho();
        trace_chirho.record_cell_update_chirho("x", "5.0", "p1");
        trace_chirho.record_cell_update_chirho("y", "3.0", "p2");
        trace_chirho.record_propagator_run_chirho("p1", vec!["a"], vec!["x"]);
        trace_chirho.record_contradiction_chirho("z", "error");

        let stats_chirho = trace_chirho.stats_chirho();
        assert_eq!(stats_chirho.cell_updates_chirho, 2);
        assert_eq!(stats_chirho.propagator_runs_chirho, 1);
        assert_eq!(stats_chirho.contradictions_chirho, 1);
        assert_eq!(stats_chirho.total_events_chirho, 4);
    }

    #[test]
    fn test_trace_limit_chirho() {
        let mut trace_chirho = PropagationTraceChirho::with_limit_chirho(3);
        trace_chirho.record_cell_update_chirho("a", "1", "");
        trace_chirho.record_cell_update_chirho("b", "2", "");
        trace_chirho.record_cell_update_chirho("c", "3", "");
        trace_chirho.record_cell_update_chirho("d", "4", ""); // Should be ignored

        assert_eq!(trace_chirho.event_count_chirho(), 3);
    }

    #[test]
    fn test_cell_history_chirho() {
        let mut history_chirho = CellHistoryChirho::new_chirho("x");

        history_chirho.record_value_chirho("[0, 100]");
        history_chirho.record_value_chirho("[10, 50]");
        history_chirho.record_value_chirho("[25, 30]");

        assert_eq!(history_chirho.change_count_chirho(), 3);
        assert_eq!(history_chirho.current_value_chirho(), Some("[25, 30]"));
    }

    #[test]
    fn test_cell_history_to_json_chirho() {
        let mut history_chirho = CellHistoryChirho::new_chirho("temp");
        history_chirho.record_value_chirho("100");

        let json_chirho = history_chirho.to_json_chirho();
        assert!(json_chirho.contains("\"temp\""));
        assert!(json_chirho.contains("\"100\""));
    }

    #[test]
    fn test_network_snapshot_chirho() {
        let mut snapshot_chirho = NetworkSnapshotChirho::new_chirho();
        snapshot_chirho
            .cells_chirho
            .insert("x".to_string(), "5".to_string());
        snapshot_chirho
            .cells_chirho
            .insert("y".to_string(), "3".to_string());
        snapshot_chirho
            .constraints_chirho
            .insert("c1".to_string(), "x + y = z".to_string());
        snapshot_chirho
            .edges_chirho
            .push(("x".to_string(), "z".to_string(), "c1".to_string()));

        let json_chirho = snapshot_chirho.to_json_chirho();
        assert!(json_chirho.contains("\"cells\""));
        assert!(json_chirho.contains("\"edges\""));
    }

    #[test]
    fn test_network_snapshot_to_dot_chirho() {
        let mut snapshot_chirho = NetworkSnapshotChirho::new_chirho();
        snapshot_chirho
            .cells_chirho
            .insert("a".to_string(), "1".to_string());
        snapshot_chirho
            .cells_chirho
            .insert("b".to_string(), "2".to_string());
        snapshot_chirho
            .edges_chirho
            .push(("a".to_string(), "b".to_string(), "eq".to_string()));

        let dot_chirho = snapshot_chirho.to_dot_chirho();
        assert!(dot_chirho.contains("digraph"));
        assert!(dot_chirho.contains("\"a\""));
        assert!(dot_chirho.contains("\"b\""));
        assert!(dot_chirho.contains("->"));
    }

    #[test]
    fn test_escape_json_chirho() {
        assert_eq!(escape_json_chirho("hello"), "hello");
        assert_eq!(escape_json_chirho("he\"llo"), "he\\\"llo");
        assert_eq!(escape_json_chirho("line\nbreak"), "line\\nbreak");
    }
}
