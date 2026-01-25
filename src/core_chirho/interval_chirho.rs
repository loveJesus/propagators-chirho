// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Interval arithmetic for partial numeric information.
//!
//! This module implements interval arithmetic as described in:
//!
//! > Moore, R. E. (1966). *Interval Analysis*. Prentice-Hall.
//!
//! Intervals represent partial knowledge about numeric values. When we don't
//! know the exact value, we can express what we do know as bounds.
//!
//! # Mathematical Foundation
//!
//! An interval `[a, b]` represents the set `{x ∈ ℝ : a ≤ x ≤ b}`.
//!
//! Interval arithmetic operations are defined to contain all possible results:
//!
//! - `[a,b] + [c,d] = [a+c, b+d]`
//! - `[a,b] - [c,d] = [a-d, b-c]`
//! - `[a,b] × [c,d] = [min(ac,ad,bc,bd), max(ac,ad,bc,bd)]`
//! - `[a,b] ÷ [c,d] = [a,b] × [1/d, 1/c]` (when 0 ∉ \[c,d\])
//!
//! # Example
//!
//! ```
//! use propagators_chirho::IntervalChirho;
//!
//! let x_chirho = IntervalChirho::new_chirho(2.0, 4.0);  // x ∈ [2, 4]
//! let y_chirho = IntervalChirho::new_chirho(1.0, 3.0);  // y ∈ [1, 3]
//!
//! let sum_chirho = x_chirho.add_chirho(&y_chirho);      // x + y ∈ [3, 7]
//! assert_eq!(sum_chirho.lo_chirho, 3.0);
//! assert_eq!(sum_chirho.hi_chirho, 7.0);
//! ```

#[cfg(not(feature = "no-std"))]
use std::fmt;

#[cfg(feature = "no-std")]
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Helper functions for math operations that differ between std and no_std

/// Compute square root, using libm in no_std mode.
#[inline]
fn sqrt_f64_chirho(x_chirho: f64) -> f64 {
    #[cfg(not(feature = "no-std"))]
    {
        x_chirho.sqrt()
    }
    #[cfg(feature = "no-std")]
    {
        libm::sqrt(x_chirho)
    }
}

/// Compute x^2 (square), using libm in no_std mode.
#[inline]
fn powi_2_f64_chirho(x_chirho: f64) -> f64 {
    #[cfg(not(feature = "no-std"))]
    {
        x_chirho.powi(2)
    }
    #[cfg(feature = "no-std")]
    {
        x_chirho * x_chirho
    }
}

/// An interval `[lo, hi]` representing partial numeric information.
///
/// Intervals are the primary representation of partial numeric knowledge in
/// propagator networks. As more constraints are applied, intervals narrow
/// (their width decreases) until they converge to a single value or become
/// empty (contradiction).
///
/// # Invariants
///
/// - `lo <= hi` for valid intervals
/// - `lo > hi` indicates a contradiction (empty interval)
///
/// # Example
///
/// ```
/// use propagators_chirho::IntervalChirho;
///
/// // "The temperature is between 20 and 30 degrees"
/// let temp_chirho = IntervalChirho::new_chirho(20.0, 30.0);
///
/// // "Actually, it's at least 25 degrees" - intersect to narrow
/// let refined_chirho = temp_chirho.intersect_chirho(
///     &IntervalChirho::new_chirho(25.0, f64::INFINITY)
/// );
/// assert_eq!(refined_chirho.lo_chirho, 25.0);
/// assert_eq!(refined_chirho.hi_chirho, 30.0);
/// ```
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IntervalChirho {
    /// Lower bound of the interval.
    pub lo_chirho: f64,
    /// Upper bound of the interval.
    pub hi_chirho: f64,
}

impl IntervalChirho {
    /// Creates a new interval with the given bounds.
    ///
    /// # Arguments
    ///
    /// * `lo_chirho` - The lower bound
    /// * `hi_chirho` - The upper bound
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let interval_chirho = IntervalChirho::new_chirho(0.0, 10.0);
    /// assert_eq!(interval_chirho.lo_chirho, 0.0);
    /// assert_eq!(interval_chirho.hi_chirho, 10.0);
    /// ```
    #[inline]
    pub fn new_chirho(lo_chirho: f64, hi_chirho: f64) -> Self {
        Self {
            lo_chirho,
            hi_chirho,
        }
    }

    /// Creates an interval representing an exact value.
    ///
    /// This is a point interval where `lo == hi`.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let exact_chirho = IntervalChirho::exact_chirho(42.0);
    /// assert_eq!(exact_chirho.lo_chirho, 42.0);
    /// assert_eq!(exact_chirho.hi_chirho, 42.0);
    /// assert!(exact_chirho.is_exact_chirho());
    /// ```
    #[inline]
    pub fn exact_chirho(value_chirho: f64) -> Self {
        Self {
            lo_chirho: value_chirho,
            hi_chirho: value_chirho,
        }
    }

    /// Creates an interval representing all real numbers.
    ///
    /// This is the "know nothing" state for numeric information.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let everything_chirho = IntervalChirho::everything_chirho();
    /// assert_eq!(everything_chirho.lo_chirho, f64::NEG_INFINITY);
    /// assert_eq!(everything_chirho.hi_chirho, f64::INFINITY);
    /// ```
    #[inline]
    pub fn everything_chirho() -> Self {
        Self {
            lo_chirho: f64::NEG_INFINITY,
            hi_chirho: f64::INFINITY,
        }
    }

    /// Creates an interval representing non-negative reals `[0, ∞)`.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let positive_chirho = IntervalChirho::non_negative_chirho();
    /// assert_eq!(positive_chirho.lo_chirho, 0.0);
    /// ```
    #[inline]
    pub fn non_negative_chirho() -> Self {
        Self {
            lo_chirho: 0.0,
            hi_chirho: f64::INFINITY,
        }
    }

    /// Creates an interval representing non-positive reals `(-∞, 0]`.
    #[inline]
    pub fn non_positive_chirho() -> Self {
        Self {
            lo_chirho: f64::NEG_INFINITY,
            hi_chirho: 0.0,
        }
    }

    /// Returns `true` if this is a point interval (exact value).
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// assert!(IntervalChirho::exact_chirho(5.0).is_exact_chirho());
    /// assert!(!IntervalChirho::new_chirho(0.0, 10.0).is_exact_chirho());
    /// ```
    #[inline]
    pub fn is_exact_chirho(&self) -> bool {
        (self.lo_chirho - self.hi_chirho).abs() < 1e-10
    }

    /// Returns `true` if this interval is empty (contradiction).
    ///
    /// An empty interval occurs when `lo > hi`, meaning no value can
    /// satisfy all constraints.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let a_chirho = IntervalChirho::new_chirho(0.0, 5.0);
    /// let b_chirho = IntervalChirho::new_chirho(10.0, 15.0);
    /// let intersection_chirho = a_chirho.intersect_chirho(&b_chirho);
    /// assert!(intersection_chirho.is_empty_chirho());
    /// ```
    #[inline]
    pub fn is_empty_chirho(&self) -> bool {
        self.lo_chirho > self.hi_chirho
    }

    /// Returns the width of the interval.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let interval_chirho = IntervalChirho::new_chirho(2.0, 7.0);
    /// assert_eq!(interval_chirho.width_chirho(), 5.0);
    /// ```
    #[inline]
    pub fn width_chirho(&self) -> f64 {
        self.hi_chirho - self.lo_chirho
    }

    /// Returns the midpoint of the interval.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let interval_chirho = IntervalChirho::new_chirho(2.0, 8.0);
    /// assert_eq!(interval_chirho.midpoint_chirho(), 5.0);
    /// ```
    #[inline]
    pub fn midpoint_chirho(&self) -> f64 {
        (self.lo_chirho + self.hi_chirho) / 2.0
    }

    /// Returns `true` if this interval contains the given value.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let interval_chirho = IntervalChirho::new_chirho(0.0, 10.0);
    /// assert!(interval_chirho.contains_chirho(5.0));
    /// assert!(!interval_chirho.contains_chirho(15.0));
    /// ```
    #[inline]
    pub fn contains_chirho(&self, value_chirho: f64) -> bool {
        self.lo_chirho <= value_chirho && value_chirho <= self.hi_chirho
    }

    /// Returns `true` if this interval is a subset of another.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let small_chirho = IntervalChirho::new_chirho(3.0, 7.0);
    /// let large_chirho = IntervalChirho::new_chirho(0.0, 10.0);
    /// assert!(small_chirho.subset_of_chirho(&large_chirho));
    /// assert!(!large_chirho.subset_of_chirho(&small_chirho));
    /// ```
    #[inline]
    pub fn subset_of_chirho(&self, other_chirho: &Self) -> bool {
        other_chirho.lo_chirho <= self.lo_chirho && self.hi_chirho <= other_chirho.hi_chirho
    }

    /// Intersects this interval with another, returning the overlap.
    ///
    /// This is the fundamental operation for combining partial information.
    /// The result is empty if the intervals don't overlap.
    ///
    /// # Mathematical Definition
    ///
    /// `[a,b] ∩ [c,d] = [max(a,c), min(b,d)]`
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let a_chirho = IntervalChirho::new_chirho(0.0, 10.0);
    /// let b_chirho = IntervalChirho::new_chirho(5.0, 15.0);
    /// let intersection_chirho = a_chirho.intersect_chirho(&b_chirho);
    /// assert_eq!(intersection_chirho.lo_chirho, 5.0);
    /// assert_eq!(intersection_chirho.hi_chirho, 10.0);
    /// ```
    #[inline]
    pub fn intersect_chirho(&self, other_chirho: &Self) -> Self {
        Self {
            lo_chirho: self.lo_chirho.max(other_chirho.lo_chirho),
            hi_chirho: self.hi_chirho.min(other_chirho.hi_chirho),
        }
    }

    /// Returns the union (hull) of two intervals.
    ///
    /// This is the smallest interval containing both input intervals.
    ///
    /// # Mathematical Definition
    ///
    /// `hull([a,b], [c,d]) = [min(a,c), max(b,d)]`
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let a_chirho = IntervalChirho::new_chirho(0.0, 5.0);
    /// let b_chirho = IntervalChirho::new_chirho(10.0, 15.0);
    /// let hull_chirho = a_chirho.hull_chirho(&b_chirho);
    /// assert_eq!(hull_chirho.lo_chirho, 0.0);
    /// assert_eq!(hull_chirho.hi_chirho, 15.0);
    /// ```
    #[inline]
    pub fn hull_chirho(&self, other_chirho: &Self) -> Self {
        Self {
            lo_chirho: self.lo_chirho.min(other_chirho.lo_chirho),
            hi_chirho: self.hi_chirho.max(other_chirho.hi_chirho),
        }
    }

    // ========================================================================
    // INTERVAL ARITHMETIC OPERATIONS
    // ========================================================================

    /// Adds two intervals.
    ///
    /// # Mathematical Definition
    ///
    /// `[a,b] + [c,d] = [a+c, b+d]`
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let x_chirho = IntervalChirho::new_chirho(1.0, 2.0);
    /// let y_chirho = IntervalChirho::new_chirho(3.0, 4.0);
    /// let sum_chirho = x_chirho.add_chirho(&y_chirho);
    /// assert_eq!(sum_chirho.lo_chirho, 4.0);
    /// assert_eq!(sum_chirho.hi_chirho, 6.0);
    /// ```
    #[inline]
    pub fn add_chirho(&self, other_chirho: &Self) -> Self {
        Self {
            lo_chirho: self.lo_chirho + other_chirho.lo_chirho,
            hi_chirho: self.hi_chirho + other_chirho.hi_chirho,
        }
    }

    /// Subtracts an interval from this one.
    ///
    /// # Mathematical Definition
    ///
    /// `[a,b] - [c,d] = [a-d, b-c]`
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let x_chirho = IntervalChirho::new_chirho(5.0, 10.0);
    /// let y_chirho = IntervalChirho::new_chirho(1.0, 3.0);
    /// let diff_chirho = x_chirho.sub_chirho(&y_chirho);
    /// assert_eq!(diff_chirho.lo_chirho, 2.0);  // 5 - 3
    /// assert_eq!(diff_chirho.hi_chirho, 9.0);  // 10 - 1
    /// ```
    #[inline]
    pub fn sub_chirho(&self, other_chirho: &Self) -> Self {
        Self {
            lo_chirho: self.lo_chirho - other_chirho.hi_chirho,
            hi_chirho: self.hi_chirho - other_chirho.lo_chirho,
        }
    }

    /// Multiplies two intervals.
    ///
    /// # Mathematical Definition
    ///
    /// `[a,b] × [c,d] = [min(ac,ad,bc,bd), max(ac,ad,bc,bd)]`
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let x_chirho = IntervalChirho::new_chirho(2.0, 3.0);
    /// let y_chirho = IntervalChirho::new_chirho(4.0, 5.0);
    /// let product_chirho = x_chirho.mul_chirho(&y_chirho);
    /// assert_eq!(product_chirho.lo_chirho, 8.0);   // 2 * 4
    /// assert_eq!(product_chirho.hi_chirho, 15.0);  // 3 * 5
    /// ```
    pub fn mul_chirho(&self, other_chirho: &Self) -> Self {
        let products_chirho = [
            self.lo_chirho * other_chirho.lo_chirho,
            self.lo_chirho * other_chirho.hi_chirho,
            self.hi_chirho * other_chirho.lo_chirho,
            self.hi_chirho * other_chirho.hi_chirho,
        ];

        Self {
            lo_chirho: products_chirho
                .iter()
                .cloned()
                .fold(f64::INFINITY, f64::min),
            hi_chirho: products_chirho
                .iter()
                .cloned()
                .fold(f64::NEG_INFINITY, f64::max),
        }
    }

    /// Divides this interval by another.
    ///
    /// # Warning
    ///
    /// Division by an interval containing zero produces `[-∞, +∞]`.
    ///
    /// # Mathematical Definition
    ///
    /// `[a,b] ÷ [c,d] = [a,b] × [1/d, 1/c]` when 0 ∉ \[c,d\]
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let x_chirho = IntervalChirho::new_chirho(6.0, 12.0);
    /// let y_chirho = IntervalChirho::new_chirho(2.0, 3.0);
    /// let quotient_chirho = x_chirho.div_chirho(&y_chirho);
    /// assert_eq!(quotient_chirho.lo_chirho, 2.0);  // 6 / 3
    /// assert_eq!(quotient_chirho.hi_chirho, 6.0);  // 12 / 2
    /// ```
    pub fn div_chirho(&self, other_chirho: &Self) -> Self {
        // Handle division by interval containing zero
        if other_chirho.lo_chirho <= 0.0 && other_chirho.hi_chirho >= 0.0 {
            return Self::everything_chirho();
        }

        let reciprocal_chirho = Self {
            lo_chirho: 1.0 / other_chirho.hi_chirho,
            hi_chirho: 1.0 / other_chirho.lo_chirho,
        };

        self.mul_chirho(&reciprocal_chirho)
    }

    /// Squares this interval.
    ///
    /// Special handling is needed when the interval contains zero.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let x_chirho = IntervalChirho::new_chirho(2.0, 3.0);
    /// let squared_chirho = x_chirho.square_chirho();
    /// assert_eq!(squared_chirho.lo_chirho, 4.0);
    /// assert_eq!(squared_chirho.hi_chirho, 9.0);
    ///
    /// // Interval containing zero
    /// let y_chirho = IntervalChirho::new_chirho(-2.0, 3.0);
    /// let y_squared_chirho = y_chirho.square_chirho();
    /// assert_eq!(y_squared_chirho.lo_chirho, 0.0);  // 0² = 0
    /// assert_eq!(y_squared_chirho.hi_chirho, 9.0);  // 3² = 9
    /// ```
    pub fn square_chirho(&self) -> Self {
        if self.lo_chirho >= 0.0 {
            // Both bounds positive
            Self {
                lo_chirho: self.lo_chirho * self.lo_chirho,
                hi_chirho: self.hi_chirho * self.hi_chirho,
            }
        } else if self.hi_chirho <= 0.0 {
            // Both bounds negative
            Self {
                lo_chirho: self.hi_chirho * self.hi_chirho,
                hi_chirho: self.lo_chirho * self.lo_chirho,
            }
        } else {
            // Interval contains zero
            Self {
                lo_chirho: 0.0,
                hi_chirho: powi_2_f64_chirho(self.lo_chirho.abs().max(self.hi_chirho.abs())),
            }
        }
    }

    /// Computes the square root of this interval.
    ///
    /// Returns an empty interval if the input is entirely negative.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let x_chirho = IntervalChirho::new_chirho(4.0, 9.0);
    /// let sqrt_chirho = x_chirho.sqrt_chirho();
    /// assert_eq!(sqrt_chirho.lo_chirho, 2.0);
    /// assert_eq!(sqrt_chirho.hi_chirho, 3.0);
    /// ```
    pub fn sqrt_chirho(&self) -> Self {
        if self.hi_chirho < 0.0 {
            // Entirely negative - no real square root
            Self {
                lo_chirho: f64::INFINITY,
                hi_chirho: f64::NEG_INFINITY,
            }
        } else {
            Self {
                lo_chirho: if self.lo_chirho < 0.0 {
                    0.0
                } else {
                    sqrt_f64_chirho(self.lo_chirho)
                },
                hi_chirho: sqrt_f64_chirho(self.hi_chirho),
            }
        }
    }

    /// Computes the absolute value of this interval.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let x_chirho = IntervalChirho::new_chirho(-5.0, 3.0);
    /// let abs_chirho = x_chirho.abs_chirho();
    /// assert_eq!(abs_chirho.lo_chirho, 0.0);
    /// assert_eq!(abs_chirho.hi_chirho, 5.0);
    /// ```
    pub fn abs_chirho(&self) -> Self {
        if self.lo_chirho >= 0.0 {
            *self
        } else if self.hi_chirho <= 0.0 {
            Self {
                lo_chirho: -self.hi_chirho,
                hi_chirho: -self.lo_chirho,
            }
        } else {
            Self {
                lo_chirho: 0.0,
                hi_chirho: self.lo_chirho.abs().max(self.hi_chirho.abs()),
            }
        }
    }

    /// Negates this interval.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let x_chirho = IntervalChirho::new_chirho(2.0, 5.0);
    /// let neg_chirho = x_chirho.neg_chirho();
    /// assert_eq!(neg_chirho.lo_chirho, -5.0);
    /// assert_eq!(neg_chirho.hi_chirho, -2.0);
    /// ```
    #[inline]
    pub fn neg_chirho(&self) -> Self {
        Self {
            lo_chirho: -self.hi_chirho,
            hi_chirho: -self.lo_chirho,
        }
    }

    /// Computes the maximum of two intervals.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let x_chirho = IntervalChirho::new_chirho(1.0, 5.0);
    /// let y_chirho = IntervalChirho::new_chirho(3.0, 7.0);
    /// let max_chirho = x_chirho.max_chirho(&y_chirho);
    /// assert_eq!(max_chirho.lo_chirho, 3.0);
    /// assert_eq!(max_chirho.hi_chirho, 7.0);
    /// ```
    #[inline]
    pub fn max_chirho(&self, other_chirho: &Self) -> Self {
        Self {
            lo_chirho: self.lo_chirho.max(other_chirho.lo_chirho),
            hi_chirho: self.hi_chirho.max(other_chirho.hi_chirho),
        }
    }

    /// Computes the minimum of two intervals.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::IntervalChirho;
    ///
    /// let x_chirho = IntervalChirho::new_chirho(1.0, 5.0);
    /// let y_chirho = IntervalChirho::new_chirho(3.0, 7.0);
    /// let min_chirho = x_chirho.min_chirho(&y_chirho);
    /// assert_eq!(min_chirho.lo_chirho, 1.0);
    /// assert_eq!(min_chirho.hi_chirho, 5.0);
    /// ```
    #[inline]
    pub fn min_chirho(&self, other_chirho: &Self) -> Self {
        Self {
            lo_chirho: self.lo_chirho.min(other_chirho.lo_chirho),
            hi_chirho: self.hi_chirho.min(other_chirho.hi_chirho),
        }
    }
}

impl fmt::Debug for IntervalChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty_chirho() {
            write!(f_chirho, "∅")
        } else if self.is_exact_chirho() {
            write!(f_chirho, "{}", self.lo_chirho)
        } else {
            write!(f_chirho, "[{}, {}]", self.lo_chirho, self.hi_chirho)
        }
    }
}

impl fmt::Display for IntervalChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f_chirho)
    }
}

impl Default for IntervalChirho {
    fn default() -> Self {
        Self::everything_chirho()
    }
}

// ============================================================================
// NUMERIC INFO LATTICE
// ============================================================================

/// Partial numeric information forming a lattice.
///
/// This represents the information lattice for numeric values:
///
/// ```text
///              Contradiction (⊤)
///                    ↑
///         ┌─────────┴─────────┐
///         ↑                   ↑
///    Interval [a,b]    Interval [c,d]  ...
///         ↑                   ↑
///         └─────────┬─────────┘
///                   ↑
///              Nothing (⊥)
/// ```
///
/// The lattice ordering is: `Nothing ⊑ Interval ⊑ Contradiction`
///
/// # Example
///
/// ```
/// use propagators_chirho::NumericInfoChirho;
///
/// let nothing_chirho = NumericInfoChirho::nothing_chirho();
/// let interval_chirho = NumericInfoChirho::interval_chirho(0.0, 10.0);
///
/// // Merging nothing with something gives something
/// let merged_chirho = nothing_chirho.merge_chirho(&interval_chirho);
/// assert!(matches!(merged_chirho, NumericInfoChirho::IntervalChirho(_)));
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NumericInfoChirho {
    /// No information known (bottom of lattice).
    NothingChirho,
    /// Partial information as an interval.
    IntervalChirho(IntervalChirho),
    /// Contradictory information (top of lattice).
    ContradictionChirho,
}

impl NumericInfoChirho {
    /// Creates the "no information" state.
    #[inline]
    pub fn nothing_chirho() -> Self {
        Self::NothingChirho
    }

    /// Creates an exact value.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::NumericInfoChirho;
    ///
    /// let exact_chirho = NumericInfoChirho::exact_chirho(42.0);
    /// assert_eq!(exact_chirho.as_interval_chirho().unwrap().lo_chirho, 42.0);
    /// ```
    #[inline]
    pub fn exact_chirho(value_chirho: f64) -> Self {
        Self::IntervalChirho(IntervalChirho::exact_chirho(value_chirho))
    }

    /// Creates an interval.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::NumericInfoChirho;
    ///
    /// let interval_chirho = NumericInfoChirho::interval_chirho(0.0, 100.0);
    /// ```
    #[inline]
    pub fn interval_chirho(lo_chirho: f64, hi_chirho: f64) -> Self {
        Self::IntervalChirho(IntervalChirho::new_chirho(lo_chirho, hi_chirho))
    }

    /// Creates a contradiction state.
    #[inline]
    pub fn contradiction_chirho() -> Self {
        Self::ContradictionChirho
    }

    /// Returns `true` if this is the "no information" state.
    #[inline]
    pub fn is_nothing_chirho(&self) -> bool {
        matches!(self, Self::NothingChirho)
    }

    /// Returns `true` if this is a contradiction.
    #[inline]
    pub fn is_contradiction_chirho(&self) -> bool {
        matches!(self, Self::ContradictionChirho)
    }

    /// Returns the interval if this is an interval, `None` otherwise.
    #[inline]
    pub fn as_interval_chirho(&self) -> Option<&IntervalChirho> {
        match self {
            Self::IntervalChirho(interval_chirho) => Some(interval_chirho),
            _ => None,
        }
    }

    /// Merges two pieces of partial information.
    ///
    /// This is the lattice join operation:
    /// - `Nothing ⊔ x = x`
    /// - `x ⊔ Nothing = x`
    /// - `Interval ⊔ Interval = intersection (or Contradiction if empty)`
    /// - `Contradiction ⊔ x = Contradiction`
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::NumericInfoChirho;
    ///
    /// let a_chirho = NumericInfoChirho::interval_chirho(0.0, 10.0);
    /// let b_chirho = NumericInfoChirho::interval_chirho(5.0, 15.0);
    /// let merged_chirho = a_chirho.merge_chirho(&b_chirho);
    ///
    /// // Intersection is [5, 10]
    /// let interval_chirho = merged_chirho.as_interval_chirho().unwrap();
    /// assert_eq!(interval_chirho.lo_chirho, 5.0);
    /// assert_eq!(interval_chirho.hi_chirho, 10.0);
    /// ```
    pub fn merge_chirho(&self, other_chirho: &Self) -> Self {
        match (self, other_chirho) {
            (Self::NothingChirho, other_chirho) => *other_chirho,
            (this_chirho, Self::NothingChirho) => *this_chirho,
            (Self::ContradictionChirho, _) | (_, Self::ContradictionChirho) => {
                Self::ContradictionChirho
            }
            (Self::IntervalChirho(a_chirho), Self::IntervalChirho(b_chirho)) => {
                let intersection_chirho = a_chirho.intersect_chirho(b_chirho);
                if intersection_chirho.is_empty_chirho() {
                    Self::ContradictionChirho
                } else {
                    Self::IntervalChirho(intersection_chirho)
                }
            }
        }
    }

    /// Returns `true` if this information is more specific than `other`.
    ///
    /// In the lattice: `Nothing ⊑ Interval ⊑ Contradiction`
    pub fn refines_chirho(&self, other_chirho: &Self) -> bool {
        match (self, other_chirho) {
            (_, Self::NothingChirho) => true,
            (Self::NothingChirho, _) => false,
            (Self::ContradictionChirho, _) => true,
            (_, Self::ContradictionChirho) => false,
            (Self::IntervalChirho(a_chirho), Self::IntervalChirho(b_chirho)) => {
                a_chirho.subset_of_chirho(b_chirho)
            }
        }
    }
}

impl Default for NumericInfoChirho {
    fn default() -> Self {
        Self::NothingChirho
    }
}

impl fmt::Display for NumericInfoChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NothingChirho => write!(f_chirho, "⊥"),
            Self::IntervalChirho(interval_chirho) => write!(f_chirho, "{interval_chirho}"),
            Self::ContradictionChirho => write!(f_chirho, "⊤ (contradiction)"),
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    // ========================================================================
    // INTERVAL TESTS
    // ========================================================================

    mod interval_tests_chirho {
        use super::*;

        #[test]
        fn test_interval_creation_chirho() {
            let interval_chirho = IntervalChirho::new_chirho(1.0, 5.0);
            assert_eq!(interval_chirho.lo_chirho, 1.0);
            assert_eq!(interval_chirho.hi_chirho, 5.0);
        }

        #[test]
        fn test_exact_interval_chirho() {
            let exact_chirho = IntervalChirho::exact_chirho(42.0);
            assert!(exact_chirho.is_exact_chirho());
            assert_eq!(exact_chirho.width_chirho(), 0.0);
        }

        #[test]
        fn test_everything_interval_chirho() {
            let everything_chirho = IntervalChirho::everything_chirho();
            assert_eq!(everything_chirho.lo_chirho, f64::NEG_INFINITY);
            assert_eq!(everything_chirho.hi_chirho, f64::INFINITY);
        }

        #[test]
        fn test_interval_intersection_chirho() {
            let a_chirho = IntervalChirho::new_chirho(0.0, 10.0);
            let b_chirho = IntervalChirho::new_chirho(5.0, 15.0);
            let intersection_chirho = a_chirho.intersect_chirho(&b_chirho);
            assert_eq!(intersection_chirho.lo_chirho, 5.0);
            assert_eq!(intersection_chirho.hi_chirho, 10.0);
        }

        #[test]
        fn test_interval_empty_intersection_chirho() {
            let a_chirho = IntervalChirho::new_chirho(0.0, 5.0);
            let b_chirho = IntervalChirho::new_chirho(10.0, 15.0);
            let intersection_chirho = a_chirho.intersect_chirho(&b_chirho);
            assert!(intersection_chirho.is_empty_chirho());
        }

        #[test]
        fn test_interval_hull_chirho() {
            let a_chirho = IntervalChirho::new_chirho(0.0, 5.0);
            let b_chirho = IntervalChirho::new_chirho(10.0, 15.0);
            let hull_chirho = a_chirho.hull_chirho(&b_chirho);
            assert_eq!(hull_chirho.lo_chirho, 0.0);
            assert_eq!(hull_chirho.hi_chirho, 15.0);
        }

        #[test]
        fn test_interval_contains_chirho() {
            let interval_chirho = IntervalChirho::new_chirho(0.0, 10.0);
            assert!(interval_chirho.contains_chirho(5.0));
            assert!(interval_chirho.contains_chirho(0.0));
            assert!(interval_chirho.contains_chirho(10.0));
            assert!(!interval_chirho.contains_chirho(-1.0));
            assert!(!interval_chirho.contains_chirho(11.0));
        }

        #[test]
        fn test_interval_add_chirho() {
            let a_chirho = IntervalChirho::new_chirho(1.0, 2.0);
            let b_chirho = IntervalChirho::new_chirho(3.0, 4.0);
            let sum_chirho = a_chirho.add_chirho(&b_chirho);
            assert_eq!(sum_chirho.lo_chirho, 4.0);
            assert_eq!(sum_chirho.hi_chirho, 6.0);
        }

        #[test]
        fn test_interval_sub_chirho() {
            let a_chirho = IntervalChirho::new_chirho(5.0, 10.0);
            let b_chirho = IntervalChirho::new_chirho(1.0, 3.0);
            let diff_chirho = a_chirho.sub_chirho(&b_chirho);
            assert_eq!(diff_chirho.lo_chirho, 2.0); // 5 - 3
            assert_eq!(diff_chirho.hi_chirho, 9.0); // 10 - 1
        }

        #[test]
        fn test_interval_mul_positive_chirho() {
            let a_chirho = IntervalChirho::new_chirho(2.0, 3.0);
            let b_chirho = IntervalChirho::new_chirho(4.0, 5.0);
            let product_chirho = a_chirho.mul_chirho(&b_chirho);
            assert_eq!(product_chirho.lo_chirho, 8.0);
            assert_eq!(product_chirho.hi_chirho, 15.0);
        }

        #[test]
        fn test_interval_mul_mixed_signs_chirho() {
            let a_chirho = IntervalChirho::new_chirho(-2.0, 3.0);
            let b_chirho = IntervalChirho::new_chirho(-1.0, 4.0);
            let product_chirho = a_chirho.mul_chirho(&b_chirho);
            assert_eq!(product_chirho.lo_chirho, -8.0); // 3 * -1 or -2 * 4
            assert_eq!(product_chirho.hi_chirho, 12.0); // 3 * 4
        }

        #[test]
        fn test_interval_div_chirho() {
            let a_chirho = IntervalChirho::new_chirho(6.0, 12.0);
            let b_chirho = IntervalChirho::new_chirho(2.0, 3.0);
            let quotient_chirho = a_chirho.div_chirho(&b_chirho);
            assert_eq!(quotient_chirho.lo_chirho, 2.0); // 6 / 3
            assert_eq!(quotient_chirho.hi_chirho, 6.0); // 12 / 2
        }

        #[test]
        fn test_interval_div_by_zero_chirho() {
            let a_chirho = IntervalChirho::new_chirho(1.0, 2.0);
            let b_chirho = IntervalChirho::new_chirho(-1.0, 1.0); // Contains zero
            let quotient_chirho = a_chirho.div_chirho(&b_chirho);
            assert_eq!(quotient_chirho.lo_chirho, f64::NEG_INFINITY);
            assert_eq!(quotient_chirho.hi_chirho, f64::INFINITY);
        }

        #[test]
        fn test_interval_square_positive_chirho() {
            let x_chirho = IntervalChirho::new_chirho(2.0, 3.0);
            let squared_chirho = x_chirho.square_chirho();
            assert_eq!(squared_chirho.lo_chirho, 4.0);
            assert_eq!(squared_chirho.hi_chirho, 9.0);
        }

        #[test]
        fn test_interval_square_negative_chirho() {
            let x_chirho = IntervalChirho::new_chirho(-3.0, -2.0);
            let squared_chirho = x_chirho.square_chirho();
            assert_eq!(squared_chirho.lo_chirho, 4.0);
            assert_eq!(squared_chirho.hi_chirho, 9.0);
        }

        #[test]
        fn test_interval_square_mixed_chirho() {
            let x_chirho = IntervalChirho::new_chirho(-2.0, 3.0);
            let squared_chirho = x_chirho.square_chirho();
            assert_eq!(squared_chirho.lo_chirho, 0.0);
            assert_eq!(squared_chirho.hi_chirho, 9.0);
        }

        #[test]
        fn test_interval_sqrt_chirho() {
            let x_chirho = IntervalChirho::new_chirho(4.0, 9.0);
            let sqrt_chirho = x_chirho.sqrt_chirho();
            assert_eq!(sqrt_chirho.lo_chirho, 2.0);
            assert_eq!(sqrt_chirho.hi_chirho, 3.0);
        }

        #[test]
        fn test_interval_sqrt_partial_negative_chirho() {
            let x_chirho = IntervalChirho::new_chirho(-4.0, 9.0);
            let sqrt_chirho = x_chirho.sqrt_chirho();
            assert_eq!(sqrt_chirho.lo_chirho, 0.0);
            assert_eq!(sqrt_chirho.hi_chirho, 3.0);
        }

        #[test]
        fn test_interval_abs_positive_chirho() {
            let x_chirho = IntervalChirho::new_chirho(2.0, 5.0);
            let abs_chirho = x_chirho.abs_chirho();
            assert_eq!(abs_chirho.lo_chirho, 2.0);
            assert_eq!(abs_chirho.hi_chirho, 5.0);
        }

        #[test]
        fn test_interval_abs_negative_chirho() {
            let x_chirho = IntervalChirho::new_chirho(-5.0, -2.0);
            let abs_chirho = x_chirho.abs_chirho();
            assert_eq!(abs_chirho.lo_chirho, 2.0);
            assert_eq!(abs_chirho.hi_chirho, 5.0);
        }

        #[test]
        fn test_interval_abs_mixed_chirho() {
            let x_chirho = IntervalChirho::new_chirho(-3.0, 5.0);
            let abs_chirho = x_chirho.abs_chirho();
            assert_eq!(abs_chirho.lo_chirho, 0.0);
            assert_eq!(abs_chirho.hi_chirho, 5.0);
        }
    }

    // ========================================================================
    // NUMERIC INFO TESTS
    // ========================================================================

    mod numeric_info_tests_chirho {
        use super::*;

        #[test]
        fn test_nothing_merge_chirho() {
            let nothing_chirho = NumericInfoChirho::nothing_chirho();
            let interval_chirho = NumericInfoChirho::interval_chirho(0.0, 10.0);

            assert_eq!(
                nothing_chirho.merge_chirho(&interval_chirho),
                interval_chirho
            );
            assert_eq!(
                interval_chirho.merge_chirho(&nothing_chirho),
                interval_chirho
            );
        }

        #[test]
        fn test_interval_merge_chirho() {
            let a_chirho = NumericInfoChirho::interval_chirho(0.0, 10.0);
            let b_chirho = NumericInfoChirho::interval_chirho(5.0, 15.0);
            let merged_chirho = a_chirho.merge_chirho(&b_chirho);

            match merged_chirho {
                NumericInfoChirho::IntervalChirho(interval_chirho) => {
                    assert_eq!(interval_chirho.lo_chirho, 5.0);
                    assert_eq!(interval_chirho.hi_chirho, 10.0);
                }
                _ => panic!("Expected interval"),
            }
        }

        #[test]
        fn test_contradiction_merge_chirho() {
            let a_chirho = NumericInfoChirho::interval_chirho(0.0, 5.0);
            let b_chirho = NumericInfoChirho::interval_chirho(10.0, 15.0);
            let merged_chirho = a_chirho.merge_chirho(&b_chirho);

            assert!(merged_chirho.is_contradiction_chirho());
        }

        #[test]
        fn test_contradiction_absorbs_chirho() {
            let contradiction_chirho = NumericInfoChirho::contradiction_chirho();
            let interval_chirho = NumericInfoChirho::interval_chirho(0.0, 10.0);

            assert!(contradiction_chirho
                .merge_chirho(&interval_chirho)
                .is_contradiction_chirho());
            assert!(interval_chirho
                .merge_chirho(&contradiction_chirho)
                .is_contradiction_chirho());
        }

        #[test]
        fn test_refines_chirho() {
            let nothing_chirho = NumericInfoChirho::nothing_chirho();
            let wide_chirho = NumericInfoChirho::interval_chirho(0.0, 100.0);
            let narrow_chirho = NumericInfoChirho::interval_chirho(40.0, 60.0);
            let contradiction_chirho = NumericInfoChirho::contradiction_chirho();

            // Nothing is refined by everything
            assert!(wide_chirho.refines_chirho(&nothing_chirho));
            assert!(narrow_chirho.refines_chirho(&nothing_chirho));
            assert!(contradiction_chirho.refines_chirho(&nothing_chirho));

            // Narrow refines wide
            assert!(narrow_chirho.refines_chirho(&wide_chirho));
            assert!(!wide_chirho.refines_chirho(&narrow_chirho));

            // Contradiction refines everything
            assert!(contradiction_chirho.refines_chirho(&wide_chirho));
            assert!(contradiction_chirho.refines_chirho(&narrow_chirho));
        }

        #[cfg(feature = "serde")]
        mod serde_tests_chirho {
            use super::*;

            #[test]
            fn test_interval_serde_roundtrip_chirho() {
                let interval_chirho = IntervalChirho::new_chirho(1.5, 3.5);
                let json_chirho = serde_json::to_string(&interval_chirho).unwrap();
                let deserialized_chirho: IntervalChirho =
                    serde_json::from_str(&json_chirho).unwrap();
                assert_eq!(interval_chirho, deserialized_chirho);
            }

            #[test]
            fn test_numeric_info_serde_roundtrip_chirho() {
                // Test Nothing
                let nothing_chirho = NumericInfoChirho::NothingChirho;
                let json_chirho = serde_json::to_string(&nothing_chirho).unwrap();
                let deserialized_chirho: NumericInfoChirho =
                    serde_json::from_str(&json_chirho).unwrap();
                assert_eq!(nothing_chirho, deserialized_chirho);

                // Test Interval
                let interval_chirho = NumericInfoChirho::interval_chirho(0.0, 100.0);
                let json_chirho = serde_json::to_string(&interval_chirho).unwrap();
                let deserialized_chirho: NumericInfoChirho =
                    serde_json::from_str(&json_chirho).unwrap();
                assert_eq!(interval_chirho, deserialized_chirho);

                // Test Contradiction
                let contradiction_chirho = NumericInfoChirho::ContradictionChirho;
                let json_chirho = serde_json::to_string(&contradiction_chirho).unwrap();
                let deserialized_chirho: NumericInfoChirho =
                    serde_json::from_str(&json_chirho).unwrap();
                assert_eq!(contradiction_chirho, deserialized_chirho);
            }
        }
    }
}
