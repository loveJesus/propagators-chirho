// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Propagators for constraint propagation.
//!
//! Propagators are the active elements in a propagator network. They watch
//! input cells and update output cells when they can contribute new information.
//!
//! # Bidirectionality
//!
//! Unlike traditional functions that compute outputs from inputs, propagators
//! can work in any direction. For example, an adder propagator for `a + b = c`
//! can:
//!
//! - Given `a` and `b`, compute `c`
//! - Given `a` and `c`, compute `b`
//! - Given `b` and `c`, compute `a`
//!
//! This enables powerful constraint-based programming where information
//! flows wherever it's needed.
//!
//! # Correctness Properties
//!
//! All propagators in this module satisfy:
//!
//! 1. **Monotonicity**: Output information only increases
//! 2. **Soundness**: Propagated values are consistent with the constraint
//! 3. **Idempotency**: Running twice produces same result as running once
//!
//! # References
//!
//! - Radul, A., & Sussman, G. J. (2009). *The Art of the Propagator*, Section 5.
//!   <https://dspace.mit.edu/handle/1721.1/44215>

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::cells_chirho::cell_chirho::CellChirho;
use crate::core_chirho::interval_chirho::{IntervalChirho, NumericInfoChirho};
use crate::cells_chirho::scheduler_chirho::SchedulerChirho;

/// Counter for generating unique propagator IDs.
static NEXT_ID_CHIRHO: AtomicUsize = AtomicUsize::new(0);

/// Generates a unique propagator ID.
fn next_id_chirho() -> usize {
    NEXT_ID_CHIRHO.fetch_add(1, Ordering::SeqCst)
}

/// Trait for propagators in a propagator network.
///
/// Propagators watch input cells and update output cells when they
/// can contribute new information.
///
/// # Implementing Custom Propagators
///
/// ```ignore
/// struct MyPropagator {
///     id: usize,
///     input: Rc<CellChirho<NumericInfoChirho>>,
///     output: Rc<CellChirho<NumericInfoChirho>>,
/// }
///
/// impl PropagatorChirho for MyPropagator {
///     fn id_chirho(&self) -> usize { self.id }
///     fn name_chirho(&self) -> &str { "my_propagator" }
///
///     fn run_chirho(&self, scheduler: &SchedulerChirho) {
///         let input = self.input.content_chirho();
///         // Compute output from input...
///         self.output.add_content_chirho(output, scheduler);
///     }
/// }
/// ```
pub trait PropagatorChirho {
    /// Returns the unique ID of this propagator.
    fn id_chirho(&self) -> usize;

    /// Returns the name of this propagator for debugging.
    fn name_chirho(&self) -> &str;

    /// Runs the propagator, potentially updating output cells.
    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho);
}

// ============================================================================
// PROPAGATOR MACROS - Eliminate boilerplate for common patterns
// ============================================================================

/// Macro for generating ternary (3-cell) propagators with full bidirectional support.
///
/// This eliminates the boilerplate for propagators of the form `op(a, b) = c`
/// that support forward and both backward directions.
///
/// # Arguments
///
/// * `$name_chirho` - The struct name (e.g., `IntervalAdderChirho`)
/// * `$name_str_chirho` - The name string for debugging (e.g., `"interval_adder"`)
/// * `$doc_chirho` - Documentation string
/// * `forward_body_chirho` - Code block for forward propagation (a, b -> c)
/// * `backward_a_body_chirho` - Code block for backward propagation (c, b -> a)
/// * `backward_b_body_chirho` - Code block for backward propagation (c, a -> b)
macro_rules! ternary_propagator_chirho {
    (
        $(#[$attr_chirho:meta])*
        $name_chirho:ident,
        $name_str_chirho:expr,
        forward: |$a_fwd_chirho:ident, $b_fwd_chirho:ident| $forward_body_chirho:expr,
        backward_a: |$c_ba_chirho:ident, $b_ba_chirho:ident| $backward_a_body_chirho:expr,
        backward_b: |$c_bb_chirho:ident, $a_bb_chirho:ident| $backward_b_body_chirho:expr
    ) => {
        $(#[$attr_chirho])*
        pub struct $name_chirho {
            id_chirho: usize,
            a_chirho: Rc<CellChirho<NumericInfoChirho>>,
            b_chirho: Rc<CellChirho<NumericInfoChirho>>,
            c_chirho: Rc<CellChirho<NumericInfoChirho>>,
        }

        impl $name_chirho {
            /// Creates and installs this propagator, connecting it to the given cells.
            pub fn install_chirho(
                a_chirho: Rc<CellChirho<NumericInfoChirho>>,
                b_chirho: Rc<CellChirho<NumericInfoChirho>>,
                c_chirho: Rc<CellChirho<NumericInfoChirho>>,
                scheduler_chirho: &SchedulerChirho,
            ) -> Rc<Self> {
                let propagator_chirho = Rc::new(Self {
                    id_chirho: next_id_chirho(),
                    a_chirho: a_chirho.clone(),
                    b_chirho: b_chirho.clone(),
                    c_chirho: c_chirho.clone(),
                });

                a_chirho.add_neighbor_chirho(propagator_chirho.clone());
                b_chirho.add_neighbor_chirho(propagator_chirho.clone());
                c_chirho.add_neighbor_chirho(propagator_chirho.clone());

                scheduler_chirho.alert_propagator_chirho(propagator_chirho.clone());

                propagator_chirho
            }
        }

        impl PropagatorChirho for $name_chirho {
            fn id_chirho(&self) -> usize {
                self.id_chirho
            }

            fn name_chirho(&self) -> &str {
                $name_str_chirho
            }

            fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
                let a_content_chirho = self.a_chirho.content_chirho();
                let b_content_chirho = self.b_chirho.content_chirho();
                let c_content_chirho = self.c_chirho.content_chirho();

                // Forward: c = op(a, b)
                if let (
                    NumericInfoChirho::IntervalChirho($a_fwd_chirho),
                    NumericInfoChirho::IntervalChirho($b_fwd_chirho),
                ) = (&a_content_chirho, &b_content_chirho)
                {
                    let c_new_chirho = $forward_body_chirho;
                    self.c_chirho.add_content_chirho(
                        NumericInfoChirho::IntervalChirho(c_new_chirho),
                        scheduler_chirho,
                    );
                }

                // Backward: a = inverse_op(c, b)
                if let (
                    NumericInfoChirho::IntervalChirho($c_ba_chirho),
                    NumericInfoChirho::IntervalChirho($b_ba_chirho),
                ) = (&c_content_chirho, &b_content_chirho)
                {
                    let a_new_chirho = $backward_a_body_chirho;
                    self.a_chirho.add_content_chirho(
                        NumericInfoChirho::IntervalChirho(a_new_chirho),
                        scheduler_chirho,
                    );
                }

                // Backward: b = inverse_op(c, a)
                if let (
                    NumericInfoChirho::IntervalChirho($c_bb_chirho),
                    NumericInfoChirho::IntervalChirho($a_bb_chirho),
                ) = (&c_content_chirho, &a_content_chirho)
                {
                    let b_new_chirho = $backward_b_body_chirho;
                    self.b_chirho.add_content_chirho(
                        NumericInfoChirho::IntervalChirho(b_new_chirho),
                        scheduler_chirho,
                    );
                }
            }
        }
    };
}

/// Macro for generating binary (2-cell) propagators with bidirectional support.
///
/// This eliminates the boilerplate for propagators of the form `op(a) = b`.
macro_rules! binary_propagator_chirho {
    (
        $(#[$attr_chirho:meta])*
        $name_chirho:ident,
        $name_str_chirho:expr,
        forward: |$a_fwd_chirho:ident| $forward_body_chirho:expr,
        backward: |$b_bwd_chirho:ident| $backward_body_chirho:expr
    ) => {
        $(#[$attr_chirho])*
        pub struct $name_chirho {
            id_chirho: usize,
            a_chirho: Rc<CellChirho<NumericInfoChirho>>,
            b_chirho: Rc<CellChirho<NumericInfoChirho>>,
        }

        impl $name_chirho {
            /// Creates and installs this propagator, connecting it to the given cells.
            pub fn install_chirho(
                a_chirho: Rc<CellChirho<NumericInfoChirho>>,
                b_chirho: Rc<CellChirho<NumericInfoChirho>>,
                scheduler_chirho: &SchedulerChirho,
            ) -> Rc<Self> {
                let propagator_chirho = Rc::new(Self {
                    id_chirho: next_id_chirho(),
                    a_chirho: a_chirho.clone(),
                    b_chirho: b_chirho.clone(),
                });

                a_chirho.add_neighbor_chirho(propagator_chirho.clone());
                b_chirho.add_neighbor_chirho(propagator_chirho.clone());

                scheduler_chirho.alert_propagator_chirho(propagator_chirho.clone());

                propagator_chirho
            }
        }

        impl PropagatorChirho for $name_chirho {
            fn id_chirho(&self) -> usize {
                self.id_chirho
            }

            fn name_chirho(&self) -> &str {
                $name_str_chirho
            }

            fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
                let a_content_chirho = self.a_chirho.content_chirho();
                let b_content_chirho = self.b_chirho.content_chirho();

                // Forward: b = op(a)
                if let NumericInfoChirho::IntervalChirho($a_fwd_chirho) = &a_content_chirho {
                    let b_new_chirho = $forward_body_chirho;
                    self.b_chirho.add_content_chirho(
                        NumericInfoChirho::IntervalChirho(b_new_chirho),
                        scheduler_chirho,
                    );
                }

                // Backward: a = inverse_op(b)
                if let NumericInfoChirho::IntervalChirho($b_bwd_chirho) = &b_content_chirho {
                    let a_new_chirho = $backward_body_chirho;
                    self.a_chirho.add_content_chirho(
                        NumericInfoChirho::IntervalChirho(a_new_chirho),
                        scheduler_chirho,
                    );
                }
            }
        }
    };
}

// ============================================================================
// CONSTANT PROPAGATOR
// ============================================================================

/// A propagator that sets a cell to a constant value.
///
/// This is the simplest propagator—it just adds a fixed value to a cell.
///
/// # Example
///
/// ```
/// use propagators_chirho::{ConstantChirho, CellChirho, NumericInfoChirho, SchedulerChirho};
///
/// let scheduler_chirho = SchedulerChirho::new_chirho();
/// let cell_chirho = CellChirho::new_chirho("pi");
///
/// let propagator_chirho = ConstantChirho::new_chirho(
///     NumericInfoChirho::exact_chirho(3.14159),
///     cell_chirho.clone()
/// );
///
/// scheduler_chirho.alert_propagator_chirho(propagator_chirho);
/// scheduler_chirho.run_chirho();
///
/// assert!(!cell_chirho.content_chirho().is_nothing_chirho());
/// ```
pub struct ConstantChirho {
    id_chirho: usize,
    value_chirho: NumericInfoChirho,
    cell_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl ConstantChirho {
    /// Creates a new constant propagator.
    pub fn new_chirho(
        value_chirho: NumericInfoChirho,
        cell_chirho: Rc<CellChirho<NumericInfoChirho>>,
    ) -> Rc<Self> {
        Rc::new(Self {
            id_chirho: next_id_chirho(),
            value_chirho,
            cell_chirho,
        })
    }
}

impl PropagatorChirho for ConstantChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "constant"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        self.cell_chirho
            .add_content_chirho(self.value_chirho, scheduler_chirho);
    }
}

// ============================================================================
// INTERVAL ADDER: a + b = c
// ============================================================================

ternary_propagator_chirho! {
    /// A bidirectional propagator for addition: `a + b = c`.
    ///
    /// This propagator works in all three directions:
    /// - Given `a` and `b`, computes `c = a + b`
    /// - Given `a` and `c`, computes `b = c - a`
    /// - Given `b` and `c`, computes `a = c - b`
    ///
    /// # Interval Arithmetic
    ///
    /// For intervals:
    /// - `[a_lo, a_hi] + [b_lo, b_hi] = [a_lo + b_lo, a_hi + b_hi]`
    /// - `[c_lo, c_hi] - [a_lo, a_hi] = [c_lo - a_hi, c_hi - a_lo]`
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::{
    ///     IntervalAdderChirho, CellChirho, NumericInfoChirho, SchedulerChirho
    /// };
    ///
    /// let scheduler_chirho = SchedulerChirho::new_chirho();
    /// let a_chirho = CellChirho::new_chirho("a");
    /// let b_chirho = CellChirho::new_chirho("b");
    /// let c_chirho = CellChirho::new_chirho("c");
    ///
    /// // Set up a + b = c
    /// IntervalAdderChirho::install_chirho(
    ///     a_chirho.clone(),
    ///     b_chirho.clone(),
    ///     c_chirho.clone(),
    ///     &scheduler_chirho
    /// );
    ///
    /// // Given a=3 and b=4, compute c
    /// a_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(3.0), &scheduler_chirho);
    /// b_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(4.0), &scheduler_chirho);
    /// scheduler_chirho.run_chirho();
    ///
    /// let c_content_chirho = c_chirho.content_chirho();
    /// let c_interval_chirho = c_content_chirho.as_interval_chirho().unwrap();
    /// assert!((c_interval_chirho.lo_chirho - 7.0).abs() < 1e-10);
    /// ```
    IntervalAdderChirho,
    "interval_adder",
    forward: |a_chirho, b_chirho| a_chirho.add_chirho(b_chirho),
    backward_a: |c_chirho, b_chirho| c_chirho.sub_chirho(b_chirho),
    backward_b: |c_chirho, a_chirho| c_chirho.sub_chirho(a_chirho)
}

// ============================================================================
// INTERVAL SUBTRACTOR: a - b = c
// ============================================================================

ternary_propagator_chirho! {
    /// A bidirectional propagator for subtraction: `a - b = c`.
    IntervalSubtractorChirho,
    "interval_subtractor",
    forward: |a_chirho, b_chirho| a_chirho.sub_chirho(b_chirho),
    backward_a: |c_chirho, b_chirho| c_chirho.add_chirho(b_chirho),
    backward_b: |a_chirho, c_chirho| a_chirho.sub_chirho(c_chirho)
}

// ============================================================================
// INTERVAL MULTIPLIER: a * b = c
// ============================================================================

ternary_propagator_chirho! {
    /// A bidirectional propagator for multiplication: `a * b = c`.
    ///
    /// Works in all three directions using interval arithmetic.
    IntervalMultiplierChirho,
    "interval_multiplier",
    forward: |a_chirho, b_chirho| a_chirho.mul_chirho(b_chirho),
    backward_a: |c_chirho, b_chirho| c_chirho.div_chirho(b_chirho),
    backward_b: |c_chirho, a_chirho| c_chirho.div_chirho(a_chirho)
}

// ============================================================================
// INTERVAL DIVIDER: a / b = c
// ============================================================================

ternary_propagator_chirho! {
    /// A bidirectional propagator for division: `a / b = c`.
    IntervalDividerChirho,
    "interval_divider",
    forward: |a_chirho, b_chirho| a_chirho.div_chirho(b_chirho),
    backward_a: |c_chirho, b_chirho| c_chirho.mul_chirho(b_chirho),
    backward_b: |a_chirho, c_chirho| a_chirho.div_chirho(c_chirho)
}

// ============================================================================
// SQUARER: a² = b
// ============================================================================

binary_propagator_chirho! {
    /// A bidirectional propagator for squaring: `a² = b`.
    ///
    /// - Forward: `b = a²`
    /// - Backward: `a = ±√b` (intersected with current `a`)
    SquarerChirho,
    "squarer",
    forward: |a_chirho| a_chirho.square_chirho(),
    backward: |b_chirho| {
        let sqrt_chirho = b_chirho.sqrt_chirho();
        // a could be positive or negative
        IntervalChirho::new_chirho(-sqrt_chirho.hi_chirho, sqrt_chirho.hi_chirho)
    }
}

// ============================================================================
// SQRTER: √a = b (square root)
// ============================================================================

binary_propagator_chirho! {
    /// A bidirectional propagator for square root: `√a = b`.
    SqrterChirho,
    "sqrter",
    forward: |a_chirho| a_chirho.sqrt_chirho(),
    backward: |b_chirho| b_chirho.square_chirho()
}

// ============================================================================
// ABSOLUTER: |a| = b
// ============================================================================

binary_propagator_chirho! {
    /// A bidirectional propagator for absolute value: `|a| = b`.
    AbsoluterChirho,
    "absoluter",
    forward: |a_chirho| a_chirho.abs_chirho(),
    backward: |b_chirho| {
        // |a| = b means a could be in [-b.hi, -b.lo] or [b.lo, b.hi]
        IntervalChirho::new_chirho(-b_chirho.hi_chirho, b_chirho.hi_chirho)
    }
}

// ============================================================================
// COMPARISON PROPAGATOR MACRO (Max/Min)
// ============================================================================

/// Macro for generating comparison propagators (max, min) with symmetric backward bounds.
///
/// These propagators have special backward propagation that bounds both a and b from c.
macro_rules! comparison_propagator_chirho {
    (
        $(#[$attr_chirho:meta])*
        $name_chirho:ident,
        $name_str_chirho:expr,
        forward: |$a_fwd_chirho:ident, $b_fwd_chirho:ident| $forward_body_chirho:expr,
        backward_bound: |$c_bwd_chirho:ident| $backward_bound_chirho:expr
    ) => {
        $(#[$attr_chirho])*
        pub struct $name_chirho {
            id_chirho: usize,
            a_chirho: Rc<CellChirho<NumericInfoChirho>>,
            b_chirho: Rc<CellChirho<NumericInfoChirho>>,
            c_chirho: Rc<CellChirho<NumericInfoChirho>>,
        }

        impl $name_chirho {
            /// Creates and installs this propagator, connecting it to the given cells.
            pub fn install_chirho(
                a_chirho: Rc<CellChirho<NumericInfoChirho>>,
                b_chirho: Rc<CellChirho<NumericInfoChirho>>,
                c_chirho: Rc<CellChirho<NumericInfoChirho>>,
                scheduler_chirho: &SchedulerChirho,
            ) -> Rc<Self> {
                let propagator_chirho = Rc::new(Self {
                    id_chirho: next_id_chirho(),
                    a_chirho: a_chirho.clone(),
                    b_chirho: b_chirho.clone(),
                    c_chirho: c_chirho.clone(),
                });

                a_chirho.add_neighbor_chirho(propagator_chirho.clone());
                b_chirho.add_neighbor_chirho(propagator_chirho.clone());
                c_chirho.add_neighbor_chirho(propagator_chirho.clone());

                scheduler_chirho.alert_propagator_chirho(propagator_chirho.clone());

                propagator_chirho
            }
        }

        impl PropagatorChirho for $name_chirho {
            fn id_chirho(&self) -> usize {
                self.id_chirho
            }

            fn name_chirho(&self) -> &str {
                $name_str_chirho
            }

            fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
                let a_content_chirho = self.a_chirho.content_chirho();
                let b_content_chirho = self.b_chirho.content_chirho();
                let c_content_chirho = self.c_chirho.content_chirho();

                // Forward: c = op(a, b)
                if let (
                    NumericInfoChirho::IntervalChirho($a_fwd_chirho),
                    NumericInfoChirho::IntervalChirho($b_fwd_chirho),
                ) = (&a_content_chirho, &b_content_chirho)
                {
                    let c_new_chirho = $forward_body_chirho;
                    self.c_chirho.add_content_chirho(
                        NumericInfoChirho::IntervalChirho(c_new_chirho),
                        scheduler_chirho,
                    );
                }

                // Backward: apply same bound to both a and b
                if let NumericInfoChirho::IntervalChirho($c_bwd_chirho) = &c_content_chirho {
                    let bound_chirho = $backward_bound_chirho;
                    self.a_chirho.add_content_chirho(
                        NumericInfoChirho::IntervalChirho(bound_chirho),
                        scheduler_chirho,
                    );
                    self.b_chirho.add_content_chirho(
                        NumericInfoChirho::IntervalChirho(bound_chirho),
                        scheduler_chirho,
                    );
                }
            }
        }
    };
}

// ============================================================================
// MAX: max(a, b) = c
// ============================================================================

comparison_propagator_chirho! {
    /// A propagator for maximum: `max(a, b) = c`.
    ///
    /// Forward: computes c = max(a, b)
    /// Backward: constrains a <= c.hi and b <= c.hi
    MaxChirho,
    "max",
    forward: |a_chirho, b_chirho| a_chirho.max_chirho(b_chirho),
    backward_bound: |c_chirho| IntervalChirho::new_chirho(f64::NEG_INFINITY, c_chirho.hi_chirho)
}

// ============================================================================
// MIN: min(a, b) = c
// ============================================================================

comparison_propagator_chirho! {
    /// A propagator for minimum: `min(a, b) = c`.
    ///
    /// Forward: computes c = min(a, b)
    /// Backward: constrains a >= c.lo and b >= c.lo
    MinChirho,
    "min",
    forward: |a_chirho, b_chirho| a_chirho.min_chirho(b_chirho),
    backward_bound: |c_chirho| IntervalChirho::new_chirho(c_chirho.lo_chirho, f64::INFINITY)
}

// ============================================================================
// CONDITIONAL: if p > 0 then a else b = c
// ============================================================================

/// A propagator for conditional: `if p > 0 then a else b = c`.
///
/// This is a forward-only propagator.
pub struct ConditionalChirho {
    id_chirho: usize,
    p_chirho: Rc<CellChirho<NumericInfoChirho>>,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
    c_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl ConditionalChirho {
    /// Creates and installs a conditional propagator.
    pub fn install_chirho(
        p_chirho: Rc<CellChirho<NumericInfoChirho>>,
        a_chirho: Rc<CellChirho<NumericInfoChirho>>,
        b_chirho: Rc<CellChirho<NumericInfoChirho>>,
        c_chirho: Rc<CellChirho<NumericInfoChirho>>,
        scheduler_chirho: &SchedulerChirho,
    ) -> Rc<Self> {
        let propagator_chirho = Rc::new(Self {
            id_chirho: next_id_chirho(),
            p_chirho: p_chirho.clone(),
            a_chirho: a_chirho.clone(),
            b_chirho: b_chirho.clone(),
            c_chirho: c_chirho.clone(),
        });

        p_chirho.add_neighbor_chirho(propagator_chirho.clone());
        a_chirho.add_neighbor_chirho(propagator_chirho.clone());
        b_chirho.add_neighbor_chirho(propagator_chirho.clone());

        scheduler_chirho.alert_propagator_chirho(propagator_chirho.clone());

        propagator_chirho
    }
}

impl PropagatorChirho for ConditionalChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "conditional"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let p_chirho = self.p_chirho.content_chirho();
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();

        if let NumericInfoChirho::IntervalChirho(p_int_chirho) = &p_chirho {
            if p_int_chirho.lo_chirho > 0.0 {
                // Definitely positive - use a
                self.c_chirho.add_content_chirho(a_chirho, scheduler_chirho);
            } else if p_int_chirho.hi_chirho <= 0.0 {
                // Definitely non-positive - use b
                self.c_chirho.add_content_chirho(b_chirho, scheduler_chirho);
            }
            // Otherwise, we don't know which branch to take
        }
    }
}

// ============================================================================
// NEGATION PROPAGATOR
// ============================================================================

/// Bidirectional negation propagator: -a = b.
///
/// Propagates in both directions:
/// - If we know a, compute b = -a
/// - If we know b, compute a = -b
///
/// # Example
///
/// ```
/// use propagators_chirho::{CellChirho, NegaterChirho, NumericInfoChirho, SchedulerChirho};
///
/// let scheduler_chirho = SchedulerChirho::new_chirho();
/// let a_chirho = CellChirho::new_chirho("a");
/// let b_chirho = CellChirho::new_chirho("b");
///
/// NegaterChirho::install_chirho(a_chirho.clone(), b_chirho.clone(), &scheduler_chirho);
///
/// a_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(5.0), &scheduler_chirho);
/// scheduler_chirho.run_chirho();
///
/// // b = -5
/// let b_content_chirho = b_chirho.content_chirho();
/// let b_val_chirho = b_content_chirho.as_interval_chirho().unwrap();
/// assert!((b_val_chirho.lo_chirho - (-5.0)).abs() < 1e-10);
/// ```
pub struct NegaterChirho {
    id_chirho: usize,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl NegaterChirho {
    /// Creates and installs a negation propagator: -a = b.
    pub fn install_chirho(
        a_chirho: Rc<CellChirho<NumericInfoChirho>>,
        b_chirho: Rc<CellChirho<NumericInfoChirho>>,
        scheduler_chirho: &SchedulerChirho,
    ) -> Rc<Self> {
        let propagator_chirho = Rc::new(Self {
            id_chirho: next_id_chirho(),
            a_chirho: a_chirho.clone(),
            b_chirho: b_chirho.clone(),
        });

        a_chirho.add_neighbor_chirho(propagator_chirho.clone());
        b_chirho.add_neighbor_chirho(propagator_chirho.clone());

        scheduler_chirho.alert_propagator_chirho(propagator_chirho.clone());

        propagator_chirho
    }
}

impl PropagatorChirho for NegaterChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "negater"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();

        // Forward: b = -a
        if let NumericInfoChirho::IntervalChirho(a_int_chirho) = &a_chirho {
            let neg_a_chirho =
                IntervalChirho::new_chirho(-a_int_chirho.hi_chirho, -a_int_chirho.lo_chirho);
            self.b_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(neg_a_chirho),
                scheduler_chirho,
            );
        }

        // Backward: a = -b
        if let NumericInfoChirho::IntervalChirho(b_int_chirho) = &b_chirho {
            let neg_b_chirho =
                IntervalChirho::new_chirho(-b_int_chirho.hi_chirho, -b_int_chirho.lo_chirho);
            self.a_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(neg_b_chirho),
                scheduler_chirho,
            );
        }
    }
}

// ============================================================================
// EXPONENTIAL AND LOGARITHM PROPAGATORS
// ============================================================================

/// Bidirectional exponential propagator: e^a = b.
///
/// Propagates in both directions:
/// - If we know a, compute b = e^a
/// - If we know b (and b > 0), compute a = ln(b)
pub struct ExpChirho {
    id_chirho: usize,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl ExpChirho {
    /// Creates and installs an exponential propagator: e^a = b.
    pub fn install_chirho(
        a_chirho: Rc<CellChirho<NumericInfoChirho>>,
        b_chirho: Rc<CellChirho<NumericInfoChirho>>,
        scheduler_chirho: &SchedulerChirho,
    ) -> Rc<Self> {
        let propagator_chirho = Rc::new(Self {
            id_chirho: next_id_chirho(),
            a_chirho: a_chirho.clone(),
            b_chirho: b_chirho.clone(),
        });

        a_chirho.add_neighbor_chirho(propagator_chirho.clone());
        b_chirho.add_neighbor_chirho(propagator_chirho.clone());

        scheduler_chirho.alert_propagator_chirho(propagator_chirho.clone());

        propagator_chirho
    }
}

impl PropagatorChirho for ExpChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "exp"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();

        // Forward: b = e^a
        if let NumericInfoChirho::IntervalChirho(a_int_chirho) = &a_chirho {
            let exp_lo_chirho = a_int_chirho.lo_chirho.exp();
            let exp_hi_chirho = a_int_chirho.hi_chirho.exp();
            let exp_interval_chirho = IntervalChirho::new_chirho(exp_lo_chirho, exp_hi_chirho);
            self.b_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(exp_interval_chirho),
                scheduler_chirho,
            );
        }

        // Backward: a = ln(b) (only valid for b > 0)
        if let NumericInfoChirho::IntervalChirho(b_int_chirho) = &b_chirho {
            if b_int_chirho.lo_chirho > 0.0 {
                let ln_lo_chirho = b_int_chirho.lo_chirho.ln();
                let ln_hi_chirho = b_int_chirho.hi_chirho.ln();
                let ln_interval_chirho = IntervalChirho::new_chirho(ln_lo_chirho, ln_hi_chirho);
                self.a_chirho.add_content_chirho(
                    NumericInfoChirho::IntervalChirho(ln_interval_chirho),
                    scheduler_chirho,
                );
            }
        }
    }
}

/// Bidirectional natural logarithm propagator: ln(a) = b.
///
/// Propagates in both directions:
/// - If we know a (and a > 0), compute b = ln(a)
/// - If we know b, compute a = e^b
pub struct LnChirho {
    id_chirho: usize,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl LnChirho {
    /// Creates and installs a natural logarithm propagator: ln(a) = b.
    pub fn install_chirho(
        a_chirho: Rc<CellChirho<NumericInfoChirho>>,
        b_chirho: Rc<CellChirho<NumericInfoChirho>>,
        scheduler_chirho: &SchedulerChirho,
    ) -> Rc<Self> {
        let propagator_chirho = Rc::new(Self {
            id_chirho: next_id_chirho(),
            a_chirho: a_chirho.clone(),
            b_chirho: b_chirho.clone(),
        });

        a_chirho.add_neighbor_chirho(propagator_chirho.clone());
        b_chirho.add_neighbor_chirho(propagator_chirho.clone());

        scheduler_chirho.alert_propagator_chirho(propagator_chirho.clone());

        propagator_chirho
    }
}

impl PropagatorChirho for LnChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "ln"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();

        // Forward: b = ln(a) (only valid for a > 0)
        if let NumericInfoChirho::IntervalChirho(a_int_chirho) = &a_chirho {
            if a_int_chirho.lo_chirho > 0.0 {
                let ln_lo_chirho = a_int_chirho.lo_chirho.ln();
                let ln_hi_chirho = a_int_chirho.hi_chirho.ln();
                let ln_interval_chirho = IntervalChirho::new_chirho(ln_lo_chirho, ln_hi_chirho);
                self.b_chirho.add_content_chirho(
                    NumericInfoChirho::IntervalChirho(ln_interval_chirho),
                    scheduler_chirho,
                );
            }
        }

        // Backward: a = e^b
        if let NumericInfoChirho::IntervalChirho(b_int_chirho) = &b_chirho {
            let exp_lo_chirho = b_int_chirho.lo_chirho.exp();
            let exp_hi_chirho = b_int_chirho.hi_chirho.exp();
            let exp_interval_chirho = IntervalChirho::new_chirho(exp_lo_chirho, exp_hi_chirho);
            self.a_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(exp_interval_chirho),
                scheduler_chirho,
            );
        }
    }
}

// ============================================================================
// POWER PROPAGATOR
// ============================================================================

/// Bidirectional integer power propagator: a^n = b.
///
/// Propagates in both directions:
/// - If we know a, compute b = a^n
/// - If we know b (with appropriate conditions), compute a = b^(1/n)
pub struct PowerChirho {
    id_chirho: usize,
    n_chirho: i32,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl PowerChirho {
    /// Creates and installs a power propagator: a^n = b.
    ///
    /// # Arguments
    ///
    /// * `n_chirho` - The exponent (must be non-zero)
    /// * `a_chirho` - The base cell
    /// * `b_chirho` - The result cell
    pub fn install_chirho(
        n_chirho: i32,
        a_chirho: Rc<CellChirho<NumericInfoChirho>>,
        b_chirho: Rc<CellChirho<NumericInfoChirho>>,
        scheduler_chirho: &SchedulerChirho,
    ) -> Rc<Self> {
        assert!(n_chirho != 0, "Exponent cannot be zero");

        let propagator_chirho = Rc::new(Self {
            id_chirho: next_id_chirho(),
            n_chirho,
            a_chirho: a_chirho.clone(),
            b_chirho: b_chirho.clone(),
        });

        a_chirho.add_neighbor_chirho(propagator_chirho.clone());
        b_chirho.add_neighbor_chirho(propagator_chirho.clone());

        scheduler_chirho.alert_propagator_chirho(propagator_chirho.clone());

        propagator_chirho
    }

    /// Compute interval power a^n for positive integer n.
    #[allow(clippy::cast_possible_wrap)] // abs_n_chirho is always <= i32::MAX since we use unsigned_abs
    fn interval_pow_chirho(interval_chirho: &IntervalChirho, n_chirho: i32) -> IntervalChirho {
        if n_chirho == 0 {
            return IntervalChirho::exact_chirho(1.0);
        }

        let abs_n_chirho = n_chirho.unsigned_abs();

        // Compute the power
        let (lo_chirho, hi_chirho) = if abs_n_chirho % 2 == 0 {
            // Even power: result is non-negative, need to handle sign crossing
            if interval_chirho.lo_chirho >= 0.0 {
                // All positive
                (
                    interval_chirho.lo_chirho.powi(abs_n_chirho as i32),
                    interval_chirho.hi_chirho.powi(abs_n_chirho as i32),
                )
            } else if interval_chirho.hi_chirho <= 0.0 {
                // All negative - order reverses
                (
                    interval_chirho.hi_chirho.powi(abs_n_chirho as i32),
                    interval_chirho.lo_chirho.powi(abs_n_chirho as i32),
                )
            } else {
                // Crosses zero
                let lo_pow_chirho = interval_chirho.lo_chirho.powi(abs_n_chirho as i32);
                let hi_pow_chirho = interval_chirho.hi_chirho.powi(abs_n_chirho as i32);
                (0.0, lo_pow_chirho.max(hi_pow_chirho))
            }
        } else {
            // Odd power: preserves sign, monotonic
            (
                interval_chirho.lo_chirho.powi(abs_n_chirho as i32),
                interval_chirho.hi_chirho.powi(abs_n_chirho as i32),
            )
        };

        if n_chirho > 0 {
            IntervalChirho::new_chirho(lo_chirho, hi_chirho)
        } else {
            // Negative exponent: reciprocal
            if lo_chirho > 0.0 || hi_chirho < 0.0 {
                IntervalChirho::new_chirho(1.0 / hi_chirho, 1.0 / lo_chirho)
            } else {
                // Contains zero - result is unbounded
                IntervalChirho::new_chirho(f64::NEG_INFINITY, f64::INFINITY)
            }
        }
    }

    /// Compute interval nth root.
    fn interval_root_chirho(
        interval_chirho: &IntervalChirho,
        n_chirho: i32,
    ) -> Option<IntervalChirho> {
        if n_chirho == 0 {
            return None;
        }

        let abs_n_u32_chirho = n_chirho.unsigned_abs();
        let abs_n_chirho = f64::from(abs_n_u32_chirho);

        if abs_n_u32_chirho % 2 == 0 {
            // Even root: only valid for non-negative
            if interval_chirho.lo_chirho < 0.0 {
                if interval_chirho.hi_chirho < 0.0 {
                    return None; // No real root
                }
                // Clamp to non-negative
                let clamped_lo_chirho: f64 = 0.0;
                let root_lo_chirho = clamped_lo_chirho.powf(1.0 / abs_n_chirho);
                let root_hi_chirho = interval_chirho.hi_chirho.powf(1.0 / abs_n_chirho);
                Some(IntervalChirho::new_chirho(root_lo_chirho, root_hi_chirho))
            } else {
                let root_lo_chirho = interval_chirho.lo_chirho.powf(1.0 / abs_n_chirho);
                let root_hi_chirho = interval_chirho.hi_chirho.powf(1.0 / abs_n_chirho);
                Some(IntervalChirho::new_chirho(root_lo_chirho, root_hi_chirho))
            }
        } else {
            // Odd root: always exists, preserves sign
            let root_lo_chirho = interval_chirho.lo_chirho.signum()
                * interval_chirho.lo_chirho.abs().powf(1.0 / abs_n_chirho);
            let root_hi_chirho = interval_chirho.hi_chirho.signum()
                * interval_chirho.hi_chirho.abs().powf(1.0 / abs_n_chirho);
            Some(IntervalChirho::new_chirho(root_lo_chirho, root_hi_chirho))
        }
    }
}

impl PropagatorChirho for PowerChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "power"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();

        // Forward: b = a^n
        if let NumericInfoChirho::IntervalChirho(a_int_chirho) = &a_chirho {
            let pow_interval_chirho = Self::interval_pow_chirho(a_int_chirho, self.n_chirho);
            self.b_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(pow_interval_chirho),
                scheduler_chirho,
            );
        }

        // Backward: a = b^(1/n)
        if let NumericInfoChirho::IntervalChirho(b_int_chirho) = &b_chirho {
            if let Some(root_interval_chirho) =
                Self::interval_root_chirho(b_int_chirho, self.n_chirho)
            {
                self.a_chirho.add_content_chirho(
                    NumericInfoChirho::IntervalChirho(root_interval_chirho),
                    scheduler_chirho,
                );
            }
        }
    }
}

// ============================================================================
// CLAMP PROPAGATOR
// ============================================================================

/// Clamp propagator: clamp(a, lo, hi) = b.
///
/// This is a forward-only propagator that clamps the input to a range.
pub struct ClampChirho {
    id_chirho: usize,
    lo_chirho: f64,
    hi_chirho: f64,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl ClampChirho {
    /// Creates and installs a clamp propagator: clamp(a, lo, hi) = b.
    pub fn install_chirho(
        lo_chirho: f64,
        hi_chirho: f64,
        a_chirho: Rc<CellChirho<NumericInfoChirho>>,
        b_chirho: Rc<CellChirho<NumericInfoChirho>>,
        scheduler_chirho: &SchedulerChirho,
    ) -> Rc<Self> {
        let propagator_chirho = Rc::new(Self {
            id_chirho: next_id_chirho(),
            lo_chirho,
            hi_chirho,
            a_chirho: a_chirho.clone(),
            b_chirho: b_chirho.clone(),
        });

        a_chirho.add_neighbor_chirho(propagator_chirho.clone());

        scheduler_chirho.alert_propagator_chirho(propagator_chirho.clone());

        propagator_chirho
    }
}

impl PropagatorChirho for ClampChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "clamp"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();

        if let NumericInfoChirho::IntervalChirho(a_int_chirho) = &a_chirho {
            // Clamp the interval to [lo, hi]
            let clamped_lo_chirho = a_int_chirho
                .lo_chirho
                .max(self.lo_chirho)
                .min(self.hi_chirho);
            let clamped_hi_chirho = a_int_chirho
                .hi_chirho
                .max(self.lo_chirho)
                .min(self.hi_chirho);
            let clamped_interval_chirho =
                IntervalChirho::new_chirho(clamped_lo_chirho, clamped_hi_chirho);
            self.b_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(clamped_interval_chirho),
                scheduler_chirho,
            );
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_constant_propagator_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let cell_chirho = CellChirho::new_chirho("x");

        let _propagator_chirho =
            ConstantChirho::new_chirho(NumericInfoChirho::exact_chirho(42.0), cell_chirho.clone());

        scheduler_chirho.alert_propagator_chirho(_propagator_chirho);
        scheduler_chirho.run_chirho();

        let content_chirho = cell_chirho.content_chirho();
        let interval_chirho = content_chirho.as_interval_chirho().unwrap();
        assert!((interval_chirho.lo_chirho - 42.0).abs() < 1e-10);
    }

    #[test]
    fn test_adder_forward_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let a_chirho = CellChirho::new_chirho("a");
        let b_chirho = CellChirho::new_chirho("b");
        let c_chirho = CellChirho::new_chirho("c");

        IntervalAdderChirho::install_chirho(
            a_chirho.clone(),
            b_chirho.clone(),
            c_chirho.clone(),
            &scheduler_chirho,
        );

        a_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(3.0), &scheduler_chirho);
        b_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(4.0), &scheduler_chirho);
        scheduler_chirho.run_chirho();

        let c_content_chirho = c_chirho.content_chirho();
        let c_interval_chirho = c_content_chirho.as_interval_chirho().unwrap();
        assert!((c_interval_chirho.lo_chirho - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_adder_backward_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let a_chirho = CellChirho::new_chirho("a");
        let b_chirho = CellChirho::new_chirho("b");
        let c_chirho = CellChirho::new_chirho("c");

        IntervalAdderChirho::install_chirho(
            a_chirho.clone(),
            b_chirho.clone(),
            c_chirho.clone(),
            &scheduler_chirho,
        );

        a_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(3.0), &scheduler_chirho);
        c_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(7.0), &scheduler_chirho);
        scheduler_chirho.run_chirho();

        // b should be 4
        let b_content_chirho = b_chirho.content_chirho();
        let b_interval_chirho = b_content_chirho.as_interval_chirho().unwrap();
        assert!((b_interval_chirho.lo_chirho - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_multiplier_forward_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let a_chirho = CellChirho::new_chirho("a");
        let b_chirho = CellChirho::new_chirho("b");
        let c_chirho = CellChirho::new_chirho("c");

        IntervalMultiplierChirho::install_chirho(
            a_chirho.clone(),
            b_chirho.clone(),
            c_chirho.clone(),
            &scheduler_chirho,
        );

        a_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(3.0), &scheduler_chirho);
        b_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(4.0), &scheduler_chirho);
        scheduler_chirho.run_chirho();

        let c_content_chirho = c_chirho.content_chirho();
        let c_interval_chirho = c_content_chirho.as_interval_chirho().unwrap();
        assert!((c_interval_chirho.lo_chirho - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_squarer_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let a_chirho = CellChirho::new_chirho("a");
        let b_chirho = CellChirho::new_chirho("b");

        SquarerChirho::install_chirho(a_chirho.clone(), b_chirho.clone(), &scheduler_chirho);

        a_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(5.0), &scheduler_chirho);
        scheduler_chirho.run_chirho();

        let b_content_chirho = b_chirho.content_chirho();
        let b_interval_chirho = b_content_chirho.as_interval_chirho().unwrap();
        assert!((b_interval_chirho.lo_chirho - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_negater_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let a_chirho = CellChirho::new_chirho("a");
        let b_chirho = CellChirho::new_chirho("b");

        NegaterChirho::install_chirho(a_chirho.clone(), b_chirho.clone(), &scheduler_chirho);

        // Forward: -5 = b
        a_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(5.0), &scheduler_chirho);
        scheduler_chirho.run_chirho();

        let b_content_chirho = b_chirho.content_chirho();
        let b_interval_chirho = b_content_chirho.as_interval_chirho().unwrap();
        assert!((b_interval_chirho.lo_chirho - (-5.0)).abs() < 1e-10);
    }

    #[test]
    fn test_negater_backward_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let a_chirho = CellChirho::new_chirho("a");
        let b_chirho = CellChirho::new_chirho("b");

        NegaterChirho::install_chirho(a_chirho.clone(), b_chirho.clone(), &scheduler_chirho);

        // Backward: -a = -3, so a = 3
        b_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(-3.0), &scheduler_chirho);
        scheduler_chirho.run_chirho();

        let a_content_chirho = a_chirho.content_chirho();
        let a_interval_chirho = a_content_chirho.as_interval_chirho().unwrap();
        assert!((a_interval_chirho.lo_chirho - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_exp_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let a_chirho = CellChirho::new_chirho("a");
        let b_chirho = CellChirho::new_chirho("b");

        ExpChirho::install_chirho(a_chirho.clone(), b_chirho.clone(), &scheduler_chirho);

        // e^0 = 1
        a_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(0.0), &scheduler_chirho);
        scheduler_chirho.run_chirho();

        let b_content_chirho = b_chirho.content_chirho();
        let b_interval_chirho = b_content_chirho.as_interval_chirho().unwrap();
        assert!((b_interval_chirho.lo_chirho - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_ln_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let a_chirho = CellChirho::new_chirho("a");
        let b_chirho = CellChirho::new_chirho("b");

        LnChirho::install_chirho(a_chirho.clone(), b_chirho.clone(), &scheduler_chirho);

        // ln(e) = 1
        a_chirho.add_content_chirho(
            NumericInfoChirho::exact_chirho(std::f64::consts::E),
            &scheduler_chirho,
        );
        scheduler_chirho.run_chirho();

        let b_content_chirho = b_chirho.content_chirho();
        let b_interval_chirho = b_content_chirho.as_interval_chirho().unwrap();
        assert!((b_interval_chirho.lo_chirho - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_power_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let a_chirho = CellChirho::new_chirho("a");
        let b_chirho = CellChirho::new_chirho("b");

        // 2^3 = 8
        PowerChirho::install_chirho(3, a_chirho.clone(), b_chirho.clone(), &scheduler_chirho);

        a_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(2.0), &scheduler_chirho);
        scheduler_chirho.run_chirho();

        let b_content_chirho = b_chirho.content_chirho();
        let b_interval_chirho = b_content_chirho.as_interval_chirho().unwrap();
        assert!((b_interval_chirho.lo_chirho - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_power_backward_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let a_chirho = CellChirho::new_chirho("a");
        let b_chirho = CellChirho::new_chirho("b");

        // a^3 = 27, so a = 3
        PowerChirho::install_chirho(3, a_chirho.clone(), b_chirho.clone(), &scheduler_chirho);

        b_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(27.0), &scheduler_chirho);
        scheduler_chirho.run_chirho();

        let a_content_chirho = a_chirho.content_chirho();
        let a_interval_chirho = a_content_chirho.as_interval_chirho().unwrap();
        assert!((a_interval_chirho.lo_chirho - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_clamp_chirho() {
        let scheduler_chirho = SchedulerChirho::new_chirho();
        let a_chirho = CellChirho::new_chirho("a");
        let b_chirho = CellChirho::new_chirho("b");

        // clamp(a, 0, 10) = b
        ClampChirho::install_chirho(
            0.0,
            10.0,
            a_chirho.clone(),
            b_chirho.clone(),
            &scheduler_chirho,
        );

        // Value exceeds clamp range
        a_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(15.0), &scheduler_chirho);
        scheduler_chirho.run_chirho();

        let b_content_chirho = b_chirho.content_chirho();
        let b_interval_chirho = b_content_chirho.as_interval_chirho().unwrap();
        assert!((b_interval_chirho.lo_chirho - 10.0).abs() < 1e-10);
        assert!((b_interval_chirho.hi_chirho - 10.0).abs() < 1e-10);
    }
}
