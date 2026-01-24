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

use crate::cell_chirho::CellChirho;
use crate::interval_chirho::{IntervalChirho, NumericInfoChirho};
use crate::scheduler_chirho::SchedulerChirho;

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
pub struct IntervalAdderChirho {
    id_chirho: usize,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
    c_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl IntervalAdderChirho {
    /// Creates and installs an adder propagator.
    ///
    /// This registers the propagator with all three cells and alerts
    /// it via the scheduler.
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

impl PropagatorChirho for IntervalAdderChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "interval_adder"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();
        let c_chirho = self.c_chirho.content_chirho();

        // Forward: c = a + b
        if let (
            NumericInfoChirho::IntervalChirho(a_int_chirho),
            NumericInfoChirho::IntervalChirho(b_int_chirho),
        ) = (&a_chirho, &b_chirho)
        {
            let c_new_chirho = a_int_chirho.add_chirho(b_int_chirho);
            self.c_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(c_new_chirho),
                scheduler_chirho,
            );
        }

        // Backward: a = c - b
        if let (
            NumericInfoChirho::IntervalChirho(c_int_chirho),
            NumericInfoChirho::IntervalChirho(b_int_chirho),
        ) = (&c_chirho, &b_chirho)
        {
            let a_new_chirho = c_int_chirho.sub_chirho(b_int_chirho);
            self.a_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(a_new_chirho),
                scheduler_chirho,
            );
        }

        // Backward: b = c - a
        if let (
            NumericInfoChirho::IntervalChirho(c_int_chirho),
            NumericInfoChirho::IntervalChirho(a_int_chirho),
        ) = (&c_chirho, &a_chirho)
        {
            let b_new_chirho = c_int_chirho.sub_chirho(a_int_chirho);
            self.b_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(b_new_chirho),
                scheduler_chirho,
            );
        }
    }
}

// ============================================================================
// INTERVAL SUBTRACTOR: a - b = c
// ============================================================================

/// A bidirectional propagator for subtraction: `a - b = c`.
pub struct IntervalSubtractorChirho {
    id_chirho: usize,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
    c_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl IntervalSubtractorChirho {
    /// Creates and installs a subtractor propagator.
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

impl PropagatorChirho for IntervalSubtractorChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "interval_subtractor"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();
        let c_chirho = self.c_chirho.content_chirho();

        // Forward: c = a - b
        if let (
            NumericInfoChirho::IntervalChirho(a_int_chirho),
            NumericInfoChirho::IntervalChirho(b_int_chirho),
        ) = (&a_chirho, &b_chirho)
        {
            let c_new_chirho = a_int_chirho.sub_chirho(b_int_chirho);
            self.c_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(c_new_chirho),
                scheduler_chirho,
            );
        }

        // Backward: a = c + b
        if let (
            NumericInfoChirho::IntervalChirho(c_int_chirho),
            NumericInfoChirho::IntervalChirho(b_int_chirho),
        ) = (&c_chirho, &b_chirho)
        {
            let a_new_chirho = c_int_chirho.add_chirho(b_int_chirho);
            self.a_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(a_new_chirho),
                scheduler_chirho,
            );
        }

        // Backward: b = a - c
        if let (
            NumericInfoChirho::IntervalChirho(a_int_chirho),
            NumericInfoChirho::IntervalChirho(c_int_chirho),
        ) = (&a_chirho, &c_chirho)
        {
            let b_new_chirho = a_int_chirho.sub_chirho(c_int_chirho);
            self.b_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(b_new_chirho),
                scheduler_chirho,
            );
        }
    }
}

// ============================================================================
// INTERVAL MULTIPLIER: a * b = c
// ============================================================================

/// A bidirectional propagator for multiplication: `a * b = c`.
///
/// Works in all three directions using interval arithmetic.
pub struct IntervalMultiplierChirho {
    id_chirho: usize,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
    c_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl IntervalMultiplierChirho {
    /// Creates and installs a multiplier propagator.
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

impl PropagatorChirho for IntervalMultiplierChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "interval_multiplier"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();
        let c_chirho = self.c_chirho.content_chirho();

        // Forward: c = a * b
        if let (
            NumericInfoChirho::IntervalChirho(a_int_chirho),
            NumericInfoChirho::IntervalChirho(b_int_chirho),
        ) = (&a_chirho, &b_chirho)
        {
            let c_new_chirho = a_int_chirho.mul_chirho(b_int_chirho);
            self.c_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(c_new_chirho),
                scheduler_chirho,
            );
        }

        // Backward: a = c / b
        if let (
            NumericInfoChirho::IntervalChirho(c_int_chirho),
            NumericInfoChirho::IntervalChirho(b_int_chirho),
        ) = (&c_chirho, &b_chirho)
        {
            let a_new_chirho = c_int_chirho.div_chirho(b_int_chirho);
            self.a_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(a_new_chirho),
                scheduler_chirho,
            );
        }

        // Backward: b = c / a
        if let (
            NumericInfoChirho::IntervalChirho(c_int_chirho),
            NumericInfoChirho::IntervalChirho(a_int_chirho),
        ) = (&c_chirho, &a_chirho)
        {
            let b_new_chirho = c_int_chirho.div_chirho(a_int_chirho);
            self.b_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(b_new_chirho),
                scheduler_chirho,
            );
        }
    }
}

// ============================================================================
// INTERVAL DIVIDER: a / b = c
// ============================================================================

/// A bidirectional propagator for division: `a / b = c`.
pub struct IntervalDividerChirho {
    id_chirho: usize,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
    c_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl IntervalDividerChirho {
    /// Creates and installs a divider propagator.
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

impl PropagatorChirho for IntervalDividerChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "interval_divider"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();
        let c_chirho = self.c_chirho.content_chirho();

        // Forward: c = a / b
        if let (
            NumericInfoChirho::IntervalChirho(a_int_chirho),
            NumericInfoChirho::IntervalChirho(b_int_chirho),
        ) = (&a_chirho, &b_chirho)
        {
            let c_new_chirho = a_int_chirho.div_chirho(b_int_chirho);
            self.c_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(c_new_chirho),
                scheduler_chirho,
            );
        }

        // Backward: a = c * b
        if let (
            NumericInfoChirho::IntervalChirho(c_int_chirho),
            NumericInfoChirho::IntervalChirho(b_int_chirho),
        ) = (&c_chirho, &b_chirho)
        {
            let a_new_chirho = c_int_chirho.mul_chirho(b_int_chirho);
            self.a_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(a_new_chirho),
                scheduler_chirho,
            );
        }

        // Backward: b = a / c
        if let (
            NumericInfoChirho::IntervalChirho(a_int_chirho),
            NumericInfoChirho::IntervalChirho(c_int_chirho),
        ) = (&a_chirho, &c_chirho)
        {
            let b_new_chirho = a_int_chirho.div_chirho(c_int_chirho);
            self.b_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(b_new_chirho),
                scheduler_chirho,
            );
        }
    }
}

// ============================================================================
// SQUARER: a² = b
// ============================================================================

/// A bidirectional propagator for squaring: `a² = b`.
///
/// - Forward: `b = a²`
/// - Backward: `a = ±√b` (intersected with current `a`)
pub struct SquarerChirho {
    id_chirho: usize,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl SquarerChirho {
    /// Creates and installs a squarer propagator.
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

impl PropagatorChirho for SquarerChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "squarer"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();

        // Forward: b = a²
        if let NumericInfoChirho::IntervalChirho(a_int_chirho) = &a_chirho {
            let b_new_chirho = a_int_chirho.square_chirho();
            self.b_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(b_new_chirho),
                scheduler_chirho,
            );
        }

        // Backward: a = ±√b
        if let NumericInfoChirho::IntervalChirho(b_int_chirho) = &b_chirho {
            let sqrt_chirho = b_int_chirho.sqrt_chirho();
            // a could be positive or negative
            let a_possible_chirho =
                IntervalChirho::new_chirho(-sqrt_chirho.hi_chirho, sqrt_chirho.hi_chirho);
            self.a_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(a_possible_chirho),
                scheduler_chirho,
            );
        }
    }
}

// ============================================================================
// SQRTER: √a = b (square root)
// ============================================================================

/// A bidirectional propagator for square root: `√a = b`.
pub struct SqrterChirho {
    id_chirho: usize,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl SqrterChirho {
    /// Creates and installs a square root propagator.
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

impl PropagatorChirho for SqrterChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "sqrter"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();

        // Forward: b = √a
        if let NumericInfoChirho::IntervalChirho(a_int_chirho) = &a_chirho {
            let b_new_chirho = a_int_chirho.sqrt_chirho();
            self.b_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(b_new_chirho),
                scheduler_chirho,
            );
        }

        // Backward: a = b²
        if let NumericInfoChirho::IntervalChirho(b_int_chirho) = &b_chirho {
            let a_new_chirho = b_int_chirho.square_chirho();
            self.a_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(a_new_chirho),
                scheduler_chirho,
            );
        }
    }
}

// ============================================================================
// ABSOLUTER: |a| = b
// ============================================================================

/// A bidirectional propagator for absolute value: `|a| = b`.
pub struct AbsoluterChirho {
    id_chirho: usize,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl AbsoluterChirho {
    /// Creates and installs an absolute value propagator.
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

impl PropagatorChirho for AbsoluterChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "absoluter"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();

        // Forward: b = |a|
        if let NumericInfoChirho::IntervalChirho(a_int_chirho) = &a_chirho {
            let b_new_chirho = a_int_chirho.abs_chirho();
            self.b_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(b_new_chirho),
                scheduler_chirho,
            );
        }

        // Backward: a ∈ [-b, b]
        if let NumericInfoChirho::IntervalChirho(b_int_chirho) = &b_chirho {
            // b must be non-negative
            let b_positive_chirho =
                b_int_chirho.intersect_chirho(&IntervalChirho::non_negative_chirho());
            if !b_positive_chirho.is_empty_chirho() {
                let a_new_chirho = IntervalChirho::new_chirho(
                    -b_positive_chirho.hi_chirho,
                    b_positive_chirho.hi_chirho,
                );
                self.a_chirho.add_content_chirho(
                    NumericInfoChirho::IntervalChirho(a_new_chirho),
                    scheduler_chirho,
                );
            }
        }
    }
}

// ============================================================================
// MAX: max(a, b) = c
// ============================================================================

/// A propagator for maximum: `max(a, b) = c`.
pub struct MaxChirho {
    id_chirho: usize,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
    c_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl MaxChirho {
    /// Creates and installs a max propagator.
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

impl PropagatorChirho for MaxChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "max"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();
        let c_chirho = self.c_chirho.content_chirho();

        // Forward: c = max(a, b)
        if let (
            NumericInfoChirho::IntervalChirho(a_int_chirho),
            NumericInfoChirho::IntervalChirho(b_int_chirho),
        ) = (&a_chirho, &b_chirho)
        {
            let c_new_chirho = a_int_chirho.max_chirho(b_int_chirho);
            self.c_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(c_new_chirho),
                scheduler_chirho,
            );
        }

        // Backward: a <= c and b <= c
        if let NumericInfoChirho::IntervalChirho(c_int_chirho) = &c_chirho {
            let upper_bound_chirho =
                IntervalChirho::new_chirho(f64::NEG_INFINITY, c_int_chirho.hi_chirho);
            self.a_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(upper_bound_chirho),
                scheduler_chirho,
            );
            self.b_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(upper_bound_chirho),
                scheduler_chirho,
            );
        }
    }
}

// ============================================================================
// MIN: min(a, b) = c
// ============================================================================

/// A propagator for minimum: `min(a, b) = c`.
pub struct MinChirho {
    id_chirho: usize,
    a_chirho: Rc<CellChirho<NumericInfoChirho>>,
    b_chirho: Rc<CellChirho<NumericInfoChirho>>,
    c_chirho: Rc<CellChirho<NumericInfoChirho>>,
}

impl MinChirho {
    /// Creates and installs a min propagator.
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

impl PropagatorChirho for MinChirho {
    fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    fn name_chirho(&self) -> &str {
        "min"
    }

    fn run_chirho(&self, scheduler_chirho: &SchedulerChirho) {
        let a_chirho = self.a_chirho.content_chirho();
        let b_chirho = self.b_chirho.content_chirho();
        let c_chirho = self.c_chirho.content_chirho();

        // Forward: c = min(a, b)
        if let (
            NumericInfoChirho::IntervalChirho(a_int_chirho),
            NumericInfoChirho::IntervalChirho(b_int_chirho),
        ) = (&a_chirho, &b_chirho)
        {
            let c_new_chirho = a_int_chirho.min_chirho(b_int_chirho);
            self.c_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(c_new_chirho),
                scheduler_chirho,
            );
        }

        // Backward: a >= c and b >= c
        if let NumericInfoChirho::IntervalChirho(c_int_chirho) = &c_chirho {
            let lower_bound_chirho =
                IntervalChirho::new_chirho(c_int_chirho.lo_chirho, f64::INFINITY);
            self.a_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(lower_bound_chirho),
                scheduler_chirho,
            );
            self.b_chirho.add_content_chirho(
                NumericInfoChirho::IntervalChirho(lower_bound_chirho),
                scheduler_chirho,
            );
        }
    }
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
}
