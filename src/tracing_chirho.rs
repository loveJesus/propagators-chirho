// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Tracing and debugging instrumentation for propagator networks.
//!
//! This module provides observability into propagator execution via the
//! `tracing` crate. It adds performance overhead when enabled, so it's
//! behind a feature flag.
//!
//! # Feature Flag
//!
//! Enable with the `tracing` feature:
//!
//! ```toml
//! [dependencies]
//! propagators-chirho = { version = "0.1", features = ["tracing"] }
//! ```
//!
//! # Usage
//!
//! The tracing output can be collected with any `tracing` subscriber:
//!
//! ```ignore
//! use tracing_subscriber;
//!
//! // Initialize a subscriber (e.g., for logging)
//! tracing_subscriber::fmt::init();
//!
//! // Now propagator operations will emit trace events
//! ```
//!
//! # Events
//!
//! The following events are emitted:
//!
//! - `propagate_start`: When propagation begins
//! - `propagate_step`: Each propagator firing
//! - `propagate_end`: When propagation reaches fixpoint
//! - `cell_update`: When a cell's value changes
//! - `contradiction`: When a contradiction is detected

#[cfg(feature = "tracing")]
use tracing::{debug, info, trace, warn};

/// Logs the start of propagation.
#[inline]
pub fn trace_propagate_start_chirho(cell_count_chirho: usize, propagator_count_chirho: usize) {
    #[cfg(feature = "tracing")]
    {
        info!(
            cells_chirho = cell_count_chirho,
            propagators_chirho = propagator_count_chirho,
            "Propagation starting"
        );
    }
    #[cfg(not(feature = "tracing"))]
    {
        let _ = (cell_count_chirho, propagator_count_chirho);
    }
}

/// Logs a single propagation step.
#[inline]
pub fn trace_propagate_step_chirho(step_chirho: usize, propagator_id_chirho: usize) {
    #[cfg(feature = "tracing")]
    {
        trace!(
            step_chirho = step_chirho,
            propagator_id_chirho = propagator_id_chirho,
            "Propagator fired"
        );
    }
    #[cfg(not(feature = "tracing"))]
    {
        let _ = (step_chirho, propagator_id_chirho);
    }
}

/// Logs the end of propagation.
#[inline]
pub fn trace_propagate_end_chirho(steps_chirho: usize, cells_updated_chirho: usize) {
    #[cfg(feature = "tracing")]
    {
        info!(
            steps_chirho = steps_chirho,
            cells_updated_chirho = cells_updated_chirho,
            "Propagation complete"
        );
    }
    #[cfg(not(feature = "tracing"))]
    {
        let _ = (steps_chirho, cells_updated_chirho);
    }
}

/// Logs a cell value update.
#[inline]
pub fn trace_cell_update_chirho(
    cell_id_chirho: usize,
    old_info_chirho: &str,
    new_info_chirho: &str,
) {
    #[cfg(feature = "tracing")]
    {
        debug!(
            cell_id_chirho = cell_id_chirho,
            old_chirho = old_info_chirho,
            new_chirho = new_info_chirho,
            "Cell updated"
        );
    }
    #[cfg(not(feature = "tracing"))]
    {
        let _ = (cell_id_chirho, old_info_chirho, new_info_chirho);
    }
}

/// Logs a contradiction detection.
#[inline]
pub fn trace_contradiction_chirho(cell_id_chirho: usize, reason_chirho: &str) {
    #[cfg(feature = "tracing")]
    {
        warn!(
            cell_id_chirho = cell_id_chirho,
            reason_chirho = reason_chirho,
            "Contradiction detected"
        );
    }
    #[cfg(not(feature = "tracing"))]
    {
        let _ = (cell_id_chirho, reason_chirho);
    }
}

/// A guard that traces the duration of a scope.
///
/// When the `tracing` feature is enabled, this uses `tracing::span`.
/// When disabled, this is a no-op.
#[cfg(feature = "tracing")]
pub struct SpanGuardChirho {
    _span_chirho: tracing::span::EnteredSpan,
}

/// A guard that traces the duration of a scope (no-op variant).
///
/// This is the no-op version used when the `tracing` feature is disabled.
#[cfg(not(feature = "tracing"))]
pub struct SpanGuardChirho;

impl SpanGuardChirho {
    /// Creates a new span guard for propagation.
    #[inline]
    pub fn propagation_chirho() -> Self {
        #[cfg(feature = "tracing")]
        {
            let span_chirho = tracing::info_span!("propagation");
            Self {
                _span_chirho: span_chirho.entered(),
            }
        }
        #[cfg(not(feature = "tracing"))]
        {
            Self
        }
    }

    /// Creates a new span guard for a specific propagator.
    #[inline]
    pub fn propagator_chirho(id_chirho: usize, name_chirho: &str) -> Self {
        #[cfg(feature = "tracing")]
        {
            let span_chirho = tracing::trace_span!(
                "propagator",
                id_chirho = id_chirho,
                name_chirho = name_chirho
            );
            Self {
                _span_chirho: span_chirho.entered(),
            }
        }
        #[cfg(not(feature = "tracing"))]
        {
            let _ = (id_chirho, name_chirho);
            Self
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_tracing_compiles_chirho() {
        // Just verify the functions compile and don't panic
        trace_propagate_start_chirho(10, 5);
        trace_propagate_step_chirho(1, 0);
        trace_cell_update_chirho(0, "[0, 10]", "[5, 10]");
        trace_contradiction_chirho(0, "empty intersection");
        trace_propagate_end_chirho(10, 3);

        let _guard_chirho = SpanGuardChirho::propagation_chirho();
        let _guard2_chirho = SpanGuardChirho::propagator_chirho(0, "adder");
    }
}
