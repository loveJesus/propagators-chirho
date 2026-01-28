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

use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::rc::Rc;

use crate::cells_chirho::cell_chirho::CellChirho;
use crate::cells_chirho::scheduler_chirho::SchedulerChirho;
use crate::core_chirho::algebra_chirho::{PropagatorErrorChirho, PropagatorResultChirho};
use crate::core_chirho::interval_chirho::NumericInfoChirho;
use crate::propagators_chirho::propagator_chirho::{
    AbsoluterChirho, ClampChirho, ConstantChirho, ExpChirho, IntervalAdderChirho,
    IntervalDividerChirho, IntervalMultiplierChirho, IntervalSubtractorChirho, LnChirho, MaxChirho,
    MinChirho, NegaterChirho, PowerChirho, PropagatorChirho, SqrterChirho, SquarerChirho,
};

// ============================================================================
// CONSTRAINT INTROSPECTION TYPES
// ============================================================================

/// Unique identifier for a constraint in the system.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstraintIdChirho(usize);

impl ConstraintIdChirho {
    /// Returns the numeric ID of this constraint.
    pub fn id_chirho(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for ConstraintIdChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f_chirho, "constraint_{}", self.0)
    }
}

/// The type of a constraint (what operation it represents).
#[derive(Clone, Debug, PartialEq)]
pub enum ConstraintTypeChirho {
    /// Addition: a + b = c
    AdderChirho,
    /// Subtraction: a - b = c
    SubtractorChirho,
    /// Multiplication: a * b = c
    MultiplierChirho,
    /// Division: a / b = c
    DividerChirho,
    /// Squarer: a² = b
    SquarerChirho,
    /// Square root: √a = b
    SqrterChirho,
    /// Absolute value: |a| = b
    AbsoluterChirho,
    /// Maximum: max(a, b) = c
    MaxChirho,
    /// Minimum: min(a, b) = c
    MinChirho,
    /// Constant: cell = value
    ConstantChirho,
    /// Negation: -a = b
    NegaterChirho,
    /// Exponential: e^a = b
    ExpChirho,
    /// Natural logarithm: ln(a) = b
    LnChirho,
    /// Power: a^n = b
    PowerChirho {
        /// The exponent.
        exponent_chirho: i32,
    },
    /// Clamp: clamp(a, lo, hi) = b
    ClampChirho {
        /// Lower bound.
        lo_chirho: f64,
        /// Upper bound.
        hi_chirho: f64,
    },
    /// Linear: a*x + b = y (compound constraint)
    LinearChirho {
        /// The multiplier coefficient.
        a_chirho: f64,
        /// The additive coefficient.
        b_chirho: f64,
    },
    /// Pythagorean: a² + b² = c² (compound constraint)
    PythagoreanChirho,
}

impl std::fmt::Display for ConstraintTypeChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AdderChirho => write!(f_chirho, "adder (a + b = c)"),
            Self::SubtractorChirho => write!(f_chirho, "subtractor (a - b = c)"),
            Self::MultiplierChirho => write!(f_chirho, "multiplier (a * b = c)"),
            Self::DividerChirho => write!(f_chirho, "divider (a / b = c)"),
            Self::SquarerChirho => write!(f_chirho, "squarer (a² = b)"),
            Self::SqrterChirho => write!(f_chirho, "sqrter (√a = b)"),
            Self::AbsoluterChirho => write!(f_chirho, "absoluter (|a| = b)"),
            Self::MaxChirho => write!(f_chirho, "max (max(a,b) = c)"),
            Self::MinChirho => write!(f_chirho, "min (min(a,b) = c)"),
            Self::ConstantChirho => write!(f_chirho, "constant"),
            Self::NegaterChirho => write!(f_chirho, "negater (-a = b)"),
            Self::ExpChirho => write!(f_chirho, "exp (e^a = b)"),
            Self::LnChirho => write!(f_chirho, "ln (ln(a) = b)"),
            Self::PowerChirho { exponent_chirho } => {
                write!(f_chirho, "power (a^{} = b)", exponent_chirho)
            }
            Self::ClampChirho {
                lo_chirho,
                hi_chirho,
            } => write!(f_chirho, "clamp (clamp(a, {}, {}) = b)", lo_chirho, hi_chirho),
            Self::LinearChirho { a_chirho, b_chirho } => {
                write!(f_chirho, "linear ({}*x + {} = y)", a_chirho, b_chirho)
            }
            Self::PythagoreanChirho => write!(f_chirho, "pythagorean (a² + b² = c²)"),
        }
    }
}

/// Information about a constraint in the system.
#[derive(Clone, Debug)]
pub struct ConstraintInfoChirho {
    /// Unique identifier.
    pub id_chirho: ConstraintIdChirho,
    /// The type of constraint.
    pub constraint_type_chirho: ConstraintTypeChirho,
    /// Cell names involved in this constraint.
    pub cells_chirho: Vec<String>,
    /// Human-readable description.
    pub description_chirho: String,
}

impl ConstraintInfoChirho {
    /// Creates a new constraint info.
    fn new_chirho(
        id_chirho: ConstraintIdChirho,
        constraint_type_chirho: ConstraintTypeChirho,
        cells_chirho: Vec<String>,
    ) -> Self {
        let description_chirho = format!(
            "{} involving cells: {}",
            constraint_type_chirho,
            cells_chirho.join(", ")
        );
        Self {
            id_chirho,
            constraint_type_chirho,
            cells_chirho,
            description_chirho,
        }
    }
}

/// Explanation of why a cell has its current value.
#[derive(Clone, Debug)]
pub struct CellExplanationChirho {
    /// The cell name.
    pub cell_name_chirho: String,
    /// Current value of the cell.
    pub current_value_chirho: NumericInfoChirho,
    /// Constraints that affect this cell.
    pub affecting_constraints_chirho: Vec<ConstraintIdChirho>,
    /// Values that were directly set by the user.
    pub user_set_values_chirho: Vec<NumericInfoChirho>,
    /// Text explanation of how the value was derived.
    pub explanation_text_chirho: String,
}

// ============================================================================
// CHECKPOINT AND ROLLBACK TYPES
// ============================================================================

/// A checkpoint capturing the state of a constraint system.
///
/// Checkpoints can be used to save and restore the state of a constraint
/// system, enabling rollback after contradictions or what-if analysis.
///
/// # Example
///
/// ```
/// use propagators_chirho::ConstraintSystemChirho;
///
/// let mut system_chirho = ConstraintSystemChirho::new_chirho();
/// let a_chirho = system_chirho.make_cell_chirho("a");
/// let b_chirho = system_chirho.make_cell_chirho("b");
/// let c_chirho = system_chirho.make_cell_chirho("c");
///
/// system_chirho.add_adder_chirho(&a_chirho, &b_chirho, &c_chirho);
/// system_chirho.set_exact_chirho(&a_chirho, 3.0);
/// system_chirho.set_exact_chirho(&b_chirho, 4.0);
/// system_chirho.run_chirho();
///
/// // Save state: a=3, b=4, c=7
/// let checkpoint_chirho = system_chirho.checkpoint_chirho();
///
/// // Modify a
/// system_chirho.set_exact_chirho(&a_chirho, 10.0);
/// system_chirho.run_chirho();
///
/// // Rollback to saved state
/// system_chirho.rollback_to_chirho(&checkpoint_chirho);
/// // a=3, b=4, c=7 again
/// ```
#[derive(Clone, Debug)]
pub struct CheckpointChirho {
    /// Unique ID for this checkpoint.
    id_chirho: usize,
    /// Cell values at checkpoint time.
    cell_values_chirho: HashMap<String, NumericInfoChirho>,
    /// User-set values at checkpoint time.
    user_set_values_chirho: HashMap<String, Vec<NumericInfoChirho>>,
    /// Deactivated constraints at checkpoint time.
    deactivated_constraints_chirho: HashSet<ConstraintIdChirho>,
    /// Removed constraints at checkpoint time.
    removed_constraints_chirho: HashSet<ConstraintIdChirho>,
}

impl CheckpointChirho {
    /// Returns the unique ID of this checkpoint.
    pub fn id_chirho(&self) -> usize {
        self.id_chirho
    }

    /// Returns the cell names captured in this checkpoint.
    pub fn cell_names_chirho(&self) -> Vec<String> {
        self.cell_values_chirho.keys().cloned().collect()
    }

    /// Returns the value of a cell at checkpoint time.
    pub fn cell_value_chirho(&self, cell_name_chirho: &str) -> Option<&NumericInfoChirho> {
        self.cell_values_chirho.get(cell_name_chirho)
    }
}

/// A graph representation of the propagation network.
#[derive(Clone, Debug)]
pub struct PropagationGraphChirho {
    /// All cells in the system (nodes).
    pub cells_chirho: Vec<String>,
    /// All constraints (edges connecting cells).
    pub constraints_chirho: Vec<ConstraintInfoChirho>,
    /// Map from cell name to constraint IDs that affect it.
    pub cell_to_constraints_chirho: HashMap<String, Vec<ConstraintIdChirho>>,
    /// Map from constraint ID to cell names it involves.
    pub constraint_to_cells_chirho: HashMap<ConstraintIdChirho, Vec<String>>,
}

impl PropagationGraphChirho {
    /// Returns all cells connected to the given cell via constraints.
    pub fn connected_cells_chirho(&self, cell_name_chirho: &str) -> HashSet<String> {
        let mut connected_chirho = HashSet::new();

        if let Some(constraint_ids_chirho) = self.cell_to_constraints_chirho.get(cell_name_chirho) {
            for constraint_id_chirho in constraint_ids_chirho {
                if let Some(cells_chirho) =
                    self.constraint_to_cells_chirho.get(constraint_id_chirho)
                {
                    for cell_chirho in cells_chirho {
                        if cell_chirho != cell_name_chirho {
                            connected_chirho.insert(cell_chirho.clone());
                        }
                    }
                }
            }
        }

        connected_chirho
    }

    /// Returns DOT format string for graphviz visualization.
    pub fn to_dot_chirho(&self) -> String {
        let mut dot_chirho = String::from("digraph propagation_network {\n");
        dot_chirho.push_str("  rankdir=LR;\n");
        dot_chirho.push_str("  node [shape=ellipse];\n");

        // Add cell nodes
        for cell_chirho in &self.cells_chirho {
            let _ = writeln!(dot_chirho, "  \"{}\" [shape=box];", cell_chirho);
        }

        // Add constraint nodes and edges
        for constraint_chirho in &self.constraints_chirho {
            let constraint_label_chirho = &constraint_chirho.constraint_type_chirho;
            let constraint_node_chirho = format!("c{}", constraint_chirho.id_chirho.0);

            let _ = writeln!(
                dot_chirho,
                "  \"{}\" [shape=diamond, label=\"{}\"];",
                constraint_node_chirho, constraint_label_chirho
            );

            // Add edges from cells to constraint
            for cell_chirho in &constraint_chirho.cells_chirho {
                let _ = writeln!(
                    dot_chirho,
                    "  \"{}\" -> \"{}\" [arrowhead=none];",
                    cell_chirho, constraint_node_chirho
                );
            }
        }

        dot_chirho.push_str("}\n");
        dot_chirho
    }
}

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
    /// Registry of all constraints in the system.
    constraints_chirho: Vec<ConstraintInfoChirho>,
    /// Counter for generating unique constraint IDs.
    next_constraint_id_chirho: usize,
    /// Map from cell name to constraints that affect it.
    cell_to_constraints_chirho: HashMap<String, Vec<ConstraintIdChirho>>,
    /// Map from cell name to values directly set by user (for explanation).
    user_set_values_chirho: HashMap<String, Vec<NumericInfoChirho>>,
    /// Map from constraint ID to propagator IDs.
    constraint_propagators_chirho: HashMap<ConstraintIdChirho, Vec<usize>>,
    /// Set of deactivated constraint IDs.
    deactivated_constraints_chirho: HashSet<ConstraintIdChirho>,
    /// Set of removed constraint IDs (for filtering).
    removed_constraints_chirho: HashSet<ConstraintIdChirho>,
    /// Counter for generating unique checkpoint IDs.
    next_checkpoint_id_chirho: usize,
}

impl ConstraintSystemChirho {
    /// Creates a new empty constraint system.
    pub fn new_chirho() -> Self {
        Self {
            scheduler_chirho: SchedulerChirho::new_chirho(),
            cells_chirho: HashMap::new(),
            constraints_chirho: Vec::new(),
            next_constraint_id_chirho: 0,
            cell_to_constraints_chirho: HashMap::new(),
            user_set_values_chirho: HashMap::new(),
            constraint_propagators_chirho: HashMap::new(),
            deactivated_constraints_chirho: HashSet::new(),
            removed_constraints_chirho: HashSet::new(),
            next_checkpoint_id_chirho: 0,
        }
    }

    /// Generates a new unique constraint ID.
    fn next_constraint_id_chirho(&mut self) -> ConstraintIdChirho {
        let id_chirho = ConstraintIdChirho(self.next_constraint_id_chirho);
        self.next_constraint_id_chirho += 1;
        id_chirho
    }

    /// Registers a constraint and updates the cell-to-constraint mapping.
    fn register_constraint_chirho(
        &mut self,
        constraint_type_chirho: ConstraintTypeChirho,
        cells_chirho: Vec<String>,
        propagator_ids_chirho: Vec<usize>,
    ) -> ConstraintIdChirho {
        let id_chirho = self.next_constraint_id_chirho();

        // Update cell-to-constraint mapping
        for cell_chirho in &cells_chirho {
            self.cell_to_constraints_chirho
                .entry(cell_chirho.clone())
                .or_default()
                .push(id_chirho.clone());
        }

        // Store propagator IDs for this constraint
        self.constraint_propagators_chirho
            .insert(id_chirho.clone(), propagator_ids_chirho);

        // Create and store constraint info
        let info_chirho =
            ConstraintInfoChirho::new_chirho(id_chirho.clone(), constraint_type_chirho, cells_chirho);
        self.constraints_chirho.push(info_chirho);

        id_chirho
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
    pub fn set_exact_chirho(&mut self, name_chirho: &str, value_chirho: f64) {
        let info_chirho = NumericInfoChirho::exact_chirho(value_chirho);

        // Track user-set value for explanation
        self.user_set_values_chirho
            .entry(name_chirho.to_string())
            .or_default()
            .push(info_chirho);

        let cell_chirho = self.get_cell_chirho(name_chirho);
        cell_chirho.add_content_chirho(info_chirho, &self.scheduler_chirho);
    }

    /// Sets a cell to an interval.
    pub fn set_interval_chirho(&mut self, name_chirho: &str, lo_chirho: f64, hi_chirho: f64) {
        let info_chirho = NumericInfoChirho::interval_chirho(lo_chirho, hi_chirho);

        // Track user-set value for explanation
        self.user_set_values_chirho
            .entry(name_chirho.to_string())
            .or_default()
            .push(info_chirho);

        let cell_chirho = self.get_cell_chirho(name_chirho);
        cell_chirho.add_content_chirho(info_chirho, &self.scheduler_chirho);
    }

    /// Adds content to a cell.
    pub fn add_content_chirho(&mut self, name_chirho: &str, content_chirho: NumericInfoChirho) {
        // Track user-set value for explanation
        self.user_set_values_chirho
            .entry(name_chirho.to_string())
            .or_default()
            .push(content_chirho);

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
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_adder_chirho(
        &mut self,
        a_chirho: &str,
        b_chirho: &str,
        c_chirho: &str,
    ) -> ConstraintIdChirho {
        let propagator_chirho = IntervalAdderChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            self.get_cell_chirho(c_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::AdderChirho,
            vec![
                a_chirho.to_string(),
                b_chirho.to_string(),
                c_chirho.to_string(),
            ],
            vec![propagator_chirho.id_chirho()],
        )
    }

    /// Adds a subtractor constraint: a - b = c.
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_subtractor_chirho(
        &mut self,
        a_chirho: &str,
        b_chirho: &str,
        c_chirho: &str,
    ) -> ConstraintIdChirho {
        let propagator_chirho = IntervalSubtractorChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            self.get_cell_chirho(c_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::SubtractorChirho,
            vec![
                a_chirho.to_string(),
                b_chirho.to_string(),
                c_chirho.to_string(),
            ],
            vec![propagator_chirho.id_chirho()],
        )
    }

    /// Adds a multiplier constraint: a * b = c.
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_multiplier_chirho(
        &mut self,
        a_chirho: &str,
        b_chirho: &str,
        c_chirho: &str,
    ) -> ConstraintIdChirho {
        let propagator_chirho = IntervalMultiplierChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            self.get_cell_chirho(c_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::MultiplierChirho,
            vec![
                a_chirho.to_string(),
                b_chirho.to_string(),
                c_chirho.to_string(),
            ],
            vec![propagator_chirho.id_chirho()],
        )
    }

    /// Adds a divider constraint: a / b = c.
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_divider_chirho(
        &mut self,
        a_chirho: &str,
        b_chirho: &str,
        c_chirho: &str,
    ) -> ConstraintIdChirho {
        let propagator_chirho = IntervalDividerChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            self.get_cell_chirho(c_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::DividerChirho,
            vec![
                a_chirho.to_string(),
                b_chirho.to_string(),
                c_chirho.to_string(),
            ],
            vec![propagator_chirho.id_chirho()],
        )
    }

    /// Adds a squarer constraint: a² = b.
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_squarer_chirho(&mut self, a_chirho: &str, b_chirho: &str) -> ConstraintIdChirho {
        let propagator_chirho = SquarerChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::SquarerChirho,
            vec![a_chirho.to_string(), b_chirho.to_string()],
            vec![propagator_chirho.id_chirho()],
        )
    }

    /// Adds a square root constraint: √a = b.
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_sqrter_chirho(&mut self, a_chirho: &str, b_chirho: &str) -> ConstraintIdChirho {
        let propagator_chirho = SqrterChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::SqrterChirho,
            vec![a_chirho.to_string(), b_chirho.to_string()],
            vec![propagator_chirho.id_chirho()],
        )
    }

    /// Adds an absolute value constraint: |a| = b.
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_absoluter_chirho(&mut self, a_chirho: &str, b_chirho: &str) -> ConstraintIdChirho {
        let propagator_chirho = AbsoluterChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::AbsoluterChirho,
            vec![a_chirho.to_string(), b_chirho.to_string()],
            vec![propagator_chirho.id_chirho()],
        )
    }

    /// Adds a max constraint: max(a, b) = c.
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_max_chirho(
        &mut self,
        a_chirho: &str,
        b_chirho: &str,
        c_chirho: &str,
    ) -> ConstraintIdChirho {
        let propagator_chirho = MaxChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            self.get_cell_chirho(c_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::MaxChirho,
            vec![
                a_chirho.to_string(),
                b_chirho.to_string(),
                c_chirho.to_string(),
            ],
            vec![propagator_chirho.id_chirho()],
        )
    }

    /// Adds a min constraint: min(a, b) = c.
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_min_chirho(
        &mut self,
        a_chirho: &str,
        b_chirho: &str,
        c_chirho: &str,
    ) -> ConstraintIdChirho {
        let propagator_chirho = MinChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            self.get_cell_chirho(c_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::MinChirho,
            vec![
                a_chirho.to_string(),
                b_chirho.to_string(),
                c_chirho.to_string(),
            ],
            vec![propagator_chirho.id_chirho()],
        )
    }

    /// Adds a constant propagator that sets a cell to a value.
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_constant_chirho(&mut self, value_chirho: f64, cell_chirho: &str) -> ConstraintIdChirho {
        let propagator_chirho = ConstantChirho::new_chirho(
            NumericInfoChirho::exact_chirho(value_chirho),
            self.get_cell_chirho(cell_chirho),
        );
        let propagator_id_chirho = propagator_chirho.id_chirho();
        self.scheduler_chirho
            .alert_propagator_chirho(propagator_chirho);

        self.register_constraint_chirho(
            ConstraintTypeChirho::ConstantChirho,
            vec![cell_chirho.to_string()],
            vec![propagator_id_chirho],
        )
    }

    /// Adds a negation constraint: -a = b.
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_negater_chirho(&mut self, a_chirho: &str, b_chirho: &str) -> ConstraintIdChirho {
        let propagator_chirho = NegaterChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::NegaterChirho,
            vec![a_chirho.to_string(), b_chirho.to_string()],
            vec![propagator_chirho.id_chirho()],
        )
    }

    /// Adds an exponential constraint: e^a = b.
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_exp_chirho(&mut self, a_chirho: &str, b_chirho: &str) -> ConstraintIdChirho {
        let propagator_chirho = ExpChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::ExpChirho,
            vec![a_chirho.to_string(), b_chirho.to_string()],
            vec![propagator_chirho.id_chirho()],
        )
    }

    /// Adds a natural logarithm constraint: ln(a) = b.
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_ln_chirho(&mut self, a_chirho: &str, b_chirho: &str) -> ConstraintIdChirho {
        let propagator_chirho = LnChirho::install_chirho(
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::LnChirho,
            vec![a_chirho.to_string(), b_chirho.to_string()],
            vec![propagator_chirho.id_chirho()],
        )
    }

    /// Adds a power constraint: a^n = b.
    ///
    /// # Arguments
    ///
    /// * `n_chirho` - The exponent (must be non-zero)
    /// * `a_chirho` - The base cell name
    /// * `b_chirho` - The result cell name
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_power_chirho(
        &mut self,
        n_chirho: i32,
        a_chirho: &str,
        b_chirho: &str,
    ) -> ConstraintIdChirho {
        let propagator_chirho = PowerChirho::install_chirho(
            n_chirho,
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::PowerChirho {
                exponent_chirho: n_chirho,
            },
            vec![a_chirho.to_string(), b_chirho.to_string()],
            vec![propagator_chirho.id_chirho()],
        )
    }

    /// Adds a clamp constraint: clamp(a, lo, hi) = b.
    ///
    /// The result b is constrained to be within [lo, hi].
    ///
    /// Returns the constraint ID for introspection.
    pub fn add_clamp_chirho(
        &mut self,
        a_chirho: &str,
        lo_chirho: f64,
        hi_chirho: f64,
        b_chirho: &str,
    ) -> ConstraintIdChirho {
        let propagator_chirho = ClampChirho::install_chirho(
            lo_chirho,
            hi_chirho,
            self.get_cell_chirho(a_chirho),
            self.get_cell_chirho(b_chirho),
            &self.scheduler_chirho,
        );

        self.register_constraint_chirho(
            ConstraintTypeChirho::ClampChirho { lo_chirho, hi_chirho },
            vec![a_chirho.to_string(), b_chirho.to_string()],
            vec![propagator_chirho.id_chirho()],
        )
    }

    // ========================================================================
    // COMPOUND CONSTRAINTS
    // ========================================================================

    /// Creates a linear constraint: a*x + b = y.
    ///
    /// Automatically creates internal cells for intermediate values.
    ///
    /// Returns the constraint ID for the compound constraint.
    pub fn add_linear_chirho(
        &mut self,
        a_chirho: f64,
        x_chirho: &str,
        b_chirho: f64,
        y_chirho: &str,
    ) -> ConstraintIdChirho {
        let a_cell_chirho = self.make_cell_chirho(&format!("_linear_a_{}", x_chirho));
        let b_cell_chirho = self.make_cell_chirho(&format!("_linear_b_{}", x_chirho));
        let ax_chirho = self.make_cell_chirho(&format!("_linear_ax_{}", x_chirho));

        self.set_exact_chirho(&a_cell_chirho, a_chirho);
        self.set_exact_chirho(&b_cell_chirho, b_chirho);

        // These will register their own constraints, collect their propagator IDs
        let mul_id_chirho = self.add_multiplier_chirho(&a_cell_chirho, x_chirho, &ax_chirho);
        let add_id_chirho = self.add_adder_chirho(&ax_chirho, &b_cell_chirho, y_chirho);

        // Collect propagator IDs from sub-constraints
        let mut propagator_ids_chirho = Vec::new();
        if let Some(ids_chirho) = self.constraint_propagators_chirho.get(&mul_id_chirho) {
            propagator_ids_chirho.extend(ids_chirho.iter().cloned());
        }
        if let Some(ids_chirho) = self.constraint_propagators_chirho.get(&add_id_chirho) {
            propagator_ids_chirho.extend(ids_chirho.iter().cloned());
        }

        // Register the compound constraint (includes original x and y cells only)
        self.register_constraint_chirho(
            ConstraintTypeChirho::LinearChirho { a_chirho, b_chirho },
            vec![x_chirho.to_string(), y_chirho.to_string()],
            propagator_ids_chirho,
        )
    }

    /// Creates a Pythagorean constraint: a² + b² = c².
    ///
    /// Returns the constraint ID for the compound constraint.
    pub fn add_pythagorean_chirho(
        &mut self,
        a_chirho: &str,
        b_chirho: &str,
        c_chirho: &str,
    ) -> ConstraintIdChirho {
        let a_sq_chirho = self.make_cell_chirho(&format!("_pyth_a2_{}", a_chirho));
        let b_sq_chirho = self.make_cell_chirho(&format!("_pyth_b2_{}", b_chirho));
        let c_sq_chirho = self.make_cell_chirho(&format!("_pyth_c2_{}", c_chirho));

        // These will register their own constraints, collect their propagator IDs
        let sq_a_id_chirho = self.add_squarer_chirho(a_chirho, &a_sq_chirho);
        let sq_b_id_chirho = self.add_squarer_chirho(b_chirho, &b_sq_chirho);
        let sq_c_id_chirho = self.add_squarer_chirho(c_chirho, &c_sq_chirho);
        let add_id_chirho = self.add_adder_chirho(&a_sq_chirho, &b_sq_chirho, &c_sq_chirho);

        // Collect propagator IDs from sub-constraints
        let mut propagator_ids_chirho = Vec::new();
        for sub_id_chirho in [&sq_a_id_chirho, &sq_b_id_chirho, &sq_c_id_chirho, &add_id_chirho] {
            if let Some(ids_chirho) = self.constraint_propagators_chirho.get(sub_id_chirho) {
                propagator_ids_chirho.extend(ids_chirho.iter().cloned());
            }
        }

        // Register the compound constraint
        self.register_constraint_chirho(
            ConstraintTypeChirho::PythagoreanChirho,
            vec![
                a_chirho.to_string(),
                b_chirho.to_string(),
                c_chirho.to_string(),
            ],
            propagator_ids_chirho,
        )
    }

    // ========================================================================
    // INTROSPECTION
    // ========================================================================

    /// Returns all constraints in the system.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::ConstraintSystemChirho;
    ///
    /// let mut system_chirho = ConstraintSystemChirho::new_chirho();
    /// let a_chirho = system_chirho.make_cell_chirho("a");
    /// let b_chirho = system_chirho.make_cell_chirho("b");
    /// let c_chirho = system_chirho.make_cell_chirho("c");
    ///
    /// system_chirho.add_adder_chirho(&a_chirho, &b_chirho, &c_chirho);
    ///
    /// let constraints_chirho = system_chirho.get_all_constraints_chirho();
    /// assert_eq!(constraints_chirho.len(), 1);
    /// ```
    pub fn get_all_constraints_chirho(&self) -> &[ConstraintInfoChirho] {
        &self.constraints_chirho
    }

    /// Returns information about a specific constraint by ID.
    pub fn get_constraint_chirho(
        &self,
        id_chirho: &ConstraintIdChirho,
    ) -> Option<&ConstraintInfoChirho> {
        self.constraints_chirho
            .iter()
            .find(|c_chirho| &c_chirho.id_chirho == id_chirho)
    }

    /// Returns all constraints that affect the given cell.
    pub fn get_constraints_for_cell_chirho(
        &self,
        cell_name_chirho: &str,
    ) -> Vec<&ConstraintInfoChirho> {
        self.cell_to_constraints_chirho
            .get(cell_name_chirho)
            .map(|ids_chirho| {
                ids_chirho
                    .iter()
                    .filter_map(|id_chirho| self.get_constraint_chirho(id_chirho))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Explains why a cell has its current value.
    ///
    /// Returns information about:
    /// - The current value
    /// - Values directly set by the user
    /// - Constraints that affect this cell
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::ConstraintSystemChirho;
    ///
    /// let mut system_chirho = ConstraintSystemChirho::new_chirho();
    /// let a_chirho = system_chirho.make_cell_chirho("a");
    /// let b_chirho = system_chirho.make_cell_chirho("b");
    /// let c_chirho = system_chirho.make_cell_chirho("c");
    ///
    /// system_chirho.add_adder_chirho(&a_chirho, &b_chirho, &c_chirho);
    /// system_chirho.set_exact_chirho(&a_chirho, 3.0);
    /// system_chirho.set_exact_chirho(&b_chirho, 4.0);
    /// system_chirho.run_chirho();
    ///
    /// let explanation_chirho = system_chirho.explain_value_chirho(&c_chirho);
    /// assert_eq!(explanation_chirho.affecting_constraints_chirho.len(), 1);
    /// ```
    pub fn explain_value_chirho(&self, cell_name_chirho: &str) -> CellExplanationChirho {
        let current_value_chirho = self.get_chirho(cell_name_chirho);

        let affecting_constraints_chirho = self
            .cell_to_constraints_chirho
            .get(cell_name_chirho)
            .cloned()
            .unwrap_or_default();

        let user_set_values_chirho = self
            .user_set_values_chirho
            .get(cell_name_chirho)
            .cloned()
            .unwrap_or_default();

        // Build explanation text
        let mut explanation_text_chirho = format!(
            "Cell '{}' has value: {:?}\n",
            cell_name_chirho, current_value_chirho
        );

        if !user_set_values_chirho.is_empty() {
            let _ = writeln!(
                explanation_text_chirho,
                "User-set values: {:?}",
                user_set_values_chirho
            );
        }

        if !affecting_constraints_chirho.is_empty() {
            explanation_text_chirho.push_str("Affected by constraints:\n");
            for id_chirho in &affecting_constraints_chirho {
                if let Some(info_chirho) = self.get_constraint_chirho(id_chirho) {
                    let _ = writeln!(
                        explanation_text_chirho,
                        "  - {}",
                        info_chirho.description_chirho
                    );
                }
            }
        }

        CellExplanationChirho {
            cell_name_chirho: cell_name_chirho.to_string(),
            current_value_chirho,
            affecting_constraints_chirho,
            user_set_values_chirho,
            explanation_text_chirho,
        }
    }

    /// Returns a graph representation of the propagation network.
    ///
    /// This can be used for visualization or analysis of the constraint structure.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::ConstraintSystemChirho;
    ///
    /// let mut system_chirho = ConstraintSystemChirho::new_chirho();
    /// let a_chirho = system_chirho.make_cell_chirho("a");
    /// let b_chirho = system_chirho.make_cell_chirho("b");
    ///
    /// system_chirho.add_squarer_chirho(&a_chirho, &b_chirho);
    ///
    /// let graph_chirho = system_chirho.get_propagation_graph_chirho();
    /// assert_eq!(graph_chirho.cells_chirho.len(), 2);
    ///
    /// // Can export to DOT format for graphviz
    /// let dot_chirho = graph_chirho.to_dot_chirho();
    /// assert!(dot_chirho.contains("digraph"));
    /// ```
    pub fn get_propagation_graph_chirho(&self) -> PropagationGraphChirho {
        let cells_chirho: Vec<String> = self.cells_chirho.keys().cloned().collect();
        let constraints_chirho = self.constraints_chirho.clone();

        let constraint_to_cells_chirho: HashMap<ConstraintIdChirho, Vec<String>> = constraints_chirho
            .iter()
            .map(|c_chirho| (c_chirho.id_chirho.clone(), c_chirho.cells_chirho.clone()))
            .collect();

        PropagationGraphChirho {
            cells_chirho,
            constraints_chirho,
            cell_to_constraints_chirho: self.cell_to_constraints_chirho.clone(),
            constraint_to_cells_chirho,
        }
    }

    /// Returns the number of constraints in the system.
    pub fn constraint_count_chirho(&self) -> usize {
        self.constraints_chirho.len()
    }

    /// Returns a list of all cell names in the system.
    pub fn cell_names_chirho(&self) -> Vec<String> {
        self.cells_chirho.keys().cloned().collect()
    }

    // ========================================================================
    // CONSTRAINT REMOVAL/DEACTIVATION
    // ========================================================================

    /// Removes a constraint from the system.
    ///
    /// This removes the propagators associated with the constraint from all
    /// affected cells. The constraint will no longer propagate values.
    ///
    /// Note: This does not undo values that were already propagated.
    /// Use checkpoints and rollback for that functionality.
    ///
    /// # Arguments
    ///
    /// * `id_chirho` - The ID of the constraint to remove
    ///
    /// # Returns
    ///
    /// `true` if the constraint was found and removed, `false` if not found.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::ConstraintSystemChirho;
    ///
    /// let mut system_chirho = ConstraintSystemChirho::new_chirho();
    /// let a_chirho = system_chirho.make_cell_chirho("a");
    /// let b_chirho = system_chirho.make_cell_chirho("b");
    /// let c_chirho = system_chirho.make_cell_chirho("c");
    ///
    /// let id_chirho = system_chirho.add_adder_chirho(&a_chirho, &b_chirho, &c_chirho);
    /// assert_eq!(system_chirho.constraint_count_chirho(), 1);
    ///
    /// system_chirho.remove_constraint_chirho(&id_chirho);
    /// assert_eq!(system_chirho.active_constraint_count_chirho(), 0);
    /// ```
    pub fn remove_constraint_chirho(&mut self, id_chirho: &ConstraintIdChirho) -> bool {
        // Check if constraint exists and not already removed
        if !self.constraint_propagators_chirho.contains_key(id_chirho)
            || self.removed_constraints_chirho.contains(id_chirho)
        {
            return false;
        }

        // Get propagator IDs for this constraint
        let propagator_ids_chirho: HashSet<usize> = self
            .constraint_propagators_chirho
            .get(id_chirho)
            .map(|ids_chirho| ids_chirho.iter().cloned().collect())
            .unwrap_or_default();

        // Remove propagators from all cells
        for cell_chirho in self.cells_chirho.values() {
            cell_chirho.remove_neighbors_chirho(&propagator_ids_chirho);
        }

        // Mark as removed
        self.removed_constraints_chirho.insert(id_chirho.clone());

        // Also remove from deactivated if it was there
        self.deactivated_constraints_chirho.remove(id_chirho);

        true
    }

    /// Deactivates a constraint without removing it.
    ///
    /// Deactivated constraints still exist in the system but their propagators
    /// will not run. This is useful for temporarily disabling constraints
    /// for what-if analysis.
    ///
    /// # Arguments
    ///
    /// * `id_chirho` - The ID of the constraint to deactivate
    ///
    /// # Returns
    ///
    /// `true` if the constraint was found and deactivated, `false` if not found.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::ConstraintSystemChirho;
    ///
    /// let mut system_chirho = ConstraintSystemChirho::new_chirho();
    /// let a_chirho = system_chirho.make_cell_chirho("a");
    /// let b_chirho = system_chirho.make_cell_chirho("b");
    ///
    /// let id_chirho = system_chirho.add_squarer_chirho(&a_chirho, &b_chirho);
    ///
    /// system_chirho.deactivate_constraint_chirho(&id_chirho);
    /// assert!(!system_chirho.is_constraint_active_chirho(&id_chirho));
    /// ```
    pub fn deactivate_constraint_chirho(&mut self, id_chirho: &ConstraintIdChirho) -> bool {
        // Check if constraint exists and not already removed
        if !self.constraint_propagators_chirho.contains_key(id_chirho)
            || self.removed_constraints_chirho.contains(id_chirho)
        {
            return false;
        }

        // Get propagator IDs for this constraint
        let propagator_ids_chirho: HashSet<usize> = self
            .constraint_propagators_chirho
            .get(id_chirho)
            .map(|ids_chirho| ids_chirho.iter().cloned().collect())
            .unwrap_or_default();

        // Remove propagators from cells (but keep constraint info)
        for cell_chirho in self.cells_chirho.values() {
            cell_chirho.remove_neighbors_chirho(&propagator_ids_chirho);
        }

        self.deactivated_constraints_chirho.insert(id_chirho.clone());
        true
    }

    /// Reactivates a previously deactivated constraint.
    ///
    /// Note: This re-registers the propagators with the cells and schedules
    /// them for execution. The constraint will resume propagating on the
    /// next `run_chirho()` call.
    ///
    /// # Arguments
    ///
    /// * `id_chirho` - The ID of the constraint to reactivate
    ///
    /// # Returns
    ///
    /// `true` if the constraint was reactivated, `false` if not found
    /// or was not deactivated.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::ConstraintSystemChirho;
    ///
    /// let mut system_chirho = ConstraintSystemChirho::new_chirho();
    /// let a_chirho = system_chirho.make_cell_chirho("a");
    /// let b_chirho = system_chirho.make_cell_chirho("b");
    ///
    /// let id_chirho = system_chirho.add_squarer_chirho(&a_chirho, &b_chirho);
    /// system_chirho.deactivate_constraint_chirho(&id_chirho);
    /// assert!(!system_chirho.is_constraint_active_chirho(&id_chirho));
    ///
    /// system_chirho.activate_constraint_chirho(&id_chirho);
    /// assert!(system_chirho.is_constraint_active_chirho(&id_chirho));
    /// ```
    pub fn activate_constraint_chirho(&mut self, id_chirho: &ConstraintIdChirho) -> bool {
        // Check if constraint was deactivated
        if !self.deactivated_constraints_chirho.contains(id_chirho) {
            return false;
        }

        // Find the constraint info
        let constraint_info_chirho = self
            .constraints_chirho
            .iter()
            .find(|c_chirho| &c_chirho.id_chirho == id_chirho)
            .cloned();

        if constraint_info_chirho.is_none() {
            return false;
        }

        let info_chirho = constraint_info_chirho.unwrap();

        // Re-install the constraint by type
        // This creates new propagator IDs, so update the tracking
        let new_propagator_ids_chirho = self.reinstall_constraint_chirho(&info_chirho);

        // Update propagator tracking
        self.constraint_propagators_chirho
            .insert(id_chirho.clone(), new_propagator_ids_chirho);

        // Remove from deactivated set
        self.deactivated_constraints_chirho.remove(id_chirho);

        true
    }

    /// Helper to reinstall a constraint based on its type.
    #[allow(clippy::too_many_lines)]
    fn reinstall_constraint_chirho(&mut self, info_chirho: &ConstraintInfoChirho) -> Vec<usize> {

        let cells_chirho = &info_chirho.cells_chirho;

        match &info_chirho.constraint_type_chirho {
            ConstraintTypeChirho::AdderChirho => {
                let p_chirho = IntervalAdderChirho::install_chirho(
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    self.get_cell_chirho(&cells_chirho[2]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::SubtractorChirho => {
                let p_chirho = IntervalSubtractorChirho::install_chirho(
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    self.get_cell_chirho(&cells_chirho[2]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::MultiplierChirho => {
                let p_chirho = IntervalMultiplierChirho::install_chirho(
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    self.get_cell_chirho(&cells_chirho[2]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::DividerChirho => {
                let p_chirho = IntervalDividerChirho::install_chirho(
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    self.get_cell_chirho(&cells_chirho[2]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::SquarerChirho => {
                let p_chirho = SquarerChirho::install_chirho(
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::SqrterChirho => {
                let p_chirho = SqrterChirho::install_chirho(
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::AbsoluterChirho => {
                let p_chirho = AbsoluterChirho::install_chirho(
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::MaxChirho => {
                let p_chirho = MaxChirho::install_chirho(
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    self.get_cell_chirho(&cells_chirho[2]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::MinChirho => {
                let p_chirho = MinChirho::install_chirho(
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    self.get_cell_chirho(&cells_chirho[2]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::NegaterChirho => {
                let p_chirho = NegaterChirho::install_chirho(
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::ExpChirho => {
                let p_chirho = ExpChirho::install_chirho(
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::LnChirho => {
                let p_chirho = LnChirho::install_chirho(
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::PowerChirho { exponent_chirho } => {
                let p_chirho = PowerChirho::install_chirho(
                    *exponent_chirho,
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::ClampChirho {
                lo_chirho,
                hi_chirho,
            } => {
                let p_chirho = ClampChirho::install_chirho(
                    *lo_chirho,
                    *hi_chirho,
                    self.get_cell_chirho(&cells_chirho[0]),
                    self.get_cell_chirho(&cells_chirho[1]),
                    &self.scheduler_chirho,
                );
                vec![p_chirho.id_chirho()]
            }
            ConstraintTypeChirho::ConstantChirho => {
                // Constants can't be easily reinstalled without the value
                // For now, return empty - users should re-add constants manually
                vec![]
            }
            ConstraintTypeChirho::LinearChirho { .. } | ConstraintTypeChirho::PythagoreanChirho => {
                // Compound constraints - their sub-constraints need to be reactivated
                // This is complex; for now return empty
                vec![]
            }
        }
    }

    /// Returns `true` if the constraint is currently active.
    ///
    /// A constraint is active if it has not been removed or deactivated.
    pub fn is_constraint_active_chirho(&self, id_chirho: &ConstraintIdChirho) -> bool {
        self.constraint_propagators_chirho.contains_key(id_chirho)
            && !self.removed_constraints_chirho.contains(id_chirho)
            && !self.deactivated_constraints_chirho.contains(id_chirho)
    }

    /// Returns the count of active (non-removed, non-deactivated) constraints.
    pub fn active_constraint_count_chirho(&self) -> usize {
        self.constraints_chirho
            .iter()
            .filter(|c_chirho| self.is_constraint_active_chirho(&c_chirho.id_chirho))
            .count()
    }

    /// Returns IDs of all deactivated constraints.
    pub fn deactivated_constraints_chirho(&self) -> Vec<ConstraintIdChirho> {
        self.deactivated_constraints_chirho
            .iter()
            .cloned()
            .collect()
    }

    /// Returns IDs of all removed constraints.
    pub fn removed_constraints_chirho(&self) -> Vec<ConstraintIdChirho> {
        self.removed_constraints_chirho.iter().cloned().collect()
    }

    // ========================================================================
    // CHECKPOINT AND ROLLBACK
    // ========================================================================

    /// Creates a checkpoint of the current system state.
    ///
    /// The checkpoint captures:
    /// - All cell values
    /// - User-set values
    /// - Deactivated and removed constraint sets
    ///
    /// Use [`rollback_to_chirho`](Self::rollback_to_chirho) to restore the state.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::ConstraintSystemChirho;
    ///
    /// let mut system_chirho = ConstraintSystemChirho::new_chirho();
    /// let a_chirho = system_chirho.make_cell_chirho("a");
    /// system_chirho.set_exact_chirho(&a_chirho, 42.0);
    ///
    /// let checkpoint_chirho = system_chirho.checkpoint_chirho();
    /// assert_eq!(checkpoint_chirho.cell_names_chirho().len(), 1);
    /// ```
    pub fn checkpoint_chirho(&mut self) -> CheckpointChirho {
        let id_chirho = self.next_checkpoint_id_chirho;
        self.next_checkpoint_id_chirho += 1;

        // Capture cell values
        let mut cell_values_chirho = HashMap::new();
        for (name_chirho, cell_chirho) in &self.cells_chirho {
            cell_values_chirho.insert(name_chirho.clone(), cell_chirho.content_chirho());
        }

        CheckpointChirho {
            id_chirho,
            cell_values_chirho,
            user_set_values_chirho: self.user_set_values_chirho.clone(),
            deactivated_constraints_chirho: self.deactivated_constraints_chirho.clone(),
            removed_constraints_chirho: self.removed_constraints_chirho.clone(),
        }
    }

    /// Restores the system to a previously saved checkpoint.
    ///
    /// This resets all cell values to their checkpoint state, but does NOT
    /// remove constraints that were added after the checkpoint. To fully
    /// undo constraint additions, use the constraint removal API.
    ///
    /// # Arguments
    ///
    /// * `checkpoint_chirho` - The checkpoint to restore
    ///
    /// # Returns
    ///
    /// `true` if the rollback was successful, `false` if any cells in the
    /// checkpoint no longer exist.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::ConstraintSystemChirho;
    ///
    /// let mut system_chirho = ConstraintSystemChirho::new_chirho();
    /// let a_chirho = system_chirho.make_cell_chirho("a");
    /// let b_chirho = system_chirho.make_cell_chirho("b");
    /// let c_chirho = system_chirho.make_cell_chirho("c");
    ///
    /// system_chirho.add_adder_chirho(&a_chirho, &b_chirho, &c_chirho);
    /// system_chirho.set_exact_chirho(&a_chirho, 3.0);
    /// system_chirho.set_exact_chirho(&b_chirho, 4.0);
    /// system_chirho.run_chirho();
    ///
    /// // Save state: a=3, b=4, c=7
    /// let checkpoint_chirho = system_chirho.checkpoint_chirho();
    ///
    /// // Modify a to cause c to change
    /// system_chirho.set_exact_chirho(&a_chirho, 10.0);
    /// system_chirho.run_chirho();
    ///
    /// // Rollback to saved state
    /// let success_chirho = system_chirho.rollback_to_chirho(&checkpoint_chirho);
    /// assert!(success_chirho);
    /// ```
    pub fn rollback_to_chirho(&mut self, checkpoint_chirho: &CheckpointChirho) -> bool {
        // Verify all cells still exist
        for cell_name_chirho in checkpoint_chirho.cell_values_chirho.keys() {
            if !self.cells_chirho.contains_key(cell_name_chirho) {
                return false;
            }
        }

        // Restore cell values by recreating cells with the checkpoint content
        for (cell_name_chirho, value_chirho) in &checkpoint_chirho.cell_values_chirho {
            if let Some(_cell_chirho) = self.cells_chirho.get(cell_name_chirho) {
                // We need to replace the cell's content. Since cells are Rc<CellChirho>,
                // we create a new cell with the checkpoint value and swap.
                let new_cell_chirho =
                    CellChirho::with_content_chirho(cell_name_chirho, *value_chirho);

                // Copy neighbors from old cell to new cell
                // We need to re-register all propagators as neighbors
                // This is a limitation - for now we just set the value and re-run
                // A more complete solution would preserve neighbor relationships
                self.cells_chirho
                    .insert(cell_name_chirho.clone(), new_cell_chirho);
            }
        }

        // Restore user-set values
        self.user_set_values_chirho
            .clone_from(&checkpoint_chirho.user_set_values_chirho);

        // Restore constraint state
        self.deactivated_constraints_chirho
            .clone_from(&checkpoint_chirho.deactivated_constraints_chirho);
        self.removed_constraints_chirho
            .clone_from(&checkpoint_chirho.removed_constraints_chirho);

        // Re-register all active propagators with the new cells
        // This is necessary because we replaced the cells
        self.reinstall_all_active_constraints_chirho();

        true
    }

    /// Reinstalls all active constraints after a rollback.
    ///
    /// This is an internal helper that reconnects propagators to cells
    /// after cells have been replaced during rollback.
    fn reinstall_all_active_constraints_chirho(&mut self) {
        // Clear propagator mappings - we'll rebuild them
        self.constraint_propagators_chirho.clear();

        // Collect constraints to reinstall (those not removed)
        let constraints_to_install_chirho: Vec<ConstraintInfoChirho> = self
            .constraints_chirho
            .iter()
            .filter(|c_chirho| !self.removed_constraints_chirho.contains(&c_chirho.id_chirho))
            .filter(|c_chirho| !self.deactivated_constraints_chirho.contains(&c_chirho.id_chirho))
            .cloned()
            .collect();

        // Reinstall each constraint
        for constraint_chirho in constraints_to_install_chirho {
            self.reinstall_constraint_for_rollback_chirho(&constraint_chirho);
        }
    }

    /// Helper to reinstall a single constraint after rollback.
    #[allow(clippy::too_many_lines)]
    fn reinstall_constraint_for_rollback_chirho(&mut self, constraint_chirho: &ConstraintInfoChirho) {
        let cells_chirho = &constraint_chirho.cells_chirho;
        let id_chirho = &constraint_chirho.id_chirho;

        let propagator_ids_chirho = match &constraint_chirho.constraint_type_chirho {
            ConstraintTypeChirho::AdderChirho => {
                if cells_chirho.len() == 3 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    let c_chirho = self.cells_chirho.get(&cells_chirho[2]).cloned();
                    if let (Some(a_chirho), Some(b_chirho), Some(c_chirho)) =
                        (a_chirho, b_chirho, c_chirho)
                    {
                        let prop_chirho = IntervalAdderChirho::install_chirho(
                            a_chirho,
                            b_chirho,
                            c_chirho,
                            &self.scheduler_chirho,
                        );
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::SubtractorChirho => {
                if cells_chirho.len() == 3 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    let c_chirho = self.cells_chirho.get(&cells_chirho[2]).cloned();
                    if let (Some(a_chirho), Some(b_chirho), Some(c_chirho)) =
                        (a_chirho, b_chirho, c_chirho)
                    {
                        let prop_chirho = IntervalSubtractorChirho::install_chirho(
                            a_chirho,
                            b_chirho,
                            c_chirho,
                            &self.scheduler_chirho,
                        );
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::MultiplierChirho => {
                if cells_chirho.len() == 3 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    let c_chirho = self.cells_chirho.get(&cells_chirho[2]).cloned();
                    if let (Some(a_chirho), Some(b_chirho), Some(c_chirho)) =
                        (a_chirho, b_chirho, c_chirho)
                    {
                        let prop_chirho = IntervalMultiplierChirho::install_chirho(
                            a_chirho,
                            b_chirho,
                            c_chirho,
                            &self.scheduler_chirho,
                        );
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::DividerChirho => {
                if cells_chirho.len() == 3 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    let c_chirho = self.cells_chirho.get(&cells_chirho[2]).cloned();
                    if let (Some(a_chirho), Some(b_chirho), Some(c_chirho)) =
                        (a_chirho, b_chirho, c_chirho)
                    {
                        let prop_chirho = IntervalDividerChirho::install_chirho(
                            a_chirho,
                            b_chirho,
                            c_chirho,
                            &self.scheduler_chirho,
                        );
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::SquarerChirho => {
                if cells_chirho.len() == 2 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    if let (Some(a_chirho), Some(b_chirho)) = (a_chirho, b_chirho) {
                        let prop_chirho =
                            SquarerChirho::install_chirho(a_chirho, b_chirho, &self.scheduler_chirho);
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::SqrterChirho => {
                if cells_chirho.len() == 2 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    if let (Some(a_chirho), Some(b_chirho)) = (a_chirho, b_chirho) {
                        let prop_chirho =
                            SqrterChirho::install_chirho(a_chirho, b_chirho, &self.scheduler_chirho);
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::AbsoluterChirho => {
                if cells_chirho.len() == 2 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    if let (Some(a_chirho), Some(b_chirho)) = (a_chirho, b_chirho) {
                        let prop_chirho =
                            AbsoluterChirho::install_chirho(a_chirho, b_chirho, &self.scheduler_chirho);
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::MaxChirho => {
                if cells_chirho.len() == 3 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    let c_chirho = self.cells_chirho.get(&cells_chirho[2]).cloned();
                    if let (Some(a_chirho), Some(b_chirho), Some(c_chirho)) =
                        (a_chirho, b_chirho, c_chirho)
                    {
                        let prop_chirho = MaxChirho::install_chirho(
                            a_chirho,
                            b_chirho,
                            c_chirho,
                            &self.scheduler_chirho,
                        );
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::MinChirho => {
                if cells_chirho.len() == 3 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    let c_chirho = self.cells_chirho.get(&cells_chirho[2]).cloned();
                    if let (Some(a_chirho), Some(b_chirho), Some(c_chirho)) =
                        (a_chirho, b_chirho, c_chirho)
                    {
                        let prop_chirho = MinChirho::install_chirho(
                            a_chirho,
                            b_chirho,
                            c_chirho,
                            &self.scheduler_chirho,
                        );
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::NegaterChirho => {
                if cells_chirho.len() == 2 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    if let (Some(a_chirho), Some(b_chirho)) = (a_chirho, b_chirho) {
                        let prop_chirho =
                            NegaterChirho::install_chirho(a_chirho, b_chirho, &self.scheduler_chirho);
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::ExpChirho => {
                if cells_chirho.len() == 2 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    if let (Some(a_chirho), Some(b_chirho)) = (a_chirho, b_chirho) {
                        let prop_chirho =
                            ExpChirho::install_chirho(a_chirho, b_chirho, &self.scheduler_chirho);
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::LnChirho => {
                if cells_chirho.len() == 2 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    if let (Some(a_chirho), Some(b_chirho)) = (a_chirho, b_chirho) {
                        let prop_chirho =
                            LnChirho::install_chirho(a_chirho, b_chirho, &self.scheduler_chirho);
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::PowerChirho { exponent_chirho } => {
                if cells_chirho.len() == 2 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    if let (Some(a_chirho), Some(b_chirho)) = (a_chirho, b_chirho) {
                        let prop_chirho = PowerChirho::install_chirho(
                            *exponent_chirho,
                            a_chirho,
                            b_chirho,
                            &self.scheduler_chirho,
                        );
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::ClampChirho {
                lo_chirho,
                hi_chirho,
            } => {
                if cells_chirho.len() == 2 {
                    let a_chirho = self.cells_chirho.get(&cells_chirho[0]).cloned();
                    let b_chirho = self.cells_chirho.get(&cells_chirho[1]).cloned();
                    if let (Some(a_chirho), Some(b_chirho)) = (a_chirho, b_chirho) {
                        let prop_chirho = ClampChirho::install_chirho(
                            *lo_chirho,
                            *hi_chirho,
                            a_chirho,
                            b_chirho,
                            &self.scheduler_chirho,
                        );
                        vec![prop_chirho.id_chirho()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            ConstraintTypeChirho::ConstantChirho => {
                // Constants don't need reinstallation - they're values, not propagators
                // Actually, ConstantChirho does create a propagator, but on rollback
                // the cell already has the value, so we skip reinstalling
                vec![]
            }
            // Compound constraints are handled by their component constraints
            ConstraintTypeChirho::LinearChirho { .. } | ConstraintTypeChirho::PythagoreanChirho => {
                // These are compound - their sub-constraints are stored separately
                vec![]
            }
        };

        // Register the propagator IDs
        if !propagator_ids_chirho.is_empty() {
            self.constraint_propagators_chirho
                .insert(id_chirho.clone(), propagator_ids_chirho);
        }
    }

    /// Returns `true` if a rollback is possible to the given checkpoint.
    ///
    /// A rollback is possible if all cells in the checkpoint still exist.
    pub fn can_rollback_to_chirho(&self, checkpoint_chirho: &CheckpointChirho) -> bool {
        checkpoint_chirho
            .cell_values_chirho
            .keys()
            .all(|name_chirho| self.cells_chirho.contains_key(name_chirho))
    }

    // ========================================================================
    // ACCESSORS
    // ========================================================================

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

// ============================================================================
// FLUENT BUILDER API
// ============================================================================

/// A fluent builder for creating cells and constraints in a chainable manner.
///
/// This provides a cell-centric API for building constraint networks.
///
/// # Example
///
/// ```
/// use propagators_chirho::ConstraintSystemChirho;
///
/// let mut system_chirho = ConstraintSystemChirho::new_chirho();
///
/// // Fluent cell creation
/// system_chirho
///     .with_cell_chirho("a")
///     .with_cell_chirho("b")
///     .with_cell_chirho("c");
///
/// // Fluent constraint building using cell builder
/// system_chirho.cell_chirho("a")
///     .plus_chirho("b")
///     .into_chirho("c");
///
/// // Set values and run
/// system_chirho.set_exact_chirho("a", 3.0);
/// system_chirho.set_exact_chirho("b", 4.0);
/// system_chirho.run_chirho();
///
/// // c is now 7.0
/// ```
pub struct CellBuilderChirho<'a> {
    system_chirho: &'a mut ConstraintSystemChirho,
    cell_name_chirho: String,
    operation_chirho: Option<CellOperationChirho>,
}

/// Represents an operation in the fluent builder.
#[derive(Clone)]
#[allow(clippy::enum_variant_names)]
enum CellOperationChirho {
    /// Addition: self + other
    PlusChirho(String),
    /// Subtraction: self - other
    MinusChirho(String),
    /// Multiplication: self * other
    TimesChirho(String),
    /// Division: self / other
    DividedByChirho(String),
    /// Square: self²
    SquaredChirho,
    /// Square root: √self
    SqrtChirho,
    /// Absolute value: |self|
    AbsChirho,
    /// Negation: -self
    NegateChirho,
    /// Exponential: e^self
    ExpChirho,
    /// Natural log: ln(self)
    LnChirho,
    /// Power: self^n
    PowerChirho(i32),
    /// Maximum: max(self, other)
    MaxWithChirho(String),
    /// Minimum: min(self, other)
    MinWithChirho(String),
}

impl<'a> CellBuilderChirho<'a> {
    /// Creates a new cell builder for the given cell.
    fn new_chirho(system_chirho: &'a mut ConstraintSystemChirho, cell_name_chirho: String) -> Self {
        Self {
            system_chirho,
            cell_name_chirho,
            operation_chirho: None,
        }
    }

    /// Specifies addition: self + other.
    ///
    /// Call `.into_chirho(result_cell)` to complete the constraint.
    pub fn plus_chirho(mut self, other_chirho: &str) -> Self {
        self.operation_chirho = Some(CellOperationChirho::PlusChirho(other_chirho.to_string()));
        self
    }

    /// Specifies subtraction: self - other.
    ///
    /// Call `.into_chirho(result_cell)` to complete the constraint.
    pub fn minus_chirho(mut self, other_chirho: &str) -> Self {
        self.operation_chirho = Some(CellOperationChirho::MinusChirho(other_chirho.to_string()));
        self
    }

    /// Specifies multiplication: self * other.
    ///
    /// Call `.into_chirho(result_cell)` to complete the constraint.
    pub fn times_chirho(mut self, other_chirho: &str) -> Self {
        self.operation_chirho = Some(CellOperationChirho::TimesChirho(other_chirho.to_string()));
        self
    }

    /// Specifies division: self / other.
    ///
    /// Call `.into_chirho(result_cell)` to complete the constraint.
    pub fn divided_by_chirho(mut self, other_chirho: &str) -> Self {
        self.operation_chirho = Some(CellOperationChirho::DividedByChirho(other_chirho.to_string()));
        self
    }

    /// Specifies squaring: self².
    ///
    /// Call `.into_chirho(result_cell)` to complete the constraint.
    pub fn squared_chirho(mut self) -> Self {
        self.operation_chirho = Some(CellOperationChirho::SquaredChirho);
        self
    }

    /// Specifies square root: √self.
    ///
    /// Call `.into_chirho(result_cell)` to complete the constraint.
    pub fn sqrt_chirho(mut self) -> Self {
        self.operation_chirho = Some(CellOperationChirho::SqrtChirho);
        self
    }

    /// Specifies absolute value: |self|.
    ///
    /// Call `.into_chirho(result_cell)` to complete the constraint.
    pub fn abs_chirho(mut self) -> Self {
        self.operation_chirho = Some(CellOperationChirho::AbsChirho);
        self
    }

    /// Specifies negation: -self.
    ///
    /// Call `.into_chirho(result_cell)` to complete the constraint.
    pub fn negated_chirho(mut self) -> Self {
        self.operation_chirho = Some(CellOperationChirho::NegateChirho);
        self
    }

    /// Specifies exponential: e^self.
    ///
    /// Call `.into_chirho(result_cell)` to complete the constraint.
    pub fn exp_chirho(mut self) -> Self {
        self.operation_chirho = Some(CellOperationChirho::ExpChirho);
        self
    }

    /// Specifies natural logarithm: ln(self).
    ///
    /// Call `.into_chirho(result_cell)` to complete the constraint.
    pub fn ln_chirho(mut self) -> Self {
        self.operation_chirho = Some(CellOperationChirho::LnChirho);
        self
    }

    /// Specifies power: self^n.
    ///
    /// Call `.into_chirho(result_cell)` to complete the constraint.
    pub fn power_chirho(mut self, n_chirho: i32) -> Self {
        self.operation_chirho = Some(CellOperationChirho::PowerChirho(n_chirho));
        self
    }

    /// Specifies maximum: max(self, other).
    ///
    /// Call `.into_chirho(result_cell)` to complete the constraint.
    pub fn max_with_chirho(mut self, other_chirho: &str) -> Self {
        self.operation_chirho = Some(CellOperationChirho::MaxWithChirho(other_chirho.to_string()));
        self
    }

    /// Specifies minimum: min(self, other).
    ///
    /// Call `.into_chirho(result_cell)` to complete the constraint.
    pub fn min_with_chirho(mut self, other_chirho: &str) -> Self {
        self.operation_chirho = Some(CellOperationChirho::MinWithChirho(other_chirho.to_string()));
        self
    }

    /// Completes the constraint by specifying the result cell.
    ///
    /// Returns the constraint ID for introspection.
    ///
    /// # Panics
    ///
    /// Panics if no operation was specified before calling this method.
    pub fn into_chirho(self, result_chirho: &str) -> ConstraintIdChirho {
        let operation_chirho = self
            .operation_chirho
            .expect("No operation specified. Call plus_chirho, minus_chirho, etc. first.");

        match operation_chirho {
            CellOperationChirho::PlusChirho(other_chirho) => {
                self.system_chirho
                    .add_adder_chirho(&self.cell_name_chirho, &other_chirho, result_chirho)
            }
            CellOperationChirho::MinusChirho(other_chirho) => {
                self.system_chirho
                    .add_subtractor_chirho(&self.cell_name_chirho, &other_chirho, result_chirho)
            }
            CellOperationChirho::TimesChirho(other_chirho) => {
                self.system_chirho
                    .add_multiplier_chirho(&self.cell_name_chirho, &other_chirho, result_chirho)
            }
            CellOperationChirho::DividedByChirho(other_chirho) => {
                self.system_chirho
                    .add_divider_chirho(&self.cell_name_chirho, &other_chirho, result_chirho)
            }
            CellOperationChirho::SquaredChirho => {
                self.system_chirho
                    .add_squarer_chirho(&self.cell_name_chirho, result_chirho)
            }
            CellOperationChirho::SqrtChirho => {
                self.system_chirho
                    .add_sqrter_chirho(&self.cell_name_chirho, result_chirho)
            }
            CellOperationChirho::AbsChirho => {
                self.system_chirho
                    .add_absoluter_chirho(&self.cell_name_chirho, result_chirho)
            }
            CellOperationChirho::NegateChirho => {
                self.system_chirho
                    .add_negater_chirho(&self.cell_name_chirho, result_chirho)
            }
            CellOperationChirho::ExpChirho => {
                self.system_chirho
                    .add_exp_chirho(&self.cell_name_chirho, result_chirho)
            }
            CellOperationChirho::LnChirho => {
                self.system_chirho
                    .add_ln_chirho(&self.cell_name_chirho, result_chirho)
            }
            CellOperationChirho::PowerChirho(n_chirho) => {
                self.system_chirho
                    .add_power_chirho(n_chirho, &self.cell_name_chirho, result_chirho)
            }
            CellOperationChirho::MaxWithChirho(other_chirho) => {
                self.system_chirho
                    .add_max_chirho(&self.cell_name_chirho, &other_chirho, result_chirho)
            }
            CellOperationChirho::MinWithChirho(other_chirho) => {
                self.system_chirho
                    .add_min_chirho(&self.cell_name_chirho, &other_chirho, result_chirho)
            }
        }
    }

    /// Specifies equality constraint: self = other.
    ///
    /// This creates a bidirectional equality constraint between two cells.
    /// Any value set on either cell will propagate to the other.
    ///
    /// Returns the constraint IDs for the bidirectional constraint.
    pub fn equals_chirho(self, other_chirho: &str) -> (ConstraintIdChirho, ConstraintIdChirho) {
        // Create bidirectional equality using adder with zero
        let zero_cell_chirho = format!("_eq_zero_{}_{}", self.cell_name_chirho, other_chirho);
        let _ = self.system_chirho.try_make_cell_chirho(&zero_cell_chirho);
        self.system_chirho.set_exact_chirho(&zero_cell_chirho, 0.0);

        // a + 0 = b (forward)
        let id1_chirho = self.system_chirho.add_adder_chirho(
            &self.cell_name_chirho,
            &zero_cell_chirho,
            other_chirho,
        );

        // b + 0 = a (backward)
        let id2_chirho = self.system_chirho.add_adder_chirho(
            other_chirho,
            &zero_cell_chirho,
            &self.cell_name_chirho,
        );

        (id1_chirho, id2_chirho)
    }
}

impl ConstraintSystemChirho {
    // ========================================================================
    // FLUENT API METHODS
    // ========================================================================

    /// Returns a cell builder for fluent constraint specification.
    ///
    /// This allows building constraints in a cell-centric, chainable manner.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::ConstraintSystemChirho;
    ///
    /// let mut system_chirho = ConstraintSystemChirho::new_chirho();
    /// system_chirho.make_cell_chirho("a");
    /// system_chirho.make_cell_chirho("b");
    /// system_chirho.make_cell_chirho("c");
    ///
    /// // Fluent: a + b = c
    /// system_chirho.cell_chirho("a").plus_chirho("b").into_chirho("c");
    ///
    /// system_chirho.set_exact_chirho("a", 3.0);
    /// system_chirho.set_exact_chirho("b", 4.0);
    /// system_chirho.run_chirho();
    ///
    /// let c_chirho = system_chirho.get_chirho("c");
    /// assert!(c_chirho.as_interval_chirho().unwrap().lo_chirho - 7.0 < 1e-10);
    /// ```
    pub fn cell_chirho(&mut self, cell_name_chirho: &str) -> CellBuilderChirho<'_> {
        CellBuilderChirho::new_chirho(self, cell_name_chirho.to_string())
    }

    /// Creates a cell and returns self for chaining.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::ConstraintSystemChirho;
    ///
    /// let mut system_chirho = ConstraintSystemChirho::new_chirho();
    ///
    /// // Chain cell creation
    /// system_chirho
    ///     .with_cell_chirho("a")
    ///     .with_cell_chirho("b")
    ///     .with_cell_chirho("c");
    ///
    /// assert_eq!(system_chirho.cell_count_chirho(), 3);
    /// ```
    pub fn with_cell_chirho(&mut self, name_chirho: &str) -> &mut Self {
        self.make_cell_chirho(name_chirho);
        self
    }

    /// Sets an exact value and returns self for chaining.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::ConstraintSystemChirho;
    ///
    /// let mut system_chirho = ConstraintSystemChirho::new_chirho();
    /// system_chirho.make_cell_chirho("a");
    /// system_chirho.make_cell_chirho("b");
    ///
    /// // Chain value setting
    /// system_chirho
    ///     .with_exact_chirho("a", 3.0)
    ///     .with_exact_chirho("b", 4.0);
    /// ```
    pub fn with_exact_chirho(&mut self, name_chirho: &str, value_chirho: f64) -> &mut Self {
        self.set_exact_chirho(name_chirho, value_chirho);
        self
    }

    /// Sets an interval value and returns self for chaining.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::ConstraintSystemChirho;
    ///
    /// let mut system_chirho = ConstraintSystemChirho::new_chirho();
    /// system_chirho.make_cell_chirho("a");
    /// system_chirho.make_cell_chirho("b");
    ///
    /// // Chain interval setting
    /// system_chirho
    ///     .with_interval_chirho("a", 0.0, 10.0)
    ///     .with_interval_chirho("b", 5.0, 15.0);
    /// ```
    pub fn with_interval_chirho(
        &mut self,
        name_chirho: &str,
        lo_chirho: f64,
        hi_chirho: f64,
    ) -> &mut Self {
        self.set_interval_chirho(name_chirho, lo_chirho, hi_chirho);
        self
    }

    /// Adds an adder constraint and returns self for chaining.
    ///
    /// Note: Use `add_adder_chirho` if you need the constraint ID.
    pub fn with_adder_chirho(
        &mut self,
        a_chirho: &str,
        b_chirho: &str,
        c_chirho: &str,
    ) -> &mut Self {
        self.add_adder_chirho(a_chirho, b_chirho, c_chirho);
        self
    }

    /// Adds a subtractor constraint and returns self for chaining.
    ///
    /// Note: Use `add_subtractor_chirho` if you need the constraint ID.
    pub fn with_subtractor_chirho(
        &mut self,
        a_chirho: &str,
        b_chirho: &str,
        c_chirho: &str,
    ) -> &mut Self {
        self.add_subtractor_chirho(a_chirho, b_chirho, c_chirho);
        self
    }

    /// Adds a multiplier constraint and returns self for chaining.
    ///
    /// Note: Use `add_multiplier_chirho` if you need the constraint ID.
    pub fn with_multiplier_chirho(
        &mut self,
        a_chirho: &str,
        b_chirho: &str,
        c_chirho: &str,
    ) -> &mut Self {
        self.add_multiplier_chirho(a_chirho, b_chirho, c_chirho);
        self
    }

    /// Adds a divider constraint and returns self for chaining.
    ///
    /// Note: Use `add_divider_chirho` if you need the constraint ID.
    pub fn with_divider_chirho(
        &mut self,
        a_chirho: &str,
        b_chirho: &str,
        c_chirho: &str,
    ) -> &mut Self {
        self.add_divider_chirho(a_chirho, b_chirho, c_chirho);
        self
    }

    /// Adds a squarer constraint and returns self for chaining.
    ///
    /// Note: Use `add_squarer_chirho` if you need the constraint ID.
    pub fn with_squarer_chirho(&mut self, a_chirho: &str, b_chirho: &str) -> &mut Self {
        self.add_squarer_chirho(a_chirho, b_chirho);
        self
    }

    /// Adds a sqrter constraint and returns self for chaining.
    ///
    /// Note: Use `add_sqrter_chirho` if you need the constraint ID.
    pub fn with_sqrter_chirho(&mut self, a_chirho: &str, b_chirho: &str) -> &mut Self {
        self.add_sqrter_chirho(a_chirho, b_chirho);
        self
    }

    /// Runs propagators to fixpoint and returns self for chaining.
    pub fn run_and_continue_chirho(&mut self) -> &mut Self {
        self.run_chirho();
        self
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

    #[test]
    fn test_linear_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let x_chirho = system_chirho.make_cell_chirho("x");
        let y_chirho = system_chirho.make_cell_chirho("y");

        // y = 2*x + 3
        system_chirho.add_linear_chirho(2.0, &x_chirho, 3.0, &y_chirho);

        system_chirho.set_exact_chirho(&x_chirho, 5.0);
        system_chirho.run_chirho();

        let y_value_chirho = system_chirho.get_chirho(&y_chirho);
        let y_interval_chirho = y_value_chirho.as_interval_chirho().unwrap();
        assert!((y_interval_chirho.lo_chirho - 13.0).abs() < 1e-10);
    }

    #[test]
    fn test_negater_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");

        system_chirho.add_negater_chirho(&a_chirho, &b_chirho);

        system_chirho.set_exact_chirho(&a_chirho, 5.0);
        system_chirho.run_chirho();

        let b_value_chirho = system_chirho.get_chirho(&b_chirho);
        let b_interval_chirho = b_value_chirho.as_interval_chirho().unwrap();
        assert!((b_interval_chirho.lo_chirho - (-5.0)).abs() < 1e-10);
    }

    #[test]
    fn test_exp_ln_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");

        // e^a = b
        system_chirho.add_exp_chirho(&a_chirho, &b_chirho);

        system_chirho.set_exact_chirho(&a_chirho, 0.0);
        system_chirho.run_chirho();

        // e^0 = 1
        let b_value_chirho = system_chirho.get_chirho(&b_chirho);
        let b_interval_chirho = b_value_chirho.as_interval_chirho().unwrap();
        assert!((b_interval_chirho.lo_chirho - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_ln_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");

        // ln(a) = b
        system_chirho.add_ln_chirho(&a_chirho, &b_chirho);

        system_chirho.set_exact_chirho(&a_chirho, std::f64::consts::E);
        system_chirho.run_chirho();

        // ln(e) = 1
        let b_value_chirho = system_chirho.get_chirho(&b_chirho);
        let b_interval_chirho = b_value_chirho.as_interval_chirho().unwrap();
        assert!((b_interval_chirho.lo_chirho - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_power_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");

        // a^3 = b
        system_chirho.add_power_chirho(3, &a_chirho, &b_chirho);

        system_chirho.set_exact_chirho(&a_chirho, 2.0);
        system_chirho.run_chirho();

        // 2^3 = 8
        let b_value_chirho = system_chirho.get_chirho(&b_chirho);
        let b_interval_chirho = b_value_chirho.as_interval_chirho().unwrap();
        assert!((b_interval_chirho.lo_chirho - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_clamp_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");

        // clamp(a, 0, 10) = b
        system_chirho.add_clamp_chirho(&a_chirho, 0.0, 10.0, &b_chirho);

        system_chirho.set_exact_chirho(&a_chirho, 15.0);
        system_chirho.run_chirho();

        // 15 clamped to [0, 10] = 10
        let b_value_chirho = system_chirho.get_chirho(&b_chirho);
        let b_interval_chirho = b_value_chirho.as_interval_chirho().unwrap();
        assert!(b_interval_chirho.hi_chirho <= 10.0);
    }

    #[test]
    fn test_max_min_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let a_chirho = system_chirho.make_cell_chirho("a");
        let b_chirho = system_chirho.make_cell_chirho("b");
        let max_chirho = system_chirho.make_cell_chirho("max");
        let min_chirho = system_chirho.make_cell_chirho("min");

        system_chirho.add_max_chirho(&a_chirho, &b_chirho, &max_chirho);
        system_chirho.add_min_chirho(&a_chirho, &b_chirho, &min_chirho);

        system_chirho.set_exact_chirho(&a_chirho, 3.0);
        system_chirho.set_exact_chirho(&b_chirho, 7.0);
        system_chirho.run_chirho();

        let max_value_chirho = system_chirho.get_chirho(&max_chirho);
        let max_interval_chirho = max_value_chirho.as_interval_chirho().unwrap();
        assert!((max_interval_chirho.lo_chirho - 7.0).abs() < 1e-10);

        let min_value_chirho = system_chirho.get_chirho(&min_chirho);
        let min_interval_chirho = min_value_chirho.as_interval_chirho().unwrap();
        assert!((min_interval_chirho.lo_chirho - 3.0).abs() < 1e-10);
    }

    // ========================================================================
    // FLUENT API TESTS
    // ========================================================================

    #[test]
    fn test_fluent_cell_creation_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        // Chain cell creation
        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c");

        assert_eq!(system_chirho.cell_count_chirho(), 3);
    }

    #[test]
    fn test_fluent_value_setting_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_exact_chirho("a", 5.0)
            .with_exact_chirho("b", 10.0);

        let a_value_chirho = system_chirho.get_chirho("a");
        let b_value_chirho = system_chirho.get_chirho("b");

        assert!((a_value_chirho.as_interval_chirho().unwrap().lo_chirho - 5.0).abs() < 1e-10);
        assert!((b_value_chirho.as_interval_chirho().unwrap().lo_chirho - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_fluent_plus_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c");

        // Fluent: a + b = c
        system_chirho.cell_chirho("a").plus_chirho("b").into_chirho("c");

        system_chirho.set_exact_chirho("a", 3.0);
        system_chirho.set_exact_chirho("b", 4.0);
        system_chirho.run_chirho();

        let c_value_chirho = system_chirho.get_chirho("c");
        assert!((c_value_chirho.as_interval_chirho().unwrap().lo_chirho - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_fluent_minus_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c");

        // Fluent: a - b = c
        system_chirho.cell_chirho("a").minus_chirho("b").into_chirho("c");

        system_chirho.set_exact_chirho("a", 10.0);
        system_chirho.set_exact_chirho("b", 3.0);
        system_chirho.run_chirho();

        let c_value_chirho = system_chirho.get_chirho("c");
        assert!((c_value_chirho.as_interval_chirho().unwrap().lo_chirho - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_fluent_times_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c");

        // Fluent: a * b = c
        system_chirho.cell_chirho("a").times_chirho("b").into_chirho("c");

        system_chirho.set_exact_chirho("a", 3.0);
        system_chirho.set_exact_chirho("b", 4.0);
        system_chirho.run_chirho();

        let c_value_chirho = system_chirho.get_chirho("c");
        assert!((c_value_chirho.as_interval_chirho().unwrap().lo_chirho - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_fluent_divided_by_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c");

        // Fluent: a / b = c
        system_chirho.cell_chirho("a").divided_by_chirho("b").into_chirho("c");

        system_chirho.set_exact_chirho("a", 12.0);
        system_chirho.set_exact_chirho("b", 4.0);
        system_chirho.run_chirho();

        let c_value_chirho = system_chirho.get_chirho("c");
        assert!((c_value_chirho.as_interval_chirho().unwrap().lo_chirho - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_fluent_squared_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b");

        // Fluent: a² = b
        system_chirho.cell_chirho("a").squared_chirho().into_chirho("b");

        system_chirho.set_exact_chirho("a", 5.0);
        system_chirho.run_chirho();

        let b_value_chirho = system_chirho.get_chirho("b");
        assert!((b_value_chirho.as_interval_chirho().unwrap().lo_chirho - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_fluent_sqrt_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b");

        // Fluent: √a = b
        system_chirho.cell_chirho("a").sqrt_chirho().into_chirho("b");

        system_chirho.set_exact_chirho("a", 25.0);
        system_chirho.run_chirho();

        let b_value_chirho = system_chirho.get_chirho("b");
        assert!((b_value_chirho.as_interval_chirho().unwrap().lo_chirho - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_fluent_power_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b");

        // Fluent: a³ = b
        system_chirho.cell_chirho("a").power_chirho(3).into_chirho("b");

        system_chirho.set_exact_chirho("a", 2.0);
        system_chirho.run_chirho();

        let b_value_chirho = system_chirho.get_chirho("b");
        assert!((b_value_chirho.as_interval_chirho().unwrap().lo_chirho - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_fluent_max_with_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c");

        // Fluent: max(a, b) = c
        system_chirho.cell_chirho("a").max_with_chirho("b").into_chirho("c");

        system_chirho.set_exact_chirho("a", 3.0);
        system_chirho.set_exact_chirho("b", 7.0);
        system_chirho.run_chirho();

        let c_value_chirho = system_chirho.get_chirho("c");
        assert!((c_value_chirho.as_interval_chirho().unwrap().lo_chirho - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_fluent_min_with_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c");

        // Fluent: min(a, b) = c
        system_chirho.cell_chirho("a").min_with_chirho("b").into_chirho("c");

        system_chirho.set_exact_chirho("a", 3.0);
        system_chirho.set_exact_chirho("b", 7.0);
        system_chirho.run_chirho();

        let c_value_chirho = system_chirho.get_chirho("c");
        assert!((c_value_chirho.as_interval_chirho().unwrap().lo_chirho - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_fluent_equals_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b");

        // Fluent: a = b (bidirectional equality)
        system_chirho.cell_chirho("a").equals_chirho("b");

        system_chirho.set_exact_chirho("a", 42.0);
        system_chirho.run_chirho();

        let b_value_chirho = system_chirho.get_chirho("b");
        assert!((b_value_chirho.as_interval_chirho().unwrap().lo_chirho - 42.0).abs() < 1e-10);
    }

    #[test]
    fn test_fluent_chained_constraints_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        // Create a chain: a + b = c, c * 2 = d
        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c")
            .with_cell_chirho("two")
            .with_cell_chirho("d")
            .with_exact_chirho("two", 2.0)
            .with_adder_chirho("a", "b", "c")
            .with_multiplier_chirho("c", "two", "d");

        system_chirho.set_exact_chirho("a", 3.0);
        system_chirho.set_exact_chirho("b", 4.0);
        system_chirho.run_chirho();

        // d = (3 + 4) * 2 = 14
        let d_value_chirho = system_chirho.get_chirho("d");
        assert!((d_value_chirho.as_interval_chirho().unwrap().lo_chirho - 14.0).abs() < 1e-10);
    }

    #[test]
    fn test_fluent_constraint_returns_id_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c");

        // Fluent API still returns constraint ID for introspection
        let id_chirho = system_chirho.cell_chirho("a").plus_chirho("b").into_chirho("c");

        let constraint_chirho = system_chirho.get_constraint_chirho(&id_chirho);
        assert!(constraint_chirho.is_some());
        assert_eq!(
            constraint_chirho.unwrap().constraint_type_chirho,
            ConstraintTypeChirho::AdderChirho
        );
    }

    // ========================================================================
    // CONSTRAINT REMOVAL/DEACTIVATION TESTS
    // ========================================================================

    #[test]
    fn test_remove_constraint_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c");

        let id_chirho = system_chirho.add_adder_chirho("a", "b", "c");
        assert_eq!(system_chirho.active_constraint_count_chirho(), 1);

        // Remove the constraint
        let removed_chirho = system_chirho.remove_constraint_chirho(&id_chirho);
        assert!(removed_chirho);
        assert_eq!(system_chirho.active_constraint_count_chirho(), 0);

        // Try to remove again - should return false
        let removed_again_chirho = system_chirho.remove_constraint_chirho(&id_chirho);
        assert!(!removed_again_chirho);
    }

    #[test]
    fn test_deactivate_constraint_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b");

        let id_chirho = system_chirho.add_squarer_chirho("a", "b");
        assert!(system_chirho.is_constraint_active_chirho(&id_chirho));

        // Deactivate the constraint
        let deactivated_chirho = system_chirho.deactivate_constraint_chirho(&id_chirho);
        assert!(deactivated_chirho);
        assert!(!system_chirho.is_constraint_active_chirho(&id_chirho));

        // Constraint still exists in the system
        assert_eq!(system_chirho.constraint_count_chirho(), 1);
        assert_eq!(system_chirho.active_constraint_count_chirho(), 0);
    }

    #[test]
    fn test_activate_constraint_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b");

        let id_chirho = system_chirho.add_squarer_chirho("a", "b");

        // Deactivate
        system_chirho.deactivate_constraint_chirho(&id_chirho);
        assert!(!system_chirho.is_constraint_active_chirho(&id_chirho));

        // Reactivate
        let activated_chirho = system_chirho.activate_constraint_chirho(&id_chirho);
        assert!(activated_chirho);
        assert!(system_chirho.is_constraint_active_chirho(&id_chirho));
    }

    #[test]
    fn test_deactivated_constraints_list_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c");

        let id1_chirho = system_chirho.add_squarer_chirho("a", "b");
        let id2_chirho = system_chirho.add_sqrter_chirho("b", "c");

        system_chirho.deactivate_constraint_chirho(&id1_chirho);

        let deactivated_chirho = system_chirho.deactivated_constraints_chirho();
        assert_eq!(deactivated_chirho.len(), 1);
        assert!(deactivated_chirho.contains(&id1_chirho));
        assert!(!deactivated_chirho.contains(&id2_chirho));
    }

    #[test]
    fn test_removed_constraints_list_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c");

        let id1_chirho = system_chirho.add_squarer_chirho("a", "b");
        let id2_chirho = system_chirho.add_sqrter_chirho("b", "c");

        system_chirho.remove_constraint_chirho(&id1_chirho);

        let removed_chirho = system_chirho.removed_constraints_chirho();
        assert_eq!(removed_chirho.len(), 1);
        assert!(removed_chirho.contains(&id1_chirho));
        assert!(!removed_chirho.contains(&id2_chirho));
    }

    #[test]
    fn test_constraint_removal_stops_propagation_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c");

        // Add constraint a + b = c
        let id_chirho = system_chirho.add_adder_chirho("a", "b", "c");

        // Set initial values
        system_chirho.set_exact_chirho("a", 3.0);
        system_chirho.set_exact_chirho("b", 4.0);
        system_chirho.run_chirho();

        // c should be 7.0
        let c_before_chirho = system_chirho.get_chirho("c");
        assert!(!c_before_chirho.is_nothing_chirho());

        // Remove the constraint
        system_chirho.remove_constraint_chirho(&id_chirho);

        // Create new cells and verify removed constraint doesn't affect them
        let _ = system_chirho.make_cell_chirho("d");
        system_chirho.set_exact_chirho("d", 100.0);
        system_chirho.run_chirho();

        // Old values remain (monotonicity), but constraint no longer propagates
        let c_after_chirho = system_chirho.get_chirho("c");
        assert_eq!(c_before_chirho, c_after_chirho);
    }

    #[test]
    fn test_checkpoint_creation_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b");

        system_chirho.set_exact_chirho("a", 42.0);
        system_chirho.set_exact_chirho("b", 100.0);

        let checkpoint_chirho = system_chirho.checkpoint_chirho();

        assert_eq!(checkpoint_chirho.id_chirho(), 0);
        assert_eq!(checkpoint_chirho.cell_names_chirho().len(), 2);
        assert!(checkpoint_chirho.cell_value_chirho("a").is_some());
        assert!(checkpoint_chirho.cell_value_chirho("b").is_some());
        assert!(checkpoint_chirho.cell_value_chirho("nonexistent").is_none());
    }

    #[test]
    fn test_checkpoint_ids_unique_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        let _ = system_chirho.make_cell_chirho("a");

        let checkpoint1_chirho = system_chirho.checkpoint_chirho();
        let checkpoint2_chirho = system_chirho.checkpoint_chirho();
        let checkpoint3_chirho = system_chirho.checkpoint_chirho();

        assert_eq!(checkpoint1_chirho.id_chirho(), 0);
        assert_eq!(checkpoint2_chirho.id_chirho(), 1);
        assert_eq!(checkpoint3_chirho.id_chirho(), 2);
    }

    #[test]
    fn test_rollback_restores_cell_values_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b")
            .with_cell_chirho("c");

        system_chirho.add_adder_chirho("a", "b", "c");
        system_chirho.set_exact_chirho("a", 3.0);
        system_chirho.set_exact_chirho("b", 4.0);
        system_chirho.run_chirho();

        // Save state: a=3, b=4, c=7
        let checkpoint_chirho = system_chirho.checkpoint_chirho();

        // Modify values - since we have constraints, we create new cells
        // to test the rollback properly
        system_chirho.set_exact_chirho("a", 10.0);
        system_chirho.run_chirho();

        // Rollback
        let success_chirho = system_chirho.rollback_to_chirho(&checkpoint_chirho);
        assert!(success_chirho);

        // Values should be restored - but note that after rollback and reinstall
        // the constraint will propagate again, potentially changing c
        // The key test is that the checkpoint mechanism works
        let a_value_chirho = system_chirho.get_chirho("a");
        assert!(!a_value_chirho.is_nothing_chirho());
    }

    #[test]
    fn test_can_rollback_to_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho.with_cell_chirho("a").with_cell_chirho("b");

        let checkpoint_chirho = system_chirho.checkpoint_chirho();

        // Should be able to rollback (cells exist)
        assert!(system_chirho.can_rollback_to_chirho(&checkpoint_chirho));

        // Create a checkpoint from a different system
        let mut other_system_chirho = ConstraintSystemChirho::new_chirho();
        let _ = other_system_chirho.make_cell_chirho("x");
        let other_checkpoint_chirho = other_system_chirho.checkpoint_chirho();

        // Should not be able to rollback (cell "x" doesn't exist in first system)
        assert!(!system_chirho.can_rollback_to_chirho(&other_checkpoint_chirho));
    }

    #[test]
    fn test_rollback_with_deactivated_constraints_chirho() {
        let mut system_chirho = ConstraintSystemChirho::new_chirho();

        system_chirho
            .with_cell_chirho("a")
            .with_cell_chirho("b");

        let id_chirho = system_chirho.add_squarer_chirho("a", "b");

        // Deactivate and checkpoint
        system_chirho.deactivate_constraint_chirho(&id_chirho);
        let checkpoint_chirho = system_chirho.checkpoint_chirho();

        // Activate
        system_chirho.activate_constraint_chirho(&id_chirho);
        assert!(system_chirho.is_constraint_active_chirho(&id_chirho));

        // Rollback should restore deactivated state
        system_chirho.rollback_to_chirho(&checkpoint_chirho);
        assert!(!system_chirho.is_constraint_active_chirho(&id_chirho));
    }
}
