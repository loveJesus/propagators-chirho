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
}
