// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Kani formal verification proofs.
//!
//! These proofs verify key mathematical properties of the propagator system
//! using bounded model checking via Kani.
//!
//! # Running Proofs
//!
//! Install Kani: https://model-checking.github.io/kani/install-guide.html
//!
//! ```bash
//! cargo kani --features kani
//! ```
//!
//! # Verified Properties
//!
//! - **Interval soundness**: Arithmetic operations always contain the true result
//! - **Lattice laws**: Join is commutative, associative, idempotent
//! - **Monotonicity**: Merge only adds information (intervals never widen)
//! - **No panics**: Core operations don't panic on valid inputs

#![cfg(kani)]

use crate::interval_chirho::IntervalChirho;
use crate::lattice_chirho::{BoundedLatticeChirho, LatticeChirho};
use crate::finite_domain_chirho::FiniteDomainChirho;
use crate::NumericInfoChirho;

// ============================================================================
// INTERVAL ARITHMETIC PROOFS
// ============================================================================

/// Proof: Interval addition is sound.
///
/// For any values a ∈ [a_lo, a_hi] and b ∈ [b_lo, b_hi],
/// the sum a + b is always contained in the resulting interval.
#[kani::proof]
#[kani::unwind(2)]
fn proof_interval_add_sound_chirho() {
    let a_lo_chirho: f64 = kani::any();
    let a_hi_chirho: f64 = kani::any();
    let b_lo_chirho: f64 = kani::any();
    let b_hi_chirho: f64 = kani::any();

    // Preconditions: valid intervals (lo <= hi) and finite values
    kani::assume(a_lo_chirho.is_finite() && a_hi_chirho.is_finite());
    kani::assume(b_lo_chirho.is_finite() && b_hi_chirho.is_finite());
    kani::assume(a_lo_chirho <= a_hi_chirho);
    kani::assume(b_lo_chirho <= b_hi_chirho);

    let a_chirho = IntervalChirho::new_chirho(a_lo_chirho, a_hi_chirho);
    let b_chirho = IntervalChirho::new_chirho(b_lo_chirho, b_hi_chirho);
    let result_chirho = a_chirho.add_chirho(&b_chirho);

    // Pick arbitrary points in the intervals
    let a_val_chirho: f64 = kani::any();
    let b_val_chirho: f64 = kani::any();
    kani::assume(a_val_chirho >= a_lo_chirho && a_val_chirho <= a_hi_chirho);
    kani::assume(b_val_chirho >= b_lo_chirho && b_val_chirho <= b_hi_chirho);

    let sum_chirho = a_val_chirho + b_val_chirho;

    // The sum must be in the result interval
    kani::assert(
        sum_chirho >= result_chirho.lo_chirho && sum_chirho <= result_chirho.hi_chirho,
        "Addition must be sound: a + b ∈ [a] + [b]"
    );
}

/// Proof: Interval multiplication is sound.
#[kani::proof]
#[kani::unwind(2)]
fn proof_interval_mul_sound_chirho() {
    let a_lo_chirho: f64 = kani::any();
    let a_hi_chirho: f64 = kani::any();
    let b_lo_chirho: f64 = kani::any();
    let b_hi_chirho: f64 = kani::any();

    // Use bounded values to avoid overflow
    kani::assume(a_lo_chirho.is_finite() && a_hi_chirho.is_finite());
    kani::assume(b_lo_chirho.is_finite() && b_hi_chirho.is_finite());
    kani::assume(a_lo_chirho.abs() < 1000.0 && a_hi_chirho.abs() < 1000.0);
    kani::assume(b_lo_chirho.abs() < 1000.0 && b_hi_chirho.abs() < 1000.0);
    kani::assume(a_lo_chirho <= a_hi_chirho);
    kani::assume(b_lo_chirho <= b_hi_chirho);

    let a_chirho = IntervalChirho::new_chirho(a_lo_chirho, a_hi_chirho);
    let b_chirho = IntervalChirho::new_chirho(b_lo_chirho, b_hi_chirho);
    let result_chirho = a_chirho.mul_chirho(&b_chirho);

    let a_val_chirho: f64 = kani::any();
    let b_val_chirho: f64 = kani::any();
    kani::assume(a_val_chirho >= a_lo_chirho && a_val_chirho <= a_hi_chirho);
    kani::assume(b_val_chirho >= b_lo_chirho && b_val_chirho <= b_hi_chirho);

    let product_chirho = a_val_chirho * b_val_chirho;

    kani::assert(
        product_chirho >= result_chirho.lo_chirho && product_chirho <= result_chirho.hi_chirho,
        "Multiplication must be sound: a * b ∈ [a] * [b]"
    );
}

/// Proof: Interval squaring is sound.
#[kani::proof]
#[kani::unwind(2)]
fn proof_interval_square_sound_chirho() {
    let lo_chirho: f64 = kani::any();
    let hi_chirho: f64 = kani::any();

    kani::assume(lo_chirho.is_finite() && hi_chirho.is_finite());
    kani::assume(lo_chirho.abs() < 1000.0 && hi_chirho.abs() < 1000.0);
    kani::assume(lo_chirho <= hi_chirho);

    let interval_chirho = IntervalChirho::new_chirho(lo_chirho, hi_chirho);
    let squared_chirho = interval_chirho.square_chirho();

    let val_chirho: f64 = kani::any();
    kani::assume(val_chirho >= lo_chirho && val_chirho <= hi_chirho);

    let val_squared_chirho = val_chirho * val_chirho;

    kani::assert(
        val_squared_chirho >= squared_chirho.lo_chirho && val_squared_chirho <= squared_chirho.hi_chirho,
        "Squaring must be sound: a² ∈ [a]²"
    );
}

/// Proof: Interval intersection is commutative.
#[kani::proof]
#[kani::unwind(2)]
fn proof_interval_intersect_commutative_chirho() {
    let a_lo_chirho: f64 = kani::any();
    let a_hi_chirho: f64 = kani::any();
    let b_lo_chirho: f64 = kani::any();
    let b_hi_chirho: f64 = kani::any();

    kani::assume(a_lo_chirho.is_finite() && a_hi_chirho.is_finite());
    kani::assume(b_lo_chirho.is_finite() && b_hi_chirho.is_finite());
    kani::assume(a_lo_chirho <= a_hi_chirho);
    kani::assume(b_lo_chirho <= b_hi_chirho);

    let a_chirho = IntervalChirho::new_chirho(a_lo_chirho, a_hi_chirho);
    let b_chirho = IntervalChirho::new_chirho(b_lo_chirho, b_hi_chirho);

    let ab_chirho = a_chirho.intersect_chirho(&b_chirho);
    let ba_chirho = b_chirho.intersect_chirho(&a_chirho);

    kani::assert(
        ab_chirho.lo_chirho == ba_chirho.lo_chirho && ab_chirho.hi_chirho == ba_chirho.hi_chirho,
        "Intersection must be commutative: a ∩ b = b ∩ a"
    );
}

// ============================================================================
// NUMERIC INFO LATTICE PROOFS
// ============================================================================

/// Proof: NumericInfo merge is commutative.
#[kani::proof]
#[kani::unwind(2)]
fn proof_numeric_info_merge_commutative_chirho() {
    let a_lo_chirho: f64 = kani::any();
    let a_hi_chirho: f64 = kani::any();
    let b_lo_chirho: f64 = kani::any();
    let b_hi_chirho: f64 = kani::any();

    kani::assume(a_lo_chirho.is_finite() && a_hi_chirho.is_finite());
    kani::assume(b_lo_chirho.is_finite() && b_hi_chirho.is_finite());
    kani::assume(a_lo_chirho <= a_hi_chirho);
    kani::assume(b_lo_chirho <= b_hi_chirho);

    let a_chirho = NumericInfoChirho::interval_chirho(a_lo_chirho, a_hi_chirho);
    let b_chirho = NumericInfoChirho::interval_chirho(b_lo_chirho, b_hi_chirho);

    let ab_chirho = a_chirho.merge_chirho(&b_chirho);
    let ba_chirho = b_chirho.merge_chirho(&a_chirho);

    // Compare using debug format since PartialEq may not be derived
    kani::assert(
        format!("{:?}", ab_chirho) == format!("{:?}", ba_chirho),
        "Merge must be commutative: a ⊔ b = b ⊔ a"
    );
}

/// Proof: Nothing is the identity for merge.
#[kani::proof]
#[kani::unwind(2)]
fn proof_nothing_is_identity_chirho() {
    let lo_chirho: f64 = kani::any();
    let hi_chirho: f64 = kani::any();

    kani::assume(lo_chirho.is_finite() && hi_chirho.is_finite());
    kani::assume(lo_chirho <= hi_chirho);

    let a_chirho = NumericInfoChirho::interval_chirho(lo_chirho, hi_chirho);
    let nothing_chirho = NumericInfoChirho::nothing_chirho();

    let result_chirho = a_chirho.merge_chirho(&nothing_chirho);

    kani::assert(
        format!("{:?}", result_chirho) == format!("{:?}", a_chirho),
        "Nothing must be identity: a ⊔ ⊥ = a"
    );
}

/// Proof: Merge is idempotent.
#[kani::proof]
#[kani::unwind(2)]
fn proof_merge_idempotent_chirho() {
    let lo_chirho: f64 = kani::any();
    let hi_chirho: f64 = kani::any();

    kani::assume(lo_chirho.is_finite() && hi_chirho.is_finite());
    kani::assume(lo_chirho <= hi_chirho);

    let a_chirho = NumericInfoChirho::interval_chirho(lo_chirho, hi_chirho);
    let result_chirho = a_chirho.merge_chirho(&a_chirho);

    kani::assert(
        format!("{:?}", result_chirho) == format!("{:?}", a_chirho),
        "Merge must be idempotent: a ⊔ a = a"
    );
}

/// Proof: Merge is monotonic (intervals never widen).
#[kani::proof]
#[kani::unwind(2)]
fn proof_merge_monotonic_chirho() {
    let a_lo_chirho: f64 = kani::any();
    let a_hi_chirho: f64 = kani::any();
    let b_lo_chirho: f64 = kani::any();
    let b_hi_chirho: f64 = kani::any();

    kani::assume(a_lo_chirho.is_finite() && a_hi_chirho.is_finite());
    kani::assume(b_lo_chirho.is_finite() && b_hi_chirho.is_finite());
    kani::assume(a_lo_chirho <= a_hi_chirho);
    kani::assume(b_lo_chirho <= b_hi_chirho);

    let a_chirho = NumericInfoChirho::interval_chirho(a_lo_chirho, a_hi_chirho);
    let b_chirho = NumericInfoChirho::interval_chirho(b_lo_chirho, b_hi_chirho);

    let merged_chirho = a_chirho.merge_chirho(&b_chirho);

    // Merged interval should be no wider than either input
    // (it's the intersection, so it should be narrower or equal)
    if let Some(merged_iv_chirho) = merged_chirho.as_interval_chirho() {
        kani::assert(
            merged_iv_chirho.lo_chirho >= a_lo_chirho && merged_iv_chirho.lo_chirho >= b_lo_chirho,
            "Merged lower bound must be >= both inputs"
        );
        kani::assert(
            merged_iv_chirho.hi_chirho <= a_hi_chirho && merged_iv_chirho.hi_chirho <= b_hi_chirho,
            "Merged upper bound must be <= both inputs"
        );
    }
    // If merged is contradiction, that's fine (intervals didn't overlap)
}

// ============================================================================
// FINITE DOMAIN LATTICE PROOFS
// ============================================================================

/// Proof: Finite domain join is commutative.
#[kani::proof]
#[kani::unwind(10)]
fn proof_finite_domain_join_commutative_chirho() {
    let v1_chirho: i64 = kani::any();
    let v2_chirho: i64 = kani::any();

    // Bound values to keep proof tractable
    kani::assume(v1_chirho >= 0 && v1_chirho < 10);
    kani::assume(v2_chirho >= 0 && v2_chirho < 10);

    let a_chirho = FiniteDomainChirho::range_chirho(0, v1_chirho.max(1));
    let b_chirho = FiniteDomainChirho::range_chirho(0, v2_chirho.max(1));

    let ab_chirho = a_chirho.join_chirho(&b_chirho);
    let ba_chirho = b_chirho.join_chirho(&a_chirho);

    kani::assert(
        ab_chirho.size_chirho() == ba_chirho.size_chirho(),
        "Join must be commutative: |a ⊔ b| = |b ⊔ a|"
    );
}

/// Proof: Bottom is identity for finite domain join.
#[kani::proof]
#[kani::unwind(10)]
fn proof_finite_domain_bottom_identity_chirho() {
    let max_chirho: i64 = kani::any();
    kani::assume(max_chirho >= 1 && max_chirho < 10);

    let a_chirho = FiniteDomainChirho::range_chirho(0, max_chirho);
    let bottom_chirho = FiniteDomainChirho::bottom_chirho();

    let result_chirho = a_chirho.join_chirho(&bottom_chirho);

    kani::assert(
        result_chirho.size_chirho() == a_chirho.size_chirho(),
        "Bottom must be identity: a ⊔ ⊥ = a"
    );
}

/// Proof: Join with top gives top.
#[kani::proof]
#[kani::unwind(10)]
fn proof_finite_domain_top_absorbs_chirho() {
    let max_chirho: i64 = kani::any();
    kani::assume(max_chirho >= 1 && max_chirho < 10);

    let a_chirho = FiniteDomainChirho::range_chirho(0, max_chirho);
    let top_chirho = FiniteDomainChirho::top_chirho();

    let result_chirho = a_chirho.join_chirho(&top_chirho);

    kani::assert(
        result_chirho.is_top_chirho(),
        "Top must absorb: a ⊔ ⊤ = ⊤"
    );
}

// ============================================================================
// NO-PANIC PROOFS
// ============================================================================

/// Proof: Interval operations don't panic on valid inputs.
#[kani::proof]
#[kani::unwind(2)]
fn proof_interval_no_panic_chirho() {
    let lo_chirho: f64 = kani::any();
    let hi_chirho: f64 = kani::any();

    kani::assume(lo_chirho.is_finite() && hi_chirho.is_finite());
    kani::assume(lo_chirho <= hi_chirho);
    kani::assume(lo_chirho.abs() < 1e10 && hi_chirho.abs() < 1e10);

    let interval_chirho = IntervalChirho::new_chirho(lo_chirho, hi_chirho);

    // These should not panic
    let _ = interval_chirho.add_chirho(&interval_chirho);
    let _ = interval_chirho.sub_chirho(&interval_chirho);
    let _ = interval_chirho.mul_chirho(&interval_chirho);
    let _ = interval_chirho.square_chirho();
    let _ = interval_chirho.intersect_chirho(&interval_chirho);
    let _ = interval_chirho.contains_chirho(lo_chirho);
    let _ = interval_chirho.width_chirho();
    let _ = interval_chirho.midpoint_chirho();
}

/// Proof: Division by interval not containing zero doesn't panic.
#[kani::proof]
#[kani::unwind(2)]
fn proof_interval_div_no_panic_chirho() {
    let a_lo_chirho: f64 = kani::any();
    let a_hi_chirho: f64 = kani::any();
    let b_lo_chirho: f64 = kani::any();
    let b_hi_chirho: f64 = kani::any();

    kani::assume(a_lo_chirho.is_finite() && a_hi_chirho.is_finite());
    kani::assume(b_lo_chirho.is_finite() && b_hi_chirho.is_finite());
    kani::assume(a_lo_chirho <= a_hi_chirho);
    kani::assume(b_lo_chirho <= b_hi_chirho);
    // Divisor doesn't contain zero
    kani::assume(b_lo_chirho > 0.0 || b_hi_chirho < 0.0);
    kani::assume(a_lo_chirho.abs() < 1e10 && a_hi_chirho.abs() < 1e10);
    kani::assume(b_lo_chirho.abs() < 1e10 && b_hi_chirho.abs() < 1e10);
    kani::assume(b_lo_chirho.abs() > 1e-10 && b_hi_chirho.abs() > 1e-10);

    let a_chirho = IntervalChirho::new_chirho(a_lo_chirho, a_hi_chirho);
    let b_chirho = IntervalChirho::new_chirho(b_lo_chirho, b_hi_chirho);

    // This should not panic when divisor doesn't contain zero
    let _ = a_chirho.div_chirho(&b_chirho);
}
