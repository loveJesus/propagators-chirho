// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Cells for holding partial information in propagator networks.
//!
//! Cells are the fundamental storage units in a propagator network. They hold
//! partial information that can only grow monotonically—you can add information
//! but never remove it.
//!
//! # Key Properties
//!
//! 1. **Monotonicity**: Information can only increase (narrow intervals, not widen)
//! 2. **Notification**: When content changes, neighboring propagators are alerted
//! 3. **Merge semantics**: New information is merged with existing content
//!
//! # References
//!
//! - Radul, A., & Sussman, G. J. (2009). *The Art of the Propagator*, Section 3.
//!   <https://dspace.mit.edu/handle/1721.1/44215>

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::scheduler_chirho::SchedulerChirho;
use crate::core_chirho::interval_chirho::NumericInfoChirho;
use crate::propagators_chirho::propagator_chirho::PropagatorChirho;

/// Trait for types that can be merged in a propagator network.
///
/// This defines the lattice operations needed for partial information.
///
/// # Mathematical Requirements
///
/// The merge operation should satisfy:
/// - **Commutativity**: `a.merge(b) = b.merge(a)`
/// - **Associativity**: `a.merge(b.merge(c)) = a.merge(b).merge(c)`
/// - **Idempotency**: `a.merge(a) = a`
///
/// # Example
///
/// ```
/// use propagators_chirho::{MergeableChirho, NumericInfoChirho};
///
/// let a_chirho = NumericInfoChirho::interval_chirho(0.0, 10.0);
/// let b_chirho = NumericInfoChirho::interval_chirho(5.0, 15.0);
/// let merged_chirho = a_chirho.merge_chirho(&b_chirho);
/// ```
pub trait MergeableChirho: Clone + Default + fmt::Debug + PartialEq {
    /// Merges two pieces of partial information.
    ///
    /// Returns the lattice join of `self` and `other`.
    fn merge_chirho(&self, other_chirho: &Self) -> Self;

    /// Returns `true` if this is the bottom element (no information).
    fn is_nothing_chirho(&self) -> bool;

    /// Returns `true` if this is the top element (contradiction).
    fn is_contradiction_chirho(&self) -> bool;
}

impl MergeableChirho for NumericInfoChirho {
    fn merge_chirho(&self, other_chirho: &Self) -> Self {
        NumericInfoChirho::merge_chirho(self, other_chirho)
    }

    fn is_nothing_chirho(&self) -> bool {
        NumericInfoChirho::is_nothing_chirho(self)
    }

    fn is_contradiction_chirho(&self) -> bool {
        NumericInfoChirho::is_contradiction_chirho(self)
    }
}

/// A cell holding partial information with neighbor notification.
///
/// When the cell's content changes, all registered neighbor propagators
/// are alerted via the scheduler.
///
/// # Type Parameters
///
/// - `T`: The type of partial information (must implement [`MergeableChirho`])
///
/// # Example
///
/// ```
/// use propagators_chirho::{CellChirho, NumericInfoChirho, SchedulerChirho};
///
/// let scheduler_chirho = SchedulerChirho::new_chirho();
/// let cell_chirho = CellChirho::new_chirho("temperature");
///
/// // Add partial information
/// cell_chirho.add_content_chirho(
///     NumericInfoChirho::interval_chirho(20.0, 30.0),
///     &scheduler_chirho
/// );
///
/// // Add more precise information
/// cell_chirho.add_content_chirho(
///     NumericInfoChirho::interval_chirho(24.0, 26.0),
///     &scheduler_chirho
/// );
///
/// // Content is now [24, 26]
/// ```
pub struct CellChirho<T: MergeableChirho> {
    /// Human-readable name for debugging.
    name_chirho: String,
    /// The current content of the cell.
    content_chirho: RefCell<T>,
    /// Propagators to notify when content changes.
    neighbors_chirho: RefCell<Vec<Rc<dyn PropagatorChirho>>>,
}

impl<T: MergeableChirho> CellChirho<T> {
    /// Creates a new cell with the given name.
    ///
    /// The cell starts with no information (bottom of the lattice).
    ///
    /// # Arguments
    ///
    /// * `name_chirho` - A human-readable name for debugging
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::{CellChirho, NumericInfoChirho};
    /// use std::rc::Rc;
    ///
    /// let cell_chirho: Rc<CellChirho<NumericInfoChirho>> = CellChirho::new_chirho("x");
    /// ```
    pub fn new_chirho(name_chirho: &str) -> Rc<Self> {
        Rc::new(Self {
            name_chirho: name_chirho.to_string(),
            content_chirho: RefCell::new(T::default()),
            neighbors_chirho: RefCell::new(Vec::new()),
        })
    }

    /// Creates a new cell with initial content.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::{CellChirho, NumericInfoChirho};
    ///
    /// let cell_chirho = CellChirho::with_content_chirho(
    ///     "pi",
    ///     NumericInfoChirho::interval_chirho(3.14, 3.15)
    /// );
    /// ```
    pub fn with_content_chirho(name_chirho: &str, content_chirho: T) -> Rc<Self> {
        Rc::new(Self {
            name_chirho: name_chirho.to_string(),
            content_chirho: RefCell::new(content_chirho),
            neighbors_chirho: RefCell::new(Vec::new()),
        })
    }

    /// Returns the name of this cell.
    #[inline]
    pub fn name_chirho(&self) -> &str {
        &self.name_chirho
    }

    /// Returns a copy of the current content.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::{CellChirho, NumericInfoChirho, SchedulerChirho};
    ///
    /// let scheduler_chirho = SchedulerChirho::new_chirho();
    /// let cell_chirho = CellChirho::new_chirho("x");
    /// cell_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(42.0), &scheduler_chirho);
    ///
    /// let content_chirho = cell_chirho.content_chirho();
    /// assert!(!content_chirho.is_nothing_chirho());
    /// ```
    pub fn content_chirho(&self) -> T {
        self.content_chirho.borrow().clone()
    }

    /// Adds new information to the cell, merging with existing content.
    ///
    /// If the merge results in new information (the content changes),
    /// all neighbor propagators are alerted via the scheduler.
    ///
    /// # Arguments
    ///
    /// * `increment_chirho` - New partial information to add
    /// * `scheduler_chirho` - The scheduler for alerting neighbors
    ///
    /// # Returns
    ///
    /// `true` if the content changed, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::{CellChirho, NumericInfoChirho, SchedulerChirho};
    ///
    /// let scheduler_chirho = SchedulerChirho::new_chirho();
    /// let cell_chirho = CellChirho::new_chirho("x");
    ///
    /// // First addition always changes content
    /// let changed_chirho = cell_chirho.add_content_chirho(
    ///     NumericInfoChirho::interval_chirho(0.0, 100.0),
    ///     &scheduler_chirho
    /// );
    /// assert!(changed_chirho);
    ///
    /// // Redundant information doesn't change content
    /// let changed_chirho = cell_chirho.add_content_chirho(
    ///     NumericInfoChirho::interval_chirho(-10.0, 200.0),  // Wider interval
    ///     &scheduler_chirho
    /// );
    /// assert!(!changed_chirho);
    /// ```
    pub fn add_content_chirho(
        &self,
        increment_chirho: T,
        scheduler_chirho: &SchedulerChirho,
    ) -> bool {
        let old_content_chirho = self.content_chirho.borrow().clone();
        let new_content_chirho = old_content_chirho.merge_chirho(&increment_chirho);

        // Check if content actually changed (using PartialEq, not string formatting)
        let changed_chirho = new_content_chirho != old_content_chirho;

        if changed_chirho {
            *self.content_chirho.borrow_mut() = new_content_chirho;

            // Alert all neighbors - clone to avoid borrow issues
            let neighbors_chirho: Vec<Rc<dyn PropagatorChirho>> =
                self.neighbors_chirho.borrow().clone();
            for neighbor_chirho in neighbors_chirho {
                scheduler_chirho.alert_propagator_chirho(neighbor_chirho);
            }
        }

        changed_chirho
    }

    /// Registers a propagator as a neighbor of this cell.
    ///
    /// The propagator will be alerted whenever the cell's content changes.
    ///
    /// # Arguments
    ///
    /// * `propagator_chirho` - The propagator to register
    pub fn add_neighbor_chirho(&self, propagator_chirho: Rc<dyn PropagatorChirho>) {
        self.neighbors_chirho.borrow_mut().push(propagator_chirho);
    }

    /// Removes a propagator from the neighbor list by its ID.
    ///
    /// Returns `true` if a propagator was removed, `false` if not found.
    ///
    /// # Arguments
    ///
    /// * `propagator_id_chirho` - The ID of the propagator to remove
    pub fn remove_neighbor_chirho(&self, propagator_id_chirho: usize) -> bool {
        let mut neighbors_chirho = self.neighbors_chirho.borrow_mut();
        let original_len_chirho = neighbors_chirho.len();
        neighbors_chirho.retain(|p_chirho| p_chirho.id_chirho() != propagator_id_chirho);
        neighbors_chirho.len() < original_len_chirho
    }

    /// Removes all propagators with IDs in the given set.
    ///
    /// Returns the number of propagators removed.
    ///
    /// # Arguments
    ///
    /// * `propagator_ids_chirho` - Set of propagator IDs to remove
    pub fn remove_neighbors_chirho(
        &self,
        propagator_ids_chirho: &std::collections::HashSet<usize>,
    ) -> usize {
        let mut neighbors_chirho = self.neighbors_chirho.borrow_mut();
        let original_len_chirho = neighbors_chirho.len();
        neighbors_chirho.retain(|p_chirho| !propagator_ids_chirho.contains(&p_chirho.id_chirho()));
        original_len_chirho - neighbors_chirho.len()
    }

    /// Returns `true` if the cell contains no information.
    #[inline]
    pub fn is_nothing_chirho(&self) -> bool {
        self.content_chirho.borrow().is_nothing_chirho()
    }

    /// Returns `true` if the cell contains a contradiction.
    #[inline]
    pub fn is_contradiction_chirho(&self) -> bool {
        self.content_chirho.borrow().is_contradiction_chirho()
    }

    /// Returns the number of registered neighbors.
    #[inline]
    pub fn neighbor_count_chirho(&self) -> usize {
        self.neighbors_chirho.borrow().len()
    }
}

impl<T: MergeableChirho> fmt::Debug for CellChirho<T> {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f_chirho,
            "Cell({}: {:?})",
            self.name_chirho,
            self.content_chirho.borrow()
        )
    }
}

impl<T: MergeableChirho + fmt::Display> fmt::Display for CellChirho<T> {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f_chirho,
            "{}: {}",
            self.name_chirho,
            self.content_chirho.borrow()
        )
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_cell_creation_chirho() {
        let cell_chirho: Rc<CellChirho<NumericInfoChirho>> = CellChirho::new_chirho("test");
        assert_eq!(cell_chirho.name_chirho(), "test");
        assert!(cell_chirho.is_nothing_chirho());
    }

    #[test]
    fn test_cell_with_content_chirho() {
        let cell_chirho =
            CellChirho::with_content_chirho("pi", NumericInfoChirho::exact_chirho(3.14159));
        assert!(!cell_chirho.is_nothing_chirho());
    }

    #[test]
    fn test_cell_add_content_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let cell_chirho: Rc<CellChirho<NumericInfoChirho>> = CellChirho::new_chirho("x");

        let changed_chirho = cell_chirho.add_content_chirho(
            NumericInfoChirho::interval_chirho(0.0, 100.0),
            &scheduler_chirho,
        );
        assert!(changed_chirho);
        assert!(!cell_chirho.is_nothing_chirho());
    }

    #[test]
    fn test_cell_merge_refines_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let cell_chirho: Rc<CellChirho<NumericInfoChirho>> = CellChirho::new_chirho("x");

        cell_chirho.add_content_chirho(
            NumericInfoChirho::interval_chirho(0.0, 100.0),
            &scheduler_chirho,
        );
        cell_chirho.add_content_chirho(
            NumericInfoChirho::interval_chirho(40.0, 60.0),
            &scheduler_chirho,
        );

        let content_chirho = cell_chirho.content_chirho();
        let interval_chirho = content_chirho.as_interval_chirho().unwrap();
        assert_eq!(interval_chirho.lo_chirho, 40.0);
        assert_eq!(interval_chirho.hi_chirho, 60.0);
    }

    #[test]
    fn test_cell_contradiction_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let cell_chirho: Rc<CellChirho<NumericInfoChirho>> = CellChirho::new_chirho("x");

        cell_chirho.add_content_chirho(
            NumericInfoChirho::interval_chirho(0.0, 10.0),
            &scheduler_chirho,
        );
        cell_chirho.add_content_chirho(
            NumericInfoChirho::interval_chirho(20.0, 30.0),
            &scheduler_chirho,
        );

        assert!(cell_chirho.is_contradiction_chirho());
    }

    #[test]
    fn test_cell_neighbor_count_chirho() {
        let cell_chirho: Rc<CellChirho<NumericInfoChirho>> = CellChirho::new_chirho("x");
        assert_eq!(cell_chirho.neighbor_count_chirho(), 0);
    }
}
