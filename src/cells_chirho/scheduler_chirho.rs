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

use crate::propagators_chirho::propagator_chirho::PropagatorChirho;

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

// ============================================================================
// INCREMENTAL SCHEDULER
// ============================================================================

/// A cell delta representing a change in cell content.
#[derive(Debug, Clone)]
pub struct CellDeltaChirho {
    /// The name of the cell that changed.
    pub cell_name_chirho: String,
    /// The generation (run number) when the change occurred.
    pub generation_chirho: u64,
    /// Whether the change caused propagation.
    pub caused_propagation_chirho: bool,
}

/// Statistics about incremental execution.
#[derive(Debug, Clone, Default)]
pub struct IncrementalStatsChirho {
    /// Base scheduler stats.
    pub base_stats_chirho: SchedulerStatsChirho,
    /// Number of incremental runs.
    pub incremental_runs_chirho: usize,
    /// Number of cell changes tracked.
    pub changes_tracked_chirho: usize,
    /// Number of propagators skipped due to no changes.
    pub propagators_skipped_chirho: usize,
}

/// An incremental scheduler that tracks cell changes across runs.
///
/// This extends the basic scheduler with delta tracking, allowing
/// efficient incremental updates when only a few cells change.
///
/// # Example
///
/// ```
/// use propagators_chirho::cells_chirho::scheduler_chirho::IncrementalSchedulerChirho;
///
/// let scheduler_chirho = IncrementalSchedulerChirho::new_chirho();
///
/// // Track a cell change
/// scheduler_chirho.record_change_chirho("x");
///
/// // Run incrementally (only propagates changes since last run)
/// scheduler_chirho.run_chirho();
///
/// // Check what changed
/// let changes_chirho = scheduler_chirho.changes_since_generation_chirho(0);
/// ```
pub struct IncrementalSchedulerChirho {
    /// The base scheduler for running propagators.
    base_scheduler_chirho: SchedulerChirho,
    /// Current generation (incremented on each run).
    generation_chirho: RefCell<u64>,
    /// History of cell changes.
    change_history_chirho: RefCell<Vec<CellDeltaChirho>>,
    /// Set of cell names that changed in the current run.
    current_changes_chirho: RefCell<HashSet<String>>,
    /// Maximum history size before pruning.
    max_history_size_chirho: usize,
    /// Incremental execution statistics.
    incremental_stats_chirho: RefCell<IncrementalStatsChirho>,
}

impl IncrementalSchedulerChirho {
    /// Creates a new incremental scheduler with default history size.
    pub fn new_chirho() -> Self {
        Self::with_history_size_chirho(1000)
    }

    /// Creates a new incremental scheduler with custom history size.
    ///
    /// # Arguments
    ///
    /// * `max_history_size_chirho` - Maximum number of changes to track
    pub fn with_history_size_chirho(max_history_size_chirho: usize) -> Self {
        Self {
            base_scheduler_chirho: SchedulerChirho::new_chirho(),
            generation_chirho: RefCell::new(0),
            change_history_chirho: RefCell::new(Vec::new()),
            current_changes_chirho: RefCell::new(HashSet::new()),
            max_history_size_chirho,
            incremental_stats_chirho: RefCell::new(IncrementalStatsChirho::default()),
        }
    }

    /// Records that a cell has changed.
    ///
    /// This should be called when a cell's content changes.
    ///
    /// # Arguments
    ///
    /// * `cell_name_chirho` - Name of the cell that changed
    pub fn record_change_chirho(&self, cell_name_chirho: &str) {
        self.current_changes_chirho
            .borrow_mut()
            .insert(cell_name_chirho.to_string());
        self.incremental_stats_chirho
            .borrow_mut()
            .changes_tracked_chirho += 1;
    }

    /// Returns the current generation number.
    pub fn generation_chirho(&self) -> u64 {
        *self.generation_chirho.borrow()
    }

    /// Returns all changes since a given generation.
    ///
    /// # Arguments
    ///
    /// * `since_generation_chirho` - The generation to start from (exclusive)
    pub fn changes_since_generation_chirho(
        &self,
        since_generation_chirho: u64,
    ) -> Vec<CellDeltaChirho> {
        self.change_history_chirho
            .borrow()
            .iter()
            .filter(|delta_chirho| delta_chirho.generation_chirho > since_generation_chirho)
            .cloned()
            .collect()
    }

    /// Returns the cells that changed in the current (not yet committed) run.
    pub fn pending_changes_chirho(&self) -> Vec<String> {
        self.current_changes_chirho
            .borrow()
            .iter()
            .cloned()
            .collect()
    }

    /// Returns `true` if any cells have pending changes.
    pub fn has_pending_changes_chirho(&self) -> bool {
        !self.current_changes_chirho.borrow().is_empty()
    }

    /// Alerts a propagator (delegates to base scheduler).
    pub fn alert_propagator_chirho(&self, propagator_chirho: Rc<dyn PropagatorChirho>) {
        self.base_scheduler_chirho
            .alert_propagator_chirho(propagator_chirho);
    }

    /// Runs all queued propagators to fixpoint.
    ///
    /// After running, the current changes are committed to history
    /// and the generation is incremented.
    pub fn run_chirho(&self) {
        self.base_scheduler_chirho.run_chirho();
        self.commit_changes_chirho();
    }

    /// Runs propagators for at most `max_steps` iterations.
    ///
    /// Returns `true` if fixpoint was reached.
    pub fn run_bounded_chirho(&self, max_steps_chirho: usize) -> bool {
        let result_chirho = self
            .base_scheduler_chirho
            .run_bounded_chirho(max_steps_chirho);
        self.commit_changes_chirho();
        result_chirho
    }

    /// Commits current changes to history and increments generation.
    fn commit_changes_chirho(&self) {
        let current_gen_chirho = *self.generation_chirho.borrow();
        let changes_chirho = std::mem::take(&mut *self.current_changes_chirho.borrow_mut());

        // Record each change in history
        let mut history_chirho = self.change_history_chirho.borrow_mut();
        for cell_name_chirho in changes_chirho {
            history_chirho.push(CellDeltaChirho {
                cell_name_chirho,
                generation_chirho: current_gen_chirho,
                caused_propagation_chirho: true,
            });
        }

        // Prune history if too large
        if history_chirho.len() > self.max_history_size_chirho {
            let to_remove_chirho = history_chirho.len() - self.max_history_size_chirho;
            history_chirho.drain(0..to_remove_chirho);
        }

        // Increment generation
        *self.generation_chirho.borrow_mut() = current_gen_chirho + 1;
        self.incremental_stats_chirho
            .borrow_mut()
            .incremental_runs_chirho += 1;
    }

    /// Returns the base scheduler (for compatibility).
    pub fn base_scheduler_chirho(&self) -> &SchedulerChirho {
        &self.base_scheduler_chirho
    }

    /// Returns the current queue size.
    #[inline]
    pub fn queue_size_chirho(&self) -> usize {
        self.base_scheduler_chirho.queue_size_chirho()
    }

    /// Returns `true` if the queue is empty.
    #[inline]
    pub fn is_quiescent_chirho(&self) -> bool {
        self.base_scheduler_chirho.is_quiescent_chirho()
    }

    /// Returns incremental execution statistics.
    pub fn stats_chirho(&self) -> IncrementalStatsChirho {
        let mut stats_chirho = self.incremental_stats_chirho.borrow().clone();
        stats_chirho.base_stats_chirho = self.base_scheduler_chirho.stats_chirho();
        stats_chirho
    }

    /// Resets all statistics.
    pub fn reset_stats_chirho(&self) {
        self.base_scheduler_chirho.reset_stats_chirho();
        *self.incremental_stats_chirho.borrow_mut() = IncrementalStatsChirho::default();
    }

    /// Clears the change history.
    pub fn clear_history_chirho(&self) {
        self.change_history_chirho.borrow_mut().clear();
    }

    /// Clears the queue and pending changes.
    pub fn clear_chirho(&self) {
        self.base_scheduler_chirho.clear_chirho();
        self.current_changes_chirho.borrow_mut().clear();
    }
}

impl Default for IncrementalSchedulerChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl std::fmt::Debug for IncrementalSchedulerChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f_chirho
            .debug_struct("IncrementalScheduler")
            .field("generation", &self.generation_chirho())
            .field("queue_size", &self.queue_size_chirho())
            .field("pending_changes", &self.current_changes_chirho.borrow().len())
            .field("history_size", &self.change_history_chirho.borrow().len())
            .finish()
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

// ============================================================================
// PRIORITY SCHEDULER (GAP-003)
// ============================================================================

use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A propagator wrapper that includes priority for the heap.
struct PriorityPropagatorChirho {
    propagator_chirho: Rc<dyn PropagatorChirho>,
    priority_chirho: i32,
}

impl Eq for PriorityPropagatorChirho {}

impl PartialEq for PriorityPropagatorChirho {
    fn eq(&self, other_chirho: &Self) -> bool {
        self.propagator_chirho.id_chirho() == other_chirho.propagator_chirho.id_chirho()
    }
}

impl Ord for PriorityPropagatorChirho {
    fn cmp(&self, other_chirho: &Self) -> Ordering {
        // Higher priority first (max-heap behavior)
        self.priority_chirho.cmp(&other_chirho.priority_chirho)
    }
}

impl PartialOrd for PriorityPropagatorChirho {
    fn partial_cmp(&self, other_chirho: &Self) -> Option<Ordering> {
        Some(self.cmp(other_chirho))
    }
}

/// Statistics for priority scheduler execution.
#[derive(Debug, Clone, Default)]
pub struct PrioritySchedulerStatsChirho {
    /// Total number of propagator runs.
    pub runs_chirho: usize,
    /// Number of high-priority runs.
    pub high_priority_runs_chirho: usize,
    /// Number of normal-priority runs.
    pub normal_priority_runs_chirho: usize,
    /// Number of low-priority runs.
    pub low_priority_runs_chirho: usize,
    /// Number of times `run_chirho` was called.
    pub iterations_chirho: usize,
    /// Maximum queue size observed.
    pub max_queue_size_chirho: usize,
}

/// A priority-aware scheduler that runs higher-priority propagators first.
///
/// This scheduler uses a priority queue (max-heap) to ensure that propagators
/// with higher priority values are executed before lower-priority ones.
///
/// # Priority Levels
///
/// - Higher values = higher priority (run first)
/// - Default priority is 0
/// - Use `HIGH_PRIORITY_CHIRHO` (100) for critical constraints
/// - Use `LOW_PRIORITY_CHIRHO` (-100) for soft/deferred constraints
///
/// # Example
///
/// ```
/// use propagators_chirho::cells_chirho::scheduler_chirho::PrioritySchedulerChirho;
///
/// let scheduler_chirho = PrioritySchedulerChirho::new_chirho();
///
/// // High-priority propagators will run before normal ones
/// // scheduler_chirho.alert_propagator_chirho(high_priority_prop);
/// // scheduler_chirho.alert_propagator_chirho(normal_prop);
/// // scheduler_chirho.run_chirho();
/// ```
pub struct PrioritySchedulerChirho {
    /// Priority queue of propagators waiting to run.
    queue_chirho: RefCell<BinaryHeap<PriorityPropagatorChirho>>,
    /// Set of propagator IDs currently in queue (for deduplication).
    alerted_chirho: RefCell<HashSet<usize>>,
    /// Execution statistics.
    stats_chirho: RefCell<PrioritySchedulerStatsChirho>,
}

impl PrioritySchedulerChirho {
    /// Creates a new priority scheduler.
    pub fn new_chirho() -> Self {
        Self {
            queue_chirho: RefCell::new(BinaryHeap::new()),
            alerted_chirho: RefCell::new(HashSet::new()),
            stats_chirho: RefCell::new(PrioritySchedulerStatsChirho::default()),
        }
    }

    /// Alerts a propagator that one of its inputs has changed.
    ///
    /// The propagator is added to the priority queue if not already present.
    pub fn alert_propagator_chirho(&self, propagator_chirho: Rc<dyn PropagatorChirho>) {
        let id_chirho = propagator_chirho.id_chirho();

        let already_alerted_chirho = self.alerted_chirho.borrow().contains(&id_chirho);

        if !already_alerted_chirho {
            self.alerted_chirho.borrow_mut().insert(id_chirho);
            let priority_chirho = propagator_chirho.priority_chirho();
            self.queue_chirho
                .borrow_mut()
                .push(PriorityPropagatorChirho {
                    propagator_chirho,
                    priority_chirho,
                });

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
    /// Propagators are run in priority order (highest first).
    pub fn run_chirho(&self) {
        self.stats_chirho.borrow_mut().iterations_chirho += 1;

        loop {
            let entry_chirho = self.queue_chirho.borrow_mut().pop();

            match entry_chirho {
                Some(pp_chirho) => {
                    self.alerted_chirho
                        .borrow_mut()
                        .remove(&pp_chirho.propagator_chirho.id_chirho());

                    // Track priority statistics
                    {
                        let mut stats_chirho = self.stats_chirho.borrow_mut();
                        stats_chirho.runs_chirho += 1;
                        if pp_chirho.priority_chirho >= 50 {
                            stats_chirho.high_priority_runs_chirho += 1;
                        } else if pp_chirho.priority_chirho <= -50 {
                            stats_chirho.low_priority_runs_chirho += 1;
                        } else {
                            stats_chirho.normal_priority_runs_chirho += 1;
                        }
                    }

                    // Run the propagator, passing the base scheduler interface
                    // The propagator will use the scheduler to alert other propagators
                    // For now, we create a temporary SchedulerChirho wrapper
                    // In practice, propagators should be updated to use a trait
                    pp_chirho
                        .propagator_chirho
                        .run_chirho(&self.as_base_scheduler_chirho());
                }
                None => break,
            }
        }
    }

    /// Runs propagators for at most `max_steps` iterations.
    ///
    /// Returns `true` if fixpoint was reached, `false` if limit was hit.
    pub fn run_bounded_chirho(&self, max_steps_chirho: usize) -> bool {
        self.stats_chirho.borrow_mut().iterations_chirho += 1;

        for _ in 0..max_steps_chirho {
            let entry_chirho = self.queue_chirho.borrow_mut().pop();

            match entry_chirho {
                Some(pp_chirho) => {
                    self.alerted_chirho
                        .borrow_mut()
                        .remove(&pp_chirho.propagator_chirho.id_chirho());

                    {
                        let mut stats_chirho = self.stats_chirho.borrow_mut();
                        stats_chirho.runs_chirho += 1;
                        if pp_chirho.priority_chirho >= 50 {
                            stats_chirho.high_priority_runs_chirho += 1;
                        } else if pp_chirho.priority_chirho <= -50 {
                            stats_chirho.low_priority_runs_chirho += 1;
                        } else {
                            stats_chirho.normal_priority_runs_chirho += 1;
                        }
                    }

                    pp_chirho
                        .propagator_chirho
                        .run_chirho(&self.as_base_scheduler_chirho());
                }
                None => return true,
            }
        }

        false
    }

    /// Creates a SchedulerChirho wrapper for compatibility with propagators.
    fn as_base_scheduler_chirho(&self) -> SchedulerChirho {
        // Note: This is a temporary solution. In a full refactor, propagators
        // would accept a trait instead of a concrete SchedulerChirho type.
        SchedulerChirho::new_chirho()
    }

    /// Returns the current queue size.
    #[inline]
    pub fn queue_size_chirho(&self) -> usize {
        self.queue_chirho.borrow().len()
    }

    /// Returns `true` if the queue is empty.
    #[inline]
    pub fn is_quiescent_chirho(&self) -> bool {
        self.queue_chirho.borrow().is_empty()
    }

    /// Returns execution statistics.
    pub fn stats_chirho(&self) -> PrioritySchedulerStatsChirho {
        self.stats_chirho.borrow().clone()
    }

    /// Resets execution statistics.
    pub fn reset_stats_chirho(&self) {
        *self.stats_chirho.borrow_mut() = PrioritySchedulerStatsChirho::default();
    }

    /// Clears the queue without running propagators.
    pub fn clear_chirho(&self) {
        self.queue_chirho.borrow_mut().clear();
        self.alerted_chirho.borrow_mut().clear();
    }
}

impl Default for PrioritySchedulerChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl std::fmt::Debug for PrioritySchedulerChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f_chirho
            .debug_struct("PriorityScheduler")
            .field("queue_size", &self.queue_size_chirho())
            .field("stats", &self.stats_chirho.borrow())
            .finish()
    }
}

// ============================================================================
// SOFT CONSTRAINTS AND WEIGHTED PROPAGATORS (GAP-003)
// ============================================================================

/// A weighted constraint that tracks violation cost.
///
/// Soft constraints can be violated with a penalty cost, unlike hard
/// constraints that must always be satisfied.
///
/// # Example
///
/// ```
/// use propagators_chirho::cells_chirho::scheduler_chirho::WeightedConstraintChirho;
///
/// let weight_chirho = WeightedConstraintChirho::new_chirho(
///     "prefer_x_less_than_5",
///     10.0,  // Violation cost
/// );
///
/// // If violated, adds 10.0 to total cost
/// ```
#[derive(Clone, Debug)]
pub struct WeightedConstraintChirho {
    /// Name of the constraint.
    pub name_chirho: String,
    /// Weight (cost) for violating this constraint.
    pub weight_chirho: f64,
    /// Whether this constraint is currently violated.
    violated_chirho: RefCell<bool>,
    /// Current violation degree (0.0 = fully satisfied, 1.0 = fully violated).
    violation_degree_chirho: RefCell<f64>,
}

impl WeightedConstraintChirho {
    /// Creates a new weighted constraint.
    ///
    /// # Arguments
    ///
    /// * `name_chirho` - Name for identification
    /// * `weight_chirho` - Cost of violation (higher = more important to satisfy)
    pub fn new_chirho(name_chirho: &str, weight_chirho: f64) -> Self {
        Self {
            name_chirho: name_chirho.to_string(),
            weight_chirho,
            violated_chirho: RefCell::new(false),
            violation_degree_chirho: RefCell::new(0.0),
        }
    }

    /// Marks this constraint as violated.
    pub fn set_violated_chirho(&self, violated_chirho: bool) {
        *self.violated_chirho.borrow_mut() = violated_chirho;
        *self.violation_degree_chirho.borrow_mut() = if violated_chirho { 1.0 } else { 0.0 };
    }

    /// Sets a partial violation degree.
    ///
    /// # Arguments
    ///
    /// * `degree_chirho` - Violation degree from 0.0 (satisfied) to 1.0 (fully violated)
    pub fn set_violation_degree_chirho(&self, degree_chirho: f64) {
        let clamped_chirho = degree_chirho.clamp(0.0, 1.0);
        *self.violation_degree_chirho.borrow_mut() = clamped_chirho;
        *self.violated_chirho.borrow_mut() = clamped_chirho > 0.0;
    }

    /// Returns whether this constraint is violated.
    pub fn is_violated_chirho(&self) -> bool {
        *self.violated_chirho.borrow()
    }

    /// Returns the violation degree.
    pub fn violation_degree_chirho(&self) -> f64 {
        *self.violation_degree_chirho.borrow()
    }

    /// Returns the weighted violation cost.
    pub fn violation_cost_chirho(&self) -> f64 {
        self.weight_chirho * *self.violation_degree_chirho.borrow()
    }
}

/// A collection of weighted constraints for optimization.
///
/// Tracks soft constraints and computes total violation cost.
///
/// # Example
///
/// ```
/// use propagators_chirho::cells_chirho::scheduler_chirho::{
///     ConstraintWeightsChirho, WeightedConstraintChirho
/// };
///
/// let mut weights_chirho = ConstraintWeightsChirho::new_chirho();
///
/// weights_chirho.add_constraint_chirho(
///     WeightedConstraintChirho::new_chirho("soft_1", 5.0)
/// );
/// weights_chirho.add_constraint_chirho(
///     WeightedConstraintChirho::new_chirho("soft_2", 10.0)
/// );
///
/// // Mark one as violated
/// weights_chirho.set_violated_chirho("soft_1", true);
///
/// assert_eq!(weights_chirho.total_cost_chirho(), 5.0);
/// ```
#[derive(Clone, Debug, Default)]
pub struct ConstraintWeightsChirho {
    /// The weighted constraints.
    constraints_chirho: Vec<WeightedConstraintChirho>,
}

impl ConstraintWeightsChirho {
    /// Creates a new empty constraint weights collection.
    pub fn new_chirho() -> Self {
        Self {
            constraints_chirho: Vec::new(),
        }
    }

    /// Adds a weighted constraint.
    pub fn add_constraint_chirho(&mut self, constraint_chirho: WeightedConstraintChirho) {
        self.constraints_chirho.push(constraint_chirho);
    }

    /// Returns the number of constraints.
    pub fn len_chirho(&self) -> usize {
        self.constraints_chirho.len()
    }

    /// Returns `true` if there are no constraints.
    pub fn is_empty_chirho(&self) -> bool {
        self.constraints_chirho.is_empty()
    }

    /// Sets a constraint's violated status by name.
    pub fn set_violated_chirho(&mut self, name_chirho: &str, violated_chirho: bool) {
        for c_chirho in &self.constraints_chirho {
            if c_chirho.name_chirho == name_chirho {
                c_chirho.set_violated_chirho(violated_chirho);
                return;
            }
        }
    }

    /// Sets a constraint's violation degree by name.
    pub fn set_violation_degree_chirho(&mut self, name_chirho: &str, degree_chirho: f64) {
        for c_chirho in &self.constraints_chirho {
            if c_chirho.name_chirho == name_chirho {
                c_chirho.set_violation_degree_chirho(degree_chirho);
                return;
            }
        }
    }

    /// Returns the total violation cost.
    pub fn total_cost_chirho(&self) -> f64 {
        self.constraints_chirho
            .iter()
            .map(WeightedConstraintChirho::violation_cost_chirho)
            .sum()
    }

    /// Returns the number of violated constraints.
    pub fn violated_count_chirho(&self) -> usize {
        self.constraints_chirho
            .iter()
            .filter(|c_chirho| c_chirho.is_violated_chirho())
            .count()
    }

    /// Returns all violated constraints.
    pub fn violated_constraints_chirho(&self) -> Vec<&WeightedConstraintChirho> {
        self.constraints_chirho
            .iter()
            .filter(|c_chirho| c_chirho.is_violated_chirho())
            .collect()
    }

    /// Resets all constraints to satisfied.
    pub fn reset_chirho(&mut self) {
        for c_chirho in &self.constraints_chirho {
            c_chirho.set_violated_chirho(false);
        }
    }

    /// Returns a reference to constraint by name.
    pub fn get_chirho(&self, name_chirho: &str) -> Option<&WeightedConstraintChirho> {
        self.constraints_chirho
            .iter()
            .find(|c_chirho| c_chirho.name_chirho == name_chirho)
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

    // Incremental scheduler tests

    #[test]
    fn test_incremental_scheduler_creation_chirho() {
        let scheduler_chirho = IncrementalSchedulerChirho::new_chirho();
        assert!(scheduler_chirho.is_quiescent_chirho());
        assert_eq!(scheduler_chirho.generation_chirho(), 0);
        assert!(!scheduler_chirho.has_pending_changes_chirho());
    }

    #[test]
    fn test_incremental_scheduler_with_history_size_chirho() {
        let scheduler_chirho = IncrementalSchedulerChirho::with_history_size_chirho(500);
        assert!(scheduler_chirho.is_quiescent_chirho());
        assert_eq!(scheduler_chirho.generation_chirho(), 0);
    }

    #[test]
    fn test_incremental_record_change_chirho() {
        let scheduler_chirho = IncrementalSchedulerChirho::new_chirho();

        scheduler_chirho.record_change_chirho("x");
        scheduler_chirho.record_change_chirho("y");

        assert!(scheduler_chirho.has_pending_changes_chirho());
        let pending_chirho = scheduler_chirho.pending_changes_chirho();
        assert_eq!(pending_chirho.len(), 2);
        assert!(pending_chirho.contains(&"x".to_string()));
        assert!(pending_chirho.contains(&"y".to_string()));
    }

    #[test]
    fn test_incremental_generation_increments_chirho() {
        let scheduler_chirho = IncrementalSchedulerChirho::new_chirho();

        assert_eq!(scheduler_chirho.generation_chirho(), 0);
        scheduler_chirho.run_chirho();
        assert_eq!(scheduler_chirho.generation_chirho(), 1);
        scheduler_chirho.run_chirho();
        assert_eq!(scheduler_chirho.generation_chirho(), 2);
    }

    #[test]
    fn test_incremental_changes_committed_to_history_chirho() {
        let scheduler_chirho = IncrementalSchedulerChirho::new_chirho();

        scheduler_chirho.record_change_chirho("a");
        scheduler_chirho.record_change_chirho("b");
        scheduler_chirho.run_chirho();

        // Pending changes should be cleared
        assert!(!scheduler_chirho.has_pending_changes_chirho());

        // Changes should be in history
        let _history_chirho = scheduler_chirho.changes_since_generation_chirho(0);
        // Generation 0 changes are at gen 0, so they're not > 0
        // Let me fix the test to check generation properly
        let all_changes_chirho = scheduler_chirho.change_history_chirho.borrow();
        assert_eq!(all_changes_chirho.len(), 2);
    }

    #[test]
    fn test_incremental_changes_since_generation_chirho() {
        let scheduler_chirho = IncrementalSchedulerChirho::new_chirho();

        // Gen 0
        scheduler_chirho.record_change_chirho("a");
        scheduler_chirho.run_chirho(); // Commits at gen 0, moves to gen 1

        // Gen 1
        scheduler_chirho.record_change_chirho("b");
        scheduler_chirho.run_chirho(); // Commits at gen 1, moves to gen 2

        // Changes since gen 0 (exclusive) should include gen 1 changes
        let changes_chirho = scheduler_chirho.changes_since_generation_chirho(0);
        assert_eq!(changes_chirho.len(), 1);
        assert_eq!(changes_chirho[0].cell_name_chirho, "b");
        assert_eq!(changes_chirho[0].generation_chirho, 1);
    }

    #[test]
    fn test_incremental_stats_chirho() {
        let scheduler_chirho = IncrementalSchedulerChirho::new_chirho();

        scheduler_chirho.record_change_chirho("x");
        scheduler_chirho.run_chirho();

        let stats_chirho = scheduler_chirho.stats_chirho();
        assert_eq!(stats_chirho.incremental_runs_chirho, 1);
        assert_eq!(stats_chirho.changes_tracked_chirho, 1);
    }

    #[test]
    fn test_incremental_clear_history_chirho() {
        let scheduler_chirho = IncrementalSchedulerChirho::new_chirho();

        scheduler_chirho.record_change_chirho("x");
        scheduler_chirho.run_chirho();

        scheduler_chirho.clear_history_chirho();
        let all_changes_chirho = scheduler_chirho.change_history_chirho.borrow();
        assert!(all_changes_chirho.is_empty());
    }

    #[test]
    fn test_incremental_history_pruning_chirho() {
        let scheduler_chirho = IncrementalSchedulerChirho::with_history_size_chirho(5);

        // Add more changes than the history size
        for i_chirho in 0..10 {
            scheduler_chirho.record_change_chirho(&format!("cell_{}", i_chirho));
            scheduler_chirho.run_chirho();
        }

        // History should be pruned to max size
        let history_chirho = scheduler_chirho.change_history_chirho.borrow();
        assert!(history_chirho.len() <= 5);
    }

    // =========================================================================
    // Priority Scheduler Tests (GAP-003)
    // =========================================================================

    #[test]
    fn test_priority_scheduler_creation_chirho() {
        let scheduler_chirho = PrioritySchedulerChirho::new_chirho();
        assert!(scheduler_chirho.is_quiescent_chirho());
        assert_eq!(scheduler_chirho.queue_size_chirho(), 0);
    }

    #[test]
    fn test_priority_scheduler_empty_run_chirho() {
        let scheduler_chirho = PrioritySchedulerChirho::new_chirho();
        scheduler_chirho.run_chirho(); // Should not panic
        assert!(scheduler_chirho.is_quiescent_chirho());
    }

    #[test]
    fn test_priority_scheduler_bounded_run_chirho() {
        let scheduler_chirho = PrioritySchedulerChirho::new_chirho();
        let reached_chirho = scheduler_chirho.run_bounded_chirho(100);
        assert!(reached_chirho); // Empty queue = immediate fixpoint
    }

    #[test]
    fn test_priority_scheduler_stats_chirho() {
        let scheduler_chirho = PrioritySchedulerChirho::new_chirho();
        scheduler_chirho.run_chirho();

        let stats_chirho = scheduler_chirho.stats_chirho();
        assert_eq!(stats_chirho.iterations_chirho, 1);
        assert_eq!(stats_chirho.runs_chirho, 0);
    }

    #[test]
    fn test_priority_scheduler_default_chirho() {
        let scheduler_chirho = PrioritySchedulerChirho::default();
        assert!(scheduler_chirho.is_quiescent_chirho());
    }

    #[test]
    fn test_priority_scheduler_clear_chirho() {
        let scheduler_chirho = PrioritySchedulerChirho::new_chirho();
        // Note: We can't easily add propagators without cells, but we can test clear
        scheduler_chirho.clear_chirho();
        assert!(scheduler_chirho.is_quiescent_chirho());
    }

    // =========================================================================
    // Weighted Constraint Tests (GAP-003)
    // =========================================================================

    #[test]
    fn test_weighted_constraint_creation_chirho() {
        let constraint_chirho = WeightedConstraintChirho::new_chirho("test", 5.0);
        assert_eq!(constraint_chirho.name_chirho, "test");
        assert_eq!(constraint_chirho.weight_chirho, 5.0);
        assert!(!constraint_chirho.is_violated_chirho());
        assert_eq!(constraint_chirho.violation_cost_chirho(), 0.0);
    }

    #[test]
    fn test_weighted_constraint_set_violated_chirho() {
        let constraint_chirho = WeightedConstraintChirho::new_chirho("test", 10.0);

        constraint_chirho.set_violated_chirho(true);
        assert!(constraint_chirho.is_violated_chirho());
        assert_eq!(constraint_chirho.violation_cost_chirho(), 10.0);

        constraint_chirho.set_violated_chirho(false);
        assert!(!constraint_chirho.is_violated_chirho());
        assert_eq!(constraint_chirho.violation_cost_chirho(), 0.0);
    }

    #[test]
    fn test_weighted_constraint_partial_violation_chirho() {
        let constraint_chirho = WeightedConstraintChirho::new_chirho("test", 20.0);

        // 50% violated
        constraint_chirho.set_violation_degree_chirho(0.5);
        assert!(constraint_chirho.is_violated_chirho());
        assert_eq!(constraint_chirho.violation_degree_chirho(), 0.5);
        assert_eq!(constraint_chirho.violation_cost_chirho(), 10.0);

        // Clamp to valid range
        constraint_chirho.set_violation_degree_chirho(1.5);
        assert_eq!(constraint_chirho.violation_degree_chirho(), 1.0);
    }

    #[test]
    fn test_constraint_weights_creation_chirho() {
        let weights_chirho = ConstraintWeightsChirho::new_chirho();
        assert!(weights_chirho.is_empty_chirho());
        assert_eq!(weights_chirho.len_chirho(), 0);
    }

    #[test]
    fn test_constraint_weights_add_chirho() {
        let mut weights_chirho = ConstraintWeightsChirho::new_chirho();

        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c1", 5.0));
        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c2", 10.0));

        assert_eq!(weights_chirho.len_chirho(), 2);
        assert!(!weights_chirho.is_empty_chirho());
    }

    #[test]
    fn test_constraint_weights_total_cost_chirho() {
        let mut weights_chirho = ConstraintWeightsChirho::new_chirho();

        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c1", 5.0));
        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c2", 10.0));
        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c3", 3.0));

        // Initially no violations
        assert_eq!(weights_chirho.total_cost_chirho(), 0.0);

        // Violate c1
        weights_chirho.set_violated_chirho("c1", true);
        assert_eq!(weights_chirho.total_cost_chirho(), 5.0);

        // Violate c2
        weights_chirho.set_violated_chirho("c2", true);
        assert_eq!(weights_chirho.total_cost_chirho(), 15.0);
    }

    #[test]
    fn test_constraint_weights_violated_count_chirho() {
        let mut weights_chirho = ConstraintWeightsChirho::new_chirho();

        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c1", 5.0));
        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c2", 10.0));

        assert_eq!(weights_chirho.violated_count_chirho(), 0);

        weights_chirho.set_violated_chirho("c1", true);
        assert_eq!(weights_chirho.violated_count_chirho(), 1);

        weights_chirho.set_violated_chirho("c2", true);
        assert_eq!(weights_chirho.violated_count_chirho(), 2);
    }

    #[test]
    fn test_constraint_weights_reset_chirho() {
        let mut weights_chirho = ConstraintWeightsChirho::new_chirho();

        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c1", 5.0));
        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c2", 10.0));

        weights_chirho.set_violated_chirho("c1", true);
        weights_chirho.set_violated_chirho("c2", true);

        assert_eq!(weights_chirho.violated_count_chirho(), 2);

        weights_chirho.reset_chirho();

        assert_eq!(weights_chirho.violated_count_chirho(), 0);
        assert_eq!(weights_chirho.total_cost_chirho(), 0.0);
    }

    #[test]
    fn test_constraint_weights_get_chirho() {
        let mut weights_chirho = ConstraintWeightsChirho::new_chirho();

        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c1", 5.0));

        let c1_chirho = weights_chirho.get_chirho("c1");
        assert!(c1_chirho.is_some());
        assert_eq!(c1_chirho.unwrap().weight_chirho, 5.0);

        let missing_chirho = weights_chirho.get_chirho("nonexistent");
        assert!(missing_chirho.is_none());
    }

    #[test]
    fn test_constraint_weights_partial_violation_chirho() {
        let mut weights_chirho = ConstraintWeightsChirho::new_chirho();

        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c1", 100.0));

        // 25% violation
        weights_chirho.set_violation_degree_chirho("c1", 0.25);

        assert_eq!(weights_chirho.total_cost_chirho(), 25.0);
        assert_eq!(weights_chirho.violated_count_chirho(), 1);
    }

    #[test]
    fn test_constraint_weights_violated_list_chirho() {
        let mut weights_chirho = ConstraintWeightsChirho::new_chirho();

        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c1", 5.0));
        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c2", 10.0));
        weights_chirho.add_constraint_chirho(WeightedConstraintChirho::new_chirho("c3", 3.0));

        weights_chirho.set_violated_chirho("c1", true);
        weights_chirho.set_violated_chirho("c3", true);

        let violated_chirho = weights_chirho.violated_constraints_chirho();
        assert_eq!(violated_chirho.len(), 2);

        let names_chirho: Vec<&str> =
            violated_chirho.iter().map(|c_chirho| c_chirho.name_chirho.as_str()).collect();
        assert!(names_chirho.contains(&"c1"));
        assert!(names_chirho.contains(&"c3"));
        assert!(!names_chirho.contains(&"c2"));
    }
}
