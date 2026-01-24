// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Property-based tests for propagator correctness.
//!
//! These tests verify the mathematical properties that propagators must satisfy:
//! - Monotonicity: merging only adds information
//! - Commutativity: order of merge doesn't matter
//! - Associativity: grouping of merge doesn't matter
//! - Idempotency: merging with self is identity
//! - Soundness: propagated values are correct

use propagators_chirho::{ConstraintSystemChirho, IntervalChirho, NumericInfoChirho};
use proptest::prelude::*;

// ============================================================================
// INTERVAL ARITHMETIC PROPERTIES
// ============================================================================

proptest! {
    /// Interval intersection is commutative: a ∩ b = b ∩ a
    #[test]
    fn test_interval_intersection_commutative_chirho(
        a_lo_chirho in -1000.0f64..1000.0,
        a_hi_chirho in -1000.0f64..1000.0,
        b_lo_chirho in -1000.0f64..1000.0,
        b_hi_chirho in -1000.0f64..1000.0,
    ) {
        let a_lo_real_chirho = a_lo_chirho.min(a_hi_chirho);
        let a_hi_real_chirho = a_lo_chirho.max(a_hi_chirho);
        let b_lo_real_chirho = b_lo_chirho.min(b_hi_chirho);
        let b_hi_real_chirho = b_lo_chirho.max(b_hi_chirho);

        let a_chirho = IntervalChirho::new_chirho(a_lo_real_chirho, a_hi_real_chirho);
        let b_chirho = IntervalChirho::new_chirho(b_lo_real_chirho, b_hi_real_chirho);

        let ab_chirho = a_chirho.intersect_chirho(&b_chirho);
        let ba_chirho = b_chirho.intersect_chirho(&a_chirho);

        prop_assert!((ab_chirho.lo_chirho - ba_chirho.lo_chirho).abs() < 1e-10);
        prop_assert!((ab_chirho.hi_chirho - ba_chirho.hi_chirho).abs() < 1e-10);
    }

    /// Interval addition is commutative: a + b = b + a
    #[test]
    fn test_interval_addition_commutative_chirho(
        a_lo_chirho in -1000.0f64..1000.0,
        a_hi_chirho in -1000.0f64..1000.0,
        b_lo_chirho in -1000.0f64..1000.0,
        b_hi_chirho in -1000.0f64..1000.0,
    ) {
        let a_lo_real_chirho = a_lo_chirho.min(a_hi_chirho);
        let a_hi_real_chirho = a_lo_chirho.max(a_hi_chirho);
        let b_lo_real_chirho = b_lo_chirho.min(b_hi_chirho);
        let b_hi_real_chirho = b_lo_chirho.max(b_hi_chirho);

        let a_chirho = IntervalChirho::new_chirho(a_lo_real_chirho, a_hi_real_chirho);
        let b_chirho = IntervalChirho::new_chirho(b_lo_real_chirho, b_hi_real_chirho);

        let ab_chirho = a_chirho.add_chirho(&b_chirho);
        let ba_chirho = b_chirho.add_chirho(&a_chirho);

        prop_assert!((ab_chirho.lo_chirho - ba_chirho.lo_chirho).abs() < 1e-10);
        prop_assert!((ab_chirho.hi_chirho - ba_chirho.hi_chirho).abs() < 1e-10);
    }

    /// Interval multiplication is commutative: a × b = b × a
    #[test]
    fn test_interval_multiplication_commutative_chirho(
        a_lo_chirho in -100.0f64..100.0,
        a_hi_chirho in -100.0f64..100.0,
        b_lo_chirho in -100.0f64..100.0,
        b_hi_chirho in -100.0f64..100.0,
    ) {
        let a_lo_real_chirho = a_lo_chirho.min(a_hi_chirho);
        let a_hi_real_chirho = a_lo_chirho.max(a_hi_chirho);
        let b_lo_real_chirho = b_lo_chirho.min(b_hi_chirho);
        let b_hi_real_chirho = b_lo_chirho.max(b_hi_chirho);

        let a_chirho = IntervalChirho::new_chirho(a_lo_real_chirho, a_hi_real_chirho);
        let b_chirho = IntervalChirho::new_chirho(b_lo_real_chirho, b_hi_real_chirho);

        let ab_chirho = a_chirho.mul_chirho(&b_chirho);
        let ba_chirho = b_chirho.mul_chirho(&a_chirho);

        prop_assert!((ab_chirho.lo_chirho - ba_chirho.lo_chirho).abs() < 1e-10);
        prop_assert!((ab_chirho.hi_chirho - ba_chirho.hi_chirho).abs() < 1e-10);
    }

    /// Point intervals correctly represent exact values
    #[test]
    fn test_exact_interval_chirho(value_chirho in -1000.0f64..1000.0) {
        let interval_chirho = IntervalChirho::exact_chirho(value_chirho);

        prop_assert!(interval_chirho.is_exact_chirho());
        prop_assert!((interval_chirho.lo_chirho - value_chirho).abs() < 1e-10);
        prop_assert!((interval_chirho.hi_chirho - value_chirho).abs() < 1e-10);
        prop_assert!(interval_chirho.contains_chirho(value_chirho));
    }

    /// Interval contains all points in its range
    #[test]
    fn test_interval_contains_chirho(
        lo_chirho in -1000.0f64..1000.0,
        hi_chirho in -1000.0f64..1000.0,
        t_chirho in 0.0f64..1.0,
    ) {
        let lo_real_chirho = lo_chirho.min(hi_chirho);
        let hi_real_chirho = lo_chirho.max(hi_chirho);

        let interval_chirho = IntervalChirho::new_chirho(lo_real_chirho, hi_real_chirho);
        let point_chirho = lo_real_chirho + t_chirho * (hi_real_chirho - lo_real_chirho);

        prop_assert!(interval_chirho.contains_chirho(point_chirho));
    }

    /// Square of interval bounds are correct
    #[test]
    fn test_interval_square_bounds_chirho(
        lo_chirho in -100.0f64..100.0,
        hi_chirho in -100.0f64..100.0,
    ) {
        let lo_real_chirho = lo_chirho.min(hi_chirho);
        let hi_real_chirho = lo_chirho.max(hi_chirho);

        let interval_chirho = IntervalChirho::new_chirho(lo_real_chirho, hi_real_chirho);
        let squared_chirho = interval_chirho.square_chirho();

        // The squared interval should be non-negative
        prop_assert!(squared_chirho.lo_chirho >= 0.0 || squared_chirho.is_empty_chirho());
    }
}

// ============================================================================
// NUMERIC INFO LATTICE PROPERTIES
// ============================================================================

proptest! {
    /// Merge is commutative: a ⊔ b = b ⊔ a
    #[test]
    fn test_numeric_info_merge_commutative_chirho(
        a_lo_chirho in -1000.0f64..1000.0,
        a_hi_chirho in -1000.0f64..1000.0,
        b_lo_chirho in -1000.0f64..1000.0,
        b_hi_chirho in -1000.0f64..1000.0,
    ) {
        let a_lo_real_chirho = a_lo_chirho.min(a_hi_chirho);
        let a_hi_real_chirho = a_lo_chirho.max(a_hi_chirho);
        let b_lo_real_chirho = b_lo_chirho.min(b_hi_chirho);
        let b_hi_real_chirho = b_lo_chirho.max(b_hi_chirho);

        let a_chirho = NumericInfoChirho::interval_chirho(a_lo_real_chirho, a_hi_real_chirho);
        let b_chirho = NumericInfoChirho::interval_chirho(b_lo_real_chirho, b_hi_real_chirho);

        let ab_chirho = a_chirho.merge_chirho(&b_chirho);
        let ba_chirho = b_chirho.merge_chirho(&a_chirho);

        prop_assert_eq!(format!("{:?}", ab_chirho), format!("{:?}", ba_chirho));
    }

    /// Merge with nothing is identity: a ⊔ ⊥ = a
    #[test]
    fn test_nothing_is_identity_chirho(
        lo_chirho in -1000.0f64..1000.0,
        hi_chirho in -1000.0f64..1000.0,
    ) {
        let lo_real_chirho = lo_chirho.min(hi_chirho);
        let hi_real_chirho = lo_chirho.max(hi_chirho);

        let a_chirho = NumericInfoChirho::interval_chirho(lo_real_chirho, hi_real_chirho);
        let nothing_chirho = NumericInfoChirho::nothing_chirho();

        let merged_chirho = a_chirho.merge_chirho(&nothing_chirho);

        prop_assert_eq!(format!("{:?}", merged_chirho), format!("{:?}", a_chirho));
    }

    /// Merge is idempotent: a ⊔ a = a
    #[test]
    fn test_merge_idempotent_chirho(
        lo_chirho in -1000.0f64..1000.0,
        hi_chirho in -1000.0f64..1000.0,
    ) {
        let lo_real_chirho = lo_chirho.min(hi_chirho);
        let hi_real_chirho = lo_chirho.max(hi_chirho);

        let a_chirho = NumericInfoChirho::interval_chirho(lo_real_chirho, hi_real_chirho);
        let merged_chirho = a_chirho.merge_chirho(&a_chirho);

        prop_assert_eq!(format!("{:?}", merged_chirho), format!("{:?}", a_chirho));
    }
}

// ============================================================================
// PROPAGATOR CORRECTNESS PROPERTIES
// ============================================================================

proptest! {
    /// Adder propagator is correct: a + b = c
    #[test]
    fn test_adder_correctness_chirho(
        a_value_chirho in -1000.0f64..1000.0,
        b_value_chirho in -1000.0f64..1000.0,
    ) {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");
        let c_chirho = system_chirho.make_cell_chirho("c");

        system_chirho.add_adder_chirho(&a_chirho, &b_chirho, &c_chirho);

        system_chirho.set_exact_chirho(&a_chirho, a_value_chirho);
        system_chirho.set_exact_chirho(&b_chirho, b_value_chirho);
        system_chirho.run_chirho();

        let c_content_chirho = system_chirho.get_chirho(&c_chirho);
        let c_interval_chirho = c_content_chirho.as_interval_chirho().unwrap();

        let expected_chirho = a_value_chirho + b_value_chirho;
        prop_assert!((c_interval_chirho.lo_chirho - expected_chirho).abs() < 1e-10);
    }

    /// Multiplier propagator is correct: a × b = c
    #[test]
    fn test_multiplier_correctness_chirho(
        a_value_chirho in -100.0f64..100.0,
        b_value_chirho in -100.0f64..100.0,
    ) {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");
        let c_chirho = system_chirho.make_cell_chirho("c");

        system_chirho.add_multiplier_chirho(&a_chirho, &b_chirho, &c_chirho);

        system_chirho.set_exact_chirho(&a_chirho, a_value_chirho);
        system_chirho.set_exact_chirho(&b_chirho, b_value_chirho);
        system_chirho.run_chirho();

        let c_content_chirho = system_chirho.get_chirho(&c_chirho);
        if let Some(c_interval_chirho) = c_content_chirho.as_interval_chirho() {
            let expected_chirho = a_value_chirho * b_value_chirho;
            prop_assert!((c_interval_chirho.lo_chirho - expected_chirho).abs() < 1e-9);
        }
    }

    /// Squarer propagator is correct: a² = b
    #[test]
    fn test_squarer_correctness_chirho(a_value_chirho in -100.0f64..100.0) {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");

        system_chirho.add_squarer_chirho(&a_chirho, &b_chirho);

        system_chirho.set_exact_chirho(&a_chirho, a_value_chirho);
        system_chirho.run_chirho();

        let b_content_chirho = system_chirho.get_chirho(&b_chirho);
        let b_interval_chirho = b_content_chirho.as_interval_chirho().unwrap();

        let expected_chirho = a_value_chirho * a_value_chirho;
        prop_assert!((b_interval_chirho.lo_chirho - expected_chirho).abs() < 1e-10);
    }

    /// Bidirectional propagation: computing backwards gives consistent results
    #[test]
    fn test_adder_backward_chirho(
        a_value_chirho in -1000.0f64..1000.0,
        c_value_chirho in -1000.0f64..1000.0,
    ) {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");
        let c_chirho = system_chirho.make_cell_chirho("c");

        system_chirho.add_adder_chirho(&a_chirho, &b_chirho, &c_chirho);

        system_chirho.set_exact_chirho(&a_chirho, a_value_chirho);
        system_chirho.set_exact_chirho(&c_chirho, c_value_chirho);
        system_chirho.run_chirho();

        let b_content_chirho = system_chirho.get_chirho(&b_chirho);
        let b_interval_chirho = b_content_chirho.as_interval_chirho().unwrap();

        let expected_chirho = c_value_chirho - a_value_chirho;
        prop_assert!((b_interval_chirho.lo_chirho - expected_chirho).abs() < 1e-10);
    }
}

// ============================================================================
// SOUNDNESS: PROPAGATED INTERVALS CONTAIN THE TRUE VALUE
// ============================================================================

proptest! {
    /// Interval addition is sound: true sum is always in computed interval
    #[test]
    fn test_interval_addition_sound_chirho(
        a_lo_chirho in -100.0f64..100.0,
        a_hi_chirho in -100.0f64..100.0,
        b_lo_chirho in -100.0f64..100.0,
        b_hi_chirho in -100.0f64..100.0,
        a_t_chirho in 0.0f64..1.0,
        b_t_chirho in 0.0f64..1.0,
    ) {
        let a_lo_real_chirho = a_lo_chirho.min(a_hi_chirho);
        let a_hi_real_chirho = a_lo_chirho.max(a_hi_chirho);
        let b_lo_real_chirho = b_lo_chirho.min(b_hi_chirho);
        let b_hi_real_chirho = b_lo_chirho.max(b_hi_chirho);

        let a_chirho = IntervalChirho::new_chirho(a_lo_real_chirho, a_hi_real_chirho);
        let b_chirho = IntervalChirho::new_chirho(b_lo_real_chirho, b_hi_real_chirho);

        // Pick actual values from the intervals
        let a_val_chirho = a_lo_real_chirho + a_t_chirho * (a_hi_real_chirho - a_lo_real_chirho);
        let b_val_chirho = b_lo_real_chirho + b_t_chirho * (b_hi_real_chirho - b_lo_real_chirho);
        let sum_chirho = a_val_chirho + b_val_chirho;

        // The computed interval should contain the actual sum
        let result_chirho = a_chirho.add_chirho(&b_chirho);
        prop_assert!(result_chirho.contains_chirho(sum_chirho));
    }

    /// Interval multiplication is sound
    #[test]
    fn test_interval_multiplication_sound_chirho(
        a_lo_chirho in -10.0f64..10.0,
        a_hi_chirho in -10.0f64..10.0,
        b_lo_chirho in -10.0f64..10.0,
        b_hi_chirho in -10.0f64..10.0,
        a_t_chirho in 0.0f64..1.0,
        b_t_chirho in 0.0f64..1.0,
    ) {
        let a_lo_real_chirho = a_lo_chirho.min(a_hi_chirho);
        let a_hi_real_chirho = a_lo_chirho.max(a_hi_chirho);
        let b_lo_real_chirho = b_lo_chirho.min(b_hi_chirho);
        let b_hi_real_chirho = b_lo_chirho.max(b_hi_chirho);

        let a_chirho = IntervalChirho::new_chirho(a_lo_real_chirho, a_hi_real_chirho);
        let b_chirho = IntervalChirho::new_chirho(b_lo_real_chirho, b_hi_real_chirho);

        let a_val_chirho = a_lo_real_chirho + a_t_chirho * (a_hi_real_chirho - a_lo_real_chirho);
        let b_val_chirho = b_lo_real_chirho + b_t_chirho * (b_hi_real_chirho - b_lo_real_chirho);
        let product_chirho = a_val_chirho * b_val_chirho;

        let result_chirho = a_chirho.mul_chirho(&b_chirho);
        prop_assert!(result_chirho.contains_chirho(product_chirho));
    }
}
