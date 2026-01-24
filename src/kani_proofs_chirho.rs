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
//! - **Interval intersection commutativity**: a ∩ b = b ∩ a
//! - **Lattice laws**: Merge is commutative, identity, idempotent, monotonic
//!
//! # Limitations
//!
//! Some proofs are disabled due to Kani's current limitations:
//!
//! - **Soundness proofs** (add, mul, square): Require reasoning about arbitrary
//!   values within intervals, which causes unwinding explosion in std library.
//! - **Finite domain proofs**: BTreeSet operations involve complex allocations
//!   and sorting that exceed Kani's verification capacity.
//! - **No-panic proofs**: Hit unwinding limits in floating-point arithmetic code.
//!
//! These properties are instead verified via property-based testing in
//! `tests/property_tests_chirho.rs`.

#![cfg(kani)]

use crate::interval_chirho::IntervalChirho;
use crate::NumericInfoChirho;

// ============================================================================
// INTERVAL ARITHMETIC PROOFS
// ============================================================================

/// Proof: Interval intersection is commutative.
///
/// This is the core lattice operation for intervals.
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
        "Intersection must be commutative: a ∩ b = b ∩ a",
    );
}

/// Proof: Interval intersection is idempotent.
#[kani::proof]
#[kani::unwind(2)]
fn proof_interval_intersect_idempotent_chirho() {
    let lo_chirho: f64 = kani::any();
    let hi_chirho: f64 = kani::any();

    kani::assume(lo_chirho.is_finite() && hi_chirho.is_finite());
    kani::assume(lo_chirho <= hi_chirho);

    let a_chirho = IntervalChirho::new_chirho(lo_chirho, hi_chirho);
    let result_chirho = a_chirho.intersect_chirho(&a_chirho);

    kani::assert(
        result_chirho.lo_chirho == lo_chirho && result_chirho.hi_chirho == hi_chirho,
        "Intersection must be idempotent: a ∩ a = a",
    );
}

/// Proof: Intersection preserves interval validity.
#[kani::proof]
#[kani::unwind(2)]
fn proof_interval_intersect_valid_chirho() {
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

    let result_chirho = a_chirho.intersect_chirho(&b_chirho);

    // Result is either empty (lo > hi) or valid (lo <= hi)
    // Both are acceptable states
    kani::assert(
        result_chirho.lo_chirho.is_finite() && result_chirho.hi_chirho.is_finite(),
        "Intersection must produce finite bounds",
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

    // Compare using direct interval access (avoiding format! which pulls in bignum)
    match (
        ab_chirho.as_interval_chirho(),
        ba_chirho.as_interval_chirho(),
    ) {
        (Some(ab_iv_chirho), Some(ba_iv_chirho)) => {
            kani::assert(
                ab_iv_chirho.lo_chirho == ba_iv_chirho.lo_chirho
                    && ab_iv_chirho.hi_chirho == ba_iv_chirho.hi_chirho,
                "Merge must be commutative: a ⊔ b = b ⊔ a",
            );
        }
        (None, None) => {
            // Both are contradictions or nothing - that's fine
        }
        _ => {
            kani::assert(false, "Merge commutativity failed: different result types");
        }
    }
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

    // Compare using direct interval access
    let result_iv_chirho = result_chirho.as_interval_chirho();
    kani::assert(
        result_iv_chirho.is_some(),
        "Merge with nothing should preserve interval",
    );
    if let Some(iv_chirho) = result_iv_chirho {
        kani::assert(
            iv_chirho.lo_chirho == lo_chirho && iv_chirho.hi_chirho == hi_chirho,
            "Nothing must be identity: a ⊔ ⊥ = a",
        );
    }
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

    // Compare using direct interval access
    let result_iv_chirho = result_chirho.as_interval_chirho();
    kani::assert(
        result_iv_chirho.is_some(),
        "Merge with self should preserve interval",
    );
    if let Some(iv_chirho) = result_iv_chirho {
        kani::assert(
            iv_chirho.lo_chirho == lo_chirho && iv_chirho.hi_chirho == hi_chirho,
            "Merge must be idempotent: a ⊔ a = a",
        );
    }
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
            "Merged lower bound must be >= both inputs",
        );
        kani::assert(
            merged_iv_chirho.hi_chirho <= a_hi_chirho && merged_iv_chirho.hi_chirho <= b_hi_chirho,
            "Merged upper bound must be <= both inputs",
        );
    }
    // If merged is contradiction, that's fine (intervals didn't overlap)
}

/// Proof: Exact value is contained in its interval.
#[kani::proof]
#[kani::unwind(2)]
fn proof_exact_contains_value_chirho() {
    let value_chirho: f64 = kani::any();
    kani::assume(value_chirho.is_finite());

    let interval_chirho = IntervalChirho::exact_chirho(value_chirho);

    kani::assert(
        interval_chirho.contains_chirho(value_chirho),
        "Exact interval must contain its value",
    );
    kani::assert(
        interval_chirho.lo_chirho == value_chirho && interval_chirho.hi_chirho == value_chirho,
        "Exact interval has lo == hi == value",
    );
}

/// Proof: Interval width is non-negative for valid intervals.
#[kani::proof]
#[kani::unwind(2)]
fn proof_interval_width_nonnegative_chirho() {
    let lo_chirho: f64 = kani::any();
    let hi_chirho: f64 = kani::any();

    kani::assume(lo_chirho.is_finite() && hi_chirho.is_finite());
    kani::assume(lo_chirho <= hi_chirho);

    let interval_chirho = IntervalChirho::new_chirho(lo_chirho, hi_chirho);
    let width_chirho = interval_chirho.width_chirho();

    kani::assert(width_chirho >= 0.0, "Interval width must be non-negative");
}

/// Proof: Intersection result is contained in both operands.
#[kani::proof]
#[kani::unwind(2)]
fn proof_intersection_containment_chirho() {
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
    let result_chirho = a_chirho.intersect_chirho(&b_chirho);

    // If result is non-empty, it must be contained in both
    if !result_chirho.is_empty_chirho() {
        kani::assert(
            result_chirho.lo_chirho >= a_lo_chirho && result_chirho.hi_chirho <= a_hi_chirho,
            "Intersection must be subset of first operand",
        );
        kani::assert(
            result_chirho.lo_chirho >= b_lo_chirho && result_chirho.hi_chirho <= b_hi_chirho,
            "Intersection must be subset of second operand",
        );
    }
}

/// Proof: Contradiction absorbs in merge (top absorbs).
#[kani::proof]
#[kani::unwind(2)]
fn proof_contradiction_absorbs_chirho() {
    let lo_chirho: f64 = kani::any();
    let hi_chirho: f64 = kani::any();

    kani::assume(lo_chirho.is_finite() && hi_chirho.is_finite());
    kani::assume(lo_chirho <= hi_chirho);

    let a_chirho = NumericInfoChirho::interval_chirho(lo_chirho, hi_chirho);
    let top_chirho = NumericInfoChirho::ContradictionChirho;

    let result_chirho = a_chirho.merge_chirho(&top_chirho);

    kani::assert(
        result_chirho.is_contradiction_chirho(),
        "Contradiction must absorb: a ⊔ ⊤ = ⊤",
    );
}

// ============================================================================
// DISABLED PROOFS (documented limitations)
// ============================================================================
//
// The following proofs are disabled due to Kani's limitations with:
// - Complex standard library code (sorting, bignum formatting)
// - Arbitrary value reasoning within bounded intervals
// - BTreeSet allocations and operations
//
// These properties are verified via property-based testing instead.
//
// ## Soundness Proofs (disabled - unwinding explosion)
//
// ```rust,ignore
// // Proof: a + b ∈ [a] + [b] for all a ∈ [a], b ∈ [b]
// fn proof_interval_add_sound_chirho() { ... }
// fn proof_interval_mul_sound_chirho() { ... }
// fn proof_interval_square_sound_chirho() { ... }
// ```
//
// ## Finite Domain Proofs (disabled - BTreeSet complexity)
//
// ```rust,ignore
// // Proof: join is commutative, bottom is identity, top absorbs
// fn proof_finite_domain_join_commutative_chirho() { ... }
// fn proof_finite_domain_bottom_identity_chirho() { ... }
// fn proof_finite_domain_top_absorbs_chirho() { ... }
// ```
//
// ## No-Panic Proofs (disabled - unwinding limits)
//
// ```rust,ignore
// // Proof: operations don't panic on valid inputs
// fn proof_interval_no_panic_chirho() { ... }
// fn proof_interval_div_no_panic_chirho() { ... }
// ```
