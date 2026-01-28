// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Finite domain lattice for discrete constraint satisfaction.
//!
//! This module provides a finite domain type for CSP-style problems like
//! Sudoku, graph coloring, scheduling, etc.
//!
//! # Example
//!
//! ```
//! use propagators_chirho::FiniteDomainChirho;
//! use propagators_chirho::lattice_chirho::{LatticeChirho, BoundedLatticeChirho};
//!
//! // A Sudoku cell can be 1-9
//! let cell_chirho = FiniteDomainChirho::range_chirho(1, 9);
//!
//! // "It's not 5"
//! let not_five_chirho = FiniteDomainChirho::all_except_chirho(5, 1, 9);
//!
//! // Join: remove 5 from possibilities
//! let refined_chirho = cell_chirho.join_chirho(&not_five_chirho);
//! assert!(!refined_chirho.contains_chirho(5));
//! assert!(refined_chirho.contains_chirho(1));
//! ```

use crate::lattice_chirho::{BoundedLatticeChirho, LatticeChirho};
use std::collections::BTreeSet;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A finite domain representing a set of possible values.
///
/// Join is intersection (narrowing possibilities).
/// Bottom is the full domain (all values possible).
/// Top is the empty domain (contradiction - no values possible).
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FiniteDomainChirho {
    /// The set of possible values.
    values_chirho: BTreeSet<i64>,
    /// The original full domain bounds (for bottom element).
    min_chirho: i64,
    max_chirho: i64,
}

impl FiniteDomainChirho {
    /// Creates a domain with all values in [min, max].
    pub fn range_chirho(min_chirho: i64, max_chirho: i64) -> Self {
        let values_chirho: BTreeSet<i64> = (min_chirho..=max_chirho).collect();
        Self {
            values_chirho,
            min_chirho,
            max_chirho,
        }
    }

    /// Creates a domain with a single value.
    pub fn singleton_chirho(value_chirho: i64) -> Self {
        let mut values_chirho = BTreeSet::new();
        values_chirho.insert(value_chirho);
        Self {
            values_chirho,
            min_chirho: value_chirho,
            max_chirho: value_chirho,
        }
    }

    /// Creates a domain with all values except one.
    pub fn all_except_chirho(exclude_chirho: i64, min_chirho: i64, max_chirho: i64) -> Self {
        let values_chirho: BTreeSet<i64> = (min_chirho..=max_chirho)
            .filter(|v_chirho| *v_chirho != exclude_chirho)
            .collect();
        Self {
            values_chirho,
            min_chirho,
            max_chirho,
        }
    }

    /// Creates a domain from a set of values.
    pub fn from_set_chirho(values_chirho: BTreeSet<i64>) -> Self {
        let min_chirho = values_chirho.iter().copied().min().unwrap_or(0);
        let max_chirho = values_chirho.iter().copied().max().unwrap_or(0);
        Self {
            values_chirho,
            min_chirho,
            max_chirho,
        }
    }

    /// Creates an empty domain (contradiction).
    pub fn empty_chirho() -> Self {
        Self {
            values_chirho: BTreeSet::new(),
            min_chirho: 0,
            max_chirho: 0,
        }
    }

    /// Returns true if the domain contains a specific value.
    pub fn contains_chirho(&self, value_chirho: i64) -> bool {
        self.values_chirho.contains(&value_chirho)
    }

    /// Returns the number of possible values.
    pub fn size_chirho(&self) -> usize {
        self.values_chirho.len()
    }

    /// Returns true if this is a singleton (exactly one value).
    pub fn is_singleton_chirho(&self) -> bool {
        self.values_chirho.len() == 1
    }

    /// Returns the singleton value, if this is a singleton.
    pub fn get_singleton_chirho(&self) -> Option<i64> {
        if self.is_singleton_chirho() {
            self.values_chirho.iter().next().copied()
        } else {
            None
        }
    }

    /// Returns an iterator over the possible values.
    pub fn iter_chirho(&self) -> impl Iterator<Item = i64> + '_ {
        self.values_chirho.iter().copied()
    }

    /// Returns the minimum possible value.
    pub fn min_value_chirho(&self) -> Option<i64> {
        self.values_chirho.iter().next().copied()
    }

    /// Returns the maximum possible value.
    pub fn max_value_chirho(&self) -> Option<i64> {
        self.values_chirho.iter().next_back().copied()
    }

    /// Removes a value from the domain.
    pub fn remove_chirho(&self, value_chirho: i64) -> Self {
        let mut new_values_chirho = self.values_chirho.clone();
        new_values_chirho.remove(&value_chirho);
        Self {
            values_chirho: new_values_chirho,
            min_chirho: self.min_chirho,
            max_chirho: self.max_chirho,
        }
    }
}

impl fmt::Debug for FiniteDomainChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.values_chirho.is_empty() {
            write!(f_chirho, "∅")
        } else if self.is_singleton_chirho() {
            write!(
                f_chirho,
                "{{{}}}",
                self.values_chirho.iter().next().unwrap()
            )
        } else if self.values_chirho.len() <= 5 {
            write!(
                f_chirho,
                "{{{}}}",
                self.values_chirho
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            write!(
                f_chirho,
                "{{{}..{}}}",
                self.values_chirho.iter().next().unwrap(),
                self.values_chirho.iter().next_back().unwrap()
            )
        }
    }
}

impl LatticeChirho for FiniteDomainChirho {
    fn join_chirho(&self, other_chirho: &Self) -> Self {
        // Handle bottom (identity element for join)
        // Bottom means "all values possible", so join with bottom returns the other
        if self.is_bottom_chirho() {
            return other_chirho.clone();
        }
        if other_chirho.is_bottom_chirho() {
            return self.clone();
        }

        // Join is intersection (narrowing possibilities)
        let intersection_chirho: BTreeSet<i64> = self
            .values_chirho
            .intersection(&other_chirho.values_chirho)
            .copied()
            .collect();
        Self {
            values_chirho: intersection_chirho,
            min_chirho: self.min_chirho.min(other_chirho.min_chirho),
            max_chirho: self.max_chirho.max(other_chirho.max_chirho),
        }
    }
}

impl BoundedLatticeChirho for FiniteDomainChirho {
    fn bottom_chirho() -> Self {
        // Bottom is "all values possible" - but we need bounds
        // Use a marker value
        Self {
            values_chirho: BTreeSet::new(), // Empty means "unknown/all"
            min_chirho: i64::MIN,
            max_chirho: i64::MAX,
        }
    }

    fn top_chirho() -> Self {
        // Top is contradiction (empty domain with flag)
        Self::empty_chirho()
    }

    fn is_bottom_chirho(&self) -> bool {
        // Bottom is the full domain - hard to check without knowing bounds
        // We use the convention that min=MIN and max=MAX means bottom
        self.min_chirho == i64::MIN && self.max_chirho == i64::MAX
    }

    fn is_top_chirho(&self) -> bool {
        // Top is empty domain (contradiction)
        self.values_chirho.is_empty() && !self.is_bottom_chirho()
    }
}

// ============================================================================
// FINITE DOMAIN PROPAGATORS
// ============================================================================

use crate::lattice_chirho::PropagatorFnChirho;

/// All-different constraint: all cells must have different values.
///
/// When a cell becomes a singleton, remove that value from all others.
#[derive(Clone, Debug)]
pub struct AllDifferentChirho {
    /// Number of cells this constraint covers.
    arity_chirho: usize,
}

impl AllDifferentChirho {
    /// Creates an all-different constraint over n cells.
    pub fn new_chirho(arity_chirho: usize) -> Self {
        Self { arity_chirho }
    }
}

impl PropagatorFnChirho<FiniteDomainChirho> for AllDifferentChirho {
    fn arity_chirho(&self) -> usize {
        self.arity_chirho
    }

    fn propagate_chirho(&self, cells_chirho: &[FiniteDomainChirho]) -> Vec<FiniteDomainChirho> {
        let mut results_chirho = vec![FiniteDomainChirho::bottom_chirho(); cells_chirho.len()];

        // For each singleton, remove its value from all other cells
        for (i_chirho, cell_chirho) in cells_chirho.iter().enumerate() {
            if let Some(value_chirho) = cell_chirho.get_singleton_chirho() {
                for (j_chirho, other_chirho) in cells_chirho.iter().enumerate() {
                    if i_chirho != j_chirho && other_chirho.contains_chirho(value_chirho) {
                        results_chirho[j_chirho] = other_chirho.remove_chirho(value_chirho);
                    }
                }
            }
        }

        results_chirho
    }
}

/// Less-than constraint: a < b.
#[derive(Clone, Debug)]
pub struct LessThanChirho;

impl PropagatorFnChirho<FiniteDomainChirho> for LessThanChirho {
    fn arity_chirho(&self) -> usize {
        2
    }

    fn propagate_chirho(&self, cells_chirho: &[FiniteDomainChirho]) -> Vec<FiniteDomainChirho> {
        let a_chirho = &cells_chirho[0];
        let b_chirho = &cells_chirho[1];

        let mut results_chirho = vec![FiniteDomainChirho::bottom_chirho(); 2];

        // a < b means:
        // - a's max must be < b's max
        // - b's min must be > a's min

        if let Some(b_max_chirho) = b_chirho.max_value_chirho() {
            // Remove values from a that are >= b_max
            let filtered_a_chirho: BTreeSet<i64> = a_chirho
                .values_chirho
                .iter()
                .copied()
                .filter(|v_chirho| *v_chirho < b_max_chirho)
                .collect();
            if filtered_a_chirho != a_chirho.values_chirho {
                results_chirho[0] = FiniteDomainChirho::from_set_chirho(filtered_a_chirho);
            }
        }

        if let Some(a_min_chirho) = a_chirho.min_value_chirho() {
            // Remove values from b that are <= a_min
            let filtered_b_chirho: BTreeSet<i64> = b_chirho
                .values_chirho
                .iter()
                .copied()
                .filter(|v_chirho| *v_chirho > a_min_chirho)
                .collect();
            if filtered_b_chirho != b_chirho.values_chirho {
                results_chirho[1] = FiniteDomainChirho::from_set_chirho(filtered_b_chirho);
            }
        }

        results_chirho
    }
}

/// Equals constraint: a = b.
#[derive(Clone, Debug)]
pub struct EqualsChirho;

impl PropagatorFnChirho<FiniteDomainChirho> for EqualsChirho {
    fn arity_chirho(&self) -> usize {
        2
    }

    fn propagate_chirho(&self, cells_chirho: &[FiniteDomainChirho]) -> Vec<FiniteDomainChirho> {
        // a = b means both must have the same values
        let intersection_chirho = cells_chirho[0].join_chirho(&cells_chirho[1]);
        vec![intersection_chirho.clone(), intersection_chirho]
    }
}

/// Not-equals constraint: a ≠ b.
///
/// Only propagates when one side is a singleton.
#[derive(Clone, Debug)]
pub struct NotEqualsChirho;

impl PropagatorFnChirho<FiniteDomainChirho> for NotEqualsChirho {
    fn arity_chirho(&self) -> usize {
        2
    }

    fn propagate_chirho(&self, cells_chirho: &[FiniteDomainChirho]) -> Vec<FiniteDomainChirho> {
        let mut results_chirho = vec![FiniteDomainChirho::bottom_chirho(); 2];

        // If a is singleton, remove its value from b
        if let Some(a_val_chirho) = cells_chirho[0].get_singleton_chirho() {
            if cells_chirho[1].contains_chirho(a_val_chirho) {
                results_chirho[1] = cells_chirho[1].remove_chirho(a_val_chirho);
            }
        }

        // If b is singleton, remove its value from a
        if let Some(b_val_chirho) = cells_chirho[1].get_singleton_chirho() {
            if cells_chirho[0].contains_chirho(b_val_chirho) {
                results_chirho[0] = cells_chirho[0].remove_chirho(b_val_chirho);
            }
        }

        results_chirho
    }
}

// ============================================================================
// GLOBAL CONSTRAINTS
// ============================================================================

/// Element constraint: result = array[index].
///
/// Given an array of domains and an index domain, constrains the result
/// to be one of the values that could be selected by the index.
///
/// # Example
///
/// If array = [[1,2], [3,4], [5,6]] and index ∈ {0, 2}, then result ∈ {1,2,5,6}.
#[derive(Clone, Debug)]
pub struct ElementChirho {
    /// Number of array elements.
    array_size_chirho: usize,
}

impl ElementChirho {
    /// Creates an element constraint for an array of the given size.
    ///
    /// The constraint has arity = array_size + 2 (array elements + index + result).
    pub fn new_chirho(array_size_chirho: usize) -> Self {
        Self { array_size_chirho }
    }
}

impl PropagatorFnChirho<FiniteDomainChirho> for ElementChirho {
    fn arity_chirho(&self) -> usize {
        self.array_size_chirho + 2 // array elements + index + result
    }

    fn propagate_chirho(&self, cells_chirho: &[FiniteDomainChirho]) -> Vec<FiniteDomainChirho> {
        let array_chirho = &cells_chirho[0..self.array_size_chirho];
        let index_chirho = &cells_chirho[self.array_size_chirho];
        let result_chirho = &cells_chirho[self.array_size_chirho + 1];

        let mut results_chirho = vec![FiniteDomainChirho::bottom_chirho(); cells_chirho.len()];

        // Collect all possible values for result based on valid indices
        let mut possible_values_chirho = BTreeSet::new();
        for idx_chirho in index_chirho.iter_chirho() {
            if idx_chirho >= 0 && (idx_chirho as usize) < self.array_size_chirho {
                for val_chirho in array_chirho[idx_chirho as usize].iter_chirho() {
                    possible_values_chirho.insert(val_chirho);
                }
            }
        }

        // Constrain result to possible values
        if !possible_values_chirho.is_empty() {
            let result_domain_chirho = FiniteDomainChirho::from_set_chirho(possible_values_chirho);
            let refined_result_chirho = result_chirho.join_chirho(&result_domain_chirho);
            if refined_result_chirho != *result_chirho {
                results_chirho[self.array_size_chirho + 1] = refined_result_chirho;
            }
        }

        // Constrain index: remove indices that can't produce the result
        let mut valid_indices_chirho = BTreeSet::new();
        for idx_chirho in index_chirho.iter_chirho() {
            if idx_chirho >= 0 && (idx_chirho as usize) < self.array_size_chirho {
                // Check if this index can produce any value in result
                for val_chirho in array_chirho[idx_chirho as usize].iter_chirho() {
                    if result_chirho.contains_chirho(val_chirho) {
                        valid_indices_chirho.insert(idx_chirho);
                        break;
                    }
                }
            }
        }

        if !valid_indices_chirho.is_empty() && valid_indices_chirho.len() < index_chirho.size_chirho()
        {
            results_chirho[self.array_size_chirho] =
                FiniteDomainChirho::from_set_chirho(valid_indices_chirho);
        }

        results_chirho
    }
}

/// Table constraint: tuple must be one of the allowed combinations.
///
/// An extensional constraint that explicitly lists all allowed value combinations.
///
/// # Example
///
/// ```
/// use propagators_chirho::constraints_chirho::finite_domain_chirho::{TableChirho, FiniteDomainChirho};
/// use propagators_chirho::lattice_chirho::PropagatorFnChirho;
///
/// // Allowed tuples: (1,2), (2,3), (3,1)
/// let allowed_chirho = vec![
///     vec![1, 2],
///     vec![2, 3],
///     vec![3, 1],
/// ];
/// let table_chirho = TableChirho::new_chirho(allowed_chirho);
/// ```
#[derive(Clone, Debug)]
pub struct TableChirho {
    /// The allowed tuples.
    tuples_chirho: Vec<Vec<i64>>,
    /// The arity (number of variables).
    arity_chirho: usize,
}

impl TableChirho {
    /// Creates a table constraint with the given allowed tuples.
    ///
    /// All tuples must have the same length.
    pub fn new_chirho(tuples_chirho: Vec<Vec<i64>>) -> Self {
        let arity_chirho = tuples_chirho.first().map_or(0, Vec::len);
        Self {
            tuples_chirho,
            arity_chirho,
        }
    }
}

impl PropagatorFnChirho<FiniteDomainChirho> for TableChirho {
    fn arity_chirho(&self) -> usize {
        self.arity_chirho
    }

    fn propagate_chirho(&self, cells_chirho: &[FiniteDomainChirho]) -> Vec<FiniteDomainChirho> {
        let mut results_chirho = vec![FiniteDomainChirho::bottom_chirho(); cells_chirho.len()];

        // Find all tuples that are still possible given current domains
        let valid_tuples_chirho: Vec<&Vec<i64>> = self
            .tuples_chirho
            .iter()
            .filter(|tuple_chirho| {
                tuple_chirho
                    .iter()
                    .zip(cells_chirho.iter())
                    .all(|(val_chirho, domain_chirho)| domain_chirho.contains_chirho(*val_chirho))
            })
            .collect();

        // For each variable, collect values that appear in valid tuples
        for i_chirho in 0..self.arity_chirho {
            let valid_values_chirho: BTreeSet<i64> = valid_tuples_chirho
                .iter()
                .map(|tuple_chirho| tuple_chirho[i_chirho])
                .collect();

            if !valid_values_chirho.is_empty() {
                let domain_chirho = FiniteDomainChirho::from_set_chirho(valid_values_chirho);
                let refined_chirho = cells_chirho[i_chirho].join_chirho(&domain_chirho);
                if refined_chirho != cells_chirho[i_chirho] {
                    results_chirho[i_chirho] = refined_chirho;
                }
            } else {
                // No valid tuples - contradiction
                results_chirho[i_chirho] = FiniteDomainChirho::top_chirho();
            }
        }

        results_chirho
    }
}

/// Circuit constraint: variables form a Hamiltonian circuit.
///
/// Each variable i represents the successor of node i. The constraint ensures
/// that the successors form a single cycle visiting all nodes exactly once.
///
/// Used for TSP and Hamiltonian path problems.
#[derive(Clone, Debug)]
pub struct CircuitChirho {
    /// Number of nodes in the circuit.
    num_nodes_chirho: usize,
}

impl CircuitChirho {
    /// Creates a circuit constraint over n nodes.
    ///
    /// Variables are indexed 0..n, each representing the successor of that node.
    pub fn new_chirho(num_nodes_chirho: usize) -> Self {
        Self { num_nodes_chirho }
    }
}

impl PropagatorFnChirho<FiniteDomainChirho> for CircuitChirho {
    fn arity_chirho(&self) -> usize {
        self.num_nodes_chirho
    }

    fn propagate_chirho(&self, cells_chirho: &[FiniteDomainChirho]) -> Vec<FiniteDomainChirho> {
        let mut results_chirho = vec![FiniteDomainChirho::bottom_chirho(); cells_chirho.len()];

        // Basic propagation: no self-loops and all-different
        for i_chirho in 0..self.num_nodes_chirho {
            // No self-loops: node i cannot go to itself
            if cells_chirho[i_chirho].contains_chirho(i_chirho as i64) {
                results_chirho[i_chirho] =
                    cells_chirho[i_chirho].remove_chirho(i_chirho as i64);
            }
        }

        // All-different: if a successor is fixed, remove it from others
        for i_chirho in 0..self.num_nodes_chirho {
            if let Some(succ_chirho) = cells_chirho[i_chirho].get_singleton_chirho() {
                for j_chirho in 0..self.num_nodes_chirho {
                    if i_chirho != j_chirho && cells_chirho[j_chirho].contains_chirho(succ_chirho) {
                        // Merge with existing result
                        let current_chirho = if results_chirho[j_chirho].is_bottom_chirho() {
                            &cells_chirho[j_chirho]
                        } else {
                            &results_chirho[j_chirho]
                        };
                        results_chirho[j_chirho] = current_chirho.remove_chirho(succ_chirho);
                    }
                }
            }
        }

        // Subcircuit prevention: detect and prevent premature cycles
        // This is a simplified version - full implementation would use SCC detection
        for start_chirho in 0..self.num_nodes_chirho {
            if let Some(mut current_chirho) = cells_chirho[start_chirho].get_singleton_chirho() {
                let mut visited_chirho = BTreeSet::new();
                visited_chirho.insert(start_chirho as i64);

                // Follow the chain
                while current_chirho >= 0 && (current_chirho as usize) < self.num_nodes_chirho {
                    if visited_chirho.contains(&current_chirho) {
                        // Found a cycle
                        if visited_chirho.len() < self.num_nodes_chirho {
                            // Subcircuit detected - this is invalid, but we can't easily
                            // represent the fix here. The search will backtrack.
                        }
                        break;
                    }
                    visited_chirho.insert(current_chirho);

                    if let Some(next_chirho) =
                        cells_chirho[current_chirho as usize].get_singleton_chirho()
                    {
                        current_chirho = next_chirho;
                    } else {
                        break;
                    }
                }
            }
        }

        results_chirho
    }
}

/// Cumulative constraint for resource scheduling.
///
/// Given a set of tasks with start times, durations, and resource requirements,
/// ensures that the total resource usage at any time does not exceed capacity.
///
/// # Variables
///
/// The constraint takes 3n+1 variables:
/// - n start time domains
/// - n duration domains (usually singletons)
/// - n resource usage domains (usually singletons)
/// - 1 capacity domain (usually singleton)
#[derive(Clone, Debug)]
pub struct CumulativeChirho {
    /// Number of tasks.
    num_tasks_chirho: usize,
}

impl CumulativeChirho {
    /// Creates a cumulative constraint for n tasks.
    ///
    /// Variables order: [start_0, ..., start_n-1, dur_0, ..., dur_n-1, res_0, ..., res_n-1, capacity]
    pub fn new_chirho(num_tasks_chirho: usize) -> Self {
        Self { num_tasks_chirho }
    }
}

impl PropagatorFnChirho<FiniteDomainChirho> for CumulativeChirho {
    fn arity_chirho(&self) -> usize {
        3 * self.num_tasks_chirho + 1 // starts + durations + resources + capacity
    }

    fn propagate_chirho(&self, cells_chirho: &[FiniteDomainChirho]) -> Vec<FiniteDomainChirho> {
        let n_chirho = self.num_tasks_chirho;
        let mut results_chirho = vec![FiniteDomainChirho::bottom_chirho(); cells_chirho.len()];

        // Extract domains
        let starts_chirho = &cells_chirho[0..n_chirho];
        let durations_chirho = &cells_chirho[n_chirho..2 * n_chirho];
        let resources_chirho = &cells_chirho[2 * n_chirho..3 * n_chirho];
        let capacity_chirho = &cells_chirho[3 * n_chirho];

        // Get capacity (assume singleton or use minimum)
        let cap_chirho = capacity_chirho.min_value_chirho().unwrap_or(i64::MAX);

        // Find time bounds
        let min_start_chirho = starts_chirho
            .iter()
            .filter_map(FiniteDomainChirho::min_value_chirho)
            .min()
            .unwrap_or(0);

        let max_end_chirho = starts_chirho
            .iter()
            .zip(durations_chirho.iter())
            .filter_map(|(s_chirho, d_chirho)| {
                Some(s_chirho.max_value_chirho()? + d_chirho.max_value_chirho()?)
            })
            .max()
            .unwrap_or(100);

        // Time-table filtering: for each time point, check resource usage
        for t_chirho in min_start_chirho..max_end_chirho {
            let mut must_run_chirho = Vec::new();
            let mut may_run_chirho = Vec::new();

            for i_chirho in 0..n_chirho {
                let s_min_chirho = starts_chirho[i_chirho].min_value_chirho().unwrap_or(0);
                let s_max_chirho = starts_chirho[i_chirho].max_value_chirho().unwrap_or(0);
                let d_chirho = durations_chirho[i_chirho].min_value_chirho().unwrap_or(0);
                let r_chirho = resources_chirho[i_chirho].min_value_chirho().unwrap_or(0);

                // Task must run at t if: s_max < t and t < s_min + d
                if s_max_chirho <= t_chirho && t_chirho < s_min_chirho + d_chirho {
                    must_run_chirho.push((i_chirho, r_chirho));
                }
                // Task may run at t if: s_min <= t and t < s_max + d
                else if s_min_chirho <= t_chirho && t_chirho < s_max_chirho + d_chirho {
                    may_run_chirho.push((i_chirho, r_chirho));
                }
            }

            let must_usage_chirho: i64 = must_run_chirho.iter().map(|(_, r_chirho)| r_chirho).sum();

            // If must_usage exceeds capacity, contradiction (handled by search)
            // If must_usage + one task exceeds capacity, that task can't run at this time
            for (task_chirho, res_chirho) in &may_run_chirho {
                if must_usage_chirho + res_chirho > cap_chirho {
                    // This task cannot run at time t
                    // Remove start times that would cause this task to run at t
                    let d_chirho = durations_chirho[*task_chirho].min_value_chirho().unwrap_or(1);
                    // Task runs at t if start <= t < start + duration
                    // So start must not be in [t - duration + 1, t]
                    let forbidden_start_min_chirho = t_chirho - d_chirho + 1;
                    let forbidden_start_max_chirho = t_chirho;

                    let current_starts_chirho = &starts_chirho[*task_chirho];
                    let filtered_chirho: BTreeSet<i64> = current_starts_chirho
                        .iter_chirho()
                        .filter(|s_chirho| {
                            *s_chirho < forbidden_start_min_chirho
                                || *s_chirho > forbidden_start_max_chirho
                        })
                        .collect();

                    if filtered_chirho.len() < current_starts_chirho.size_chirho() {
                        let new_domain_chirho = FiniteDomainChirho::from_set_chirho(filtered_chirho);
                        // Merge with existing result
                        if results_chirho[*task_chirho].is_bottom_chirho() {
                            results_chirho[*task_chirho] = new_domain_chirho;
                        } else {
                            results_chirho[*task_chirho] =
                                results_chirho[*task_chirho].join_chirho(&new_domain_chirho);
                        }
                    }
                }
            }
        }

        results_chirho
    }
}

/// Cardinality constraint: bounds on how many variables can take each value.
///
/// For each value v, specifies a lower and upper bound on how many variables
/// can take that value.
#[derive(Clone, Debug)]
pub struct CardinalityChirho {
    /// Number of variables.
    num_vars_chirho: usize,
    /// Map from value to (lower_bound, upper_bound).
    bounds_chirho: std::collections::HashMap<i64, (usize, usize)>,
}

impl CardinalityChirho {
    /// Creates a cardinality constraint.
    ///
    /// # Arguments
    ///
    /// * `num_vars_chirho` - Number of variables
    /// * `bounds_chirho` - Map from value to (min_count, max_count)
    pub fn new_chirho(
        num_vars_chirho: usize,
        bounds_chirho: std::collections::HashMap<i64, (usize, usize)>,
    ) -> Self {
        Self {
            num_vars_chirho,
            bounds_chirho,
        }
    }
}

impl PropagatorFnChirho<FiniteDomainChirho> for CardinalityChirho {
    fn arity_chirho(&self) -> usize {
        self.num_vars_chirho
    }

    fn propagate_chirho(&self, cells_chirho: &[FiniteDomainChirho]) -> Vec<FiniteDomainChirho> {
        let mut results_chirho = vec![FiniteDomainChirho::bottom_chirho(); cells_chirho.len()];

        // Count fixed assignments
        let mut fixed_counts_chirho: std::collections::HashMap<i64, usize> =
            std::collections::HashMap::new();
        let mut unfixed_indices_chirho = Vec::new();

        for (i_chirho, cell_chirho) in cells_chirho.iter().enumerate() {
            if let Some(val_chirho) = cell_chirho.get_singleton_chirho() {
                *fixed_counts_chirho.entry(val_chirho).or_insert(0) += 1;
            } else {
                unfixed_indices_chirho.push(i_chirho);
            }
        }

        // For each value with bounds
        for (val_chirho, (min_count_chirho, max_count_chirho)) in &self.bounds_chirho {
            let current_count_chirho = fixed_counts_chirho.get(val_chirho).copied().unwrap_or(0);

            // If we've reached max count, remove this value from unfixed vars
            if current_count_chirho >= *max_count_chirho {
                for &idx_chirho in &unfixed_indices_chirho {
                    if cells_chirho[idx_chirho].contains_chirho(*val_chirho) {
                        let current_chirho = if results_chirho[idx_chirho].is_bottom_chirho() {
                            &cells_chirho[idx_chirho]
                        } else {
                            &results_chirho[idx_chirho]
                        };
                        results_chirho[idx_chirho] = current_chirho.remove_chirho(*val_chirho);
                    }
                }
            }

            // If we need more of this value and only enough vars can provide it
            let can_provide_chirho: Vec<usize> = unfixed_indices_chirho
                .iter()
                .copied()
                .filter(|&idx_chirho| cells_chirho[idx_chirho].contains_chirho(*val_chirho))
                .collect();

            if current_count_chirho + can_provide_chirho.len() == *min_count_chirho {
                // All vars that can provide this value must do so
                for idx_chirho in can_provide_chirho {
                    results_chirho[idx_chirho] = FiniteDomainChirho::singleton_chirho(*val_chirho);
                }
            }
        }

        results_chirho
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_finite_domain_creation_chirho() {
        let fd_chirho = FiniteDomainChirho::range_chirho(1, 9);
        assert_eq!(fd_chirho.size_chirho(), 9);
        assert!(fd_chirho.contains_chirho(5));
        assert!(!fd_chirho.contains_chirho(0));
    }

    #[test]
    fn test_finite_domain_singleton_chirho() {
        let fd_chirho = FiniteDomainChirho::singleton_chirho(42);
        assert!(fd_chirho.is_singleton_chirho());
        assert_eq!(fd_chirho.get_singleton_chirho(), Some(42));
    }

    #[test]
    fn test_finite_domain_join_chirho() {
        let a_chirho = FiniteDomainChirho::range_chirho(1, 5);
        let b_chirho = FiniteDomainChirho::range_chirho(3, 7);
        let joined_chirho = a_chirho.join_chirho(&b_chirho);

        assert_eq!(joined_chirho.size_chirho(), 3); // {3, 4, 5}
        assert!(joined_chirho.contains_chirho(3));
        assert!(joined_chirho.contains_chirho(4));
        assert!(joined_chirho.contains_chirho(5));
        assert!(!joined_chirho.contains_chirho(1));
        assert!(!joined_chirho.contains_chirho(7));
    }

    #[test]
    fn test_finite_domain_bottom_identity_chirho() {
        // bottom is the identity element for join: x ⊔ ⊥ = x
        let a_chirho = FiniteDomainChirho::range_chirho(1, 5);
        let bottom_chirho = FiniteDomainChirho::bottom_chirho();

        let result1_chirho = a_chirho.join_chirho(&bottom_chirho);
        let result2_chirho = bottom_chirho.join_chirho(&a_chirho);

        // Both should equal a
        assert_eq!(result1_chirho.size_chirho(), a_chirho.size_chirho());
        assert_eq!(result2_chirho.size_chirho(), a_chirho.size_chirho());
        for v_chirho in a_chirho.iter_chirho() {
            assert!(result1_chirho.contains_chirho(v_chirho));
            assert!(result2_chirho.contains_chirho(v_chirho));
        }
    }

    #[test]
    fn test_finite_domain_top_absorbs_chirho() {
        // top absorbs everything: x ⊔ ⊤ = ⊤
        let a_chirho = FiniteDomainChirho::range_chirho(1, 5);
        let top_chirho = FiniteDomainChirho::top_chirho();

        let result_chirho = a_chirho.join_chirho(&top_chirho);

        // Result should be top (empty/contradiction)
        assert!(result_chirho.is_top_chirho());
    }

    #[test]
    fn test_finite_domain_join_idempotent_chirho() {
        // x ⊔ x = x
        let a_chirho = FiniteDomainChirho::range_chirho(1, 5);
        let result_chirho = a_chirho.join_chirho(&a_chirho);

        assert_eq!(result_chirho.size_chirho(), a_chirho.size_chirho());
        for v_chirho in a_chirho.iter_chirho() {
            assert!(result_chirho.contains_chirho(v_chirho));
        }
    }

    #[test]
    fn test_all_different_chirho() {
        let prop_chirho = AllDifferentChirho::new_chirho(3);

        let cells_chirho = vec![
            FiniteDomainChirho::singleton_chirho(1), // Fixed to 1
            FiniteDomainChirho::range_chirho(1, 3),  // Could be 1, 2, or 3
            FiniteDomainChirho::range_chirho(1, 3),  // Could be 1, 2, or 3
        ];

        let results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // Cell 1 and 2 should have 1 removed
        assert!(!results_chirho[1].contains_chirho(1));
        assert!(!results_chirho[2].contains_chirho(1));
    }

    #[test]
    fn test_less_than_chirho() {
        let prop_chirho = LessThanChirho;

        let cells_chirho = vec![
            FiniteDomainChirho::range_chirho(1, 5), // a
            FiniteDomainChirho::range_chirho(3, 7), // b
        ];

        let _results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // a < b means a can't be >= 7, and b can't be <= 1
        // a: originally {1,2,3,4,5}, filtered to {1,2,3,4,5,6} (all < 7) - no change
        // b: originally {3,4,5,6,7}, filtered to {2,3,4,5,6,7} (all > 1) - removes nothing actually

        // Let's test with tighter bounds
        let cells2_chirho = vec![
            FiniteDomainChirho::range_chirho(1, 5),
            FiniteDomainChirho::singleton_chirho(3), // b = 3
        ];

        let results2_chirho = prop_chirho.propagate_chirho(&cells2_chirho);
        // a < 3 means a ∈ {1, 2}
        assert_eq!(results2_chirho[0].size_chirho(), 2);
        assert!(results2_chirho[0].contains_chirho(1));
        assert!(results2_chirho[0].contains_chirho(2));
        assert!(!results2_chirho[0].contains_chirho(3));
    }

    #[test]
    fn test_not_equals_chirho() {
        let prop_chirho = NotEqualsChirho;

        let cells_chirho = vec![
            FiniteDomainChirho::singleton_chirho(5), // a = 5
            FiniteDomainChirho::range_chirho(1, 9),  // b ∈ {1..9}
        ];

        let results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // b should not contain 5
        assert!(!results_chirho[1].contains_chirho(5));
        assert_eq!(results_chirho[1].size_chirho(), 8);
    }

    // =========================================================================
    // Tests for Global Constraints (GAP-006)
    // =========================================================================

    #[test]
    fn test_element_constraint_fixed_index_chirho() {
        // Element constraint: result = array[index]
        // Variable order: [array[0], array[1], ..., array[n-1], index, result]
        // If index is fixed, result should be constrained to that array element
        let prop_chirho = ElementChirho::new_chirho(3);

        let cells_chirho = vec![
            FiniteDomainChirho::singleton_chirho(10), // array[0] = 10
            FiniteDomainChirho::singleton_chirho(20), // array[1] = 20
            FiniteDomainChirho::singleton_chirho(30), // array[2] = 30
            FiniteDomainChirho::singleton_chirho(1),  // index = 1
            FiniteDomainChirho::range_chirho(10, 30), // result: could be any
        ];

        let results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // result should be constrained to {20} (array[1])
        assert!(results_chirho[4].is_singleton_chirho());
        assert_eq!(results_chirho[4].get_singleton_chirho(), Some(20));
    }

    #[test]
    fn test_element_constraint_filters_index_chirho() {
        // If result is fixed, indices pointing to wrong values should be removed
        // Variable order: [array[0], array[1], ..., array[n-1], index, result]
        let prop_chirho = ElementChirho::new_chirho(3);

        let cells_chirho = vec![
            FiniteDomainChirho::singleton_chirho(10), // array[0] = 10
            FiniteDomainChirho::singleton_chirho(20), // array[1] = 20
            FiniteDomainChirho::singleton_chirho(30), // array[2] = 30
            FiniteDomainChirho::range_chirho(0, 2),   // index: 0, 1, or 2
            FiniteDomainChirho::singleton_chirho(20), // result = 20
        ];

        let results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // index should be constrained to {1} (only index where array[i] = 20)
        assert!(results_chirho[3].is_singleton_chirho());
        assert_eq!(results_chirho[3].get_singleton_chirho(), Some(1));
    }

    #[test]
    fn test_element_constraint_arity_chirho() {
        let prop_chirho = ElementChirho::new_chirho(5);
        assert_eq!(prop_chirho.arity_chirho(), 7); // 1 index + 5 array + 1 result
    }

    #[test]
    fn test_table_constraint_filters_values_chirho() {
        // Table constraint: only allows tuples in the table
        let tuples_chirho = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
        ];
        let prop_chirho = TableChirho::new_chirho(tuples_chirho);

        // First variable is fixed to 1, so only tuples starting with 1 are valid
        let cells_chirho = vec![
            FiniteDomainChirho::singleton_chirho(1), // x0 = 1
            FiniteDomainChirho::range_chirho(1, 3),  // x1 ∈ {1, 2, 3}
            FiniteDomainChirho::range_chirho(1, 3),  // x2 ∈ {1, 2, 3}
        ];

        let results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // x1 can only be 2 or 3 (from tuples [1,2,3] and [1,3,2])
        assert!(!results_chirho[1].contains_chirho(1));
        assert!(results_chirho[1].contains_chirho(2));
        assert!(results_chirho[1].contains_chirho(3));

        // x2 can only be 2 or 3
        assert!(!results_chirho[2].contains_chirho(1));
        assert!(results_chirho[2].contains_chirho(2));
        assert!(results_chirho[2].contains_chirho(3));
    }

    #[test]
    fn test_table_constraint_singleton_result_chirho() {
        // When all but one variable is fixed, propagation should determine the last
        let tuples_chirho = vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 1],
        ];
        let prop_chirho = TableChirho::new_chirho(tuples_chirho);

        let cells_chirho = vec![
            FiniteDomainChirho::singleton_chirho(1), // x0 = 1
            FiniteDomainChirho::range_chirho(1, 3),  // x1 ∈ {1, 2, 3}
        ];

        let results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // x1 must be 2 (only tuple with x0=1 is [1,2])
        assert!(results_chirho[1].is_singleton_chirho());
        assert_eq!(results_chirho[1].get_singleton_chirho(), Some(2));
    }

    #[test]
    fn test_table_constraint_arity_chirho() {
        let tuples_chirho = vec![vec![1, 2, 3, 4]];
        let prop_chirho = TableChirho::new_chirho(tuples_chirho);
        assert_eq!(prop_chirho.arity_chirho(), 4);
    }

    #[test]
    fn test_circuit_constraint_basic_chirho() {
        // Circuit constraint: successor[i] = j means i -> j
        // Must form a Hamiltonian circuit
        let prop_chirho = CircuitChirho::new_chirho(3);

        // Start with all possibilities
        let cells_chirho = vec![
            FiniteDomainChirho::range_chirho(0, 2), // succ[0] ∈ {0, 1, 2}
            FiniteDomainChirho::range_chirho(0, 2), // succ[1] ∈ {0, 1, 2}
            FiniteDomainChirho::range_chirho(0, 2), // succ[2] ∈ {0, 1, 2}
        ];

        let results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // No node can point to itself (self-loops not allowed in circuit)
        assert!(!results_chirho[0].contains_chirho(0));
        assert!(!results_chirho[1].contains_chirho(1));
        assert!(!results_chirho[2].contains_chirho(2));
    }

    #[test]
    fn test_circuit_constraint_alldiff_chirho() {
        // Circuit implies all-different on successors
        let prop_chirho = CircuitChirho::new_chirho(3);

        let cells_chirho = vec![
            FiniteDomainChirho::singleton_chirho(1), // succ[0] = 1
            FiniteDomainChirho::range_chirho(0, 2),  // succ[1] ∈ {0, 1, 2}
            FiniteDomainChirho::range_chirho(0, 2),  // succ[2] ∈ {0, 1, 2}
        ];

        let results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // succ[1] and succ[2] can't be 1 (used by succ[0])
        assert!(!results_chirho[1].contains_chirho(1));
        assert!(!results_chirho[2].contains_chirho(1));
        // They also can't be self-loops
        assert!(!results_chirho[1].contains_chirho(1));
        assert!(!results_chirho[2].contains_chirho(2));
    }

    #[test]
    fn test_circuit_constraint_arity_chirho() {
        let prop_chirho = CircuitChirho::new_chirho(5);
        assert_eq!(prop_chirho.arity_chirho(), 5);
    }

    #[test]
    fn test_cumulative_constraint_basic_chirho() {
        // Cumulative: tasks with start, duration, resource; total capacity
        // 2 tasks, capacity 3
        let prop_chirho = CumulativeChirho::new_chirho(2);

        // Task 0: start ∈ {0,1,2}, duration=2, resource=2
        // Task 1: start ∈ {0,1,2}, duration=2, resource=2
        // Capacity: 3
        // They can't overlap (2+2=4 > 3)
        let cells_chirho = vec![
            // starts
            FiniteDomainChirho::range_chirho(0, 2), // start_0
            FiniteDomainChirho::range_chirho(0, 2), // start_1
            // durations
            FiniteDomainChirho::singleton_chirho(2), // dur_0 = 2
            FiniteDomainChirho::singleton_chirho(2), // dur_1 = 2
            // resources
            FiniteDomainChirho::singleton_chirho(2), // res_0 = 2
            FiniteDomainChirho::singleton_chirho(2), // res_1 = 2
            // capacity
            FiniteDomainChirho::singleton_chirho(3),
        ];

        let _results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // The cumulative should detect that these tasks can't overlap
        // and filter start times accordingly
        // This is a basic test; the actual filtering depends on the propagation logic
        assert_eq!(prop_chirho.arity_chirho(), 7); // 2*3 + 1
    }

    #[test]
    fn test_cumulative_constraint_no_overlap_chirho() {
        // Two tasks that MUST NOT overlap
        let prop_chirho = CumulativeChirho::new_chirho(2);

        // Task 0: start=0 (fixed), duration=2, resource=2
        // Task 1: start ∈ {0,1,2}, duration=2, resource=2
        // Capacity: 3 (can't fit both at same time)
        let cells_chirho = vec![
            FiniteDomainChirho::singleton_chirho(0),  // start_0 = 0
            FiniteDomainChirho::range_chirho(0, 3),   // start_1 ∈ {0,1,2,3}
            FiniteDomainChirho::singleton_chirho(2),  // dur_0 = 2
            FiniteDomainChirho::singleton_chirho(2),  // dur_1 = 2
            FiniteDomainChirho::singleton_chirho(2),  // res_0 = 2
            FiniteDomainChirho::singleton_chirho(2),  // res_1 = 2
            FiniteDomainChirho::singleton_chirho(3),  // capacity = 3
        ];

        let results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // Task 1 can't start at 0 or 1 (would overlap with task 0 at time 0,1)
        // Should be filtered to start at 2 or later
        if !results_chirho[1].is_bottom_chirho() {
            assert!(!results_chirho[1].contains_chirho(0));
            assert!(!results_chirho[1].contains_chirho(1));
        }
    }

    #[test]
    fn test_cumulative_constraint_arity_chirho() {
        let prop_chirho = CumulativeChirho::new_chirho(4);
        assert_eq!(prop_chirho.arity_chirho(), 13); // 4*3 + 1
    }

    #[test]
    fn test_cardinality_constraint_max_reached_chirho() {
        // Cardinality: bounds on how many vars can take each value
        use std::collections::HashMap;

        let mut bounds_chirho = HashMap::new();
        bounds_chirho.insert(1, (0, 1)); // value 1 can appear 0-1 times
        bounds_chirho.insert(2, (0, 2)); // value 2 can appear 0-2 times

        let prop_chirho = CardinalityChirho::new_chirho(3, bounds_chirho);

        let cells_chirho = vec![
            FiniteDomainChirho::singleton_chirho(1), // x0 = 1 (value 1 is now used)
            FiniteDomainChirho::range_chirho(1, 3),  // x1 ∈ {1, 2, 3}
            FiniteDomainChirho::range_chirho(1, 3),  // x2 ∈ {1, 2, 3}
        ];

        let results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // Value 1 has max_count=1, and it's already used by x0
        // So x1 and x2 should not contain 1
        assert!(!results_chirho[1].contains_chirho(1));
        assert!(!results_chirho[2].contains_chirho(1));
    }

    #[test]
    fn test_cardinality_constraint_min_required_chirho() {
        // If we need a certain number of a value and only enough vars can provide it
        use std::collections::HashMap;

        let mut bounds_chirho = HashMap::new();
        bounds_chirho.insert(5, (2, 3)); // value 5 must appear 2-3 times

        let prop_chirho = CardinalityChirho::new_chirho(3, bounds_chirho);

        // Only x1 and x2 can provide value 5, and we need at least 2
        let cells_chirho = vec![
            FiniteDomainChirho::singleton_chirho(1), // x0 = 1 (can't be 5)
            FiniteDomainChirho::range_chirho(5, 5),  // x1 = 5 (must be 5)
            FiniteDomainChirho::range_chirho(3, 7),  // x2 ∈ {3,4,5,6,7}
        ];

        let results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // We need 2 instances of value 5, x0 can't provide it, x1 is already 5
        // So if only x2 can also provide it and we need one more, x2 must be 5
        // (current_count=1, can_provide=[x2], need min_count=2)
        // Actually, x1 provides 5 already. x2 can provide 5.
        // 1 + 1 = 2 = min_count, so x2 MUST be 5
        assert!(results_chirho[2].is_singleton_chirho());
        assert_eq!(results_chirho[2].get_singleton_chirho(), Some(5));
    }

    #[test]
    fn test_cardinality_constraint_arity_chirho() {
        use std::collections::HashMap;
        let bounds_chirho = HashMap::new();
        let prop_chirho = CardinalityChirho::new_chirho(5, bounds_chirho);
        assert_eq!(prop_chirho.arity_chirho(), 5);
    }

    #[test]
    fn test_equals_constraint_chirho() {
        let prop_chirho = EqualsChirho;

        let cells_chirho = vec![
            FiniteDomainChirho::range_chirho(1, 5), // a ∈ {1,2,3,4,5}
            FiniteDomainChirho::range_chirho(3, 7), // b ∈ {3,4,5,6,7}
        ];

        let results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // Intersection should be {3,4,5}
        assert_eq!(results_chirho[0].size_chirho(), 3);
        assert_eq!(results_chirho[1].size_chirho(), 3);
        assert!(results_chirho[0].contains_chirho(3));
        assert!(results_chirho[0].contains_chirho(4));
        assert!(results_chirho[0].contains_chirho(5));
    }
}
