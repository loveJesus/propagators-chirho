// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Algebraic structures for propagator networks.
//!
//! This module provides formal algebraic abstractions following the style
//! of Edward Kmett's Haskell libraries. Each trait has its laws documented
//! and verified via property tests and Kani proofs where applicable.
//!
//! # Algebraic Hierarchy
//!
//! ```text
//! SemigroupChirho          -- associative binary operation
//!        |
//!        v
//!  MonoidChirho            -- semigroup + identity element
//!        |
//!        v
//! LatticeChirho            -- idempotent + commutative monoid (join-semilattice)
//!        |
//!        v
//! BoundedLatticeChirho     -- lattice + top element
//! ```
//!
//! # Laws
//!
//! All implementations must satisfy these laws, which are tested via
//! property-based testing and formally verified where possible.

#[cfg(not(feature = "no-std"))]
use std::fmt::{self, Debug};

#[cfg(feature = "no-std")]
use core::fmt::{self, Debug};

#[cfg(feature = "no-std")]
use alloc::string::String;

/// A semigroup is a set with an associative binary operation.
///
/// # Laws
///
/// **Associativity**: For all `a`, `b`, `c`:
/// ```text
/// a.combine(b.combine(c)) = a.combine(b).combine(c)
/// ```
///
/// # Example
///
/// ```
/// use propagators_chirho::algebra_chirho::SemigroupChirho;
///
/// // Wrapper type for string concatenation semigroup
/// #[derive(Clone)]
/// struct ConcatChirho(String);
///
/// impl SemigroupChirho for ConcatChirho {
///     fn combine_chirho(&self, other_chirho: &Self) -> Self {
///         ConcatChirho(format!("{}{}", self.0, other_chirho.0))
///     }
/// }
///
/// let a_chirho = ConcatChirho("Hello".to_string());
/// let b_chirho = ConcatChirho(" World".to_string());
/// let result_chirho = a_chirho.combine_chirho(&b_chirho);
/// assert_eq!(result_chirho.0, "Hello World");
/// ```
pub trait SemigroupChirho: Clone {
    /// Combines two elements associatively.
    ///
    /// This operation must be associative:
    /// `a.combine(b.combine(c)) = a.combine(b).combine(c)`
    fn combine_chirho(&self, other_chirho: &Self) -> Self;

    /// Combines multiple elements left-to-right.
    ///
    /// `sconcat([a, b, c]) = a.combine(b).combine(c)`
    fn sconcat_chirho(items_chirho: &[Self]) -> Option<Self>
    where
        Self: Sized,
    {
        let mut iter_chirho = items_chirho.iter();
        let first_chirho = iter_chirho.next()?.clone();
        Some(iter_chirho.fold(first_chirho, |acc_chirho, x_chirho| {
            acc_chirho.combine_chirho(x_chirho)
        }))
    }

    /// Repeats the element n times using the semigroup operation.
    ///
    /// `stimes(3, a) = a.combine(a).combine(a)`
    ///
    /// Uses O(log n) operations via binary exponentiation.
    fn stimes_chirho(&self, n_chirho: usize) -> Self
    where
        Self: Sized,
    {
        assert!(n_chirho > 0, "stimes requires n > 0 for semigroups");
        if n_chirho == 1 {
            return self.clone();
        }

        let mut result_chirho = self.clone();
        let mut base_chirho = self.clone();
        let mut exp_chirho = n_chirho - 1;

        while exp_chirho > 0 {
            if exp_chirho % 2 == 1 {
                result_chirho = result_chirho.combine_chirho(&base_chirho);
            }
            base_chirho = base_chirho.combine_chirho(&base_chirho);
            exp_chirho /= 2;
        }

        result_chirho
    }
}

/// A monoid is a semigroup with an identity element.
///
/// # Laws
///
/// **Identity**: For all `a`:
/// ```text
/// empty.combine(a) = a
/// a.combine(empty) = a
/// ```
///
/// Plus all semigroup laws (associativity).
///
/// # Example
///
/// ```
/// use propagators_chirho::algebra_chirho::{SemigroupChirho, MonoidChirho};
///
/// // Addition forms a monoid with 0 as identity
/// #[derive(Clone, PartialEq, Debug)]
/// struct SumChirho(i64);
///
/// impl SemigroupChirho for SumChirho {
///     fn combine_chirho(&self, other_chirho: &Self) -> Self {
///         SumChirho(self.0 + other_chirho.0)
///     }
/// }
///
/// impl MonoidChirho for SumChirho {
///     fn empty_chirho() -> Self {
///         SumChirho(0)
///     }
/// }
/// ```
pub trait MonoidChirho: SemigroupChirho {
    /// Returns the identity element.
    ///
    /// Must satisfy:
    /// - `empty.combine(a) = a` (left identity)
    /// - `a.combine(empty) = a` (right identity)
    fn empty_chirho() -> Self;

    /// Returns true if this is the identity element.
    fn is_empty_chirho(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::empty_chirho()
    }

    /// Combines multiple elements, returning empty for an empty list.
    ///
    /// `mconcat([]) = empty`
    /// `mconcat([a, b, c]) = a.combine(b).combine(c)`
    fn mconcat_chirho(items_chirho: &[Self]) -> Self
    where
        Self: Sized,
    {
        items_chirho
            .iter()
            .fold(Self::empty_chirho(), |acc_chirho, x_chirho| {
                acc_chirho.combine_chirho(x_chirho)
            })
    }

    /// Repeats the element n times, returning empty for n=0.
    fn mtimes_chirho(&self, n_chirho: usize) -> Self
    where
        Self: Sized,
    {
        if n_chirho == 0 {
            return Self::empty_chirho();
        }
        self.stimes_chirho(n_chirho)
    }
}

/// A commutative semigroup where the operation is order-independent.
///
/// # Laws
///
/// **Commutativity**: For all `a`, `b`:
/// ```text
/// a.combine(b) = b.combine(a)
/// ```
///
/// Plus all semigroup laws (associativity).
pub trait CommutativeSemigroupChirho: SemigroupChirho {}

/// An idempotent semigroup where combining an element with itself is a no-op.
///
/// # Laws
///
/// **Idempotence**: For all `a`:
/// ```text
/// a.combine(a) = a
/// ```
///
/// Plus all semigroup laws (associativity).
pub trait IdempotentSemigroupChirho: SemigroupChirho {}

/// A join-semilattice: an idempotent commutative monoid.
///
/// This is the core abstraction for propagator networks. The `join` operation
/// combines partial information monotonically.
///
/// # Laws
///
/// - **Associativity**: `a.join(b.join(c)) = a.join(b).join(c)`
/// - **Commutativity**: `a.join(b) = b.join(a)`
/// - **Idempotence**: `a.join(a) = a`
/// - **Identity**: `bottom.join(a) = a`
///
/// These laws ensure monotonic information flow: once we learn something,
/// we never "unlearn" it.
pub trait JoinSemilatticeChirho:
    MonoidChirho + CommutativeSemigroupChirho + IdempotentSemigroupChirho
{
    /// Joins two elements (alias for combine).
    ///
    /// In propagator networks, this represents merging partial information.
    fn join_lattice_chirho(&self, other_chirho: &Self) -> Self {
        self.combine_chirho(other_chirho)
    }

    /// Returns the bottom element (alias for empty).
    ///
    /// Represents "no information".
    fn lattice_bottom_chirho() -> Self {
        Self::empty_chirho()
    }

    /// Returns true if this is the bottom element.
    fn is_lattice_bottom_chirho(&self) -> bool
    where
        Self: PartialEq,
    {
        self.is_empty_chirho()
    }

    /// Returns true if `self ≤ other` in the lattice order.
    ///
    /// `a ≤ b` iff `a.join(b) = b`
    ///
    /// Note: This is the lattice-theoretic definition. For domain-specific
    /// "refinement" semantics, see the type's own `refines_chirho` method.
    fn lattice_leq_chirho(&self, other_chirho: &Self) -> bool
    where
        Self: PartialEq,
    {
        self.join_lattice_chirho(other_chirho) == *other_chirho
    }
}

/// A bounded join-semilattice with a top element.
///
/// The top element represents "contradiction" - incompatible information.
/// Once any cell reaches top, the constraint system is inconsistent.
///
/// # Laws
///
/// **Absorption**: For all `a`:
/// ```text
/// a.join(top) = top
/// ```
pub trait BoundedJoinSemilatticeChirho: JoinSemilatticeChirho {
    /// Returns the top element (contradiction).
    fn lattice_top_chirho() -> Self;

    /// Returns true if this is the top element.
    fn is_lattice_top_chirho(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::lattice_top_chirho()
    }
}

// ============================================================================
// IMPLEMENTATIONS FOR NUMERICINFOCHIRHO
// ============================================================================

use super::interval_chirho::NumericInfoChirho;

impl SemigroupChirho for NumericInfoChirho {
    fn combine_chirho(&self, other_chirho: &Self) -> Self {
        self.merge_chirho(other_chirho)
    }
}

impl MonoidChirho for NumericInfoChirho {
    fn empty_chirho() -> Self {
        Self::NothingChirho
    }
}

impl CommutativeSemigroupChirho for NumericInfoChirho {}
impl IdempotentSemigroupChirho for NumericInfoChirho {}

impl JoinSemilatticeChirho for NumericInfoChirho {}

impl BoundedJoinSemilatticeChirho for NumericInfoChirho {
    fn lattice_top_chirho() -> Self {
        Self::ContradictionChirho
    }
}

// ============================================================================
// RESULT TYPE FOR FALLIBLE OPERATIONS
// ============================================================================

/// Error types for propagator operations.
#[derive(Debug, Clone, PartialEq)]
pub enum PropagatorErrorChirho {
    /// Cell not found with the given name or ID.
    CellNotFoundChirho {
        /// The cell ID that was not found.
        cell_id_chirho: usize,
    },
    /// Cell not found with the given name.
    CellNameNotFoundChirho {
        /// The cell name that was not found.
        name_chirho: String,
    },
    /// Cell already exists with the given name.
    CellAlreadyExistsChirho {
        /// The cell name that already exists.
        name_chirho: String,
    },
    /// Invalid interval (lo > hi).
    InvalidIntervalChirho {
        /// Lower bound.
        lo_chirho: f64,
        /// Upper bound.
        hi_chirho: f64,
    },
    /// Contradiction detected in the network.
    ContradictionChirho {
        /// Optional message describing the contradiction.
        message_chirho: Option<String>,
    },
    /// Division by zero or interval containing zero.
    DivisionByZeroChirho,
    /// Square root of negative number.
    NegativeSqrtChirho {
        /// The negative value.
        value_chirho: f64,
    },
    /// Feature not enabled.
    FeatureNotEnabledChirho {
        /// The feature name.
        feature_chirho: &'static str,
    },
}

impl fmt::Display for PropagatorErrorChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CellNotFoundChirho { cell_id_chirho } => {
                write!(f_chirho, "Cell not found: {}", cell_id_chirho)
            }
            Self::CellNameNotFoundChirho { name_chirho } => {
                write!(f_chirho, "Cell not found: '{}'", name_chirho)
            }
            Self::CellAlreadyExistsChirho { name_chirho } => {
                write!(f_chirho, "Cell already exists: '{}'", name_chirho)
            }
            Self::InvalidIntervalChirho {
                lo_chirho,
                hi_chirho,
            } => {
                write!(f_chirho, "Invalid interval: [{}, {}]", lo_chirho, hi_chirho)
            }
            Self::ContradictionChirho { message_chirho } => {
                if let Some(msg_chirho) = message_chirho {
                    write!(f_chirho, "Contradiction: {}", msg_chirho)
                } else {
                    write!(f_chirho, "Contradiction detected")
                }
            }
            Self::DivisionByZeroChirho => write!(f_chirho, "Division by zero"),
            Self::NegativeSqrtChirho { value_chirho } => {
                write!(f_chirho, "Square root of negative: {}", value_chirho)
            }
            Self::FeatureNotEnabledChirho { feature_chirho } => {
                write!(f_chirho, "Feature not enabled: {}", feature_chirho)
            }
        }
    }
}

// std::error::Error requires std; not available in no_std mode
#[cfg(not(feature = "no-std"))]
impl std::error::Error for PropagatorErrorChirho {}

/// Result type for propagator operations.
pub type PropagatorResultChirho<T> = Result<T, PropagatorErrorChirho>;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[cfg(feature = "no-std")]
    use alloc::vec;

    #[test]
    fn test_semigroup_associativity_chirho() {
        let a_chirho = NumericInfoChirho::interval_chirho(0.0, 10.0);
        let b_chirho = NumericInfoChirho::interval_chirho(5.0, 15.0);
        let c_chirho = NumericInfoChirho::interval_chirho(8.0, 12.0);

        // (a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)
        let left_chirho = a_chirho.combine_chirho(&b_chirho).combine_chirho(&c_chirho);
        let right_chirho = a_chirho.combine_chirho(&b_chirho.combine_chirho(&c_chirho));
        assert_eq!(left_chirho, right_chirho);
    }

    #[test]
    fn test_monoid_identity_chirho() {
        let a_chirho = NumericInfoChirho::interval_chirho(5.0, 10.0);
        let empty_chirho = NumericInfoChirho::empty_chirho();

        // empty ⊕ a = a
        assert_eq!(empty_chirho.combine_chirho(&a_chirho), a_chirho);
        // a ⊕ empty = a
        assert_eq!(a_chirho.combine_chirho(&empty_chirho), a_chirho);
    }

    #[test]
    fn test_idempotence_chirho() {
        let a_chirho = NumericInfoChirho::interval_chirho(5.0, 10.0);

        // a ⊕ a = a
        assert_eq!(a_chirho.combine_chirho(&a_chirho), a_chirho);
    }

    #[test]
    fn test_commutativity_chirho() {
        let a_chirho = NumericInfoChirho::interval_chirho(0.0, 10.0);
        let b_chirho = NumericInfoChirho::interval_chirho(5.0, 15.0);

        // a ⊕ b = b ⊕ a
        assert_eq!(
            a_chirho.combine_chirho(&b_chirho),
            b_chirho.combine_chirho(&a_chirho)
        );
    }

    #[test]
    fn test_top_absorption_chirho() {
        let a_chirho = NumericInfoChirho::interval_chirho(5.0, 10.0);
        let top_chirho = NumericInfoChirho::lattice_top_chirho();

        // a ⊕ ⊤ = ⊤
        assert_eq!(a_chirho.join_lattice_chirho(&top_chirho), top_chirho);
    }

    #[test]
    fn test_stimes_chirho() {
        let a_chirho = NumericInfoChirho::interval_chirho(5.0, 10.0);

        // Due to idempotence, stimes(n, a) = a for any n > 0
        assert_eq!(a_chirho.stimes_chirho(1), a_chirho);
        assert_eq!(a_chirho.stimes_chirho(5), a_chirho);
        assert_eq!(a_chirho.stimes_chirho(100), a_chirho);
    }

    #[test]
    fn test_mconcat_chirho() {
        let items_chirho = vec![
            NumericInfoChirho::interval_chirho(0.0, 10.0),
            NumericInfoChirho::interval_chirho(5.0, 15.0),
            NumericInfoChirho::interval_chirho(8.0, 12.0),
        ];

        let result_chirho = NumericInfoChirho::mconcat_chirho(&items_chirho);

        // Should be intersection of all: [8, 10]
        if let NumericInfoChirho::IntervalChirho(iv_chirho) = result_chirho {
            assert_eq!(iv_chirho.lo_chirho, 8.0);
            assert_eq!(iv_chirho.hi_chirho, 10.0);
        } else {
            panic!("Expected interval");
        }
    }

    #[test]
    fn test_lattice_leq_chirho() {
        let narrow_chirho = NumericInfoChirho::interval_chirho(5.0, 7.0);
        let wide_chirho = NumericInfoChirho::interval_chirho(0.0, 10.0);

        // In our lattice, join = intersection (merge)
        // lattice_leq(a, b) means a.join(b) = b
        //
        // narrow.join(wide) = [5,7].intersect([0,10]) = [5,7] ≠ wide
        // So narrow ≰ wide in lattice order
        //
        // wide.join(narrow) = [0,10].intersect([5,7]) = [5,7] = narrow
        // So wide ≤ narrow in the lattice order (wide has less info)

        assert!(!narrow_chirho.lattice_leq_chirho(&wide_chirho));
        assert!(wide_chirho.lattice_leq_chirho(&narrow_chirho));

        // Bottom ≤ everything
        let bottom_chirho = NumericInfoChirho::lattice_bottom_chirho();
        assert!(bottom_chirho.lattice_leq_chirho(&narrow_chirho));
        assert!(bottom_chirho.lattice_leq_chirho(&wide_chirho));

        // Everything ≤ top
        let top_chirho = NumericInfoChirho::lattice_top_chirho();
        assert!(narrow_chirho.lattice_leq_chirho(&top_chirho));
        assert!(wide_chirho.lattice_leq_chirho(&top_chirho));
    }

    #[test]
    #[cfg(not(feature = "no-std"))]
    fn test_error_display_chirho() {
        let err_chirho = PropagatorErrorChirho::InvalidIntervalChirho {
            lo_chirho: 10.0,
            hi_chirho: 5.0,
        };
        assert!(err_chirho.to_string().contains("Invalid interval"));
    }
}
