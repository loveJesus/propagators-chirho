// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! High-level API for building constraint systems.
//!
//! The [`ConstraintSystemChirho`] provides a convenient builder-style interface
//! for creating and managing propagator networks.
//!
//! # Example
//!
//! ```
//! use propagators_chirho::ConstraintSystemChirho;
//!
//! let mut system_chirho = ConstraintSystemChirho::new_chirho();
//!
//! // Create cells
//! let a_chirho = system_chirho.make_cell_chirho("a");
//! let b_chirho = system_chirho.make_cell_chirho("b");
//! let c_chirho = system_chirho.make_cell_chirho("c");
//!
//! // Add constraint: a + b = c
//! system_chirho.add_adder_chirho(&a_chirho, &b_chirho, &c_chirho);
//!
//! // Set values and propagate
//! system_chirho.set_exact_chirho(&a_chirho, 3.0);
//! system_chirho.set_exact_chirho(&b_chirho, 4.0);
//! system_chirho.run_chirho();
//!
//! // c is now 7.0
//! let c_value_chirho = system_chirho.get_chirho(&c_chirho);
//! ```

use std::collections::HashMap;
use std::rc::Rc;

use crate::algebra_chirho::{PropagatorErrorChirho, PropagatorResultChirho};
use crate::cell_chirho::CellChirho;
use crate::interval_chirho::NumericInfoChirho;
use crate::propagator_chirho::{
    AbsoluterChirho, ClampChirho, ConstantChirho, ExpChirho, IntervalAdderChirho,
    IntervalDividerChirho, IntervalMultiplierChirho, IntervalSubtractorChirho, LnChirho, MaxChirho,
    MinChirho, NegaterChirho, PowerChirho, SqrterChirho, SquarerChirho,
};
use crate::scheduler_chirho::SchedulerChirho;

/// A high-level interface for building and running constraint systems.
///
/// This provides a convenient API that handles cell and propagator management,
/// abstracting away the details of the scheduler and cell references.
///
/// # Example
///
/// ```
/// use propagators_chirho::ConstraintSystemChirho;
///
/// let mut system_chirho = ConstraintSystemChirho::new_chirho();
///
/// // Temperature conversion: F = C * 9/5 + 32
/// let c_chirho = system_chirho.make_cell_chirho("celsius");
/// let f_chirho = system_chirho.make_cell_chirho("fahrenheit");
///
/// // Internal cells for computation
/// let nine_fifths_chirho = system_chirho.make_cell_chirho("9/5");
/// let thirty_two_chirho = system_chirho.make_cell_chirho("32");
/// let product_chirho = system_chirho.make_cell_chirho("c*9/5");
///
/// // Set constants
/// system_chirho.set_exact_chirho(&nine_fifths_chirho, 1.8);
/// system_chirho.set_exact_chirho(&thirty_two_chirho, 32.0);
///
/// // F = C * 9/5 + 32
/// system_chirho.add_multiplier_chirho(&c_chirho, &nine_fifths_chirho, &product_chirho);
/// system_chirho.add_adder_chirho(&product_chirho, &thirty_two_chirho, &f_chirho);
///
/// // Set Celsius, get Fahrenheit
/// system_chirho.set_exact_chirho(&c_chirho, 100.0);
/// system_chirho.run_chirho();
///
/// let f_value_chirho = system_chirho.get_chirho(&f_chirho);
/// // f_value should be 212.0
/// ```
pub struct ConstraintSystemChirho {
    /// The scheduler for running propagators.
    scheduler_chirho: SchedulerChirho,
    /// Map from cell names to cell references.
    cells_chirho: HashMap<String, Rc<CellChirho<NumericInfoChirho>>>,
}

impl ConstraintSystemChirho {
    /// Creates a new empty constraint system.
    pub fn new_chirho() -> Self {
        Self {
            scheduler_chirho: SchedulerChirho::new_chirho(),
            cells_chirho: HashMap::new(),
        }
    }

    /// Creates a new cell with the given name.
    ///
    /// Returns the cell name for future reference.
    ///
    /// # Panics
    ///
    /// Panics if a cell with this name already exists.
    /// Use [`try_make_cell_chirho`](Self::try_make_cell_chirho) for a non-panicking version.
    pub fn make_cell_chirho(&mut self, name_chirho: &str) -> String {
        self.try_make_cell_chirho(name_chirho)
            .expect("Cell already exists")
    }

    /// Creates a new cell with the given name (non-panicking version).
    ///
    /// Returns `Ok(name)` on success, or an error if the cell already exists.
    ///
    /// # Errors
    ///
    /// Returns `PropagatorErrorChirho::CellAlreadyExistsChirho` if a cell
    /// with the given name already exists in the system.
    pub fn try_make_cell_chirho(&mut self, name_chirho: &str) -> PropagatorResultChirho<String> {
        if self.cells_chirho.contains_key(name_chirho) {
            return Err(PropagatorErrorChirho::CellAlreadyExistsChirho {
                name_chirho: name_chirho.to_string(),
            });
        }

        let cell_chirho = CellChirho::new_chirho(name_chirho);
        self.cells_chirho
            .insert(name_chirho.to_string(), cell_chirho);
        Ok(name_chirho.to_string())
    }

    /// Gets a cell by name.
    fn get_cell_chirho(&self, name_chirho: &str) -> Rc<CellChirho<NumericInfoChirho>> {
        self.try_get_cell_chirho(name_chirho)
            .expect("Cell not found")
    }

    /// Gets a cell by name (non-panicking version).
    fn try_get_cell_chirho(
        &self,
        name_chirho: &str,
    ) -> PropagatorResultChirho<Rc<CellChirho<NumericInfoChirho>>> {
        self.cells_chirho.get(name_chirho).cloned().ok_or_else(|| {
            PropagatorErrorChirho::CellNameNotFoundChirho {
                name_chirho: name_chirho.to_string(),
            }
        })
    }

    /// Gets the current value of a cell.
    pub fn get_chirho(&self, name_chirho: &str) -> NumericInfoChirho {
        self.get_cell_chirho(name_chirho).content_chirho()
    }

    /// Sets a cell to an exact value.
    pub fn set_exact_chirho(&self, name_chirho: &str, value_chirho: f64) {
        let cell_chirho = self.get_cell_chirho(name_chirho);
        cell_chirho.add_content_chirho(
            NumericInfoChirho::exact_chirho(value_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Sets a cell to an interval.
    pub fn set_interval_chirho(&self, name_chirho: &str, lo_chirho: f64, hi_chirho: f64) {
        let cell_chirho = self.get_cell_chirho(name_chirho);
        cell_chirho.add_content_chirho(
            NumericInfoChirho::interval_chirho(lo_chirho, hi_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds content to a cell.
    pub fn add_content_chirho(&self, name_chirho: &str, content_chirho: NumericInfoChirho) {
        let cell_chirho = self.get_cell_chirho(name_chirho);
        cell_chirho.add_content_chirho(content_chirho, &self.scheduler_chirho);
    }

    /// Runs propagators to fixpoint.
    pub fn run_chirho(&self) {
        self.scheduler_chirho.run_chirho();
    }

    /// Returns `true` if any cell contains a contradiction.
    pub fn has_contradiction_chirho(&self) -> bool {
        self.cells_chirho
            .values()
            .any(|cell_chirho| cell_chirho.is_contradiction_chirho())
    }

    /// Returns the names of cells containing contradictions.
    pub fn contradicted_cells_chirho(&self) -> Vec<String> {
        self.cells_chirho
            .iter()
            .filter(|(_, cell_chirho)| cell_chirho.is_contradiction_chirho())
            .map(|(name_chirho, _)| name_chirho.clone())
            .collect()
    }

    // ========================================================================
    // CONSTRAINT BUILDERS
    // ========================================================================

    /// Adds an adder constraint: a + b = c.
    pub fn add_adder_chirho(&self, a_chirho: &str, b_chirho: &str, c_chirho: &str) {
        IntervalAdderChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            self.get_cell_chirho(c_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds a subtractor constraint: a - b = c.
    pub fn add_subtractor_chirho(&self, a_chirho: &str, b_chirho: &str, c_chirho: &str) {
        IntervalSubtractorChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            self.get_cell_chirho(c_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds a multiplier constraint: a * b = c.
    pub fn add_multiplier_chirho(&self, a_chirho: &str, b_chirho: &str, c_chirho: &str) {
        IntervalMultiplierChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            self.get_cell_chirho(c_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds a divider constraint: a / b = c.
    pub fn add_divider_chirho(&self, a_chirho: &str, b_chirho: &str, c_chirho: &str) {
        IntervalDividerChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            self.get_cell_chirho(c_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds a squarer constraint: a² = b.
    pub fn add_squarer_chirho(&self, a_chirho: &str, b_chirho: &str) {
        SquarerChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds a square root constraint: √a = b.
    pub fn add_sqrter_chirho(&self, a_chirho: &str, b_chirho: &str) {
        SqrterChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds an absolute value constraint: |a| = b.
    pub fn add_absoluter_chirho(&self, a_chirho: &str, b_chirho: &str) {
        AbsoluterChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds a max constraint: max(a, b) = c.
    pub fn add_max_chirho(&self, a_chirho: &str, b_chirho: &str, c_chirho: &str) {
        MaxChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            self.get_cell_chirho(c_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds a min constraint: min(a, b) = c.
    pub fn add_min_chirho(&self, a_chirho: &str, b_chirho: &str, c_chirho: &str) {
        MinChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            self.get_cell_chirho(c_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds a constant propagator that sets a cell to a value.
    pub fn add_constant_chirho(&self, value_chirho: f64, cell_chirho: &str) {
        let propagator_chirho = ConstantChirho::new_chirho(
            NumericInfoChirho::exact_chirho(value_chirho),
            self.get_cell_chirho(cell_chirho),
        );
        self.scheduler_chirho
            .alert_propagator_chirho(propagator_chirho);
    }

    /// Adds a negation constraint: -a = b.
    pub fn add_negater_chirho(&self, a_chirho: &str, b_chirho: &str) {
        NegaterChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds an exponential constraint: e^a = b.
    pub fn add_exp_chirho(&self, a_chirho: &str, b_chirho: &str) {
        ExpChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds a natural logarithm constraint: ln(a) = b.
    pub fn add_ln_chirho(&self, a_chirho: &str, b_chirho: &str) {
        LnChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds a power constraint: a^n = b.
    ///
    /// # Arguments
    ///
    /// * `n_chirho` - The exponent (must be non-zero)
    /// * `a_chirho` - The base cell name
    /// * `b_chirho` - The result cell name
    pub fn add_power_chirho(&self, n_chirho: i32, a_chirho: &str, b_chirho: &str) {
        PowerChirho::install_chirho(
            n_chirho,
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );
    }

    /// Adds a clamp constraint: clamp(a, lo, hi) = b.
    ///
    /// The result b is constrained to be within [lo, hi].
    pub fn add_clamp_chirho(&self, a_chirho: &str, lo_chirho: f64, hi_chirho: f64, b_chirho: &str) {
        ClampChirho::install_chirho(
            lo_chirho,
            hi_chirho,
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );
    }

    // ========================================================================
    // COMPOUND CONSTRAINTS
    // ========================================================================

    /// Creates a linear constraint: a*x + b = y.
    ///
    /// Automatically creates internal cells for intermediate values.
    pub fn add_linear_chirho(
        &mut self,
        a_chirho: f64,
        x_chirho: &str,
        b_chirho: f64,
        y_chirho: &str,
    ) {
        let a_cell_chirho = self.make_cell_chirho(&format!("_linear_a_{}", x_chirho));
        let b_cell_chirho = self.make_cell_chirho(&format!("_linear_b_{}", x_chirho));
        let ax_chirho = self.make_cell_chirho(&format!("_linear_ax_{}", x_chirho));

        self.set_exact_chirho(&a_cell_chirho, a_chirho);
        self.set_exact_chirho(&b_cell_chirho, b_chirho);

        self.add_multiplier_chirho(&a_cell_chirho, x_chirho, &ax_chirho);
        self.add_adder_chirho(&ax_chirho, &b_cell_chirho, y_chirho);
    }

    /// Creates a Pythagorean constraint: a² + b² = c².
    pub fn add_pythagorean_chirho(&mut self, a_chirho: &str, b_chirho: &str, c_chirho: &str) {
        let a_sq_chirho = self.make_cell_chirho(&format!("_pyth_a2_{}", a_chirho));
        let b_sq_chirho = self.make_cell_chirho(&format!("_pyth_b2_{}", b_chirho));
        let c_sq_chirho = self.make_cell_chirho(&format!("_pyth_c2_{}", c_chirho));

        self.add_squarer_chirho(a_chirho, &a_sq_chirho);
        self.add_squarer_chirho(b_chirho, &b_sq_chirho);
        self.add_squarer_chirho(c_chirho, &c_sq_chirho);

        self.add_adder_chirho(&a_sq_chirho, &b_sq_chirho, &c_sq_chirho);
    }

    /// Returns the scheduler for advanced usage.
    pub fn scheduler_chirho(&self) -> &SchedulerChirho {
        &self.scheduler_chirho
    }

    /// Returns the number of cells in the system.
    pub fn cell_count_chirho(&self) -> usize {
        self.cells_chirho.len()
    }
}

impl Default for ConstraintSystemChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl std::fmt::Debug for ConstraintSystemChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f_chirho
            .debug_struct("ConstraintSystem")
            .field("cells", &self.cells_chirho.len())
            .finish()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_simple_addition_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");
        let c_chirho = system_chirho.make_cell_chirho("c");

        system_chirho.add_adder_chirho(&a_chirho, &b_chirho, &c_chirho);

        system_chirho.set_exact_chirho(&a_chirho, 3.0);
        system_chirho.set_exact_chirho(&b_chirho, 4.0);
        system_chirho.run_chirho();

        let c_value_chirho = system_chirho.get_chirho(&c_chirho);
        let c_interval_chirho = c_value_chirho.as_interval_chirho().unwrap();
        assert!((c_interval_chirho.lo_chirho - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_backward_propagation_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");
        let c_chirho = system_chirho.make_cell_chirho("c");

        system_chirho.add_adder_chirho(&a_chirho, &b_chirho, &c_chirho);

        system_chirho.set_exact_chirho(&a_chirho, 3.0);
        system_chirho.set_exact_chirho(&c_chirho, 7.0);
        system_chirho.run_chirho();

        let b_value_chirho = system_chirho.get_chirho(&b_chirho);
        let b_interval_chirho = b_value_chirho.as_interval_chirho().unwrap();
        assert!((b_interval_chirho.lo_chirho - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_temperature_conversion_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let c_chirho = system_chirho.make_cell_chirho("celsius");
        let f_chirho = system_chirho.make_cell_chirho("fahrenheit");
        let nine_fifths_chirho = system_chirho.make_cell_chirho("nine_fifths");
        let thirty_two_chirho = system_chirho.make_cell_chirho("thirty_two");
        let product_chirho = system_chirho.make_cell_chirho("product");

        system_chirho.set_exact_chirho(&nine_fifths_chirho, 1.8);
        system_chirho.set_exact_chirho(&thirty_two_chirho, 32.0);

        system_chirho.add_multiplier_chirho(&c_chirho, &nine_fifths_chirho, &product_chirho);
        system_chirho.add_adder_chirho(&product_chirho, &thirty_two_chirho, &f_chirho);

        // Forward: C=100 → F=212
        system_chirho.set_exact_chirho(&c_chirho, 100.0);
        system_chirho.run_chirho();

        let f_value_chirho = system_chirho.get_chirho(&f_chirho);
        let f_interval_chirho = f_value_chirho.as_interval_chirho().unwrap();
        assert!((f_interval_chirho.lo_chirho - 212.0).abs() < 1e-10);
    }

    #[test]
    fn test_contradiction_detection_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let x_chirho = system_chirho.make_cell_chirho("x");

        system_chirho.set_interval_chirho(&x_chirho, 0.0, 10.0);
        system_chirho.set_interval_chirho(&x_chirho, 20.0, 30.0);
        system_chirho.run_chirho();

        assert!(system_chirho.has_contradiction_chirho());
        assert!(system_chirho
            .contradicted_cells_chirho()
            .contains(&"x".to_string()));
    }

    #[test]
    fn test_pythagorean_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");
        let c_chirho = system_chirho.make_cell_chirho("c");

        system_chirho.add_pythagorean_chirho(&a_chirho, &b_chirho, &c_chirho);

        // 3² + 4² = 5²
        system_chirho.set_exact_chirho(&a_chirho, 3.0);
        system_chirho.set_exact_chirho(&b_chirho, 4.0);
        system_chirho.run_chirho();

        // c should be constrained by c² = 25
        let c_value_chirho = system_chirho.get_chirho(&c_chirho);
        // c could be ±5, but typically we'd constrain to positive
        assert!(!c_value_chirho.is_nothing_chirho());
    }
}
