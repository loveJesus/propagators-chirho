// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Scheduler for running propagators to fixpoint.
//!
//! The scheduler maintains a queue of alerted propagators and runs them
//! until no more propagators have new information to contribute (fixpoint).
//!
//! # Algorithm
//!
//! ```text
//! while queue is not empty:
//!     propagator = queue.pop()
//!     propagator.run()    // May alert more propagators
//! ```
//!
//! The order of execution doesn't affect correctness, only efficiency.
//! This is because propagators only add information monotonically.
//!
//! # References
//!
//! - Radul, A., & Sussman, G. J. (2009). *The Art of the Propagator*, Section 4.
//!   <https://dspace.mit.edu/handle/1721.1/44215>

use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

use crate::propagator_chirho::PropagatorChirho;

/// Statistics about scheduler execution.
#[derive(Debug, Clone, Default)]
pub struct SchedulerStatsChirho {
    /// Total number of propagator runs.
    pub runs_chirho: usize,
    /// Number of times `run_chirho` was called.
    pub iterations_chirho: usize,
    /// Maximum queue size observed.
    pub max_queue_size_chirho: usize,
}

/// A scheduler that runs propagators to fixpoint.
///
/// The scheduler maintains a queue of "alerted" propagators—those whose
/// input cells have changed and may have new information to contribute.
///
/// # Example
///
/// ```
/// use propagators_chirho::{SchedulerChirho, CellChirho, NumericInfoChirho};
///
/// let scheduler_chirho = SchedulerChirho::new_chirho();
///
/// // Create cells and propagators...
/// let cell_chirho = CellChirho::new_chirho("x");
///
/// // Add content (propagators would be alerted)
/// cell_chirho.add_content_chirho(
///     NumericInfoChirho::exact_chirho(42.0),
///     &scheduler_chirho
/// );
///
/// // Run all propagators to fixpoint
/// scheduler_chirho.run_chirho();
/// ```
pub struct SchedulerChirho {
    /// Queue of propagators waiting to run.
    queue_chirho: RefCell<VecDeque<Rc<dyn PropagatorChirho>>>,
    /// Set of propagator IDs currently in queue (for deduplication).
    alerted_chirho: RefCell<HashSet<usize>>,
    /// Execution statistics.
    stats_chirho: RefCell<SchedulerStatsChirho>,
}

impl SchedulerChirho {
    /// Creates a new scheduler.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::SchedulerChirho;
    ///
    /// let scheduler_chirho = SchedulerChirho::new_chirho();
    /// ```
    pub fn new_chirho() -> Self {
        Self {
            queue_chirho: RefCell::new(VecDeque::new()),
            alerted_chirho: RefCell::new(HashSet::new()),
            stats_chirho: RefCell::new(SchedulerStatsChirho::default()),
        }
    }

    /// Alerts a propagator that one of its inputs has changed.
    ///
    /// The propagator is added to the queue if not already present.
    ///
    /// # Arguments
    ///
    /// * `propagator_chirho` - The propagator to alert
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::{SchedulerChirho, ConstantChirho, CellChirho, NumericInfoChirho};
    /// use std::rc::Rc;
    ///
    /// let scheduler_chirho = SchedulerChirho::new_chirho();
    /// let cell_chirho = CellChirho::new_chirho("x");
    /// let propagator_chirho = ConstantChirho::new_chirho(
    ///     NumericInfoChirho::exact_chirho(42.0),
    ///     cell_chirho
    /// );
    ///
    /// scheduler_chirho.alert_propagator_chirho(propagator_chirho);
    /// assert_eq!(scheduler_chirho.queue_size_chirho(), 1);
    /// ```
    pub fn alert_propagator_chirho(&self, propagator_chirho: Rc<dyn PropagatorChirho>) {
        let id_chirho = propagator_chirho.id_chirho();

        // Check if already alerted (without holding borrow)
        let already_alerted_chirho = self.alerted_chirho.borrow().contains(&id_chirho);

        if !already_alerted_chirho {
            self.alerted_chirho.borrow_mut().insert(id_chirho);
            self.queue_chirho.borrow_mut().push_back(propagator_chirho);

            // Update max queue size
            let queue_size_chirho = self.queue_chirho.borrow().len();
            let mut stats_chirho = self.stats_chirho.borrow_mut();
            if queue_size_chirho > stats_chirho.max_queue_size_chirho {
                stats_chirho.max_queue_size_chirho = queue_size_chirho;
            }
        }
    }

    /// Runs all queued propagators until fixpoint is reached.
    ///
    /// A fixpoint is reached when no propagators have new information
    /// to contribute (the queue becomes empty).
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::SchedulerChirho;
    ///
    /// let scheduler_chirho = SchedulerChirho::new_chirho();
    /// // ... add cells and propagators ...
    /// scheduler_chirho.run_chirho();
    /// ```
    pub fn run_chirho(&self) {
        self.stats_chirho.borrow_mut().iterations_chirho += 1;

        loop {
            // Pop without holding borrow
            let propagator_chirho = self.queue_chirho.borrow_mut().pop_front();

            match propagator_chirho {
                Some(p_chirho) => {
                    self.alerted_chirho
                        .borrow_mut()
                        .remove(&p_chirho.id_chirho());
                    self.stats_chirho.borrow_mut().runs_chirho += 1;
                    p_chirho.run_chirho(self);
                }
                None => break,
            }
        }
    }

    /// Runs propagators for at most `max_steps` iterations.
    ///
    /// Returns `true` if fixpoint was reached, `false` if limit was hit.
    ///
    /// # Arguments
    ///
    /// * `max_steps_chirho` - Maximum number of propagator runs
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::SchedulerChirho;
    ///
    /// let scheduler_chirho = SchedulerChirho::new_chirho();
    /// let reached_fixpoint_chirho = scheduler_chirho.run_bounded_chirho(1000);
    /// assert!(reached_fixpoint_chirho);  // No propagators, so fixpoint immediately
    /// ```
    pub fn run_bounded_chirho(&self, max_steps_chirho: usize) -> bool {
        self.stats_chirho.borrow_mut().iterations_chirho += 1;

        for _ in 0..max_steps_chirho {
            let propagator_chirho = self.queue_chirho.borrow_mut().pop_front();

            match propagator_chirho {
                Some(p_chirho) => {
                    self.alerted_chirho
                        .borrow_mut()
                        .remove(&p_chirho.id_chirho());
                    self.stats_chirho.borrow_mut().runs_chirho += 1;
                    p_chirho.run_chirho(self);
                }
                None => return true, // Fixpoint reached
            }
        }

        false // Limit hit
    }

    /// Returns the current queue size.
    #[inline]
    pub fn queue_size_chirho(&self) -> usize {
        self.queue_chirho.borrow().len()
    }

    /// Returns `true` if the queue is empty (fixpoint reached).
    #[inline]
    pub fn is_quiescent_chirho(&self) -> bool {
        self.queue_chirho.borrow().is_empty()
    }

    /// Returns execution statistics.
    pub fn stats_chirho(&self) -> SchedulerStatsChirho {
        self.stats_chirho.borrow().clone()
    }

    /// Resets execution statistics.
    pub fn reset_stats_chirho(&self) {
        *self.stats_chirho.borrow_mut() = SchedulerStatsChirho::default();
    }

    /// Clears the queue without running propagators.
    ///
    /// Use with caution—this may leave the system in an inconsistent state.
    pub fn clear_chirho(&self) {
        self.queue_chirho.borrow_mut().clear();
        self.alerted_chirho.borrow_mut().clear();
    }
}

impl Default for SchedulerChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl std::fmt::Debug for SchedulerChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f_chirho
            .debug_struct("Scheduler")
            .field("queue_size", &self.queue_size_chirho())
            .field("stats", &self.stats_chirho.borrow())
            .finish()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_scheduler_creation_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        assert!(scheduler_chirho.is_quiescent_chirho());
        assert_eq!(scheduler_chirho.queue_size_chirho(), 0);
    }

    #[test]
    fn test_scheduler_empty_run_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        scheduler_chirho.run_chirho(); // Should not panic
        assert!(scheduler_chirho.is_quiescent_chirho());
    }

    #[test]
    fn test_scheduler_bounded_run_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let reached_chirho = scheduler_chirho.run_bounded_chirho(100);
        assert!(reached_chirho); // Empty queue = immediate fixpoint
    }

    #[test]
    fn test_scheduler_stats_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        scheduler_chirho.run_chirho();

        let stats_chirho = scheduler_chirho.stats_chirho();
        assert_eq!(stats_chirho.iterations_chirho, 1);
        assert_eq!(stats_chirho.runs_chirho, 0);
    }
}
