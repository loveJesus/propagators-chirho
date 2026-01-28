// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Unified composable network architecture.
//!
//! This module provides a single `UnifiedNetworkChirho` type that can be configured
//! with different features at compile-time using type parameters:
//!
//! - **TMS Support**: Track beliefs with justifications
//! - **Arena Allocation**: High-performance memory management
//! - **Parallel Propagation**: Multi-threaded execution
//!
//! # Example
//!
//! ```
//! use propagators_chirho::{
//!     UnifiedNetworkChirho, StandardModeChirho, TmsModeChirho,
//!     HeapAllocChirho, SequentialChirho, NumericInfoChirho,
//! };
//!
//! // Standard network (like ConstraintSystemChirho)
//! let mut network_chirho: UnifiedNetworkChirho<NumericInfoChirho> =
//!     UnifiedNetworkChirho::new_chirho();
//!
//! // With TMS enabled
//! let mut tms_network_chirho: UnifiedNetworkChirho<NumericInfoChirho, TmsModeChirho> =
//!     UnifiedNetworkChirho::with_tms_chirho();
//! ```
//!
//! # Architecture
//!
//! The unified network uses phantom types to encode configuration:
//!
//! ```text
//! UnifiedNetworkChirho<T, TmsMode, AllocMode, ExecMode>
//!                      │    │         │         │
//!                      │    │         │         └─ SequentialChirho or ParallelChirho
//!                      │    │         └─ HeapAllocChirho or ArenaAllocChirho
//!                      │    └─ StandardModeChirho or TmsModeChirho
//!                      └─ Lattice type (e.g., NumericInfoChirho)
//! ```

use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::cells_chirho::cell_chirho::CellChirho;
use crate::cells_chirho::scheduler_chirho::SchedulerChirho;
use crate::core_chirho::interval_chirho::NumericInfoChirho;
use crate::propagators_chirho::propagator_chirho::{
    IntervalAdderChirho, IntervalSubtractorChirho, IntervalMultiplierChirho,
    IntervalDividerChirho, SquarerChirho, SqrterChirho,
};

// ============================================================================
// MODE MARKERS (Phantom Types)
// ============================================================================

/// Standard mode - no TMS, simple lattice merge.
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardModeChirho;

/// TMS mode - track beliefs with justifications and premises.
#[derive(Debug, Clone, Copy, Default)]
pub struct TmsModeChirho;

/// Heap allocation mode - uses Rc for cells.
#[derive(Debug, Clone, Copy, Default)]
pub struct HeapAllocChirho;

/// Arena allocation mode - uses arena for cells.
#[cfg(feature = "arena")]
#[derive(Debug, Clone, Copy, Default)]
pub struct ArenaAllocChirho;

/// Sequential execution mode.
#[derive(Debug, Clone, Copy, Default)]
pub struct SequentialChirho;

/// Parallel execution mode.
#[cfg(feature = "parallel")]
#[derive(Debug, Clone, Copy, Default)]
pub struct ParallelChirho;

// ============================================================================
// UNIFIED CELL
// ============================================================================

/// A unified cell that can optionally support TMS features.
#[derive(Debug)]
pub struct UnifiedCellChirho<T, TmsMode = StandardModeChirho> {
    /// Cell name for debugging.
    name_chirho: String,
    /// Current content.
    content_chirho: RefCell<T>,
    /// TMS data (beliefs, justifications) - only used in TmsModeChirho.
    tms_data_chirho: RefCell<Option<TmsDataChirho<T>>>,
    /// Phantom for TMS mode.
    _tms_mode_chirho: PhantomData<TmsMode>,
}

/// TMS-specific data for a cell.
#[derive(Debug, Clone)]
pub struct TmsDataChirho<T> {
    /// Beliefs with their supporting premises.
    beliefs_chirho: Vec<(T, Vec<String>)>,
    /// Current active worldview premises.
    active_premises_chirho: Vec<String>,
}

impl<T> Default for TmsDataChirho<T> {
    fn default() -> Self {
        Self {
            beliefs_chirho: Vec::new(),
            active_premises_chirho: Vec::new(),
        }
    }
}

impl<T: Clone + Default> UnifiedCellChirho<T, StandardModeChirho> {
    /// Creates a new standard cell.
    pub fn new_chirho(name_chirho: &str) -> Self {
        Self {
            name_chirho: name_chirho.to_string(),
            content_chirho: RefCell::new(T::default()),
            tms_data_chirho: RefCell::new(None),
            _tms_mode_chirho: PhantomData,
        }
    }

    /// Returns the cell's content.
    pub fn content_chirho(&self) -> T {
        self.content_chirho.borrow().clone()
    }

    /// Sets the cell's content.
    pub fn set_content_chirho(&self, value_chirho: T) {
        *self.content_chirho.borrow_mut() = value_chirho;
    }
}

impl<T: Clone + Default> UnifiedCellChirho<T, TmsModeChirho> {
    /// Creates a new TMS-enabled cell.
    pub fn new_tms_chirho(name_chirho: &str) -> Self {
        Self {
            name_chirho: name_chirho.to_string(),
            content_chirho: RefCell::new(T::default()),
            tms_data_chirho: RefCell::new(Some(TmsDataChirho::default())),
            _tms_mode_chirho: PhantomData,
        }
    }

    /// Adds a belief with supporting premises.
    pub fn add_belief_chirho(&self, value_chirho: T, premises_chirho: Vec<String>) {
        if let Some(ref mut tms_chirho) = *self.tms_data_chirho.borrow_mut() {
            tms_chirho.beliefs_chirho.push((value_chirho, premises_chirho));
        }
    }

    /// Returns beliefs that are supported by active premises.
    pub fn active_beliefs_chirho(&self) -> Vec<T> {
        let tms_chirho = self.tms_data_chirho.borrow();
        if let Some(ref tms_data_chirho) = *tms_chirho {
            tms_data_chirho
                .beliefs_chirho
                .iter()
                .filter(|(_, premises_chirho)| {
                    premises_chirho.iter().all(|p_chirho| {
                        tms_data_chirho.active_premises_chirho.contains(p_chirho)
                    })
                })
                .map(|(v_chirho, _)| v_chirho.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Assumes a premise.
    pub fn assume_premise_chirho(&self, premise_chirho: &str) {
        if let Some(ref mut tms_chirho) = *self.tms_data_chirho.borrow_mut() {
            if !tms_chirho.active_premises_chirho.contains(&premise_chirho.to_string()) {
                tms_chirho.active_premises_chirho.push(premise_chirho.to_string());
            }
        }
    }

    /// Retracts a premise.
    pub fn retract_premise_chirho(&self, premise_chirho: &str) {
        if let Some(ref mut tms_chirho) = *self.tms_data_chirho.borrow_mut() {
            tms_chirho.active_premises_chirho.retain(|p_chirho| p_chirho != premise_chirho);
        }
    }
}

impl<T, TmsMode> UnifiedCellChirho<T, TmsMode> {
    /// Returns the cell's name.
    pub fn name_chirho(&self) -> &str {
        &self.name_chirho
    }
}

// ============================================================================
// UNIFIED NETWORK
// ============================================================================

/// A unified network that composes TMS, allocation, and execution modes.
///
/// # Type Parameters
///
/// - `T`: The lattice type for cell values
/// - `TmsMode`: `StandardModeChirho` or `TmsModeChirho`
/// - `AllocMode`: `HeapAllocChirho` (or `ArenaAllocChirho` with feature)
/// - `ExecMode`: `SequentialChirho` (or `ParallelChirho` with feature)
pub struct UnifiedNetworkChirho<
    T = NumericInfoChirho,
    TmsMode = StandardModeChirho,
    AllocMode = HeapAllocChirho,
    ExecMode = SequentialChirho,
> {
    /// Cells indexed by name.
    cells_chirho: HashMap<String, Rc<UnifiedCellChirho<T, TmsMode>>>,
    /// The scheduler.
    scheduler_chirho: SchedulerChirho,
    /// Constraint counter.
    next_constraint_id_chirho: usize,
    /// Phantom markers.
    _alloc_mode_chirho: PhantomData<AllocMode>,
    _exec_mode_chirho: PhantomData<ExecMode>,
}

impl<T: Clone + Default> UnifiedNetworkChirho<T, StandardModeChirho, HeapAllocChirho, SequentialChirho> {
    /// Creates a new standard network.
    pub fn new_chirho() -> Self {
        Self {
            cells_chirho: HashMap::new(),
            scheduler_chirho: SchedulerChirho::new_chirho(),
            next_constraint_id_chirho: 0,
            _alloc_mode_chirho: PhantomData,
            _exec_mode_chirho: PhantomData,
        }
    }
}

impl<T: Clone + Default> UnifiedNetworkChirho<T, TmsModeChirho, HeapAllocChirho, SequentialChirho> {
    /// Creates a new TMS-enabled network.
    pub fn with_tms_chirho() -> Self {
        Self {
            cells_chirho: HashMap::new(),
            scheduler_chirho: SchedulerChirho::new_chirho(),
            next_constraint_id_chirho: 0,
            _alloc_mode_chirho: PhantomData,
            _exec_mode_chirho: PhantomData,
        }
    }
}

impl<T, TmsMode, AllocMode, ExecMode> UnifiedNetworkChirho<T, TmsMode, AllocMode, ExecMode>
where
    T: Clone + Default,
{
    /// Creates or retrieves a cell by name.
    pub fn cell_chirho(&mut self, name_chirho: &str) -> Rc<UnifiedCellChirho<T, TmsMode>>
    where
        TmsMode: Default,
    {
        self.cells_chirho
            .entry(name_chirho.to_string())
            .or_insert_with(|| {
                Rc::new(UnifiedCellChirho {
                    name_chirho: name_chirho.to_string(),
                    content_chirho: RefCell::new(T::default()),
                    tms_data_chirho: RefCell::new(None),
                    _tms_mode_chirho: PhantomData,
                })
            })
            .clone()
    }

    /// Returns the number of cells.
    pub fn cell_count_chirho(&self) -> usize {
        self.cells_chirho.len()
    }

    /// Runs propagation to fixpoint.
    pub fn run_chirho(&self) {
        self.scheduler_chirho.run_chirho();
    }

    /// Returns the scheduler.
    pub fn scheduler_chirho(&self) -> &SchedulerChirho {
        &self.scheduler_chirho
    }
}

// ============================================================================
// NUMERIC CONSTRAINTS (for NumericInfoChirho)
// ============================================================================

impl<TmsMode, AllocMode, ExecMode> UnifiedNetworkChirho<NumericInfoChirho, TmsMode, AllocMode, ExecMode>
where
    TmsMode: Default,
{
    /// Creates a cell and sets it to an exact value.
    pub fn make_exact_chirho(&mut self, name_chirho: &str, value_chirho: f64) -> Rc<CellChirho<NumericInfoChirho>> {
        let cell_chirho = CellChirho::new_chirho(name_chirho);
        cell_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(value_chirho), &self.scheduler_chirho);
        cell_chirho
    }

    /// Creates a cell with an interval value.
    pub fn make_interval_chirho(
        &mut self,
        name_chirho: &str,
        lo_chirho: f64,
        hi_chirho: f64,
    ) -> Rc<CellChirho<NumericInfoChirho>> {
        let cell_chirho = CellChirho::new_chirho(name_chirho);
        cell_chirho.add_content_chirho(
            NumericInfoChirho::interval_chirho(lo_chirho, hi_chirho),
            &self.scheduler_chirho,
        );
        cell_chirho
    }

    /// Creates a cell with no initial value.
    pub fn make_cell_chirho(&mut self, name_chirho: &str) -> Rc<CellChirho<NumericInfoChirho>> {
        CellChirho::new_chirho(name_chirho)
    }

    /// Adds an adder constraint: a + b = c.
    pub fn add_adder_chirho(
        &mut self,
        a_chirho: Rc<CellChirho<NumericInfoChirho>>,
        b_chirho: Rc<CellChirho<NumericInfoChirho>>,
        c_chirho: Rc<CellChirho<NumericInfoChirho>>,
    ) -> usize {
        let id_chirho = self.next_constraint_id_chirho;
        self.next_constraint_id_chirho += 1;
        IntervalAdderChirho::install_chirho(a_chirho, b_chirho, c_chirho, &self.scheduler_chirho);
        id_chirho
    }

    /// Adds a subtractor constraint: a - b = c.
    pub fn add_subtractor_chirho(
        &mut self,
        a_chirho: Rc<CellChirho<NumericInfoChirho>>,
        b_chirho: Rc<CellChirho<NumericInfoChirho>>,
        c_chirho: Rc<CellChirho<NumericInfoChirho>>,
    ) -> usize {
        let id_chirho = self.next_constraint_id_chirho;
        self.next_constraint_id_chirho += 1;
        IntervalSubtractorChirho::install_chirho(a_chirho, b_chirho, c_chirho, &self.scheduler_chirho);
        id_chirho
    }

    /// Adds a multiplier constraint: a * b = c.
    pub fn add_multiplier_chirho(
        &mut self,
        a_chirho: Rc<CellChirho<NumericInfoChirho>>,
        b_chirho: Rc<CellChirho<NumericInfoChirho>>,
        c_chirho: Rc<CellChirho<NumericInfoChirho>>,
    ) -> usize {
        let id_chirho = self.next_constraint_id_chirho;
        self.next_constraint_id_chirho += 1;
        IntervalMultiplierChirho::install_chirho(a_chirho, b_chirho, c_chirho, &self.scheduler_chirho);
        id_chirho
    }

    /// Adds a divider constraint: a / b = c.
    pub fn add_divider_chirho(
        &mut self,
        a_chirho: Rc<CellChirho<NumericInfoChirho>>,
        b_chirho: Rc<CellChirho<NumericInfoChirho>>,
        c_chirho: Rc<CellChirho<NumericInfoChirho>>,
    ) -> usize {
        let id_chirho = self.next_constraint_id_chirho;
        self.next_constraint_id_chirho += 1;
        IntervalDividerChirho::install_chirho(a_chirho, b_chirho, c_chirho, &self.scheduler_chirho);
        id_chirho
    }

    /// Adds a squarer constraint: a² = b.
    pub fn add_squarer_chirho(
        &mut self,
        a_chirho: Rc<CellChirho<NumericInfoChirho>>,
        b_chirho: Rc<CellChirho<NumericInfoChirho>>,
    ) -> usize {
        let id_chirho = self.next_constraint_id_chirho;
        self.next_constraint_id_chirho += 1;
        SquarerChirho::install_chirho(a_chirho, b_chirho, &self.scheduler_chirho);
        id_chirho
    }

    /// Adds a square root constraint: √a = b.
    pub fn add_sqrter_chirho(
        &mut self,
        a_chirho: Rc<CellChirho<NumericInfoChirho>>,
        b_chirho: Rc<CellChirho<NumericInfoChirho>>,
    ) -> usize {
        let id_chirho = self.next_constraint_id_chirho;
        self.next_constraint_id_chirho += 1;
        SqrterChirho::install_chirho(a_chirho, b_chirho, &self.scheduler_chirho);
        id_chirho
    }
}

// ============================================================================
// BUILDER PATTERN
// ============================================================================

/// Builder for creating unified networks with desired configuration.
///
/// # Example
///
/// ```
/// use propagators_chirho::{NetworkBuilderChirho, NumericInfoChirho};
///
/// let network_chirho = NetworkBuilderChirho::<NumericInfoChirho>::new_chirho()
///     .with_tms_chirho()
///     .build_chirho();
/// ```
pub struct NetworkBuilderChirho<T = NumericInfoChirho> {
    _marker_chirho: PhantomData<T>,
    use_tms_chirho: bool,
}

impl<T: Clone + Default> NetworkBuilderChirho<T> {
    /// Creates a new network builder.
    pub fn new_chirho() -> Self {
        Self {
            _marker_chirho: PhantomData,
            use_tms_chirho: false,
        }
    }

    /// Enables TMS mode.
    pub fn with_tms_chirho(mut self) -> Self {
        self.use_tms_chirho = true;
        self
    }

    /// Builds a standard network.
    pub fn build_chirho(self) -> UnifiedNetworkChirho<T, StandardModeChirho, HeapAllocChirho, SequentialChirho> {
        UnifiedNetworkChirho::new_chirho()
    }

    /// Builds a TMS-enabled network.
    pub fn build_tms_chirho(self) -> UnifiedNetworkChirho<T, TmsModeChirho, HeapAllocChirho, SequentialChirho> {
        UnifiedNetworkChirho::with_tms_chirho()
    }
}

impl<T: Clone + Default> Default for NetworkBuilderChirho<T> {
    fn default() -> Self {
        Self::new_chirho()
    }
}

// ============================================================================
// TYPE ALIASES
// ============================================================================

/// Standard numeric network (equivalent to ConstraintSystemChirho).
pub type NumericNetworkChirho = UnifiedNetworkChirho<NumericInfoChirho, StandardModeChirho, HeapAllocChirho, SequentialChirho>;

/// TMS-enabled numeric network.
pub type TmsNumericNetworkChirho = UnifiedNetworkChirho<NumericInfoChirho, TmsModeChirho, HeapAllocChirho, SequentialChirho>;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_standard_network_creation_chirho() {
        let network_chirho: NumericNetworkChirho = UnifiedNetworkChirho::new_chirho();
        assert_eq!(network_chirho.cell_count_chirho(), 0);
    }

    #[test]
    fn test_tms_network_creation_chirho() {
        let network_chirho: TmsNumericNetworkChirho = UnifiedNetworkChirho::with_tms_chirho();
        assert_eq!(network_chirho.cell_count_chirho(), 0);
    }

    #[test]
    fn test_builder_standard_chirho() {
        let network_chirho = NetworkBuilderChirho::<NumericInfoChirho>::new_chirho().build_chirho();
        assert_eq!(network_chirho.cell_count_chirho(), 0);
    }

    #[test]
    fn test_builder_tms_chirho() {
        let network_chirho = NetworkBuilderChirho::<NumericInfoChirho>::new_chirho().build_tms_chirho();
        assert_eq!(network_chirho.cell_count_chirho(), 0);
    }

    #[test]
    fn test_standard_cell_chirho() {
        let cell_chirho = UnifiedCellChirho::<NumericInfoChirho, StandardModeChirho>::new_chirho("x");
        assert_eq!(cell_chirho.name_chirho(), "x");
    }

    #[test]
    fn test_tms_cell_beliefs_chirho() {
        let cell_chirho = UnifiedCellChirho::<NumericInfoChirho, TmsModeChirho>::new_tms_chirho("x");

        // Add beliefs
        cell_chirho.add_belief_chirho(NumericInfoChirho::exact_chirho(5.0), vec!["sensor_a".to_string()]);
        cell_chirho.add_belief_chirho(NumericInfoChirho::exact_chirho(10.0), vec!["sensor_b".to_string()]);

        // No premises assumed - no active beliefs
        assert_eq!(cell_chirho.active_beliefs_chirho().len(), 0);

        // Assume sensor_a
        cell_chirho.assume_premise_chirho("sensor_a");
        let beliefs_chirho = cell_chirho.active_beliefs_chirho();
        assert_eq!(beliefs_chirho.len(), 1);

        // Assume sensor_b too
        cell_chirho.assume_premise_chirho("sensor_b");
        let beliefs_chirho = cell_chirho.active_beliefs_chirho();
        assert_eq!(beliefs_chirho.len(), 2);

        // Retract sensor_a
        cell_chirho.retract_premise_chirho("sensor_a");
        let beliefs_chirho = cell_chirho.active_beliefs_chirho();
        assert_eq!(beliefs_chirho.len(), 1);
    }

    #[test]
    fn test_numeric_constraints_chirho() {
        let mut network_chirho: NumericNetworkChirho = UnifiedNetworkChirho::new_chirho();

        let a_chirho = network_chirho.make_exact_chirho("a", 3.0);
        let b_chirho = network_chirho.make_exact_chirho("b", 4.0);
        let c_chirho = network_chirho.make_cell_chirho("c");

        network_chirho.add_adder_chirho(a_chirho.clone(), b_chirho.clone(), c_chirho.clone());
        network_chirho.run_chirho();

        let result_chirho = c_chirho.content_chirho();
        if let Some(interval_chirho) = result_chirho.as_interval_chirho() {
            assert!((interval_chirho.lo_chirho - 7.0).abs() < 0.001);
            assert!((interval_chirho.hi_chirho - 7.0).abs() < 0.001);
        } else {
            panic!("Expected interval result");
        }
    }

    #[test]
    fn test_temperature_conversion_chirho() {
        let mut network_chirho: NumericNetworkChirho = UnifiedNetworkChirho::new_chirho();

        // F = C * 9/5 + 32
        let celsius_chirho = network_chirho.make_exact_chirho("celsius", 100.0);
        let nine_fifths_chirho = network_chirho.make_exact_chirho("nine_fifths", 1.8);
        let thirty_two_chirho = network_chirho.make_exact_chirho("thirty_two", 32.0);
        let product_chirho = network_chirho.make_cell_chirho("product");
        let fahrenheit_chirho = network_chirho.make_cell_chirho("fahrenheit");

        network_chirho.add_multiplier_chirho(
            celsius_chirho.clone(),
            nine_fifths_chirho.clone(),
            product_chirho.clone(),
        );
        network_chirho.add_adder_chirho(
            product_chirho.clone(),
            thirty_two_chirho.clone(),
            fahrenheit_chirho.clone(),
        );

        network_chirho.run_chirho();

        let result_chirho = fahrenheit_chirho.content_chirho();
        if let Some(interval_chirho) = result_chirho.as_interval_chirho() {
            assert!((interval_chirho.lo_chirho - 212.0).abs() < 0.001);
        } else {
            panic!("Expected interval result");
        }
    }

    #[test]
    fn test_bidirectional_chirho() {
        let mut network_chirho: NumericNetworkChirho = UnifiedNetworkChirho::new_chirho();

        // a + b = c, but we know c and a, should derive b
        let a_chirho = network_chirho.make_exact_chirho("a", 3.0);
        let b_chirho = network_chirho.make_cell_chirho("b");
        let c_chirho = network_chirho.make_exact_chirho("c", 7.0);

        network_chirho.add_adder_chirho(a_chirho.clone(), b_chirho.clone(), c_chirho.clone());
        network_chirho.run_chirho();

        let result_chirho = b_chirho.content_chirho();
        if let Some(interval_chirho) = result_chirho.as_interval_chirho() {
            assert!((interval_chirho.lo_chirho - 4.0).abs() < 0.001);
        } else {
            panic!("Expected interval result");
        }
    }

    #[test]
    fn test_type_alias_equivalence_chirho() {
        // These should be the same type
        let _n1: NumericNetworkChirho = UnifiedNetworkChirho::new_chirho();
        let _n2: UnifiedNetworkChirho<NumericInfoChirho, StandardModeChirho, HeapAllocChirho, SequentialChirho> =
            UnifiedNetworkChirho::new_chirho();
    }

    #[test]
    fn test_subtractor_constraint_chirho() {
        let mut network_chirho: NumericNetworkChirho = UnifiedNetworkChirho::new_chirho();

        let a_chirho = network_chirho.make_exact_chirho("a", 10.0);
        let b_chirho = network_chirho.make_exact_chirho("b", 3.0);
        let c_chirho = network_chirho.make_cell_chirho("c");

        network_chirho.add_subtractor_chirho(a_chirho.clone(), b_chirho.clone(), c_chirho.clone());
        network_chirho.run_chirho();

        let result_chirho = c_chirho.content_chirho();
        if let Some(interval_chirho) = result_chirho.as_interval_chirho() {
            assert!((interval_chirho.lo_chirho - 7.0).abs() < 0.001);
        } else {
            panic!("Expected interval result");
        }
    }

    #[test]
    fn test_divider_constraint_chirho() {
        let mut network_chirho: NumericNetworkChirho = UnifiedNetworkChirho::new_chirho();

        let a_chirho = network_chirho.make_exact_chirho("a", 20.0);
        let b_chirho = network_chirho.make_exact_chirho("b", 4.0);
        let c_chirho = network_chirho.make_cell_chirho("c");

        network_chirho.add_divider_chirho(a_chirho.clone(), b_chirho.clone(), c_chirho.clone());
        network_chirho.run_chirho();

        let result_chirho = c_chirho.content_chirho();
        if let Some(interval_chirho) = result_chirho.as_interval_chirho() {
            assert!((interval_chirho.lo_chirho - 5.0).abs() < 0.001);
        } else {
            panic!("Expected interval result");
        }
    }

    #[test]
    fn test_squarer_constraint_chirho() {
        let mut network_chirho: NumericNetworkChirho = UnifiedNetworkChirho::new_chirho();

        let a_chirho = network_chirho.make_exact_chirho("a", 5.0);
        let b_chirho = network_chirho.make_cell_chirho("b");

        network_chirho.add_squarer_chirho(a_chirho.clone(), b_chirho.clone());
        network_chirho.run_chirho();

        let result_chirho = b_chirho.content_chirho();
        if let Some(interval_chirho) = result_chirho.as_interval_chirho() {
            assert!((interval_chirho.lo_chirho - 25.0).abs() < 0.001);
        } else {
            panic!("Expected interval result");
        }
    }

    #[test]
    fn test_sqrter_constraint_chirho() {
        let mut network_chirho: NumericNetworkChirho = UnifiedNetworkChirho::new_chirho();

        let a_chirho = network_chirho.make_exact_chirho("a", 16.0);
        let b_chirho = network_chirho.make_cell_chirho("b");

        network_chirho.add_sqrter_chirho(a_chirho.clone(), b_chirho.clone());
        network_chirho.run_chirho();

        let result_chirho = b_chirho.content_chirho();
        if let Some(interval_chirho) = result_chirho.as_interval_chirho() {
            assert!((interval_chirho.lo_chirho - 4.0).abs() < 0.001);
        } else {
            panic!("Expected interval result");
        }
    }

    #[test]
    fn test_make_interval_chirho() {
        let mut network_chirho: NumericNetworkChirho = UnifiedNetworkChirho::new_chirho();

        let cell_chirho = network_chirho.make_interval_chirho("temp", 20.0, 25.0);

        let content_chirho = cell_chirho.content_chirho();
        if let Some(interval_chirho) = content_chirho.as_interval_chirho() {
            assert!((interval_chirho.lo_chirho - 20.0).abs() < 0.001);
            assert!((interval_chirho.hi_chirho - 25.0).abs() < 0.001);
        } else {
            panic!("Expected interval result");
        }
    }

    #[test]
    fn test_pythagorean_unified_chirho() {
        // a² + b² = c² where a=3, b=4 => c=5
        let mut network_chirho: NumericNetworkChirho = UnifiedNetworkChirho::new_chirho();

        let a_chirho = network_chirho.make_exact_chirho("a", 3.0);
        let b_chirho = network_chirho.make_exact_chirho("b", 4.0);
        let a_sq_chirho = network_chirho.make_cell_chirho("a_sq");
        let b_sq_chirho = network_chirho.make_cell_chirho("b_sq");
        let c_sq_chirho = network_chirho.make_cell_chirho("c_sq");
        let c_chirho = network_chirho.make_cell_chirho("c");

        network_chirho.add_squarer_chirho(a_chirho.clone(), a_sq_chirho.clone());
        network_chirho.add_squarer_chirho(b_chirho.clone(), b_sq_chirho.clone());
        network_chirho.add_adder_chirho(a_sq_chirho.clone(), b_sq_chirho.clone(), c_sq_chirho.clone());
        network_chirho.add_sqrter_chirho(c_sq_chirho.clone(), c_chirho.clone());

        network_chirho.run_chirho();

        let result_chirho = c_chirho.content_chirho();
        if let Some(interval_chirho) = result_chirho.as_interval_chirho() {
            assert!((interval_chirho.lo_chirho - 5.0).abs() < 0.001);
        } else {
            panic!("Expected interval result");
        }
    }
}
