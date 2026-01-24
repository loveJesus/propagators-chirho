// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Generic cells that work with any lattice type.
//!
//! This module provides cells that can hold any type implementing
//! `BoundedLatticeChirho`, not just `NumericInfoChirho`.
//!
//! # Example
//!
//! ```
//! use propagators_chirho::generic_cell_chirho::GenericCellChirho;
//! use propagators_chirho::finite_domain_chirho::FiniteDomainChirho;
//! use propagators_chirho::lattice_chirho::BoundedLatticeChirho;
//!
//! // Create a cell for finite domain values
//! let cell_chirho = GenericCellChirho::new_chirho(FiniteDomainChirho::range_chirho(1, 9));
//!
//! // Add information
//! cell_chirho.add_info_chirho(FiniteDomainChirho::all_except_chirho(5, 1, 9));
//!
//! // Get refined value
//! let value_chirho = cell_chirho.get_chirho();
//! assert!(!value_chirho.contains_chirho(5));
//! ```

use crate::lattice_chirho::BoundedLatticeChirho;
use std::cell::RefCell;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

static CELL_ID_COUNTER_CHIRHO: AtomicUsize = AtomicUsize::new(0);

/// A generic cell that can hold any lattice type.
pub struct GenericCellChirho<L: BoundedLatticeChirho> {
    id_chirho: usize,
    name_chirho: String,
    content_chirho: RefCell<L>,
    /// Callbacks to invoke when content changes.
    watchers_chirho: RefCell<Vec<Box<dyn Fn(&L) + Send + Sync>>>,
}

impl<L: BoundedLatticeChirho> GenericCellChirho<L> {
    /// Creates a new cell with an initial value.
    pub fn new_chirho(initial_chirho: L) -> Self {
        let id_chirho = CELL_ID_COUNTER_CHIRHO.fetch_add(1, Ordering::SeqCst);
        Self {
            id_chirho,
            name_chirho: format!("cell_{}", id_chirho),
            content_chirho: RefCell::new(initial_chirho),
            watchers_chirho: RefCell::new(Vec::new()),
        }
    }

    /// Creates a new cell with a name and initial value.
    pub fn with_name_chirho(name_chirho: &str, initial_chirho: L) -> Self {
        let id_chirho = CELL_ID_COUNTER_CHIRHO.fetch_add(1, Ordering::SeqCst);
        Self {
            id_chirho,
            name_chirho: name_chirho.to_string(),
            content_chirho: RefCell::new(initial_chirho),
            watchers_chirho: RefCell::new(Vec::new()),
        }
    }

    /// Creates a new cell at bottom (no information).
    pub fn bottom_chirho() -> Self {
        Self::new_chirho(L::bottom_chirho())
    }

    /// Creates a new named cell at bottom.
    pub fn bottom_with_name_chirho(name_chirho: &str) -> Self {
        Self::with_name_chirho(name_chirho, L::bottom_chirho())
    }

    /// Returns the cell's unique ID.
    pub fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    /// Returns the cell's name.
    pub fn name_chirho(&self) -> &str {
        &self.name_chirho
    }

    /// Gets the current content.
    pub fn get_chirho(&self) -> L {
        self.content_chirho.borrow().clone()
    }

    /// Adds information to the cell (joins with current content).
    ///
    /// Returns true if the content changed.
    pub fn add_info_chirho(&self, info_chirho: L) -> bool {
        let old_chirho = self.content_chirho.borrow().clone();
        let new_chirho = old_chirho.join_chirho(&info_chirho);

        if new_chirho != old_chirho {
            *self.content_chirho.borrow_mut() = new_chirho.clone();

            // Notify watchers
            for watcher_chirho in self.watchers_chirho.borrow().iter() {
                watcher_chirho(&new_chirho);
            }

            true
        } else {
            false
        }
    }

    /// Sets the content directly (replacing, not joining).
    pub fn set_chirho(&self, value_chirho: L) {
        let old_chirho = self.content_chirho.borrow().clone();
        if value_chirho != old_chirho {
            *self.content_chirho.borrow_mut() = value_chirho.clone();

            for watcher_chirho in self.watchers_chirho.borrow().iter() {
                watcher_chirho(&value_chirho);
            }
        }
    }

    /// Returns true if the cell is at bottom (no information).
    pub fn is_bottom_chirho(&self) -> bool {
        self.content_chirho.borrow().is_bottom_chirho()
    }

    /// Returns true if the cell is at top (contradiction).
    pub fn is_top_chirho(&self) -> bool {
        self.content_chirho.borrow().is_top_chirho()
    }

    /// Adds a watcher callback that's invoked when content changes.
    pub fn watch_chirho<F>(&self, callback_chirho: F)
    where
        F: Fn(&L) + Send + Sync + 'static,
    {
        self.watchers_chirho
            .borrow_mut()
            .push(Box::new(callback_chirho));
    }

    /// Returns the number of watchers.
    pub fn watcher_count_chirho(&self) -> usize {
        self.watchers_chirho.borrow().len()
    }
}

impl<L: BoundedLatticeChirho> fmt::Debug for GenericCellChirho<L> {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        f_chirho
            .debug_struct("GenericCellChirho")
            .field("id", &self.id_chirho)
            .field("name", &self.name_chirho)
            .field("content", &self.content_chirho.borrow())
            .finish()
    }
}

// ============================================================================
// GENERIC PROPAGATOR NETWORK
// ============================================================================

use crate::lattice_chirho::PropagatorFnChirho;
use std::collections::VecDeque;
use std::rc::Rc;

/// Type alias for a propagator function stored in the network.
///
/// We use a boxed closure instead of a trait object because
/// `PropagatorFnChirho` requires `Clone` which makes it not dyn-compatible.
type PropagatorBoxChirho<L> = Box<dyn Fn(&[L]) -> Vec<L>>;

/// A generic propagator network that works with any lattice type.
pub struct GenericNetworkChirho<L: BoundedLatticeChirho> {
    cells_chirho: Vec<Rc<GenericCellChirho<L>>>,
    propagators_chirho: Vec<(PropagatorBoxChirho<L>, Vec<usize>)>,
    queue_chirho: RefCell<VecDeque<usize>>,
    propagation_count_chirho: RefCell<usize>,
}

impl<L: BoundedLatticeChirho + 'static> GenericNetworkChirho<L> {
    /// Creates a new empty network.
    pub fn new_chirho() -> Self {
        Self {
            cells_chirho: Vec::new(),
            propagators_chirho: Vec::new(),
            queue_chirho: RefCell::new(VecDeque::new()),
            propagation_count_chirho: RefCell::new(0),
        }
    }

    /// Adds a cell to the network.
    pub fn add_cell_chirho(&mut self, cell_chirho: Rc<GenericCellChirho<L>>) -> usize {
        let idx_chirho = self.cells_chirho.len();
        self.cells_chirho.push(cell_chirho);
        idx_chirho
    }

    /// Creates and adds a new cell at bottom.
    pub fn make_cell_chirho(&mut self) -> usize {
        self.add_cell_chirho(Rc::new(GenericCellChirho::bottom_chirho()))
    }

    /// Creates and adds a new named cell at bottom.
    pub fn make_named_cell_chirho(&mut self, name_chirho: &str) -> usize {
        self.add_cell_chirho(Rc::new(GenericCellChirho::bottom_with_name_chirho(
            name_chirho,
        )))
    }

    /// Gets a cell by index.
    pub fn get_cell_chirho(&self, idx_chirho: usize) -> Option<&Rc<GenericCellChirho<L>>> {
        self.cells_chirho.get(idx_chirho)
    }

    /// Adds a propagator to the network.
    ///
    /// The propagator is converted to a closure internally to work around
    /// dyn-compatibility requirements.
    pub fn add_propagator_chirho<P: PropagatorFnChirho<L> + 'static>(
        &mut self,
        propagator_chirho: P,
        cell_indices_chirho: Vec<usize>,
    ) {
        let prop_idx_chirho = self.propagators_chirho.len();

        // Convert PropagatorFnChirho to a closure
        let closure_chirho: PropagatorBoxChirho<L> =
            Box::new(move |cells_chirho: &[L]| propagator_chirho.propagate_chirho(cells_chirho));

        self.propagators_chirho
            .push((closure_chirho, cell_indices_chirho));

        // Queue the propagator initially
        self.queue_chirho.borrow_mut().push_back(prop_idx_chirho);
    }

    /// Adds a closure-based propagator to the network.
    ///
    /// This is useful for ad-hoc propagators that don't implement
    /// `PropagatorFnChirho`.
    pub fn add_closure_propagator_chirho<F>(
        &mut self,
        propagator_fn_chirho: F,
        cell_indices_chirho: Vec<usize>,
    ) where
        F: Fn(&[L]) -> Vec<L> + 'static,
    {
        let prop_idx_chirho = self.propagators_chirho.len();
        self.propagators_chirho
            .push((Box::new(propagator_fn_chirho), cell_indices_chirho));

        // Queue the propagator initially
        self.queue_chirho.borrow_mut().push_back(prop_idx_chirho);
    }

    /// Sets a cell's value and queues affected propagators.
    pub fn set_cell_chirho(&mut self, cell_idx_chirho: usize, value_chirho: L) {
        if let Some(cell_chirho) = self.cells_chirho.get(cell_idx_chirho) {
            let changed_chirho = cell_chirho.add_info_chirho(value_chirho);
            if changed_chirho {
                self.queue_affected_chirho(cell_idx_chirho);
            }
        }
    }

    fn queue_affected_chirho(&self, cell_idx_chirho: usize) {
        for (prop_idx_chirho, (_, cell_indices_chirho)) in self.propagators_chirho.iter().enumerate()
        {
            if cell_indices_chirho.contains(&cell_idx_chirho) {
                let mut queue_chirho = self.queue_chirho.borrow_mut();
                if !queue_chirho.contains(&prop_idx_chirho) {
                    queue_chirho.push_back(prop_idx_chirho);
                }
            }
        }
    }

    /// Runs propagation to fixpoint.
    pub fn propagate_chirho(&mut self) {
        loop {
            // Pop from queue in its own scope to release borrow
            let prop_idx_opt_chirho = self.queue_chirho.borrow_mut().pop_front();

            let prop_idx_chirho = match prop_idx_opt_chirho {
                Some(idx_chirho) => idx_chirho,
                None => break,
            };

            *self.propagation_count_chirho.borrow_mut() += 1;

            // Get propagator info (clone indices to avoid borrow issues)
            let cell_indices_chirho = self.propagators_chirho[prop_idx_chirho].1.clone();

            // Get current cell values
            let inputs_chirho: Vec<L> = cell_indices_chirho
                .iter()
                .map(|idx_chirho| self.cells_chirho[*idx_chirho].get_chirho())
                .collect();

            // Run propagator
            let outputs_chirho = (self.propagators_chirho[prop_idx_chirho].0)(&inputs_chirho);

            // Apply outputs
            for (i_chirho, output_chirho) in outputs_chirho.into_iter().enumerate() {
                if !output_chirho.is_bottom_chirho() {
                    let cell_idx_chirho = cell_indices_chirho[i_chirho];
                    let cell_chirho = &self.cells_chirho[cell_idx_chirho];
                    let changed_chirho = cell_chirho.add_info_chirho(output_chirho);
                    if changed_chirho {
                        self.queue_affected_chirho(cell_idx_chirho);
                    }
                }
            }
        }
    }

    /// Returns the number of propagations performed.
    pub fn propagation_count_chirho(&self) -> usize {
        *self.propagation_count_chirho.borrow()
    }

    /// Returns true if any cell is in contradiction.
    pub fn has_contradiction_chirho(&self) -> bool {
        self.cells_chirho
            .iter()
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
}

impl<L: BoundedLatticeChirho + 'static> Default for GenericNetworkChirho<L> {
    fn default() -> Self {
        Self::new_chirho()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::finite_domain_chirho::{FiniteDomainChirho, NotEqualsChirho};
    use crate::interval_chirho::NumericInfoChirho;
    use crate::lattice_chirho::AddPropagatorChirho;

    #[test]
    fn test_generic_cell_creation_chirho() {
        let cell_chirho: GenericCellChirho<NumericInfoChirho> = GenericCellChirho::bottom_chirho();
        assert!(cell_chirho.is_bottom_chirho());
    }

    #[test]
    fn test_generic_cell_add_info_chirho() {
        let cell_chirho = GenericCellChirho::new_chirho(NumericInfoChirho::interval_chirho(0.0, 100.0));

        let changed_chirho =
            cell_chirho.add_info_chirho(NumericInfoChirho::interval_chirho(50.0, 150.0));
        assert!(changed_chirho);

        // Should be intersection: [50, 100]
        if let NumericInfoChirho::IntervalChirho(iv_chirho) = cell_chirho.get_chirho() {
            assert_eq!(iv_chirho.lo_chirho, 50.0);
            assert_eq!(iv_chirho.hi_chirho, 100.0);
        }
    }

    #[test]
    fn test_generic_cell_finite_domain_chirho() {
        let cell_chirho =
            GenericCellChirho::new_chirho(FiniteDomainChirho::range_chirho(1, 9));

        cell_chirho.add_info_chirho(FiniteDomainChirho::all_except_chirho(5, 1, 9));

        let value_chirho = cell_chirho.get_chirho();
        assert!(!value_chirho.contains_chirho(5));
        assert!(value_chirho.contains_chirho(1));
    }

    #[test]
    fn test_generic_network_numeric_chirho() {
        let mut network_chirho: GenericNetworkChirho<NumericInfoChirho> =
            GenericNetworkChirho::new_chirho();

        let a_chirho = network_chirho.make_cell_chirho();
        let b_chirho = network_chirho.make_cell_chirho();
        let c_chirho = network_chirho.make_cell_chirho();

        network_chirho.add_propagator_chirho(
            AddPropagatorChirho,
            vec![a_chirho, b_chirho, c_chirho],
        );

        network_chirho.set_cell_chirho(a_chirho, NumericInfoChirho::exact_chirho(3.0));
        network_chirho.set_cell_chirho(b_chirho, NumericInfoChirho::exact_chirho(4.0));

        network_chirho.propagate_chirho();

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
    fn test_generic_network_finite_domain_chirho() {
        let mut network_chirho: GenericNetworkChirho<FiniteDomainChirho> =
            GenericNetworkChirho::new_chirho();

        // Three cells that must all be different
        let cell_a_chirho = network_chirho.make_cell_chirho();
        let cell_b_chirho = network_chirho.make_cell_chirho();
        let cell_c_chirho = network_chirho.make_cell_chirho();

        // Initialize with domain {1, 2, 3}
        network_chirho.set_cell_chirho(cell_a_chirho, FiniteDomainChirho::range_chirho(1, 3));
        network_chirho.set_cell_chirho(cell_b_chirho, FiniteDomainChirho::range_chirho(1, 3));
        network_chirho.set_cell_chirho(cell_c_chirho, FiniteDomainChirho::range_chirho(1, 3));

        // Add not-equals constraints
        network_chirho.add_propagator_chirho(
            NotEqualsChirho,
            vec![cell_a_chirho, cell_b_chirho],
        );
        network_chirho.add_propagator_chirho(
            NotEqualsChirho,
            vec![cell_b_chirho, cell_c_chirho],
        );
        network_chirho.add_propagator_chirho(
            NotEqualsChirho,
            vec![cell_a_chirho, cell_c_chirho],
        );

        // Fix cell_a to 1
        network_chirho.set_cell_chirho(cell_a_chirho, FiniteDomainChirho::singleton_chirho(1));

        network_chirho.propagate_chirho();

        // cell_b and cell_c should not contain 1
        if let Some(cell_chirho) = network_chirho.get_cell_chirho(cell_b_chirho) {
            assert!(!cell_chirho.get_chirho().contains_chirho(1));
        }
        if let Some(cell_chirho) = network_chirho.get_cell_chirho(cell_c_chirho) {
            assert!(!cell_chirho.get_chirho().contains_chirho(1));
        }
    }

    #[test]
    fn test_cell_watcher_chirho() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let called_chirho = Arc::new(AtomicBool::new(false));
        let called_clone_chirho = called_chirho.clone();

        let cell_chirho =
            GenericCellChirho::new_chirho(NumericInfoChirho::interval_chirho(0.0, 100.0));

        cell_chirho.watch_chirho(move |_value_chirho| {
            called_clone_chirho.store(true, Ordering::SeqCst);
        });

        cell_chirho.add_info_chirho(NumericInfoChirho::interval_chirho(50.0, 75.0));

        assert!(called_chirho.load(Ordering::SeqCst));
    }
}
