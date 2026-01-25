// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! High-performance arena-based propagator network.
//!
//! This module provides an optimized implementation using:
//! - Arena allocation (no Rc overhead)
//! - Index-based cell references (cache-friendly)
//! - Static dispatch (no vtable lookups)
//!
//! Enable with the `arena` feature:
//! ```toml
//! propagators-chirho = { version = "0.1", features = ["arena"] }
//! ```
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "arena")]
//! use propagators_chirho::arena_chirho::ArenaNetworkChirho;
//!
//! # #[cfg(feature = "arena")]
//! # fn main() {
//! let mut network_chirho = ArenaNetworkChirho::new_chirho();
//!
//! let a_chirho = network_chirho.make_cell_chirho();
//! let b_chirho = network_chirho.make_cell_chirho();
//! let c_chirho = network_chirho.make_cell_chirho();
//!
//! network_chirho.add_adder_chirho(a_chirho, b_chirho, c_chirho);
//!
//! network_chirho.set_exact_chirho(a_chirho, 3.0);
//! network_chirho.set_exact_chirho(b_chirho, 4.0);
//! network_chirho.propagate_chirho();
//!
//! assert_eq!(network_chirho.get_exact_chirho(c_chirho), Some(7.0));
//! # }
//! # #[cfg(not(feature = "arena"))]
//! # fn main() {}
//! ```
//!
//! # Performance
//!
//! For small networks (< 100 cells), the standard `ConstraintSystemChirho` may
//! be faster due to lower overhead. Arena allocation shines with thousands of
//! cells and complex constraint networks.

use std::collections::VecDeque;

use crate::core_chirho::interval_chirho::{IntervalChirho, NumericInfoChirho};

// ============================================================================
// HELPER MACROS - Eliminate boilerplate for adding propagators
// ============================================================================

/// Macro for generating add_*_chirho methods for ternary (3-cell) propagators.
macro_rules! impl_add_ternary_chirho {
    ($method_name_chirho:ident, $prop_type_chirho:expr) => {
        /// Adds a ternary propagator (bidirectional).
        pub fn $method_name_chirho(
            &mut self,
            a_chirho: CellIdChirho,
            b_chirho: CellIdChirho,
            c_chirho: CellIdChirho,
        ) {
            let idx_chirho = self.propagators_chirho.len();
            self.propagators_chirho.push(PropagatorEntryChirho {
                type_chirho: $prop_type_chirho,
                cells_chirho: vec![a_chirho, b_chirho, c_chirho],
            });

            // Register with all cells (bidirectional)
            self.cell_to_propagators_chirho[a_chirho.0].push(idx_chirho);
            self.cell_to_propagators_chirho[b_chirho.0].push(idx_chirho);
            self.cell_to_propagators_chirho[c_chirho.0].push(idx_chirho);
        }
    };
}

/// Macro for generating add_*_chirho methods for binary (2-cell) propagators.
macro_rules! impl_add_binary_chirho {
    ($method_name_chirho:ident, $prop_type_chirho:expr) => {
        /// Adds a binary propagator (bidirectional).
        pub fn $method_name_chirho(&mut self, a_chirho: CellIdChirho, b_chirho: CellIdChirho) {
            let idx_chirho = self.propagators_chirho.len();
            self.propagators_chirho.push(PropagatorEntryChirho {
                type_chirho: $prop_type_chirho,
                cells_chirho: vec![a_chirho, b_chirho],
            });

            self.cell_to_propagators_chirho[a_chirho.0].push(idx_chirho);
            self.cell_to_propagators_chirho[b_chirho.0].push(idx_chirho);
        }
    };
}

/// Index handle to a cell in the arena.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CellIdChirho(pub usize);

impl CellIdChirho {
    /// Creates a CellId from a raw index.
    #[inline]
    pub fn from_index_chirho(index_chirho: usize) -> Self {
        Self(index_chirho)
    }

    /// Returns the underlying index.
    #[inline]
    pub fn index_chirho(self) -> usize {
        self.0
    }
}

/// Type of propagator operation.
#[derive(Clone, Copy, Debug)]
pub enum PropagatorTypeChirho {
    /// a + b = c
    AdderChirho,
    /// a - b = c
    SubtractorChirho,
    /// a * b = c
    MultiplierChirho,
    /// a / b = c
    DividerChirho,
    /// a² = b
    SquarerChirho,
    /// √a = b
    SqrterChirho,
    /// |a| = b
    AbsoluterChirho,
    /// -a = b
    NegaterChirho,
}

/// A propagator connecting cells.
#[derive(Clone, Debug)]
struct PropagatorEntryChirho {
    type_chirho: PropagatorTypeChirho,
    /// All connected cells (bidirectional propagators read/write all).
    cells_chirho: Vec<CellIdChirho>,
}

/// High-performance arena-based propagator network.
///
/// All cells are stored contiguously in a `Vec`, with propagators
/// referencing them by index. This eliminates Rc overhead and
/// improves cache locality.
pub struct ArenaNetworkChirho {
    /// Cell contents stored contiguously.
    cells_chirho: Vec<NumericInfoChirho>,
    /// Propagators in the network.
    propagators_chirho: Vec<PropagatorEntryChirho>,
    /// Map from cell to its dependent propagators.
    cell_to_propagators_chirho: Vec<Vec<usize>>,
    /// Queue of propagators to run.
    queue_chirho: VecDeque<usize>,
    /// Statistics.
    propagations_chirho: usize,
}

impl ArenaNetworkChirho {
    /// Creates a new empty network.
    pub fn new_chirho() -> Self {
        Self {
            cells_chirho: Vec::new(),
            propagators_chirho: Vec::new(),
            cell_to_propagators_chirho: Vec::new(),
            queue_chirho: VecDeque::new(),
            propagations_chirho: 0,
        }
    }

    /// Creates a new cell, returning its handle.
    #[inline]
    pub fn make_cell_chirho(&mut self) -> CellIdChirho {
        let id_chirho = self.cells_chirho.len();
        self.cells_chirho.push(NumericInfoChirho::NothingChirho);
        self.cell_to_propagators_chirho.push(Vec::new());
        CellIdChirho(id_chirho)
    }

    /// Gets the current value of a cell as an exact f64 (if known).
    #[inline]
    pub fn get_exact_chirho(&self, cell_chirho: CellIdChirho) -> Option<f64> {
        match &self.cells_chirho[cell_chirho.0] {
            NumericInfoChirho::IntervalChirho(iv_chirho) if iv_chirho.is_exact_chirho() => {
                Some(iv_chirho.lo_chirho)
            }
            _ => None,
        }
    }

    /// Gets the current interval of a cell.
    #[inline]
    pub fn get_interval_chirho(&self, cell_chirho: CellIdChirho) -> Option<IntervalChirho> {
        match &self.cells_chirho[cell_chirho.0] {
            NumericInfoChirho::IntervalChirho(iv_chirho) => Some(*iv_chirho),
            _ => None,
        }
    }

    /// Gets the raw content of a cell.
    #[inline]
    pub fn get_content_chirho(&self, cell_chirho: CellIdChirho) -> NumericInfoChirho {
        self.cells_chirho[cell_chirho.0]
    }

    /// Sets a cell to an exact value.
    pub fn set_exact_chirho(&mut self, cell_chirho: CellIdChirho, value_chirho: f64) {
        self.add_info_chirho(cell_chirho, NumericInfoChirho::exact_chirho(value_chirho));
    }

    /// Sets a cell to an interval.
    pub fn set_interval_chirho(
        &mut self,
        cell_chirho: CellIdChirho,
        lo_chirho: f64,
        hi_chirho: f64,
    ) {
        self.add_info_chirho(
            cell_chirho,
            NumericInfoChirho::interval_chirho(lo_chirho, hi_chirho),
        );
    }

    /// Adds information to a cell, triggering propagation.
    fn add_info_chirho(&mut self, cell_chirho: CellIdChirho, info_chirho: NumericInfoChirho) {
        let old_chirho = self.cells_chirho[cell_chirho.0];
        let new_chirho = old_chirho.merge_chirho(&info_chirho);

        if new_chirho != old_chirho {
            self.cells_chirho[cell_chirho.0] = new_chirho;

            // Queue dependent propagators
            for &prop_idx_chirho in &self.cell_to_propagators_chirho[cell_chirho.0] {
                if !self.queue_chirho.contains(&prop_idx_chirho) {
                    self.queue_chirho.push_back(prop_idx_chirho);
                }
            }
        }
    }

    // ========================================================================
    // TERNARY PROPAGATORS (a, b, c)
    // ========================================================================

    impl_add_ternary_chirho!(add_adder_chirho, PropagatorTypeChirho::AdderChirho);
    impl_add_ternary_chirho!(
        add_subtractor_chirho,
        PropagatorTypeChirho::SubtractorChirho
    );
    impl_add_ternary_chirho!(
        add_multiplier_chirho,
        PropagatorTypeChirho::MultiplierChirho
    );
    impl_add_ternary_chirho!(add_divider_chirho, PropagatorTypeChirho::DividerChirho);

    // ========================================================================
    // BINARY PROPAGATORS (a, b)
    // ========================================================================

    impl_add_binary_chirho!(add_squarer_chirho, PropagatorTypeChirho::SquarerChirho);
    impl_add_binary_chirho!(add_sqrter_chirho, PropagatorTypeChirho::SqrterChirho);
    impl_add_binary_chirho!(add_absoluter_chirho, PropagatorTypeChirho::AbsoluterChirho);
    impl_add_binary_chirho!(add_negater_chirho, PropagatorTypeChirho::NegaterChirho);

    /// Runs propagation to fixpoint.
    pub fn propagate_chirho(&mut self) {
        while let Some(prop_idx_chirho) = self.queue_chirho.pop_front() {
            self.propagations_chirho += 1;
            self.run_propagator_chirho(prop_idx_chirho);
        }
    }

    /// Runs propagation with a maximum number of steps.
    pub fn propagate_bounded_chirho(&mut self, max_steps_chirho: usize) -> bool {
        let mut steps_chirho = 0;
        while let Some(prop_idx_chirho) = self.queue_chirho.pop_front() {
            if steps_chirho >= max_steps_chirho {
                return false;
            }
            self.propagations_chirho += 1;
            steps_chirho += 1;
            self.run_propagator_chirho(prop_idx_chirho);
        }
        true
    }

    fn run_propagator_chirho(&mut self, prop_idx_chirho: usize) {
        // Copy propagator data to avoid borrow conflicts
        let type_chirho = self.propagators_chirho[prop_idx_chirho].type_chirho;
        let cells_chirho = self.propagators_chirho[prop_idx_chirho]
            .cells_chirho
            .clone();

        match type_chirho {
            // Ternary propagators (a, b, c)
            PropagatorTypeChirho::AdderChirho => {
                self.run_ternary_chirho(
                    &cells_chirho,
                    IntervalChirho::add_chirho,
                    IntervalChirho::sub_chirho,
                    IntervalChirho::sub_chirho,
                );
            }
            PropagatorTypeChirho::SubtractorChirho => {
                self.run_ternary_chirho(
                    &cells_chirho,
                    IntervalChirho::sub_chirho,
                    IntervalChirho::add_chirho,
                    |c_chirho, a_chirho| a_chirho.sub_chirho(c_chirho),
                );
            }
            PropagatorTypeChirho::MultiplierChirho => {
                self.run_ternary_with_guard_chirho(
                    &cells_chirho,
                    IntervalChirho::mul_chirho,
                    IntervalChirho::div_chirho,
                    IntervalChirho::div_chirho,
                );
            }
            PropagatorTypeChirho::DividerChirho => {
                self.run_ternary_with_guard_chirho(
                    &cells_chirho,
                    IntervalChirho::div_chirho,
                    IntervalChirho::mul_chirho,
                    |c_chirho, a_chirho| a_chirho.div_chirho(c_chirho),
                );
            }

            // Binary propagators (a, b)
            PropagatorTypeChirho::SquarerChirho => {
                self.run_binary_with_guard_chirho(
                    &cells_chirho,
                    IntervalChirho::square_chirho,
                    IntervalChirho::sqrt_chirho,
                );
            }
            PropagatorTypeChirho::SqrterChirho => {
                self.run_binary_with_guard_chirho(
                    &cells_chirho,
                    IntervalChirho::sqrt_chirho,
                    IntervalChirho::square_chirho,
                );
            }
            PropagatorTypeChirho::AbsoluterChirho => {
                self.run_binary_absoluter_chirho(&cells_chirho);
            }
            PropagatorTypeChirho::NegaterChirho => {
                self.run_binary_chirho(
                    &cells_chirho,
                    IntervalChirho::neg_chirho,
                    IntervalChirho::neg_chirho,
                );
            }
        }
    }

    // ========================================================================
    // HELPER METHODS FOR RUNNING PROPAGATORS
    // ========================================================================

    /// Runs a ternary propagator: op(a, b) = c (bidirectional).
    fn run_ternary_chirho<F, G, H>(
        &mut self,
        cells_chirho: &[CellIdChirho],
        forward_chirho: F,
        backward_a_chirho: G,
        backward_b_chirho: H,
    ) where
        F: Fn(&IntervalChirho, &IntervalChirho) -> IntervalChirho,
        G: Fn(&IntervalChirho, &IntervalChirho) -> IntervalChirho,
        H: Fn(&IntervalChirho, &IntervalChirho) -> IntervalChirho,
    {
        let a_chirho = self.cells_chirho[cells_chirho[0].0];
        let b_chirho = self.cells_chirho[cells_chirho[1].0];
        let c_chirho = self.cells_chirho[cells_chirho[2].0];

        // Forward: c = op(a, b)
        if let (Some(a_iv_chirho), Some(b_iv_chirho)) =
            (a_chirho.as_interval_chirho(), b_chirho.as_interval_chirho())
        {
            let result_chirho = forward_chirho(a_iv_chirho, b_iv_chirho);
            self.add_info_chirho(
                cells_chirho[2],
                NumericInfoChirho::IntervalChirho(result_chirho),
            );
        }

        // Backward: a = inv_op(c, b)
        if let (Some(c_iv_chirho), Some(b_iv_chirho)) =
            (c_chirho.as_interval_chirho(), b_chirho.as_interval_chirho())
        {
            let result_chirho = backward_a_chirho(c_iv_chirho, b_iv_chirho);
            self.add_info_chirho(
                cells_chirho[0],
                NumericInfoChirho::IntervalChirho(result_chirho),
            );
        }

        // Backward: b = inv_op(c, a)
        if let (Some(c_iv_chirho), Some(a_iv_chirho)) =
            (c_chirho.as_interval_chirho(), a_chirho.as_interval_chirho())
        {
            let result_chirho = backward_b_chirho(c_iv_chirho, a_iv_chirho);
            self.add_info_chirho(
                cells_chirho[1],
                NumericInfoChirho::IntervalChirho(result_chirho),
            );
        }
    }

    /// Runs a ternary propagator with empty-interval guards for division-like ops.
    fn run_ternary_with_guard_chirho<F, G, H>(
        &mut self,
        cells_chirho: &[CellIdChirho],
        forward_chirho: F,
        backward_a_chirho: G,
        backward_b_chirho: H,
    ) where
        F: Fn(&IntervalChirho, &IntervalChirho) -> IntervalChirho,
        G: Fn(&IntervalChirho, &IntervalChirho) -> IntervalChirho,
        H: Fn(&IntervalChirho, &IntervalChirho) -> IntervalChirho,
    {
        let a_chirho = self.cells_chirho[cells_chirho[0].0];
        let b_chirho = self.cells_chirho[cells_chirho[1].0];
        let c_chirho = self.cells_chirho[cells_chirho[2].0];

        // Forward: c = op(a, b)
        if let (Some(a_iv_chirho), Some(b_iv_chirho)) =
            (a_chirho.as_interval_chirho(), b_chirho.as_interval_chirho())
        {
            let result_chirho = forward_chirho(a_iv_chirho, b_iv_chirho);
            if !result_chirho.is_empty_chirho() {
                self.add_info_chirho(
                    cells_chirho[2],
                    NumericInfoChirho::IntervalChirho(result_chirho),
                );
            }
        }

        // Backward: a = inv_op(c, b)
        if let (Some(c_iv_chirho), Some(b_iv_chirho)) =
            (c_chirho.as_interval_chirho(), b_chirho.as_interval_chirho())
        {
            let result_chirho = backward_a_chirho(c_iv_chirho, b_iv_chirho);
            if !result_chirho.is_empty_chirho() {
                self.add_info_chirho(
                    cells_chirho[0],
                    NumericInfoChirho::IntervalChirho(result_chirho),
                );
            }
        }

        // Backward: b = inv_op(c, a)
        if let (Some(c_iv_chirho), Some(a_iv_chirho)) =
            (c_chirho.as_interval_chirho(), a_chirho.as_interval_chirho())
        {
            let result_chirho = backward_b_chirho(c_iv_chirho, a_iv_chirho);
            if !result_chirho.is_empty_chirho() {
                self.add_info_chirho(
                    cells_chirho[1],
                    NumericInfoChirho::IntervalChirho(result_chirho),
                );
            }
        }
    }

    /// Runs a binary propagator: op(a) = b (bidirectional).
    fn run_binary_chirho<F, G>(
        &mut self,
        cells_chirho: &[CellIdChirho],
        forward_chirho: F,
        backward_chirho: G,
    ) where
        F: Fn(&IntervalChirho) -> IntervalChirho,
        G: Fn(&IntervalChirho) -> IntervalChirho,
    {
        let a_chirho = self.cells_chirho[cells_chirho[0].0];
        let b_chirho = self.cells_chirho[cells_chirho[1].0];

        // Forward: b = op(a)
        if let Some(a_iv_chirho) = a_chirho.as_interval_chirho() {
            let result_chirho = forward_chirho(a_iv_chirho);
            self.add_info_chirho(
                cells_chirho[1],
                NumericInfoChirho::IntervalChirho(result_chirho),
            );
        }

        // Backward: a = inv_op(b)
        if let Some(b_iv_chirho) = b_chirho.as_interval_chirho() {
            let result_chirho = backward_chirho(b_iv_chirho);
            self.add_info_chirho(
                cells_chirho[0],
                NumericInfoChirho::IntervalChirho(result_chirho),
            );
        }
    }

    /// Runs a binary propagator with empty-interval guard.
    fn run_binary_with_guard_chirho<F, G>(
        &mut self,
        cells_chirho: &[CellIdChirho],
        forward_chirho: F,
        backward_chirho: G,
    ) where
        F: Fn(&IntervalChirho) -> IntervalChirho,
        G: Fn(&IntervalChirho) -> IntervalChirho,
    {
        let a_chirho = self.cells_chirho[cells_chirho[0].0];
        let b_chirho = self.cells_chirho[cells_chirho[1].0];

        // Forward: b = op(a)
        if let Some(a_iv_chirho) = a_chirho.as_interval_chirho() {
            let result_chirho = forward_chirho(a_iv_chirho);
            if !result_chirho.is_empty_chirho() {
                self.add_info_chirho(
                    cells_chirho[1],
                    NumericInfoChirho::IntervalChirho(result_chirho),
                );
            }
        }

        // Backward: a = inv_op(b)
        if let Some(b_iv_chirho) = b_chirho.as_interval_chirho() {
            let result_chirho = backward_chirho(b_iv_chirho);
            if !result_chirho.is_empty_chirho() {
                self.add_info_chirho(
                    cells_chirho[0],
                    NumericInfoChirho::IntervalChirho(result_chirho),
                );
            }
        }
    }

    /// Special handler for absolute value (non-invertible).
    fn run_binary_absoluter_chirho(&mut self, cells_chirho: &[CellIdChirho]) {
        let a_chirho = self.cells_chirho[cells_chirho[0].0];
        let b_chirho = self.cells_chirho[cells_chirho[1].0];

        // Forward: b = |a|
        if let Some(a_iv_chirho) = a_chirho.as_interval_chirho() {
            let result_chirho = a_iv_chirho.abs_chirho();
            self.add_info_chirho(
                cells_chirho[1],
                NumericInfoChirho::IntervalChirho(result_chirho),
            );
        }

        // Backward: a could be in [-b, -b_lo] or [b_lo, b] - constrain only
        if let (Some(a_iv_chirho), Some(b_iv_chirho)) =
            (a_chirho.as_interval_chirho(), b_chirho.as_interval_chirho())
        {
            // If |a| = b, then a is in [-b_hi, b_hi] intersected with current a
            let bound_chirho = b_iv_chirho.hi_chirho.max(b_iv_chirho.lo_chirho.abs());
            let constrained_chirho = IntervalChirho::new_chirho(-bound_chirho, bound_chirho);
            let result_chirho = a_iv_chirho.intersect_chirho(&constrained_chirho);
            if !result_chirho.is_empty_chirho() {
                self.add_info_chirho(
                    cells_chirho[0],
                    NumericInfoChirho::IntervalChirho(result_chirho),
                );
            }
        }
    }

    /// Returns the number of propagations performed.
    #[inline]
    pub fn propagation_count_chirho(&self) -> usize {
        self.propagations_chirho
    }

    /// Returns the number of cells.
    #[inline]
    pub fn cell_count_chirho(&self) -> usize {
        self.cells_chirho.len()
    }

    /// Returns the number of propagators.
    #[inline]
    pub fn propagator_count_chirho(&self) -> usize {
        self.propagators_chirho.len()
    }

    /// Returns true if any cell contains a contradiction.
    pub fn has_contradiction_chirho(&self) -> bool {
        self.cells_chirho
            .iter()
            .any(crate::core_chirho::interval_chirho::NumericInfoChirho::is_contradiction_chirho)
    }

    /// Resets the network, clearing all cell values.
    pub fn reset_chirho(&mut self) {
        for cell_chirho in &mut self.cells_chirho {
            *cell_chirho = NumericInfoChirho::NothingChirho;
        }
        self.queue_chirho.clear();
        self.propagations_chirho = 0;
    }
}

impl Default for ArenaNetworkChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_arena_simple_addition_chirho() {
        let mut net_chirho = ArenaNetworkChirho::new_chirho();

        let a_chirho = net_chirho.make_cell_chirho();
        let b_chirho = net_chirho.make_cell_chirho();
        let c_chirho = net_chirho.make_cell_chirho();

        net_chirho.add_adder_chirho(a_chirho, b_chirho, c_chirho);

        net_chirho.set_exact_chirho(a_chirho, 3.0);
        net_chirho.set_exact_chirho(b_chirho, 4.0);
        net_chirho.propagate_chirho();

        assert_eq!(net_chirho.get_exact_chirho(c_chirho), Some(7.0));
    }

    #[test]
    fn test_arena_backward_propagation_chirho() {
        let mut net_chirho = ArenaNetworkChirho::new_chirho();

        let a_chirho = net_chirho.make_cell_chirho();
        let b_chirho = net_chirho.make_cell_chirho();
        let c_chirho = net_chirho.make_cell_chirho();

        net_chirho.add_adder_chirho(a_chirho, b_chirho, c_chirho);

        net_chirho.set_exact_chirho(a_chirho, 3.0);
        net_chirho.set_exact_chirho(c_chirho, 7.0);
        net_chirho.propagate_chirho();

        assert_eq!(net_chirho.get_exact_chirho(b_chirho), Some(4.0));
    }

    #[test]
    fn test_arena_multiplier_chirho() {
        let mut net_chirho = ArenaNetworkChirho::new_chirho();

        let a_chirho = net_chirho.make_cell_chirho();
        let b_chirho = net_chirho.make_cell_chirho();
        let c_chirho = net_chirho.make_cell_chirho();

        net_chirho.add_multiplier_chirho(a_chirho, b_chirho, c_chirho);

        net_chirho.set_exact_chirho(a_chirho, 3.0);
        net_chirho.set_exact_chirho(b_chirho, 4.0);
        net_chirho.propagate_chirho();

        assert_eq!(net_chirho.get_exact_chirho(c_chirho), Some(12.0));
    }

    #[test]
    fn test_arena_squarer_chirho() {
        let mut net_chirho = ArenaNetworkChirho::new_chirho();

        let a_chirho = net_chirho.make_cell_chirho();
        let b_chirho = net_chirho.make_cell_chirho();

        net_chirho.add_squarer_chirho(a_chirho, b_chirho);

        net_chirho.set_exact_chirho(a_chirho, 5.0);
        net_chirho.propagate_chirho();

        assert_eq!(net_chirho.get_exact_chirho(b_chirho), Some(25.0));
    }

    #[test]
    fn test_arena_temperature_conversion_chirho() {
        let mut net_chirho = ArenaNetworkChirho::new_chirho();

        // F = C * 9/5 + 32
        let celsius_chirho = net_chirho.make_cell_chirho();
        let nine_fifths_chirho = net_chirho.make_cell_chirho();
        let product_chirho = net_chirho.make_cell_chirho();
        let thirty_two_chirho = net_chirho.make_cell_chirho();
        let fahrenheit_chirho = net_chirho.make_cell_chirho();

        net_chirho.add_multiplier_chirho(celsius_chirho, nine_fifths_chirho, product_chirho);
        net_chirho.add_adder_chirho(product_chirho, thirty_two_chirho, fahrenheit_chirho);

        net_chirho.set_exact_chirho(nine_fifths_chirho, 1.8);
        net_chirho.set_exact_chirho(thirty_two_chirho, 32.0);
        net_chirho.set_exact_chirho(celsius_chirho, 100.0);

        net_chirho.propagate_chirho();

        assert_eq!(net_chirho.get_exact_chirho(fahrenheit_chirho), Some(212.0));
    }

    #[test]
    fn test_arena_subtractor_chirho() {
        let mut net_chirho = ArenaNetworkChirho::new_chirho();

        let a_chirho = net_chirho.make_cell_chirho();
        let b_chirho = net_chirho.make_cell_chirho();
        let c_chirho = net_chirho.make_cell_chirho();

        net_chirho.add_subtractor_chirho(a_chirho, b_chirho, c_chirho);

        net_chirho.set_exact_chirho(a_chirho, 10.0);
        net_chirho.set_exact_chirho(b_chirho, 3.0);
        net_chirho.propagate_chirho();

        assert_eq!(net_chirho.get_exact_chirho(c_chirho), Some(7.0));
    }

    #[test]
    fn test_arena_subtractor_backward_chirho() {
        let mut net_chirho = ArenaNetworkChirho::new_chirho();

        let a_chirho = net_chirho.make_cell_chirho();
        let b_chirho = net_chirho.make_cell_chirho();
        let c_chirho = net_chirho.make_cell_chirho();

        net_chirho.add_subtractor_chirho(a_chirho, b_chirho, c_chirho);

        net_chirho.set_exact_chirho(a_chirho, 10.0);
        net_chirho.set_exact_chirho(c_chirho, 7.0);
        net_chirho.propagate_chirho();

        assert_eq!(net_chirho.get_exact_chirho(b_chirho), Some(3.0));
    }

    #[test]
    fn test_arena_divider_chirho() {
        let mut net_chirho = ArenaNetworkChirho::new_chirho();

        let a_chirho = net_chirho.make_cell_chirho();
        let b_chirho = net_chirho.make_cell_chirho();
        let c_chirho = net_chirho.make_cell_chirho();

        net_chirho.add_divider_chirho(a_chirho, b_chirho, c_chirho);

        net_chirho.set_exact_chirho(a_chirho, 12.0);
        net_chirho.set_exact_chirho(b_chirho, 4.0);
        net_chirho.propagate_chirho();

        assert_eq!(net_chirho.get_exact_chirho(c_chirho), Some(3.0));
    }

    #[test]
    fn test_arena_sqrter_chirho() {
        let mut net_chirho = ArenaNetworkChirho::new_chirho();

        let a_chirho = net_chirho.make_cell_chirho();
        let b_chirho = net_chirho.make_cell_chirho();

        net_chirho.add_sqrter_chirho(a_chirho, b_chirho);

        net_chirho.set_exact_chirho(a_chirho, 25.0);
        net_chirho.propagate_chirho();

        assert_eq!(net_chirho.get_exact_chirho(b_chirho), Some(5.0));
    }

    #[test]
    fn test_arena_sqrter_backward_chirho() {
        let mut net_chirho = ArenaNetworkChirho::new_chirho();

        let a_chirho = net_chirho.make_cell_chirho();
        let b_chirho = net_chirho.make_cell_chirho();

        net_chirho.add_sqrter_chirho(a_chirho, b_chirho);

        net_chirho.set_exact_chirho(b_chirho, 5.0);
        net_chirho.propagate_chirho();

        assert_eq!(net_chirho.get_exact_chirho(a_chirho), Some(25.0));
    }

    #[test]
    fn test_arena_negater_chirho() {
        let mut net_chirho = ArenaNetworkChirho::new_chirho();

        let a_chirho = net_chirho.make_cell_chirho();
        let b_chirho = net_chirho.make_cell_chirho();

        net_chirho.add_negater_chirho(a_chirho, b_chirho);

        net_chirho.set_exact_chirho(a_chirho, 5.0);
        net_chirho.propagate_chirho();

        assert_eq!(net_chirho.get_exact_chirho(b_chirho), Some(-5.0));
    }

    #[test]
    fn test_arena_negater_backward_chirho() {
        let mut net_chirho = ArenaNetworkChirho::new_chirho();

        let a_chirho = net_chirho.make_cell_chirho();
        let b_chirho = net_chirho.make_cell_chirho();

        net_chirho.add_negater_chirho(a_chirho, b_chirho);

        net_chirho.set_exact_chirho(b_chirho, -5.0);
        net_chirho.propagate_chirho();

        assert_eq!(net_chirho.get_exact_chirho(a_chirho), Some(5.0));
    }

    #[test]
    fn test_arena_absoluter_chirho() {
        let mut net_chirho = ArenaNetworkChirho::new_chirho();

        let a_chirho = net_chirho.make_cell_chirho();
        let b_chirho = net_chirho.make_cell_chirho();

        net_chirho.add_absoluter_chirho(a_chirho, b_chirho);

        net_chirho.set_exact_chirho(a_chirho, -5.0);
        net_chirho.propagate_chirho();

        assert_eq!(net_chirho.get_exact_chirho(b_chirho), Some(5.0));
    }
}
