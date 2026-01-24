// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Parallel propagation using rayon.
//!
//! This module provides parallel variants of the propagator network that
//! use rayon for concurrent propagation of independent propagators.
//!
//! # When to Use Parallel Propagation
//!
//! Parallel propagation adds overhead from thread synchronization. It's only
//! beneficial when:
//!
//! 1. **Large networks** - Thousands of cells and propagators
//! 2. **Expensive propagators** - Each propagator does significant work (ms, not µs)
//! 3. **High independence** - Many propagators operate on disjoint cell sets
//!
//! For small networks or simple interval arithmetic, sequential propagation
//! (via `propagate_sequential_chirho()`) is typically faster.
//!
//! # Feature Flag
//!
//! This module requires the `parallel` feature:
//!
//! ```toml
//! [dependencies]
//! propagators-chirho = { version = "0.1", features = ["parallel"] }
//! ```
//!
//! # Example
//!
//! ```
//! # #[cfg(feature = "parallel")]
//! use propagators_chirho::parallel_chirho::{ParallelNetworkChirho, NumericParallelNetworkChirho};
//! # #[cfg(feature = "parallel")]
//! use propagators_chirho::NumericInfoChirho;
//!
//! # #[cfg(feature = "parallel")]
//! # fn main() {
//! // Use the NumericParallelNetworkChirho type alias for numeric values
//! let mut network_chirho = NumericParallelNetworkChirho::new_chirho();
//!
//! // Add cells and propagators as usual
//! let a_chirho = network_chirho.make_cell_chirho();
//! let b_chirho = network_chirho.make_cell_chirho();
//!
//! // Propagators run in parallel when possible
//! network_chirho.propagate_parallel_chirho();
//! # }
//! # #[cfg(not(feature = "parallel"))]
//! # fn main() {}
//! ```

use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use crate::interval_chirho::NumericInfoChirho;
use crate::lattice_chirho::{BoundedLatticeChirho, PropagatorFnChirho};

static PARALLEL_CELL_ID_COUNTER_CHIRHO: AtomicUsize = AtomicUsize::new(0);

/// A thread-safe cell for parallel propagation.
pub struct ParallelCellChirho<L: BoundedLatticeChirho + Send + Sync> {
    id_chirho: usize,
    name_chirho: String,
    content_chirho: RwLock<L>,
}

impl<L: BoundedLatticeChirho + Send + Sync> ParallelCellChirho<L> {
    /// Creates a new cell with an initial value.
    pub fn new_chirho(initial_chirho: L) -> Self {
        let id_chirho = PARALLEL_CELL_ID_COUNTER_CHIRHO.fetch_add(1, Ordering::SeqCst);
        Self {
            id_chirho,
            name_chirho: format!("parallel_cell_{}", id_chirho),
            content_chirho: RwLock::new(initial_chirho),
        }
    }

    /// Creates a new named cell.
    pub fn with_name_chirho(name_chirho: &str, initial_chirho: L) -> Self {
        let id_chirho = PARALLEL_CELL_ID_COUNTER_CHIRHO.fetch_add(1, Ordering::SeqCst);
        Self {
            id_chirho,
            name_chirho: name_chirho.to_string(),
            content_chirho: RwLock::new(initial_chirho),
        }
    }

    /// Creates a cell at bottom (no information).
    pub fn bottom_chirho() -> Self {
        Self::new_chirho(L::bottom_chirho())
    }

    /// Creates a named cell at bottom.
    pub fn bottom_with_name_chirho(name_chirho: &str) -> Self {
        Self::with_name_chirho(name_chirho, L::bottom_chirho())
    }

    /// Returns the cell's ID.
    pub fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    /// Returns the cell's name.
    pub fn name_chirho(&self) -> &str {
        &self.name_chirho
    }

    /// Gets the current content (thread-safe read).
    pub fn get_chirho(&self) -> L {
        self.content_chirho.read().unwrap().clone()
    }

    /// Adds information by joining (thread-safe).
    ///
    /// Returns true if the content changed.
    pub fn add_info_chirho(&self, info_chirho: L) -> bool {
        let mut content_chirho = self.content_chirho.write().unwrap();
        let old_chirho = content_chirho.clone();
        let new_chirho = old_chirho.join_chirho(&info_chirho);

        if new_chirho == old_chirho {
            false
        } else {
            *content_chirho = new_chirho;
            true
        }
    }

    /// Returns true if at bottom.
    pub fn is_bottom_chirho(&self) -> bool {
        self.content_chirho.read().unwrap().is_bottom_chirho()
    }

    /// Returns true if at top (contradiction).
    pub fn is_top_chirho(&self) -> bool {
        self.content_chirho.read().unwrap().is_top_chirho()
    }
}

impl<L: BoundedLatticeChirho + Send + Sync> std::fmt::Debug for ParallelCellChirho<L> {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f_chirho
            .debug_struct("ParallelCellChirho")
            .field("id", &self.id_chirho)
            .field("name", &self.name_chirho)
            .field("content", &*self.content_chirho.read().unwrap())
            .finish()
    }
}

/// A propagator entry for parallel networks.
#[allow(clippy::type_complexity)] // Thread-safe propagator callbacks require this signature
struct ParallelPropagatorChirho<L: BoundedLatticeChirho + Send + Sync> {
    /// The propagator function (wrapped for thread safety).
    propagator_fn_chirho: Box<dyn Fn(&[L]) -> Vec<L> + Send + Sync>,
    /// Cell indices this propagator reads/writes.
    cell_indices_chirho: Vec<usize>,
}

/// A parallel propagator network.
///
/// This network runs propagators in parallel using rayon when they
/// operate on independent cells.
pub struct ParallelNetworkChirho<L: BoundedLatticeChirho + Send + Sync + 'static> {
    cells_chirho: Vec<Arc<ParallelCellChirho<L>>>,
    propagators_chirho: Vec<ParallelPropagatorChirho<L>>,
    queue_chirho: Mutex<VecDeque<usize>>,
    propagation_count_chirho: AtomicUsize,
}

impl<L: BoundedLatticeChirho + Send + Sync + 'static> ParallelNetworkChirho<L> {
    /// Creates a new empty parallel network.
    pub fn new_chirho() -> Self {
        Self {
            cells_chirho: Vec::new(),
            propagators_chirho: Vec::new(),
            queue_chirho: Mutex::new(VecDeque::new()),
            propagation_count_chirho: AtomicUsize::new(0),
        }
    }

    /// Adds a cell to the network.
    pub fn add_cell_chirho(&mut self, cell_chirho: Arc<ParallelCellChirho<L>>) -> usize {
        let idx_chirho = self.cells_chirho.len();
        self.cells_chirho.push(cell_chirho);
        idx_chirho
    }

    /// Creates and adds a new cell at bottom.
    pub fn make_cell_chirho(&mut self) -> usize {
        self.add_cell_chirho(Arc::new(ParallelCellChirho::bottom_chirho()))
    }

    /// Creates and adds a new named cell.
    pub fn make_named_cell_chirho(&mut self, name_chirho: &str) -> usize {
        self.add_cell_chirho(Arc::new(ParallelCellChirho::bottom_with_name_chirho(
            name_chirho,
        )))
    }

    /// Gets a cell by index.
    pub fn get_cell_chirho(&self, idx_chirho: usize) -> Option<&Arc<ParallelCellChirho<L>>> {
        self.cells_chirho.get(idx_chirho)
    }

    /// Adds a propagator to the network.
    pub fn add_propagator_chirho<F>(
        &mut self,
        propagator_fn_chirho: F,
        cell_indices_chirho: Vec<usize>,
    ) where
        F: Fn(&[L]) -> Vec<L> + Send + Sync + 'static,
    {
        let prop_idx_chirho = self.propagators_chirho.len();

        self.propagators_chirho.push(ParallelPropagatorChirho {
            propagator_fn_chirho: Box::new(propagator_fn_chirho),
            cell_indices_chirho,
        });

        // Queue initially
        self.queue_chirho.lock().unwrap().push_back(prop_idx_chirho);
    }

    /// Adds a trait-based propagator.
    pub fn add_trait_propagator_chirho<P>(
        &mut self,
        propagator_chirho: P,
        cell_indices_chirho: Vec<usize>,
    ) where
        P: PropagatorFnChirho<L> + Send + Sync + 'static,
    {
        self.add_propagator_chirho(
            move |cells_chirho: &[L]| propagator_chirho.propagate_chirho(cells_chirho),
            cell_indices_chirho,
        );
    }

    /// Sets a cell's value.
    pub fn set_cell_chirho(&self, cell_idx_chirho: usize, value_chirho: L) {
        if let Some(cell_chirho) = self.cells_chirho.get(cell_idx_chirho) {
            let changed_chirho = cell_chirho.add_info_chirho(value_chirho);
            if changed_chirho {
                self.queue_affected_chirho(cell_idx_chirho);
            }
        }
    }

    fn queue_affected_chirho(&self, cell_idx_chirho: usize) {
        let mut queue_chirho = self.queue_chirho.lock().unwrap();
        for (prop_idx_chirho, prop_chirho) in self.propagators_chirho.iter().enumerate() {
            if prop_chirho.cell_indices_chirho.contains(&cell_idx_chirho)
                && !queue_chirho
                    .iter()
                    .any(|&idx_chirho| idx_chirho == prop_idx_chirho)
            {
                queue_chirho.push_back(prop_idx_chirho);
            }
        }
    }

    /// Runs propagation sequentially (for comparison/debugging).
    pub fn propagate_sequential_chirho(&self) {
        loop {
            let prop_idx_opt_chirho = self.queue_chirho.lock().unwrap().pop_front();

            let Some(prop_idx_chirho) = prop_idx_opt_chirho else {
                break;
            };

            self.propagation_count_chirho.fetch_add(1, Ordering::SeqCst);

            let prop_chirho = &self.propagators_chirho[prop_idx_chirho];

            // Get inputs
            let inputs_chirho: Vec<L> = prop_chirho
                .cell_indices_chirho
                .iter()
                .map(|&idx_chirho| self.cells_chirho[idx_chirho].get_chirho())
                .collect();

            // Run propagator
            let outputs_chirho = (prop_chirho.propagator_fn_chirho)(&inputs_chirho);

            // Apply outputs
            for (i_chirho, output_chirho) in outputs_chirho.into_iter().enumerate() {
                if !output_chirho.is_bottom_chirho() {
                    let cell_idx_chirho = prop_chirho.cell_indices_chirho[i_chirho];
                    let changed_chirho =
                        self.cells_chirho[cell_idx_chirho].add_info_chirho(output_chirho);
                    if changed_chirho {
                        self.queue_affected_chirho(cell_idx_chirho);
                    }
                }
            }
        }
    }

    /// Runs propagation in parallel using rayon.
    ///
    /// Independent propagators (those operating on non-overlapping cells)
    /// run concurrently.
    pub fn propagate_parallel_chirho(&self) {
        loop {
            // Collect a batch of independent propagators
            let batch_chirho = self.collect_independent_batch_chirho();

            if batch_chirho.is_empty() {
                break;
            }

            // Run batch in parallel
            let results_chirho: Vec<(usize, Vec<L>)> = batch_chirho
                .par_iter()
                .map(|&prop_idx_chirho| {
                    self.propagation_count_chirho.fetch_add(1, Ordering::SeqCst);

                    let prop_chirho = &self.propagators_chirho[prop_idx_chirho];

                    // Get inputs
                    let inputs_chirho: Vec<L> = prop_chirho
                        .cell_indices_chirho
                        .iter()
                        .map(|&idx_chirho| self.cells_chirho[idx_chirho].get_chirho())
                        .collect();

                    // Run propagator
                    let outputs_chirho = (prop_chirho.propagator_fn_chirho)(&inputs_chirho);

                    (prop_idx_chirho, outputs_chirho)
                })
                .collect();

            // Apply results sequentially to maintain consistency
            for (prop_idx_chirho, outputs_chirho) in results_chirho {
                let prop_chirho = &self.propagators_chirho[prop_idx_chirho];

                for (i_chirho, output_chirho) in outputs_chirho.into_iter().enumerate() {
                    if !output_chirho.is_bottom_chirho() {
                        let cell_idx_chirho = prop_chirho.cell_indices_chirho[i_chirho];
                        let changed_chirho =
                            self.cells_chirho[cell_idx_chirho].add_info_chirho(output_chirho);
                        if changed_chirho {
                            self.queue_affected_chirho(cell_idx_chirho);
                        }
                    }
                }
            }
        }
    }

    /// Collects a batch of propagators that operate on independent cells.
    ///
    /// Optimized version using a cell-to-propagator index for O(1) overlap checking.
    fn collect_independent_batch_chirho(&self) -> Vec<usize> {
        let mut queue_chirho = self.queue_chirho.lock().unwrap();

        if queue_chirho.is_empty() {
            return Vec::new();
        }

        // For small queues, use simple algorithm
        if queue_chirho.len() <= 16 {
            return self.collect_batch_simple_chirho(&mut queue_chirho);
        }

        // For larger queues, use optimized algorithm
        self.collect_batch_optimized_chirho(&mut queue_chirho)
    }

    /// Simple batch collection for small queues.
    fn collect_batch_simple_chirho(&self, queue_chirho: &mut VecDeque<usize>) -> Vec<usize> {
        let mut batch_chirho: Vec<usize> = Vec::new();
        let mut used_cells_chirho: HashSet<usize> = HashSet::new();
        let mut remaining_chirho: VecDeque<usize> = VecDeque::new();

        while let Some(prop_idx_chirho) = queue_chirho.pop_front() {
            let prop_chirho = &self.propagators_chirho[prop_idx_chirho];

            let overlaps_chirho = prop_chirho
                .cell_indices_chirho
                .iter()
                .any(|idx_chirho| used_cells_chirho.contains(idx_chirho));

            if overlaps_chirho {
                remaining_chirho.push_back(prop_idx_chirho);
            } else {
                batch_chirho.push(prop_idx_chirho);
                used_cells_chirho.extend(prop_chirho.cell_indices_chirho.iter().cloned());
            }
        }

        *queue_chirho = remaining_chirho;
        batch_chirho
    }

    /// Optimized batch collection using greedy coloring approach.
    fn collect_batch_optimized_chirho(&self, queue_chirho: &mut VecDeque<usize>) -> Vec<usize> {
        // Build a quick lookup: which propagators touch each cell
        let queue_vec_chirho: Vec<usize> = queue_chirho.drain(..).collect();
        let queue_len_chirho = queue_vec_chirho.len();

        // Track which cells are "taken" by batch propagators
        let mut cell_taken_chirho: HashSet<usize> = HashSet::with_capacity(queue_len_chirho * 3);
        let mut batch_chirho: Vec<usize> = Vec::with_capacity(queue_len_chirho / 2);
        let mut remaining_chirho: Vec<usize> = Vec::with_capacity(queue_len_chirho / 2);

        for prop_idx_chirho in queue_vec_chirho {
            let prop_chirho = &self.propagators_chirho[prop_idx_chirho];

            // Check overlap using the taken set
            let overlaps_chirho = prop_chirho
                .cell_indices_chirho
                .iter()
                .any(|idx_chirho| cell_taken_chirho.contains(idx_chirho));

            if overlaps_chirho {
                remaining_chirho.push(prop_idx_chirho);
            } else {
                // Mark cells as taken
                for &cell_idx_chirho in &prop_chirho.cell_indices_chirho {
                    cell_taken_chirho.insert(cell_idx_chirho);
                }
                batch_chirho.push(prop_idx_chirho);
            }
        }

        // Rebuild queue from remaining
        *queue_chirho = remaining_chirho.into_iter().collect();
        batch_chirho
    }

    /// Returns the number of propagations performed.
    pub fn propagation_count_chirho(&self) -> usize {
        self.propagation_count_chirho.load(Ordering::SeqCst)
    }

    /// Returns true if any cell is in contradiction.
    pub fn has_contradiction_chirho(&self) -> bool {
        self.cells_chirho
            .par_iter()
            .any(|cell_chirho| cell_chirho.is_top_chirho())
    }

    /// Returns the number of cells.
    pub fn cell_count_chirho(&self) -> usize {
        self.cells_chirho.len()
    }

    /// Returns the number of propagators.
    pub fn propagator_count_chirho(&self) -> usize {
        self.propagators_chirho.len()
    }

    /// Resets the propagation counter.
    pub fn reset_counter_chirho(&self) {
        self.propagation_count_chirho.store(0, Ordering::SeqCst);
    }
}

impl<L: BoundedLatticeChirho + Send + Sync + 'static> Default for ParallelNetworkChirho<L> {
    fn default() -> Self {
        Self::new_chirho()
    }
}

// ============================================================================
// SPECIALIZED IMPLEMENTATION FOR NUMERIC INFO
// ============================================================================

/// Type alias for numeric parallel network.
pub type NumericParallelNetworkChirho = ParallelNetworkChirho<NumericInfoChirho>;

impl NumericParallelNetworkChirho {
    /// Adds an interval adder propagator: a + b = c
    pub fn add_adder_chirho(&mut self, a_chirho: usize, b_chirho: usize, c_chirho: usize) {
        use crate::lattice_chirho::AddPropagatorChirho;
        self.add_trait_propagator_chirho(AddPropagatorChirho, vec![a_chirho, b_chirho, c_chirho]);
    }

    /// Adds a multiplier propagator: a * b = c
    pub fn add_multiplier_chirho(&mut self, a_chirho: usize, b_chirho: usize, c_chirho: usize) {
        use crate::lattice_chirho::MulPropagatorChirho;
        self.add_trait_propagator_chirho(MulPropagatorChirho, vec![a_chirho, b_chirho, c_chirho]);
    }

    /// Adds a squarer propagator: a² = b
    pub fn add_squarer_chirho(&mut self, a_chirho: usize, b_chirho: usize) {
        use crate::lattice_chirho::SquarePropagatorChirho;
        self.add_trait_propagator_chirho(SquarePropagatorChirho, vec![a_chirho, b_chirho]);
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::interval_chirho::NumericInfoChirho;

    #[test]
    fn test_parallel_cell_chirho() {
        let cell_chirho: ParallelCellChirho<NumericInfoChirho> =
            ParallelCellChirho::bottom_chirho();
        assert!(cell_chirho.is_bottom_chirho());

        let changed_chirho = cell_chirho.add_info_chirho(NumericInfoChirho::exact_chirho(5.0));
        assert!(changed_chirho);

        if let NumericInfoChirho::IntervalChirho(iv_chirho) = cell_chirho.get_chirho() {
            assert_eq!(iv_chirho.lo_chirho, 5.0);
            assert_eq!(iv_chirho.hi_chirho, 5.0);
        } else {
            panic!("Expected interval");
        }
    }

    #[test]
    fn test_parallel_network_sequential_chirho() {
        let mut network_chirho = NumericParallelNetworkChirho::new_chirho();

        let a_chirho = network_chirho.make_cell_chirho();
        let b_chirho = network_chirho.make_cell_chirho();
        let c_chirho = network_chirho.make_cell_chirho();

        network_chirho.add_adder_chirho(a_chirho, b_chirho, c_chirho);

        network_chirho.set_cell_chirho(a_chirho, NumericInfoChirho::exact_chirho(3.0));
        network_chirho.set_cell_chirho(b_chirho, NumericInfoChirho::exact_chirho(4.0));

        network_chirho.propagate_sequential_chirho();

        if let Some(cell_chirho) = network_chirho.get_cell_chirho(c_chirho) {
            if let NumericInfoChirho::IntervalChirho(iv_chirho) = cell_chirho.get_chirho() {
                assert_eq!(iv_chirho.lo_chirho, 7.0);
                assert_eq!(iv_chirho.hi_chirho, 7.0);
            } else {
                panic!("Expected interval");
            }
        }
    }

    #[test]
    fn test_parallel_network_parallel_chirho() {
        let mut network_chirho = NumericParallelNetworkChirho::new_chirho();

        let a_chirho = network_chirho.make_cell_chirho();
        let b_chirho = network_chirho.make_cell_chirho();
        let c_chirho = network_chirho.make_cell_chirho();

        network_chirho.add_adder_chirho(a_chirho, b_chirho, c_chirho);

        network_chirho.set_cell_chirho(a_chirho, NumericInfoChirho::exact_chirho(3.0));
        network_chirho.set_cell_chirho(b_chirho, NumericInfoChirho::exact_chirho(4.0));

        network_chirho.propagate_parallel_chirho();

        if let Some(cell_chirho) = network_chirho.get_cell_chirho(c_chirho) {
            if let NumericInfoChirho::IntervalChirho(iv_chirho) = cell_chirho.get_chirho() {
                assert_eq!(iv_chirho.lo_chirho, 7.0);
                assert_eq!(iv_chirho.hi_chirho, 7.0);
            } else {
                panic!("Expected interval");
            }
        }
    }

    #[test]
    fn test_parallel_independent_propagators_chirho() {
        // Create two independent constraint systems that can run in parallel
        let mut network_chirho = NumericParallelNetworkChirho::new_chirho();

        // First system: a + b = c
        let a1_chirho = network_chirho.make_cell_chirho();
        let b1_chirho = network_chirho.make_cell_chirho();
        let c1_chirho = network_chirho.make_cell_chirho();
        network_chirho.add_adder_chirho(a1_chirho, b1_chirho, c1_chirho);

        // Second system: x * y = z (independent of first)
        let x_chirho = network_chirho.make_cell_chirho();
        let y_chirho = network_chirho.make_cell_chirho();
        let z_chirho = network_chirho.make_cell_chirho();
        network_chirho.add_multiplier_chirho(x_chirho, y_chirho, z_chirho);

        // Set values for both systems
        network_chirho.set_cell_chirho(a1_chirho, NumericInfoChirho::exact_chirho(10.0));
        network_chirho.set_cell_chirho(b1_chirho, NumericInfoChirho::exact_chirho(20.0));
        network_chirho.set_cell_chirho(x_chirho, NumericInfoChirho::exact_chirho(3.0));
        network_chirho.set_cell_chirho(y_chirho, NumericInfoChirho::exact_chirho(7.0));

        // Propagate in parallel
        network_chirho.propagate_parallel_chirho();

        // Check first system
        if let Some(cell_chirho) = network_chirho.get_cell_chirho(c1_chirho) {
            if let NumericInfoChirho::IntervalChirho(iv_chirho) = cell_chirho.get_chirho() {
                assert_eq!(iv_chirho.lo_chirho, 30.0);
            }
        }

        // Check second system
        if let Some(cell_chirho) = network_chirho.get_cell_chirho(z_chirho) {
            if let NumericInfoChirho::IntervalChirho(iv_chirho) = cell_chirho.get_chirho() {
                assert_eq!(iv_chirho.lo_chirho, 21.0);
            }
        }
    }

    #[test]
    fn test_parallel_backward_propagation_chirho() {
        let mut network_chirho = NumericParallelNetworkChirho::new_chirho();

        let a_chirho = network_chirho.make_cell_chirho();
        let b_chirho = network_chirho.make_cell_chirho();
        let c_chirho = network_chirho.make_cell_chirho();

        network_chirho.add_adder_chirho(a_chirho, b_chirho, c_chirho);

        // Set a and c, propagate to find b
        network_chirho.set_cell_chirho(a_chirho, NumericInfoChirho::exact_chirho(3.0));
        network_chirho.set_cell_chirho(c_chirho, NumericInfoChirho::exact_chirho(10.0));

        network_chirho.propagate_parallel_chirho();

        // b should be 7
        if let Some(cell_chirho) = network_chirho.get_cell_chirho(b_chirho) {
            if let NumericInfoChirho::IntervalChirho(iv_chirho) = cell_chirho.get_chirho() {
                assert_eq!(iv_chirho.lo_chirho, 7.0);
                assert_eq!(iv_chirho.hi_chirho, 7.0);
            } else {
                panic!("Expected interval");
            }
        }
    }

    #[test]
    fn test_has_contradiction_parallel_chirho() {
        let mut network_chirho = NumericParallelNetworkChirho::new_chirho();

        let a_chirho = network_chirho.make_cell_chirho();

        // Set conflicting information
        network_chirho.set_cell_chirho(a_chirho, NumericInfoChirho::interval_chirho(0.0, 5.0));
        network_chirho.set_cell_chirho(a_chirho, NumericInfoChirho::interval_chirho(10.0, 20.0));

        assert!(network_chirho.has_contradiction_chirho());
    }
}
