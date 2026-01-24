// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Kmett-style lattice abstractions for propagator networks.
//!
//! This module provides the algebraic foundations for propagator networks,
//! inspired by Edward Kmett's Haskell propagators library.
//!
//! # Lattice Theory
//!
//! A bounded lattice has:
//! - A bottom element (⊥) representing "no information"
//! - A top element (⊤) representing "contradiction"
//! - A join operation (⊔) that combines information monotonically
//!
//! # Example
//!
//! ```
//! use propagators_chirho::lattice_chirho::{LatticeChirho, BoundedLatticeChirho};
//! use propagators_chirho::NumericInfoChirho;
//!
//! // NumericInfoChirho implements BoundedLatticeChirho
//! let a_chirho = NumericInfoChirho::interval_chirho(0.0, 10.0);
//! let b_chirho = NumericInfoChirho::interval_chirho(5.0, 15.0);
//!
//! // Join narrows the interval to [5, 10]
//! let joined_chirho = a_chirho.join_chirho(&b_chirho);
//! ```

use std::fmt::Debug;

/// A join-semilattice with a partial order.
///
/// The join operation must satisfy:
/// - **Commutativity**: `a.join(b) = b.join(a)`
/// - **Associativity**: `a.join(b.join(c)) = (a.join(b)).join(c)`
/// - **Idempotency**: `a.join(a) = a`
///
/// The partial order is induced by: `a ≤ b` iff `a.join(b) = b`
pub trait LatticeChirho: Clone + Debug + PartialEq {
    /// Joins two elements, returning the least upper bound.
    ///
    /// This represents combining partial information.
    fn join_chirho(&self, other_chirho: &Self) -> Self;

    /// Returns true if `self ≤ other` in the lattice order.
    ///
    /// Default implementation: `self.join(other) == other`
    fn refines_chirho(&self, other_chirho: &Self) -> bool {
        self.join_chirho(other_chirho) == *other_chirho
    }
}

/// A bounded lattice with top and bottom elements.
///
/// - **Bottom (⊥)**: The least element, representing "no information"
/// - **Top (⊤)**: The greatest element, representing "contradiction"
pub trait BoundedLatticeChirho: LatticeChirho {
    /// Returns the bottom element (no information).
    fn bottom_chirho() -> Self;

    /// Returns the top element (contradiction).
    fn top_chirho() -> Self;

    /// Returns true if this is the bottom element.
    fn is_bottom_chirho(&self) -> bool;

    /// Returns true if this is the top element (contradiction).
    fn is_top_chirho(&self) -> bool;
}

/// A propagator function that takes inputs and produces outputs.
///
/// This is the Kmett-style functional propagator interface.
pub trait PropagatorFnChirho<L: LatticeChirho>: Clone {
    /// The number of cells this propagator connects.
    fn arity_chirho(&self) -> usize;

    /// Given current cell values, compute new values to merge.
    ///
    /// Returns a vector of the same length as inputs, where each
    /// element is the new information to merge into that cell.
    fn propagate_chirho(&self, cells_chirho: &[L]) -> Vec<L>;
}

/// A composable propagator that can be chained.
#[derive(Clone)]
pub struct ComposedChirho<L, P1, P2>
where
    L: LatticeChirho,
    P1: PropagatorFnChirho<L>,
    P2: PropagatorFnChirho<L>,
{
    first_chirho: P1,
    second_chirho: P2,
    _phantom_chirho: std::marker::PhantomData<L>,
}

impl<L, P1, P2> ComposedChirho<L, P1, P2>
where
    L: LatticeChirho,
    P1: PropagatorFnChirho<L>,
    P2: PropagatorFnChirho<L>,
{
    /// Composes two propagators sequentially.
    pub fn new_chirho(first_chirho: P1, second_chirho: P2) -> Self {
        Self {
            first_chirho,
            second_chirho,
            _phantom_chirho: std::marker::PhantomData,
        }
    }
}

/// Extension trait for composing propagators.
pub trait PropagatorComposeChirho<L: LatticeChirho>: PropagatorFnChirho<L> + Sized {
    /// Composes this propagator with another.
    fn then_chirho<P2: PropagatorFnChirho<L>>(
        self,
        other_chirho: P2,
    ) -> ComposedChirho<L, Self, P2> {
        ComposedChirho::new_chirho(self, other_chirho)
    }
}

impl<L: LatticeChirho, P: PropagatorFnChirho<L>> PropagatorComposeChirho<L> for P {}

/// A lifted value with provenance tracking.
///
/// This is like Kmett's `Supported` - a value plus the premises that support it.
#[derive(Clone, Debug, PartialEq)]
pub struct SupportedValueChirho<L: LatticeChirho> {
    /// The lattice value.
    pub value_chirho: L,
    /// Premises that support this value (as string identifiers).
    pub premises_chirho: std::collections::HashSet<String>,
}

impl<L: LatticeChirho> SupportedValueChirho<L> {
    /// Creates a new supported value with no premises (axiomatic).
    pub fn axiom_chirho(value_chirho: L) -> Self {
        Self {
            value_chirho,
            premises_chirho: std::collections::HashSet::new(),
        }
    }

    /// Creates a supported value with premises.
    pub fn with_premises_chirho(
        value_chirho: L,
        premises_chirho: std::collections::HashSet<String>,
    ) -> Self {
        Self {
            value_chirho,
            premises_chirho,
        }
    }
}

impl<L: BoundedLatticeChirho> LatticeChirho for SupportedValueChirho<L> {
    fn join_chirho(&self, other_chirho: &Self) -> Self {
        // Join values and union premises
        let joined_value_chirho = self.value_chirho.join_chirho(&other_chirho.value_chirho);
        let mut joined_premises_chirho = self.premises_chirho.clone();
        joined_premises_chirho.extend(other_chirho.premises_chirho.iter().cloned());

        Self {
            value_chirho: joined_value_chirho,
            premises_chirho: joined_premises_chirho,
        }
    }
}

impl<L: BoundedLatticeChirho> BoundedLatticeChirho for SupportedValueChirho<L> {
    fn bottom_chirho() -> Self {
        Self::axiom_chirho(L::bottom_chirho())
    }

    fn top_chirho() -> Self {
        Self::axiom_chirho(L::top_chirho())
    }

    fn is_bottom_chirho(&self) -> bool {
        self.value_chirho.is_bottom_chirho()
    }

    fn is_top_chirho(&self) -> bool {
        self.value_chirho.is_top_chirho()
    }
}

// ============================================================================
// IMPLEMENTATIONS FOR NUMERICINFOSCHIRHO
// ============================================================================

use crate::interval_chirho::NumericInfoChirho;

impl LatticeChirho for NumericInfoChirho {
    fn join_chirho(&self, other_chirho: &Self) -> Self {
        self.merge_chirho(other_chirho)
    }
}

impl BoundedLatticeChirho for NumericInfoChirho {
    fn bottom_chirho() -> Self {
        Self::NothingChirho
    }

    fn top_chirho() -> Self {
        Self::ContradictionChirho
    }

    fn is_bottom_chirho(&self) -> bool {
        matches!(self, Self::NothingChirho)
    }

    fn is_top_chirho(&self) -> bool {
        matches!(self, Self::ContradictionChirho)
    }
}

// ============================================================================
// STANDARD PROPAGATOR FUNCTIONS
// ============================================================================

/// Addition propagator: a + b = c (bidirectional).
#[derive(Clone, Debug)]
pub struct AddPropagatorChirho;

impl PropagatorFnChirho<NumericInfoChirho> for AddPropagatorChirho {
    fn arity_chirho(&self) -> usize {
        3
    }

    fn propagate_chirho(&self, cells_chirho: &[NumericInfoChirho]) -> Vec<NumericInfoChirho> {
        let a_chirho = &cells_chirho[0];
        let b_chirho = &cells_chirho[1];
        let c_chirho = &cells_chirho[2];

        let mut results_chirho = vec![NumericInfoChirho::NothingChirho; 3];

        // Forward: c = a + b
        if let (Some(a_iv_chirho), Some(b_iv_chirho)) =
            (a_chirho.as_interval_chirho(), b_chirho.as_interval_chirho())
        {
            results_chirho[2] =
                NumericInfoChirho::IntervalChirho(a_iv_chirho.add_chirho(&b_iv_chirho));
        }

        // Backward: a = c - b
        if let (Some(c_iv_chirho), Some(b_iv_chirho)) =
            (c_chirho.as_interval_chirho(), b_chirho.as_interval_chirho())
        {
            results_chirho[0] =
                NumericInfoChirho::IntervalChirho(c_iv_chirho.sub_chirho(&b_iv_chirho));
        }

        // Backward: b = c - a
        if let (Some(c_iv_chirho), Some(a_iv_chirho)) =
            (c_chirho.as_interval_chirho(), a_chirho.as_interval_chirho())
        {
            results_chirho[1] =
                NumericInfoChirho::IntervalChirho(c_iv_chirho.sub_chirho(&a_iv_chirho));
        }

        results_chirho
    }
}

/// Multiplication propagator: a × b = c (bidirectional).
#[derive(Clone, Debug)]
pub struct MulPropagatorChirho;

impl PropagatorFnChirho<NumericInfoChirho> for MulPropagatorChirho {
    fn arity_chirho(&self) -> usize {
        3
    }

    fn propagate_chirho(&self, cells_chirho: &[NumericInfoChirho]) -> Vec<NumericInfoChirho> {
        let a_chirho = &cells_chirho[0];
        let b_chirho = &cells_chirho[1];
        let c_chirho = &cells_chirho[2];

        let mut results_chirho = vec![NumericInfoChirho::NothingChirho; 3];

        // Forward: c = a * b
        if let (Some(a_iv_chirho), Some(b_iv_chirho)) =
            (a_chirho.as_interval_chirho(), b_chirho.as_interval_chirho())
        {
            results_chirho[2] =
                NumericInfoChirho::IntervalChirho(a_iv_chirho.mul_chirho(&b_iv_chirho));
        }

        // Backward: a = c / b
        if let (Some(c_iv_chirho), Some(b_iv_chirho)) =
            (c_chirho.as_interval_chirho(), b_chirho.as_interval_chirho())
        {
            let div_result_chirho = c_iv_chirho.div_chirho(&b_iv_chirho);
            if !div_result_chirho.is_empty_chirho() {
                results_chirho[0] = NumericInfoChirho::IntervalChirho(div_result_chirho);
            }
        }

        // Backward: b = c / a
        if let (Some(c_iv_chirho), Some(a_iv_chirho)) =
            (c_chirho.as_interval_chirho(), a_chirho.as_interval_chirho())
        {
            let div_result_chirho = c_iv_chirho.div_chirho(&a_iv_chirho);
            if !div_result_chirho.is_empty_chirho() {
                results_chirho[1] = NumericInfoChirho::IntervalChirho(div_result_chirho);
            }
        }

        results_chirho
    }
}

/// Square propagator: a² = b (bidirectional).
#[derive(Clone, Debug)]
pub struct SquarePropagatorChirho;

impl PropagatorFnChirho<NumericInfoChirho> for SquarePropagatorChirho {
    fn arity_chirho(&self) -> usize {
        2
    }

    fn propagate_chirho(&self, cells_chirho: &[NumericInfoChirho]) -> Vec<NumericInfoChirho> {
        let a_chirho = &cells_chirho[0];
        let b_chirho = &cells_chirho[1];

        let mut results_chirho = vec![NumericInfoChirho::NothingChirho; 2];

        // Forward: b = a²
        if let Some(a_iv_chirho) = a_chirho.as_interval_chirho() {
            results_chirho[1] = NumericInfoChirho::IntervalChirho(a_iv_chirho.square_chirho());
        }

        // Backward: a = √b
        if let Some(b_iv_chirho) = b_chirho.as_interval_chirho() {
            let sqrt_result_chirho = b_iv_chirho.sqrt_chirho();
            if !sqrt_result_chirho.is_empty_chirho() {
                results_chirho[0] = NumericInfoChirho::IntervalChirho(sqrt_result_chirho);
            }
        }

        results_chirho
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_lattice_join_chirho() {
        let a_chirho = NumericInfoChirho::interval_chirho(0.0, 10.0);
        let b_chirho = NumericInfoChirho::interval_chirho(5.0, 15.0);
        let joined_chirho = a_chirho.join_chirho(&b_chirho);

        // Should be intersection: [5, 10]
        if let NumericInfoChirho::IntervalChirho(iv_chirho) = joined_chirho {
            assert_eq!(iv_chirho.lo_chirho, 5.0);
            assert_eq!(iv_chirho.hi_chirho, 10.0);
        } else {
            panic!("Expected interval");
        }
    }

    #[test]
    fn test_bounded_lattice_chirho() {
        let bottom_chirho = NumericInfoChirho::bottom_chirho();
        let top_chirho = NumericInfoChirho::top_chirho();

        assert!(bottom_chirho.is_bottom_chirho());
        assert!(top_chirho.is_top_chirho());

        // Join with bottom is identity
        let a_chirho = NumericInfoChirho::exact_chirho(5.0);
        let joined_chirho = bottom_chirho.join_chirho(&a_chirho);
        assert_eq!(joined_chirho, a_chirho);

        // Join with top gives top
        let joined_top_chirho = a_chirho.join_chirho(&top_chirho);
        assert!(joined_top_chirho.is_top_chirho());
    }

    #[test]
    fn test_add_propagator_chirho() {
        let prop_chirho = AddPropagatorChirho;
        let cells_chirho = vec![
            NumericInfoChirho::exact_chirho(3.0),
            NumericInfoChirho::exact_chirho(4.0),
            NumericInfoChirho::NothingChirho,
        ];

        let results_chirho = prop_chirho.propagate_chirho(&cells_chirho);

        // c should be 7
        if let NumericInfoChirho::IntervalChirho(iv_chirho) = &results_chirho[2] {
            assert_eq!(iv_chirho.lo_chirho, 7.0);
            assert_eq!(iv_chirho.hi_chirho, 7.0);
        } else {
            panic!("Expected interval for c");
        }
    }

    #[test]
    fn test_supported_value_chirho() {
        let a_chirho = SupportedValueChirho::with_premises_chirho(
            NumericInfoChirho::exact_chirho(5.0),
            ["sensor_a".to_string()].into_iter().collect(),
        );

        let b_chirho = SupportedValueChirho::with_premises_chirho(
            NumericInfoChirho::interval_chirho(4.0, 6.0),
            ["sensor_b".to_string()].into_iter().collect(),
        );

        let joined_chirho = a_chirho.join_chirho(&b_chirho);

        // Value should be intersection
        assert!(matches!(
            joined_chirho.value_chirho,
            NumericInfoChirho::IntervalChirho(_)
        ));

        // Premises should be union
        assert!(joined_chirho.premises_chirho.contains("sensor_a"));
        assert!(joined_chirho.premises_chirho.contains("sensor_b"));
    }
}
