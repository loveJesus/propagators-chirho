// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Python bindings for propagator networks via PyO3.
//!
//! This module provides Python-friendly bindings for using propagator
//! networks from Python code.
//!
//! # Feature Flag
//!
//! This module requires the `python` feature:
//!
//! ```toml
//! [dependencies]
//! propagators-chirho = { version = "0.1", features = ["python"] }
//! ```
//!
//! # Building the Python Module
//!
//! Using maturin:
//! ```bash
//! pip install maturin
//! maturin develop --features python
//! ```
//!
//! # Python Usage
//!
//! ```python
//! from propagators_chirho import PropagatorNetworkChirho, IntervalChirho
//!
//! # Create a network
//! net_chirho = PropagatorNetworkChirho()
//!
//! # Create cells
//! a_chirho = net_chirho.make_cell_chirho()
//! b_chirho = net_chirho.make_cell_chirho()
//! c_chirho = net_chirho.make_cell_chirho()
//!
//! # Add constraint: a + b = c
//! net_chirho.add_adder_chirho(a_chirho, b_chirho, c_chirho)
//!
//! # Set values
//! net_chirho.set_exact_chirho(a_chirho, 3.0)
//! net_chirho.set_exact_chirho(b_chirho, 4.0)
//!
//! # Propagate
//! net_chirho.propagate_chirho()
//!
//! # Get result
//! print(net_chirho.get_exact_chirho(c_chirho))  # 7.0
//! ```

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::arena_chirho::{ArenaNetworkChirho, CellIdChirho};
use crate::interval_chirho::NumericInfoChirho;

/// A propagator network for constraint propagation.
///
/// This is the main entry point for using propagators from Python.
#[pyclass(name = "PropagatorNetworkChirho")]
pub struct PyNetworkChirho {
    network_chirho: ArenaNetworkChirho,
}

#[pymethods]
impl PyNetworkChirho {
    /// Creates a new empty propagator network.
    #[new]
    pub fn new_chirho() -> Self {
        Self {
            network_chirho: ArenaNetworkChirho::new_chirho(),
        }
    }

    /// Creates a new cell and returns its ID.
    ///
    /// Returns:
    ///     int: The cell's unique identifier
    pub fn make_cell_chirho(&mut self) -> usize {
        self.network_chirho.make_cell_chirho().index_chirho()
    }

    /// Sets a cell to an exact value.
    ///
    /// Args:
    ///     cell_id: The cell's ID
    ///     value: The exact value to set
    pub fn set_exact_chirho(&mut self, cell_id_chirho: usize, value_chirho: f64) {
        let cell_chirho = CellIdChirho::from_index_chirho(cell_id_chirho);
        self.network_chirho
            .set_exact_chirho(cell_chirho, value_chirho);
    }

    /// Sets a cell to an interval [lo, hi].
    ///
    /// Args:
    ///     cell_id: The cell's ID
    ///     lo: Lower bound
    ///     hi: Upper bound
    pub fn set_interval_chirho(&mut self, cell_id_chirho: usize, lo_chirho: f64, hi_chirho: f64) {
        let cell_chirho = CellIdChirho::from_index_chirho(cell_id_chirho);
        self.network_chirho
            .set_interval_chirho(cell_chirho, lo_chirho, hi_chirho);
    }

    /// Adds an addition constraint: a + b = c
    ///
    /// Args:
    ///     a: First operand cell ID
    ///     b: Second operand cell ID
    ///     c: Result cell ID
    pub fn add_adder_chirho(&mut self, a_chirho: usize, b_chirho: usize, c_chirho: usize) {
        let a_cell_chirho = CellIdChirho::from_index_chirho(a_chirho);
        let b_cell_chirho = CellIdChirho::from_index_chirho(b_chirho);
        let c_cell_chirho = CellIdChirho::from_index_chirho(c_chirho);
        self.network_chirho
            .add_adder_chirho(a_cell_chirho, b_cell_chirho, c_cell_chirho);
    }

    /// Adds a subtraction constraint via addition: a - b = c means a = b + c
    ///
    /// Args:
    ///     a: Minuend cell ID
    ///     b: Subtrahend cell ID
    ///     c: Difference cell ID
    pub fn add_subtractor_chirho(&mut self, a_chirho: usize, b_chirho: usize, c_chirho: usize) {
        // a - b = c is equivalent to b + c = a
        let a_cell_chirho = CellIdChirho::from_index_chirho(a_chirho);
        let b_cell_chirho = CellIdChirho::from_index_chirho(b_chirho);
        let c_cell_chirho = CellIdChirho::from_index_chirho(c_chirho);
        self.network_chirho
            .add_adder_chirho(b_cell_chirho, c_cell_chirho, a_cell_chirho);
    }

    /// Adds a multiplication constraint: a * b = c
    ///
    /// Args:
    ///     a: First factor cell ID
    ///     b: Second factor cell ID
    ///     c: Product cell ID
    pub fn add_multiplier_chirho(&mut self, a_chirho: usize, b_chirho: usize, c_chirho: usize) {
        let a_cell_chirho = CellIdChirho::from_index_chirho(a_chirho);
        let b_cell_chirho = CellIdChirho::from_index_chirho(b_chirho);
        let c_cell_chirho = CellIdChirho::from_index_chirho(c_chirho);
        self.network_chirho
            .add_multiplier_chirho(a_cell_chirho, b_cell_chirho, c_cell_chirho);
    }

    /// Adds a square constraint: a² = b
    ///
    /// Args:
    ///     a: Base cell ID
    ///     b: Square cell ID
    pub fn add_squarer_chirho(&mut self, a_chirho: usize, b_chirho: usize) {
        let a_cell_chirho = CellIdChirho::from_index_chirho(a_chirho);
        let b_cell_chirho = CellIdChirho::from_index_chirho(b_chirho);
        self.network_chirho
            .add_squarer_chirho(a_cell_chirho, b_cell_chirho);
    }

    /// Runs propagation until fixpoint.
    pub fn propagate_chirho(&mut self) {
        self.network_chirho.propagate_chirho();
    }

    /// Gets the exact value of a cell (if it's a single point).
    ///
    /// Args:
    ///     cell_id: The cell's ID
    ///
    /// Returns:
    ///     float | None: The exact value, or None if not exact
    pub fn get_exact_chirho(&self, cell_id_chirho: usize) -> Option<f64> {
        let cell_chirho = CellIdChirho::from_index_chirho(cell_id_chirho);
        self.network_chirho.get_exact_chirho(cell_chirho)
    }

    /// Gets the interval bounds of a cell.
    ///
    /// Args:
    ///     cell_id: The cell's ID
    ///
    /// Returns:
    ///     tuple[float, float] | None: (lo, hi) bounds, or None if no interval
    pub fn get_interval_chirho(&self, cell_id_chirho: usize) -> Option<(f64, f64)> {
        let cell_chirho = CellIdChirho::from_index_chirho(cell_id_chirho);
        self.network_chirho
            .get_interval_chirho(cell_chirho)
            .map(|iv_chirho| (iv_chirho.lo_chirho, iv_chirho.hi_chirho))
    }

    /// Checks if a cell is in contradiction.
    ///
    /// Args:
    ///     cell_id: The cell's ID
    ///
    /// Returns:
    ///     bool: True if the cell has contradictory information
    pub fn is_contradiction_chirho(&self, cell_id_chirho: usize) -> bool {
        let cell_chirho = CellIdChirho::from_index_chirho(cell_id_chirho);
        matches!(
            self.network_chirho.get_content_chirho(cell_chirho),
            NumericInfoChirho::ContradictionChirho
        )
    }

    /// Checks if any cell in the network has a contradiction.
    ///
    /// Returns:
    ///     bool: True if any cell is contradictory
    pub fn has_contradiction_chirho(&self) -> bool {
        self.network_chirho.has_contradiction_chirho()
    }

    /// Returns the number of cells in the network.
    ///
    /// Returns:
    ///     int: Number of cells
    pub fn cell_count_chirho(&self) -> usize {
        self.network_chirho.cell_count_chirho()
    }

    /// Returns the number of propagators in the network.
    ///
    /// Returns:
    ///     int: Number of propagators
    pub fn propagator_count_chirho(&self) -> usize {
        self.network_chirho.propagator_count_chirho()
    }

    /// Returns the number of propagation steps performed.
    ///
    /// Returns:
    ///     int: Number of propagation steps
    pub fn propagation_count_chirho(&self) -> usize {
        self.network_chirho.propagation_count_chirho()
    }

    /// String representation of the network.
    fn __repr__(&self) -> String {
        format!(
            "PropagatorNetworkChirho(cells_chirho={}, propagators_chirho={}, propagations_chirho={})",
            self.network_chirho.cell_count_chirho(),
            self.network_chirho.propagator_count_chirho(),
            self.network_chirho.propagation_count_chirho()
        )
    }
}

/// An interval representing partial numeric information.
///
/// An interval [lo, hi] represents all values x where lo <= x <= hi.
#[pyclass(name = "IntervalChirho")]
#[derive(Clone)]
pub struct PyIntervalChirho {
    /// Lower bound
    #[pyo3(get)]
    pub lo_chirho: f64,
    /// Upper bound
    #[pyo3(get)]
    pub hi_chirho: f64,
}

#[pymethods]
impl PyIntervalChirho {
    /// Creates a new interval.
    ///
    /// Args:
    ///     lo: Lower bound
    ///     hi: Upper bound
    ///
    /// # Errors
    ///
    /// Returns `PyValueError` if `lo > hi`.
    #[new]
    pub fn new_chirho(lo_chirho: f64, hi_chirho: f64) -> PyResult<Self> {
        if lo_chirho > hi_chirho {
            return Err(PyValueError::new_err("lo must be <= hi"));
        }
        Ok(Self {
            lo_chirho,
            hi_chirho,
        })
    }

    /// Creates an exact value interval.
    ///
    /// Args:
    ///     value: The exact value
    ///
    /// Returns:
    ///     Interval: An interval [value, value]
    #[staticmethod]
    pub fn exact_chirho(value_chirho: f64) -> Self {
        Self {
            lo_chirho: value_chirho,
            hi_chirho: value_chirho,
        }
    }

    /// Returns the width of the interval (hi - lo).
    pub fn width_chirho(&self) -> f64 {
        self.hi_chirho - self.lo_chirho
    }

    /// Returns the midpoint of the interval.
    pub fn midpoint_chirho(&self) -> f64 {
        (self.lo_chirho + self.hi_chirho) / 2.0
    }

    /// Returns True if this is an exact value (lo == hi).
    pub fn is_exact_chirho(&self) -> bool {
        (self.hi_chirho - self.lo_chirho).abs() < 1e-10
    }

    /// Returns True if the interval is empty (contradiction).
    pub fn is_empty_chirho(&self) -> bool {
        self.lo_chirho > self.hi_chirho
    }

    /// Returns True if the interval contains a value.
    pub fn contains_chirho(&self, value_chirho: f64) -> bool {
        self.lo_chirho <= value_chirho && value_chirho <= self.hi_chirho
    }

    /// Adds two intervals.
    pub fn add_chirho(&self, other_chirho: &PyIntervalChirho) -> PyIntervalChirho {
        PyIntervalChirho {
            lo_chirho: self.lo_chirho + other_chirho.lo_chirho,
            hi_chirho: self.hi_chirho + other_chirho.hi_chirho,
        }
    }

    /// Subtracts two intervals.
    pub fn sub_chirho(&self, other_chirho: &PyIntervalChirho) -> PyIntervalChirho {
        PyIntervalChirho {
            lo_chirho: self.lo_chirho - other_chirho.hi_chirho,
            hi_chirho: self.hi_chirho - other_chirho.lo_chirho,
        }
    }

    /// Multiplies two intervals.
    pub fn mul_chirho(&self, other_chirho: &PyIntervalChirho) -> PyIntervalChirho {
        let products_chirho = [
            self.lo_chirho * other_chirho.lo_chirho,
            self.lo_chirho * other_chirho.hi_chirho,
            self.hi_chirho * other_chirho.lo_chirho,
            self.hi_chirho * other_chirho.hi_chirho,
        ];

        let lo_chirho = products_chirho
            .iter()
            .cloned()
            .fold(f64::INFINITY, f64::min);
        let hi_chirho = products_chirho
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        PyIntervalChirho {
            lo_chirho,
            hi_chirho,
        }
    }

    /// Intersects two intervals.
    pub fn intersect_chirho(&self, other_chirho: &PyIntervalChirho) -> PyIntervalChirho {
        PyIntervalChirho {
            lo_chirho: self.lo_chirho.max(other_chirho.lo_chirho),
            hi_chirho: self.hi_chirho.min(other_chirho.hi_chirho),
        }
    }

    /// String representation.
    fn __repr__(&self) -> String {
        if self.is_exact_chirho() {
            format!("IntervalChirho.exact_chirho({})", self.lo_chirho)
        } else {
            format!("IntervalChirho({}, {})", self.lo_chirho, self.hi_chirho)
        }
    }

    /// Support for + operator
    fn __add__(&self, other_chirho: &PyIntervalChirho) -> PyIntervalChirho {
        self.add_chirho(other_chirho)
    }

    /// Support for - operator
    fn __sub__(&self, other_chirho: &PyIntervalChirho) -> PyIntervalChirho {
        self.sub_chirho(other_chirho)
    }

    /// Support for * operator
    fn __mul__(&self, other_chirho: &PyIntervalChirho) -> PyIntervalChirho {
        self.mul_chirho(other_chirho)
    }

    /// Support for & operator (intersection)
    fn __and__(&self, other_chirho: &PyIntervalChirho) -> PyIntervalChirho {
        self.intersect_chirho(other_chirho)
    }
}

/// Initialize the Python module.
#[pymodule]
fn propagators_chirho(m_chirho: &Bound<'_, PyModule>) -> PyResult<()> {
    m_chirho.add_class::<PyNetworkChirho>()?;
    m_chirho.add_class::<PyIntervalChirho>()?;
    Ok(())
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_py_network_chirho() {
        let mut network_chirho = PyNetworkChirho::new_chirho();

        let a_chirho = network_chirho.make_cell_chirho();
        let b_chirho = network_chirho.make_cell_chirho();
        let c_chirho = network_chirho.make_cell_chirho();

        network_chirho.add_adder_chirho(a_chirho, b_chirho, c_chirho);
        network_chirho.set_exact_chirho(a_chirho, 3.0);
        network_chirho.set_exact_chirho(b_chirho, 4.0);
        network_chirho.propagate_chirho();

        assert_eq!(network_chirho.get_exact_chirho(c_chirho), Some(7.0));
    }

    #[test]
    fn test_py_interval_chirho() {
        let a_chirho = PyIntervalChirho::new_chirho(1.0, 5.0).unwrap();
        let b_chirho = PyIntervalChirho::new_chirho(2.0, 4.0).unwrap();

        let sum_chirho = a_chirho.add_chirho(&b_chirho);
        assert_eq!(sum_chirho.lo_chirho, 3.0);
        assert_eq!(sum_chirho.hi_chirho, 9.0);
    }
}
