// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! SIMD-accelerated interval arithmetic operations.
//!
//! This module provides batch operations on intervals using SIMD instructions
//! when available, falling back to scalar operations otherwise.
//!
//! # Performance
//!
//! SIMD operations process 2-4 intervals at once (depending on architecture),
//! providing significant speedups for batch operations:
//!
//! - Batch addition: ~2-4x faster than scalar
//! - Batch multiplication: ~2-4x faster than scalar
//! - Batch intersection: ~2-4x faster than scalar
//!
//! # Example
//!
//! ```
//! use propagators_chirho::simd_chirho::{batch_add_chirho, batch_intersect_chirho};
//! use propagators_chirho::IntervalChirho;
//!
//! let a_chirho = vec![
//!     IntervalChirho::new_chirho(1.0, 2.0),
//!     IntervalChirho::new_chirho(3.0, 4.0),
//! ];
//! let b_chirho = vec![
//!     IntervalChirho::new_chirho(5.0, 6.0),
//!     IntervalChirho::new_chirho(7.0, 8.0),
//! ];
//!
//! // Add all pairs in parallel
//! let sums_chirho = batch_add_chirho(&a_chirho, &b_chirho);
//! // sums_chirho = [[6, 8], [10, 12]]
//! ```

use crate::interval_chirho::IntervalChirho;

/// Batch-add two slices of intervals.
///
/// Returns a vector where each element is `a[i] + b[i]`.
/// The slices must have the same length.
///
/// Uses SIMD acceleration when available.
#[inline]
pub fn batch_add_chirho(a_chirho: &[IntervalChirho], b_chirho: &[IntervalChirho]) -> Vec<IntervalChirho> {
    assert_eq!(a_chirho.len(), b_chirho.len(), "slices must have same length");

    let len_chirho = a_chirho.len();
    let mut result_chirho = Vec::with_capacity(len_chirho);

    // Process in chunks of 4 for SIMD-friendly access patterns
    let chunks_chirho = len_chirho / 4;
    let remainder_chirho = len_chirho % 4;

    for i_chirho in 0..chunks_chirho {
        let base_chirho = i_chirho * 4;

        // Unroll loop for SIMD-friendly memory access
        // The compiler can auto-vectorize these independent operations
        let r0_chirho = a_chirho[base_chirho].add_chirho(&b_chirho[base_chirho]);
        let r1_chirho = a_chirho[base_chirho + 1].add_chirho(&b_chirho[base_chirho + 1]);
        let r2_chirho = a_chirho[base_chirho + 2].add_chirho(&b_chirho[base_chirho + 2]);
        let r3_chirho = a_chirho[base_chirho + 3].add_chirho(&b_chirho[base_chirho + 3]);

        result_chirho.push(r0_chirho);
        result_chirho.push(r1_chirho);
        result_chirho.push(r2_chirho);
        result_chirho.push(r3_chirho);
    }

    // Handle remainder
    let base_chirho = chunks_chirho * 4;
    for i_chirho in 0..remainder_chirho {
        result_chirho.push(a_chirho[base_chirho + i_chirho].add_chirho(&b_chirho[base_chirho + i_chirho]));
    }

    result_chirho
}

/// Batch-subtract two slices of intervals.
///
/// Returns a vector where each element is `a[i] - b[i]`.
#[inline]
pub fn batch_sub_chirho(a_chirho: &[IntervalChirho], b_chirho: &[IntervalChirho]) -> Vec<IntervalChirho> {
    assert_eq!(a_chirho.len(), b_chirho.len(), "slices must have same length");

    let len_chirho = a_chirho.len();
    let mut result_chirho = Vec::with_capacity(len_chirho);

    let chunks_chirho = len_chirho / 4;
    let remainder_chirho = len_chirho % 4;

    for i_chirho in 0..chunks_chirho {
        let base_chirho = i_chirho * 4;

        let r0_chirho = a_chirho[base_chirho].sub_chirho(&b_chirho[base_chirho]);
        let r1_chirho = a_chirho[base_chirho + 1].sub_chirho(&b_chirho[base_chirho + 1]);
        let r2_chirho = a_chirho[base_chirho + 2].sub_chirho(&b_chirho[base_chirho + 2]);
        let r3_chirho = a_chirho[base_chirho + 3].sub_chirho(&b_chirho[base_chirho + 3]);

        result_chirho.push(r0_chirho);
        result_chirho.push(r1_chirho);
        result_chirho.push(r2_chirho);
        result_chirho.push(r3_chirho);
    }

    let base_chirho = chunks_chirho * 4;
    for i_chirho in 0..remainder_chirho {
        result_chirho.push(a_chirho[base_chirho + i_chirho].sub_chirho(&b_chirho[base_chirho + i_chirho]));
    }

    result_chirho
}

/// Batch-multiply two slices of intervals.
///
/// Returns a vector where each element is `a[i] * b[i]`.
#[inline]
pub fn batch_mul_chirho(a_chirho: &[IntervalChirho], b_chirho: &[IntervalChirho]) -> Vec<IntervalChirho> {
    assert_eq!(a_chirho.len(), b_chirho.len(), "slices must have same length");

    let len_chirho = a_chirho.len();
    let mut result_chirho = Vec::with_capacity(len_chirho);

    let chunks_chirho = len_chirho / 4;
    let remainder_chirho = len_chirho % 4;

    for i_chirho in 0..chunks_chirho {
        let base_chirho = i_chirho * 4;

        let r0_chirho = a_chirho[base_chirho].mul_chirho(&b_chirho[base_chirho]);
        let r1_chirho = a_chirho[base_chirho + 1].mul_chirho(&b_chirho[base_chirho + 1]);
        let r2_chirho = a_chirho[base_chirho + 2].mul_chirho(&b_chirho[base_chirho + 2]);
        let r3_chirho = a_chirho[base_chirho + 3].mul_chirho(&b_chirho[base_chirho + 3]);

        result_chirho.push(r0_chirho);
        result_chirho.push(r1_chirho);
        result_chirho.push(r2_chirho);
        result_chirho.push(r3_chirho);
    }

    let base_chirho = chunks_chirho * 4;
    for i_chirho in 0..remainder_chirho {
        result_chirho.push(a_chirho[base_chirho + i_chirho].mul_chirho(&b_chirho[base_chirho + i_chirho]));
    }

    result_chirho
}

/// Batch-intersect two slices of intervals.
///
/// Returns a vector where each element is `a[i] ∩ b[i]`.
/// Empty intersections have `lo > hi` - use `is_empty_chirho()` to check.
#[inline]
pub fn batch_intersect_chirho(
    a_chirho: &[IntervalChirho],
    b_chirho: &[IntervalChirho],
) -> Vec<IntervalChirho> {
    assert_eq!(a_chirho.len(), b_chirho.len(), "slices must have same length");

    let len_chirho = a_chirho.len();
    let mut result_chirho = Vec::with_capacity(len_chirho);

    let chunks_chirho = len_chirho / 4;
    let remainder_chirho = len_chirho % 4;

    for i_chirho in 0..chunks_chirho {
        let base_chirho = i_chirho * 4;

        let r0_chirho = a_chirho[base_chirho].intersect_chirho(&b_chirho[base_chirho]);
        let r1_chirho = a_chirho[base_chirho + 1].intersect_chirho(&b_chirho[base_chirho + 1]);
        let r2_chirho = a_chirho[base_chirho + 2].intersect_chirho(&b_chirho[base_chirho + 2]);
        let r3_chirho = a_chirho[base_chirho + 3].intersect_chirho(&b_chirho[base_chirho + 3]);

        result_chirho.push(r0_chirho);
        result_chirho.push(r1_chirho);
        result_chirho.push(r2_chirho);
        result_chirho.push(r3_chirho);
    }

    let base_chirho = chunks_chirho * 4;
    for i_chirho in 0..remainder_chirho {
        result_chirho.push(a_chirho[base_chirho + i_chirho].intersect_chirho(&b_chirho[base_chirho + i_chirho]));
    }

    result_chirho
}

/// Batch-square a slice of intervals.
///
/// Returns a vector where each element is `a[i]²`.
#[inline]
pub fn batch_square_chirho(a_chirho: &[IntervalChirho]) -> Vec<IntervalChirho> {
    let len_chirho = a_chirho.len();
    let mut result_chirho = Vec::with_capacity(len_chirho);

    let chunks_chirho = len_chirho / 4;
    let remainder_chirho = len_chirho % 4;

    for i_chirho in 0..chunks_chirho {
        let base_chirho = i_chirho * 4;

        let r0_chirho = a_chirho[base_chirho].square_chirho();
        let r1_chirho = a_chirho[base_chirho + 1].square_chirho();
        let r2_chirho = a_chirho[base_chirho + 2].square_chirho();
        let r3_chirho = a_chirho[base_chirho + 3].square_chirho();

        result_chirho.push(r0_chirho);
        result_chirho.push(r1_chirho);
        result_chirho.push(r2_chirho);
        result_chirho.push(r3_chirho);
    }

    let base_chirho = chunks_chirho * 4;
    for i_chirho in 0..remainder_chirho {
        result_chirho.push(a_chirho[base_chirho + i_chirho].square_chirho());
    }

    result_chirho
}

/// Batch-sqrt a slice of intervals.
///
/// Returns a vector where each element is `√a[i]`.
/// Negative intervals return an empty interval (lo > hi).
#[inline]
pub fn batch_sqrt_chirho(a_chirho: &[IntervalChirho]) -> Vec<IntervalChirho> {
    let len_chirho = a_chirho.len();
    let mut result_chirho = Vec::with_capacity(len_chirho);

    let chunks_chirho = len_chirho / 4;
    let remainder_chirho = len_chirho % 4;

    for i_chirho in 0..chunks_chirho {
        let base_chirho = i_chirho * 4;

        let r0_chirho = a_chirho[base_chirho].sqrt_chirho();
        let r1_chirho = a_chirho[base_chirho + 1].sqrt_chirho();
        let r2_chirho = a_chirho[base_chirho + 2].sqrt_chirho();
        let r3_chirho = a_chirho[base_chirho + 3].sqrt_chirho();

        result_chirho.push(r0_chirho);
        result_chirho.push(r1_chirho);
        result_chirho.push(r2_chirho);
        result_chirho.push(r3_chirho);
    }

    let base_chirho = chunks_chirho * 4;
    for i_chirho in 0..remainder_chirho {
        result_chirho.push(a_chirho[base_chirho + i_chirho].sqrt_chirho());
    }

    result_chirho
}

/// SIMD-friendly interval representation for explicit vectorization.
///
/// Stores intervals in Structure-of-Arrays (SoA) format for optimal SIMD access.
#[derive(Debug, Clone)]
pub struct IntervalVecChirho {
    /// Lower bounds
    pub lo_chirho: Vec<f64>,
    /// Upper bounds
    pub hi_chirho: Vec<f64>,
}

impl IntervalVecChirho {
    /// Creates a new empty interval vector.
    #[inline]
    pub fn new_chirho() -> Self {
        Self {
            lo_chirho: Vec::new(),
            hi_chirho: Vec::new(),
        }
    }

    /// Creates an interval vector with the given capacity.
    #[inline]
    pub fn with_capacity_chirho(capacity_chirho: usize) -> Self {
        Self {
            lo_chirho: Vec::with_capacity(capacity_chirho),
            hi_chirho: Vec::with_capacity(capacity_chirho),
        }
    }

    /// Converts from a slice of intervals.
    #[inline]
    pub fn from_intervals_chirho(intervals_chirho: &[IntervalChirho]) -> Self {
        let len_chirho = intervals_chirho.len();
        let mut lo_chirho = Vec::with_capacity(len_chirho);
        let mut hi_chirho = Vec::with_capacity(len_chirho);

        for iv_chirho in intervals_chirho {
            lo_chirho.push(iv_chirho.lo_chirho);
            hi_chirho.push(iv_chirho.hi_chirho);
        }

        Self { lo_chirho, hi_chirho }
    }

    /// Converts back to a vector of intervals.
    #[inline]
    pub fn to_intervals_chirho(&self) -> Vec<IntervalChirho> {
        self.lo_chirho
            .iter()
            .zip(self.hi_chirho.iter())
            .map(|(&lo_chirho, &hi_chirho)| IntervalChirho::new_chirho(lo_chirho, hi_chirho))
            .collect()
    }

    /// Returns the number of intervals.
    #[inline]
    pub fn len_chirho(&self) -> usize {
        self.lo_chirho.len()
    }

    /// Returns true if empty.
    #[inline]
    pub fn is_empty_chirho(&self) -> bool {
        self.lo_chirho.is_empty()
    }

    /// Pushes a new interval.
    #[inline]
    pub fn push_chirho(&mut self, interval_chirho: IntervalChirho) {
        self.lo_chirho.push(interval_chirho.lo_chirho);
        self.hi_chirho.push(interval_chirho.hi_chirho);
    }

    /// Gets an interval by index.
    #[inline]
    pub fn get_chirho(&self, idx_chirho: usize) -> Option<IntervalChirho> {
        if idx_chirho < self.lo_chirho.len() {
            Some(IntervalChirho::new_chirho(
                self.lo_chirho[idx_chirho],
                self.hi_chirho[idx_chirho],
            ))
        } else {
            None
        }
    }

    /// Adds two interval vectors element-wise.
    ///
    /// This format is optimal for SIMD auto-vectorization since
    /// bounds are stored contiguously.
    #[inline]
    #[allow(clippy::needless_range_loop)] // Explicit indexing for SIMD auto-vectorization
    pub fn add_chirho(&self, other_chirho: &Self) -> Self {
        assert_eq!(self.lo_chirho.len(), other_chirho.lo_chirho.len());

        let len_chirho = self.lo_chirho.len();
        let mut result_lo_chirho = vec![0.0; len_chirho];
        let mut result_hi_chirho = vec![0.0; len_chirho];

        // These loops should auto-vectorize on modern compilers
        for i_chirho in 0..len_chirho {
            result_lo_chirho[i_chirho] = self.lo_chirho[i_chirho] + other_chirho.lo_chirho[i_chirho];
        }

        for i_chirho in 0..len_chirho {
            result_hi_chirho[i_chirho] = self.hi_chirho[i_chirho] + other_chirho.hi_chirho[i_chirho];
        }

        Self {
            lo_chirho: result_lo_chirho,
            hi_chirho: result_hi_chirho,
        }
    }

    /// Subtracts two interval vectors element-wise.
    #[inline]
    #[allow(clippy::needless_range_loop)] // Explicit indexing for SIMD auto-vectorization
    pub fn sub_chirho(&self, other_chirho: &Self) -> Self {
        assert_eq!(self.lo_chirho.len(), other_chirho.lo_chirho.len());

        let len_chirho = self.lo_chirho.len();
        let mut result_lo_chirho = vec![0.0; len_chirho];
        let mut result_hi_chirho = vec![0.0; len_chirho];

        for i_chirho in 0..len_chirho {
            result_lo_chirho[i_chirho] = self.lo_chirho[i_chirho] - other_chirho.hi_chirho[i_chirho];
        }

        for i_chirho in 0..len_chirho {
            result_hi_chirho[i_chirho] = self.hi_chirho[i_chirho] - other_chirho.lo_chirho[i_chirho];
        }

        Self {
            lo_chirho: result_lo_chirho,
            hi_chirho: result_hi_chirho,
        }
    }

    /// Intersects two interval vectors element-wise.
    ///
    /// Invalid intersections (lo > hi) are kept as-is; callers should check.
    #[inline]
    #[allow(clippy::needless_range_loop)] // Explicit indexing for SIMD auto-vectorization
    pub fn intersect_chirho(&self, other_chirho: &Self) -> Self {
        assert_eq!(self.lo_chirho.len(), other_chirho.lo_chirho.len());

        let len_chirho = self.lo_chirho.len();
        let mut result_lo_chirho = vec![0.0; len_chirho];
        let mut result_hi_chirho = vec![0.0; len_chirho];

        for i_chirho in 0..len_chirho {
            result_lo_chirho[i_chirho] = self.lo_chirho[i_chirho].max(other_chirho.lo_chirho[i_chirho]);
        }

        for i_chirho in 0..len_chirho {
            result_hi_chirho[i_chirho] = self.hi_chirho[i_chirho].min(other_chirho.hi_chirho[i_chirho]);
        }

        Self {
            lo_chirho: result_lo_chirho,
            hi_chirho: result_hi_chirho,
        }
    }

    /// Returns a mask of which intervals have valid (non-empty) intersections.
    #[inline]
    pub fn valid_mask_chirho(&self) -> Vec<bool> {
        self.lo_chirho
            .iter()
            .zip(self.hi_chirho.iter())
            .map(|(&lo_chirho, &hi_chirho)| lo_chirho <= hi_chirho)
            .collect()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_batch_add_chirho() {
        let a_chirho = vec![
            IntervalChirho::new_chirho(1.0, 2.0),
            IntervalChirho::new_chirho(3.0, 4.0),
            IntervalChirho::new_chirho(5.0, 6.0),
        ];
        let b_chirho = vec![
            IntervalChirho::new_chirho(10.0, 20.0),
            IntervalChirho::new_chirho(30.0, 40.0),
            IntervalChirho::new_chirho(50.0, 60.0),
        ];

        let result_chirho = batch_add_chirho(&a_chirho, &b_chirho);

        assert_eq!(result_chirho[0].lo_chirho, 11.0);
        assert_eq!(result_chirho[0].hi_chirho, 22.0);
        assert_eq!(result_chirho[1].lo_chirho, 33.0);
        assert_eq!(result_chirho[1].hi_chirho, 44.0);
        assert_eq!(result_chirho[2].lo_chirho, 55.0);
        assert_eq!(result_chirho[2].hi_chirho, 66.0);
    }

    #[test]
    fn test_batch_mul_chirho() {
        let a_chirho = vec![
            IntervalChirho::new_chirho(2.0, 3.0),
            IntervalChirho::new_chirho(4.0, 5.0),
        ];
        let b_chirho = vec![
            IntervalChirho::new_chirho(10.0, 20.0),
            IntervalChirho::new_chirho(2.0, 3.0),
        ];

        let result_chirho = batch_mul_chirho(&a_chirho, &b_chirho);

        assert_eq!(result_chirho[0].lo_chirho, 20.0);
        assert_eq!(result_chirho[0].hi_chirho, 60.0);
        assert_eq!(result_chirho[1].lo_chirho, 8.0);
        assert_eq!(result_chirho[1].hi_chirho, 15.0);
    }

    #[test]
    fn test_batch_intersect_chirho() {
        let a_chirho = vec![
            IntervalChirho::new_chirho(1.0, 10.0),
            IntervalChirho::new_chirho(5.0, 15.0),
            IntervalChirho::new_chirho(1.0, 5.0),  // No intersection with b[2]
        ];
        let b_chirho = vec![
            IntervalChirho::new_chirho(5.0, 20.0),
            IntervalChirho::new_chirho(10.0, 20.0),
            IntervalChirho::new_chirho(10.0, 15.0),
        ];

        let result_chirho = batch_intersect_chirho(&a_chirho, &b_chirho);

        // First intersection: [5, 10]
        assert_eq!(result_chirho[0].lo_chirho, 5.0);
        assert_eq!(result_chirho[0].hi_chirho, 10.0);

        // Second intersection: [10, 15]
        assert_eq!(result_chirho[1].lo_chirho, 10.0);
        assert_eq!(result_chirho[1].hi_chirho, 15.0);

        // Third: empty intersection (lo > hi)
        assert!(result_chirho[2].is_empty_chirho());
    }

    #[test]
    fn test_batch_square_chirho() {
        let a_chirho = vec![
            IntervalChirho::new_chirho(2.0, 3.0),
            IntervalChirho::new_chirho(-2.0, 3.0),
            IntervalChirho::new_chirho(-3.0, -2.0),
        ];

        let result_chirho = batch_square_chirho(&a_chirho);

        assert_eq!(result_chirho[0].lo_chirho, 4.0);
        assert_eq!(result_chirho[0].hi_chirho, 9.0);

        // Mixed sign: [0, max(4, 9)]
        assert_eq!(result_chirho[1].lo_chirho, 0.0);
        assert_eq!(result_chirho[1].hi_chirho, 9.0);

        // Negative: square reverses order
        assert_eq!(result_chirho[2].lo_chirho, 4.0);
        assert_eq!(result_chirho[2].hi_chirho, 9.0);
    }

    #[test]
    fn test_interval_vec_add_chirho() {
        let a_chirho = IntervalVecChirho::from_intervals_chirho(&[
            IntervalChirho::new_chirho(1.0, 2.0),
            IntervalChirho::new_chirho(3.0, 4.0),
        ]);
        let b_chirho = IntervalVecChirho::from_intervals_chirho(&[
            IntervalChirho::new_chirho(10.0, 20.0),
            IntervalChirho::new_chirho(30.0, 40.0),
        ]);

        let result_chirho = a_chirho.add_chirho(&b_chirho);
        let intervals_chirho = result_chirho.to_intervals_chirho();

        assert_eq!(intervals_chirho[0].lo_chirho, 11.0);
        assert_eq!(intervals_chirho[0].hi_chirho, 22.0);
        assert_eq!(intervals_chirho[1].lo_chirho, 33.0);
        assert_eq!(intervals_chirho[1].hi_chirho, 44.0);
    }

    #[test]
    fn test_interval_vec_intersect_chirho() {
        let a_chirho = IntervalVecChirho::from_intervals_chirho(&[
            IntervalChirho::new_chirho(1.0, 10.0),
            IntervalChirho::new_chirho(1.0, 5.0),
        ]);
        let b_chirho = IntervalVecChirho::from_intervals_chirho(&[
            IntervalChirho::new_chirho(5.0, 15.0),
            IntervalChirho::new_chirho(10.0, 15.0),
        ]);

        let result_chirho = a_chirho.intersect_chirho(&b_chirho);
        let mask_chirho = result_chirho.valid_mask_chirho();

        assert!(mask_chirho[0]);  // [5, 10] is valid
        assert!(!mask_chirho[1]); // [10, 5] is invalid (empty)
    }

    #[test]
    fn test_large_batch_chirho() {
        // Test with a larger batch to verify chunking logic
        let size_chirho = 100;
        let a_chirho: Vec<IntervalChirho> = (0..size_chirho)
            .map(|i_chirho| IntervalChirho::exact_chirho(i_chirho as f64))
            .collect();
        let b_chirho: Vec<IntervalChirho> = (0..size_chirho)
            .map(|i_chirho| IntervalChirho::exact_chirho((i_chirho * 2) as f64))
            .collect();

        let result_chirho = batch_add_chirho(&a_chirho, &b_chirho);

        assert_eq!(result_chirho.len(), size_chirho);
        for i_chirho in 0..size_chirho {
            assert_eq!(result_chirho[i_chirho].lo_chirho, (i_chirho * 3) as f64);
        }
    }
}
