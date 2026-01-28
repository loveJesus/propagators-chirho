// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Amb operator for nondeterministic choice.
//!
//! The `amb` operator (ambiguous choice) is a powerful primitive for
//! constraint satisfaction. It represents a choice among multiple values,
//! with automatic backtracking when contradictions occur.
//!
//! # Background
//!
//! The amb operator was introduced by John McCarthy and has been used
//! extensively in logic programming and constraint solving.
//!
//! # Features
//!
//! - `backtrack`: Enables dependency-directed backtracking, which uses
//!   nogood information to focus the search and skip irrelevant choices.
//!
//! # References
//!
//! - McCarthy, J. (1963). *A Basis for a Mathematical Theory of Computation*.
//!
//! - Radul, A., & Sussman, G. J. (2009). *The Art of the Propagator*, Section 9.
//!   <https://dspace.mit.edu/handle/1721.1/44215>
//!
//! - Stallman, R. M., & Sussman, G. J. (1977). *Forward Reasoning and
//!   Dependency-Directed Backtracking*. Artificial Intelligence, 9(2), 135-196.

use std::cell::RefCell;
use std::collections::HashSet;

use crate::core_chirho::interval_chirho::NumericInfoChirho;

/// A nondeterministic choice among values.
///
/// An `amb` represents a choice point where any of the given values
/// might be the "right" one. During search, we try each value until
/// we find one that doesn't lead to contradiction.
///
/// # Example
///
/// ```
/// use propagators_chirho::{AmbChirho, NumericInfoChirho};
///
/// // Create a choice between 1, 2, and 3
/// let amb_chirho = AmbChirho::new_chirho(vec![
///     NumericInfoChirho::exact_chirho(1.0),
///     NumericInfoChirho::exact_chirho(2.0),
///     NumericInfoChirho::exact_chirho(3.0),
/// ]);
///
/// // Try each choice
/// for (i, choice) in amb_chirho.choices_chirho().iter().enumerate() {
///     println!("Choice {}: {:?}", i, choice);
/// }
/// ```
#[derive(Clone, Debug)]
pub struct AmbChirho {
    /// The possible values.
    choices_chirho: Vec<NumericInfoChirho>,
    /// Current choice index (for iteration).
    current_index_chirho: RefCell<usize>,
}

impl AmbChirho {
    /// Creates a new amb with the given choices.
    pub fn new_chirho(choices_chirho: Vec<NumericInfoChirho>) -> Self {
        Self {
            choices_chirho,
            current_index_chirho: RefCell::new(0),
        }
    }

    /// Creates an amb for integer values in a range.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::AmbChirho;
    ///
    /// // Choices: 1, 2, 3, 4, 5
    /// let amb_chirho = AmbChirho::range_chirho(1, 5);
    /// assert_eq!(amb_chirho.choices_chirho().len(), 5);
    /// ```
    pub fn range_chirho(lo_chirho: i32, hi_chirho: i32) -> Self {
        let choices_chirho = (lo_chirho..=hi_chirho)
            .map(|i_chirho| NumericInfoChirho::exact_chirho(f64::from(i_chirho)))
            .collect();
        Self::new_chirho(choices_chirho)
    }

    /// Returns a reference to the choices.
    pub fn choices_chirho(&self) -> &[NumericInfoChirho] {
        &self.choices_chirho
    }

    /// Returns the number of choices.
    pub fn choice_count_chirho(&self) -> usize {
        self.choices_chirho.len()
    }

    /// Returns the current choice (if any).
    pub fn current_chirho(&self) -> Option<NumericInfoChirho> {
        let index_chirho = *self.current_index_chirho.borrow();
        self.choices_chirho.get(index_chirho).copied()
    }

    /// Advances to the next choice.
    ///
    /// Returns `true` if there was a next choice, `false` if exhausted.
    pub fn next_chirho(&self) -> bool {
        let mut index_chirho = self.current_index_chirho.borrow_mut();
        if *index_chirho + 1 < self.choices_chirho.len() {
            *index_chirho += 1;
            true
        } else {
            false
        }
    }

    /// Resets to the first choice.
    pub fn reset_chirho(&self) {
        *self.current_index_chirho.borrow_mut() = 0;
    }

    /// Returns `true` if all choices have been exhausted.
    pub fn is_exhausted_chirho(&self) -> bool {
        *self.current_index_chirho.borrow() >= self.choices_chirho.len()
    }
}

/// Result of a backtracking search.
#[derive(Clone, Debug)]
pub enum SearchResultChirho {
    /// A solution was found with these cell values.
    SolutionChirho(Vec<NumericInfoChirho>),
    /// No solution exists.
    NoSolutionChirho,
    /// Search was abandoned (e.g., timeout).
    AbandonedChirho,
}

/// A backtracking search engine for constraint satisfaction.
///
/// Given cells with amb choices and constraints (propagators), finds
/// assignments that satisfy all constraints.
///
/// # Algorithm
///
/// 1. Select an unassigned amb
/// 2. Try each choice in order
/// 3. Propagate constraints
/// 4. If contradiction, backtrack and try next choice
/// 5. If all choices exhausted, backtrack further
/// 6. If all ambs assigned without contradiction, solution found
///
/// # Example
///
/// ```
/// use propagators_chirho::{
///     BacktrackingSearchChirho, AmbChirho, CellChirho,
///     NumericInfoChirho, SchedulerChirho
/// };
/// use std::rc::Rc;
///
/// let scheduler_chirho = SchedulerChirho::new_chirho();
///
/// // Create cells with amb choices
/// let x_chirho: Rc<CellChirho<NumericInfoChirho>> = CellChirho::new_chirho("x");
/// let y_chirho: Rc<CellChirho<NumericInfoChirho>> = CellChirho::new_chirho("y");
///
/// // Create search engine
/// let search_chirho = BacktrackingSearchChirho::new_chirho();
/// ```
pub struct BacktrackingSearchChirho {
    /// Maximum number of backtracks before giving up.
    max_backtracks_chirho: usize,
    /// Number of backtracks so far.
    backtrack_count_chirho: RefCell<usize>,
}

impl BacktrackingSearchChirho {
    /// Creates a new search engine with default settings.
    pub fn new_chirho() -> Self {
        Self {
            max_backtracks_chirho: 10000,
            backtrack_count_chirho: RefCell::new(0),
        }
    }

    /// Creates a search engine with a custom backtrack limit.
    pub fn with_limit_chirho(max_backtracks_chirho: usize) -> Self {
        Self {
            max_backtracks_chirho,
            backtrack_count_chirho: RefCell::new(0),
        }
    }

    /// Returns the number of backtracks performed.
    pub fn backtrack_count_chirho(&self) -> usize {
        *self.backtrack_count_chirho.borrow()
    }

    /// Resets the backtrack counter.
    pub fn reset_chirho(&self) {
        *self.backtrack_count_chirho.borrow_mut() = 0;
    }

    /// Searches for a solution that satisfies all constraints.
    ///
    /// # Arguments
    ///
    /// * `ambs_chirho` - The amb choices indexed by cell
    /// * `cells_chirho` - The cells to assign
    /// * `check_consistent_chirho` - Function to check if current state is consistent
    ///
    /// This is a simplified interface. For full constraint networks,
    /// use the constraint system API.
    pub fn search_simple_chirho<F>(
        &self,
        ambs_chirho: &[AmbChirho],
        check_consistent_chirho: F,
    ) -> SearchResultChirho
    where
        F: Fn(&[NumericInfoChirho]) -> bool,
    {
        self.reset_chirho();

        let mut assignment_chirho: Vec<Option<NumericInfoChirho>> = vec![None; ambs_chirho.len()];

        if self.search_recursive_chirho(
            ambs_chirho,
            &mut assignment_chirho,
            0,
            &check_consistent_chirho,
        ) {
            let solution_chirho: Vec<NumericInfoChirho> = assignment_chirho
                .into_iter()
                .map(|opt_chirho| opt_chirho.unwrap())
                .collect();
            SearchResultChirho::SolutionChirho(solution_chirho)
        } else if *self.backtrack_count_chirho.borrow() >= self.max_backtracks_chirho {
            SearchResultChirho::AbandonedChirho
        } else {
            SearchResultChirho::NoSolutionChirho
        }
    }

    fn search_recursive_chirho<F>(
        &self,
        ambs_chirho: &[AmbChirho],
        assignment_chirho: &mut [Option<NumericInfoChirho>],
        index_chirho: usize,
        check_consistent_chirho: &F,
    ) -> bool
    where
        F: Fn(&[NumericInfoChirho]) -> bool,
    {
        // Check backtrack limit
        if *self.backtrack_count_chirho.borrow() >= self.max_backtracks_chirho {
            return false;
        }

        // Base case: all variables assigned
        if index_chirho >= ambs_chirho.len() {
            let values_chirho: Vec<NumericInfoChirho> = assignment_chirho
                .iter()
                .map(|opt_chirho| opt_chirho.unwrap())
                .collect();
            return check_consistent_chirho(&values_chirho);
        }

        // Try each choice for this variable
        for choice_chirho in ambs_chirho[index_chirho].choices_chirho() {
            assignment_chirho[index_chirho] = Some(*choice_chirho);

            // Early pruning: check partial consistency
            let partial_chirho: Vec<NumericInfoChirho> = assignment_chirho
                .iter()
                .take(index_chirho + 1)
                .map(|opt_chirho| opt_chirho.unwrap())
                .collect();

            if check_consistent_chirho(&partial_chirho) {
                if self.search_recursive_chirho(
                    ambs_chirho,
                    assignment_chirho,
                    index_chirho + 1,
                    check_consistent_chirho,
                ) {
                    return true;
                }
            }

            *self.backtrack_count_chirho.borrow_mut() += 1;
        }

        assignment_chirho[index_chirho] = None;
        false
    }
}

impl Default for BacktrackingSearchChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

/// Dependency-directed backtracking search.
///
/// Unlike chronological backtracking (which always backtracks to the most
/// recent choice), dependency-directed backtracking analyzes the conflict
/// to determine which choice actually caused the contradiction, and jumps
/// directly to that choice.
///
/// This can dramatically reduce search time by avoiding futile exploration
/// of unrelated choices.
///
/// # Feature Flag
///
/// Requires the `backtrack` feature:
///
/// ```toml
/// [dependencies]
/// propagators-chirho = { version = "0.1", features = ["backtrack"] }
/// ```
///
/// # Algorithm
///
/// 1. When a contradiction is detected, collect the premises involved
/// 2. Find the most recent choice that contributed to those premises
/// 3. Backtrack directly to that choice (skipping intermediate choices)
/// 4. Record the nogood so we don't repeat the same mistake
///
/// # Example
///
/// ```
/// use propagators_chirho::{
///     DependencyDirectedSearchChirho, AmbChirho, NumericInfoChirho
/// };
///
/// let search_chirho = DependencyDirectedSearchChirho::new_chirho();
///
/// // Create amb choices with premise names
/// let ambs_chirho = vec![
///     ("x".to_string(), AmbChirho::range_chirho(1, 5)),
///     ("y".to_string(), AmbChirho::range_chirho(1, 5)),
/// ];
/// ```
#[derive(Clone, Debug)]
pub struct DependencyDirectedSearchChirho {
    /// Maximum backtracks before giving up.
    max_backtracks_chirho: usize,
    /// Current backtrack count.
    backtrack_count_chirho: RefCell<usize>,
    /// Nogoods learned during search.
    nogoods_chirho: RefCell<Vec<HashSet<String>>>,
    /// Statistics: chronological backtracks avoided.
    jumps_chirho: RefCell<usize>,
}

impl DependencyDirectedSearchChirho {
    /// Creates a new dependency-directed search engine.
    pub fn new_chirho() -> Self {
        Self {
            max_backtracks_chirho: 10000,
            backtrack_count_chirho: RefCell::new(0),
            nogoods_chirho: RefCell::new(Vec::new()),
            jumps_chirho: RefCell::new(0),
        }
    }

    /// Creates a search engine with a custom backtrack limit.
    pub fn with_limit_chirho(max_backtracks_chirho: usize) -> Self {
        Self {
            max_backtracks_chirho,
            backtrack_count_chirho: RefCell::new(0),
            nogoods_chirho: RefCell::new(Vec::new()),
            jumps_chirho: RefCell::new(0),
        }
    }

    /// Returns the number of backtracks performed.
    pub fn backtrack_count_chirho(&self) -> usize {
        *self.backtrack_count_chirho.borrow()
    }

    /// Returns the number of dependency-directed jumps (skipped levels).
    pub fn jump_count_chirho(&self) -> usize {
        *self.jumps_chirho.borrow()
    }

    /// Returns the learned nogoods.
    pub fn nogoods_chirho(&self) -> Vec<HashSet<String>> {
        self.nogoods_chirho.borrow().clone()
    }

    /// Resets all counters and learned nogoods.
    pub fn reset_chirho(&self) {
        *self.backtrack_count_chirho.borrow_mut() = 0;
        *self.jumps_chirho.borrow_mut() = 0;
        self.nogoods_chirho.borrow_mut().clear();
    }

    /// Searches for a solution using dependency-directed backtracking.
    ///
    /// # Arguments
    ///
    /// * `ambs_chirho` - Named amb choices (premise name, amb)
    /// * `check_consistent_chirho` - Returns `Ok(())` if consistent, or
    ///   `Err(conflicting_premises)` if contradiction
    pub fn search_chirho<F>(
        &self,
        ambs_chirho: &[(String, AmbChirho)],
        check_consistent_chirho: F,
    ) -> SearchResultChirho
    where
        F: Fn(&[(String, NumericInfoChirho)]) -> Result<(), HashSet<String>>,
    {
        self.reset_chirho();

        let mut assignment_chirho: Vec<Option<(String, NumericInfoChirho)>> =
            vec![None; ambs_chirho.len()];

        match self.search_recursive_dd_chirho(
            ambs_chirho,
            &mut assignment_chirho,
            0,
            &check_consistent_chirho,
        ) {
            Ok(()) => {
                let solution_chirho: Vec<NumericInfoChirho> = assignment_chirho
                    .into_iter()
                    .map(|opt_chirho| opt_chirho.unwrap().1)
                    .collect();
                SearchResultChirho::SolutionChirho(solution_chirho)
            }
            Err(_) => {
                if *self.backtrack_count_chirho.borrow() >= self.max_backtracks_chirho {
                    SearchResultChirho::AbandonedChirho
                } else {
                    SearchResultChirho::NoSolutionChirho
                }
            }
        }
    }

    fn search_recursive_dd_chirho<F>(
        &self,
        ambs_chirho: &[(String, AmbChirho)],
        assignment_chirho: &mut [Option<(String, NumericInfoChirho)>],
        index_chirho: usize,
        check_consistent_chirho: &F,
    ) -> Result<(), HashSet<String>>
    where
        F: Fn(&[(String, NumericInfoChirho)]) -> Result<(), HashSet<String>>,
    {
        // Check backtrack limit
        if *self.backtrack_count_chirho.borrow() >= self.max_backtracks_chirho {
            return Err(HashSet::new());
        }

        // Base case: all variables assigned
        if index_chirho >= ambs_chirho.len() {
            let values_chirho: Vec<(String, NumericInfoChirho)> = assignment_chirho
                .iter()
                .map(|opt_chirho| opt_chirho.clone().unwrap())
                .collect();
            return check_consistent_chirho(&values_chirho);
        }

        let (premise_name_chirho, amb_chirho) = &ambs_chirho[index_chirho];

        // Check if current assignment contains a known nogood
        let current_premises_chirho: HashSet<String> = assignment_chirho
            .iter()
            .take(index_chirho)
            .filter_map(|opt_chirho| {
                opt_chirho
                    .as_ref()
                    .map(|(name_chirho, _)| name_chirho.clone())
            })
            .collect();

        for nogood_chirho in self.nogoods_chirho.borrow().iter() {
            if nogood_chirho.is_subset(&current_premises_chirho) {
                // Already known to be inconsistent
                return Err(nogood_chirho.clone());
            }
        }

        let mut conflict_premises_chirho: HashSet<String> = HashSet::new();

        for choice_chirho in amb_chirho.choices_chirho() {
            assignment_chirho[index_chirho] = Some((premise_name_chirho.clone(), *choice_chirho));

            // Check partial consistency
            let partial_chirho: Vec<(String, NumericInfoChirho)> = assignment_chirho
                .iter()
                .take(index_chirho + 1)
                .filter_map(Option::clone)
                .collect();

            match check_consistent_chirho(&partial_chirho) {
                Ok(()) => {
                    // Recurse
                    match self.search_recursive_dd_chirho(
                        ambs_chirho,
                        assignment_chirho,
                        index_chirho + 1,
                        check_consistent_chirho,
                    ) {
                        Ok(()) => return Ok(()),
                        Err(child_conflict_chirho) => {
                            // Check if this level is involved in the conflict
                            if !child_conflict_chirho.contains(premise_name_chirho) {
                                // This level is not involved - jump back!
                                *self.jumps_chirho.borrow_mut() += 1;
                                assignment_chirho[index_chirho] = None;
                                return Err(child_conflict_chirho);
                            }
                            // This level is involved, continue trying
                            conflict_premises_chirho.extend(child_conflict_chirho);
                        }
                    }
                }
                Err(local_conflict_chirho) => {
                    conflict_premises_chirho.extend(local_conflict_chirho);
                }
            }

            *self.backtrack_count_chirho.borrow_mut() += 1;
        }

        // All choices failed - record nogood
        if !conflict_premises_chirho.is_empty() {
            self.nogoods_chirho
                .borrow_mut()
                .push(conflict_premises_chirho.clone());
        }

        assignment_chirho[index_chirho] = None;
        Err(conflict_premises_chirho)
    }
}

impl Default for DependencyDirectedSearchChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

// ============================================================================
// SEARCH HEURISTICS (GAP-007)
// ============================================================================

/// Trait for variable ordering heuristics.
///
/// Variable ordering determines which unassigned variable to select next
/// during search. Good orderings can dramatically reduce search effort.
///
/// # Example
///
/// ```
/// use propagators_chirho::constraints_chirho::amb_chirho::{
///     VariableOrderingChirho, FirstFailChirho
/// };
///
/// let heuristic_chirho = FirstFailChirho::new_chirho();
///
/// // Domain sizes for 3 variables
/// let domain_sizes_chirho = vec![3, 1, 5];
///
/// // Select variable with smallest domain (index 1, size 1)
/// let selected_chirho = heuristic_chirho.select_variable_chirho(&domain_sizes_chirho, &[false, false, false]);
/// assert_eq!(selected_chirho, Some(1));
/// ```
pub trait VariableOrderingChirho {
    /// Selects the next variable to assign.
    ///
    /// # Arguments
    ///
    /// * `domain_sizes_chirho` - Current domain size for each variable
    /// * `assigned_chirho` - Whether each variable is already assigned
    ///
    /// # Returns
    ///
    /// Index of the variable to assign next, or `None` if all assigned.
    fn select_variable_chirho(
        &self,
        domain_sizes_chirho: &[usize],
        assigned_chirho: &[bool],
    ) -> Option<usize>;
}

/// First-fail heuristic: select the variable with smallest domain.
///
/// This is one of the most effective general-purpose heuristics. The
/// intuition is that variables with small domains are likely to cause
/// failure, so it's better to detect failures early.
///
/// # References
///
/// Haralick, R. M., & Elliott, G. L. (1980). *Increasing tree search
/// efficiency for constraint satisfaction problems*. Artificial Intelligence.
#[derive(Clone, Debug, Default)]
pub struct FirstFailChirho {
    /// Whether to break ties randomly (reserved for future use).
    #[allow(dead_code)]
    random_tiebreak_chirho: bool,
}

impl FirstFailChirho {
    /// Creates a new first-fail heuristic.
    pub fn new_chirho() -> Self {
        Self {
            random_tiebreak_chirho: false,
        }
    }

    /// Creates a first-fail heuristic with random tie-breaking.
    pub fn with_random_tiebreak_chirho() -> Self {
        Self {
            random_tiebreak_chirho: true,
        }
    }
}

impl VariableOrderingChirho for FirstFailChirho {
    fn select_variable_chirho(
        &self,
        domain_sizes_chirho: &[usize],
        assigned_chirho: &[bool],
    ) -> Option<usize> {
        let mut best_idx_chirho: Option<usize> = None;
        let mut best_size_chirho: usize = usize::MAX;

        for (idx_chirho, (&size_chirho, &is_assigned_chirho)) in
            domain_sizes_chirho.iter().zip(assigned_chirho.iter()).enumerate()
        {
            if !is_assigned_chirho && size_chirho > 0 && size_chirho < best_size_chirho {
                best_size_chirho = size_chirho;
                best_idx_chirho = Some(idx_chirho);
            }
        }

        best_idx_chirho
    }
}

/// Domain over weighted degree (dom/wdeg) heuristic.
///
/// This heuristic combines domain size with failure information. The
/// "weighted degree" of a variable is the sum of weights of constraints
/// involving that variable, where weights increase each time a constraint
/// fails.
///
/// Variables with small domain / high weight are selected first.
///
/// # Algorithm
///
/// 1. Initially all constraint weights = 1
/// 2. When propagation fails, increment weight of failed constraint
/// 3. Select variable with minimum: domain_size / sum(constraint_weights)
///
/// # References
///
/// Boussemart, F., Hemery, F., Lecoutre, C., & Sais, L. (2004).
/// *Boosting systematic search by weighting constraints*. ECAI.
#[derive(Clone, Debug)]
pub struct DomWdegChirho {
    /// Weight for each constraint, indexed by constraint ID.
    constraint_weights_chirho: RefCell<Vec<f64>>,
    /// Which constraints involve which variables.
    /// constraint_variables_chirho[constraint_id] = vec![var_indices]
    constraint_variables_chirho: Vec<Vec<usize>>,
    /// Number of variables.
    num_vars_chirho: usize,
}

impl DomWdegChirho {
    /// Creates a new dom/wdeg heuristic.
    ///
    /// # Arguments
    ///
    /// * `num_vars_chirho` - Number of variables
    /// * `constraint_variables_chirho` - For each constraint, the variable indices it involves
    pub fn new_chirho(
        num_vars_chirho: usize,
        constraint_variables_chirho: Vec<Vec<usize>>,
    ) -> Self {
        let num_constraints_chirho = constraint_variables_chirho.len();
        Self {
            constraint_weights_chirho: RefCell::new(vec![1.0; num_constraints_chirho]),
            constraint_variables_chirho,
            num_vars_chirho,
        }
    }

    /// Increments the weight of a constraint after it fails.
    pub fn record_failure_chirho(&self, constraint_id_chirho: usize) {
        let mut weights_chirho = self.constraint_weights_chirho.borrow_mut();
        if constraint_id_chirho < weights_chirho.len() {
            weights_chirho[constraint_id_chirho] += 1.0;
        }
    }

    /// Returns the weighted degree of a variable.
    pub fn weighted_degree_chirho(&self, var_idx_chirho: usize) -> f64 {
        let weights_chirho = self.constraint_weights_chirho.borrow();
        let mut wdeg_chirho = 0.0;

        for (cid_chirho, vars_chirho) in self.constraint_variables_chirho.iter().enumerate() {
            if vars_chirho.contains(&var_idx_chirho) {
                wdeg_chirho += weights_chirho[cid_chirho];
            }
        }

        wdeg_chirho
    }

    /// Resets all constraint weights to 1.
    pub fn reset_weights_chirho(&self) {
        let mut weights_chirho = self.constraint_weights_chirho.borrow_mut();
        for w_chirho in weights_chirho.iter_mut() {
            *w_chirho = 1.0;
        }
    }
}

impl VariableOrderingChirho for DomWdegChirho {
    fn select_variable_chirho(
        &self,
        domain_sizes_chirho: &[usize],
        assigned_chirho: &[bool],
    ) -> Option<usize> {
        let mut best_idx_chirho: Option<usize> = None;
        let mut best_ratio_chirho: f64 = f64::MAX;

        for idx_chirho in 0..self.num_vars_chirho {
            if !assigned_chirho[idx_chirho] && domain_sizes_chirho[idx_chirho] > 0 {
                let wdeg_chirho = self.weighted_degree_chirho(idx_chirho);
                // Avoid division by zero; if wdeg is 0, treat as 1
                let wdeg_safe_chirho = if wdeg_chirho < 1e-10 { 1.0 } else { wdeg_chirho };
                let ratio_chirho = domain_sizes_chirho[idx_chirho] as f64 / wdeg_safe_chirho;

                if ratio_chirho < best_ratio_chirho {
                    best_ratio_chirho = ratio_chirho;
                    best_idx_chirho = Some(idx_chirho);
                }
            }
        }

        best_idx_chirho
    }
}

/// Impact-based search heuristic.
///
/// This heuristic measures the "impact" of assigning a value to a variable,
/// defined as the reduction in search space size. Variables with high
/// average impact are selected first.
///
/// # Algorithm
///
/// 1. For each (variable, value) pair, track the impact
/// 2. Impact = 1 - (product of domain sizes after) / (product before)
/// 3. Select variable with highest average impact
///
/// # References
///
/// Refalo, P. (2004). *Impact-based search strategies for constraint
/// programming*. CP.
#[derive(Clone, Debug)]
pub struct ImpactBasedChirho {
    /// Average impact for each variable.
    impacts_chirho: RefCell<Vec<f64>>,
    /// Number of times each variable has been sampled.
    sample_counts_chirho: RefCell<Vec<usize>>,
    /// Number of variables.
    num_vars_chirho: usize,
}

impl ImpactBasedChirho {
    /// Creates a new impact-based heuristic.
    pub fn new_chirho(num_vars_chirho: usize) -> Self {
        Self {
            impacts_chirho: RefCell::new(vec![0.5; num_vars_chirho]), // Initial estimate
            sample_counts_chirho: RefCell::new(vec![0; num_vars_chirho]),
            num_vars_chirho,
        }
    }

    /// Records the impact of assigning a variable.
    ///
    /// # Arguments
    ///
    /// * `var_idx_chirho` - Variable that was assigned
    /// * `impact_chirho` - Measured impact (0 to 1, higher = more pruning)
    pub fn record_impact_chirho(&self, var_idx_chirho: usize, impact_chirho: f64) {
        let mut impacts_chirho = self.impacts_chirho.borrow_mut();
        let mut counts_chirho = self.sample_counts_chirho.borrow_mut();

        if var_idx_chirho < self.num_vars_chirho {
            let count_chirho = counts_chirho[var_idx_chirho];
            // Running average
            impacts_chirho[var_idx_chirho] = (impacts_chirho[var_idx_chirho]
                * count_chirho as f64
                + impact_chirho)
                / (count_chirho + 1) as f64;
            counts_chirho[var_idx_chirho] += 1;
        }
    }

    /// Returns the current impact estimate for a variable.
    pub fn impact_chirho(&self, var_idx_chirho: usize) -> f64 {
        self.impacts_chirho.borrow().get(var_idx_chirho).copied().unwrap_or(0.5)
    }

    /// Resets all impacts to initial estimates.
    pub fn reset_chirho(&self) {
        let mut impacts_chirho = self.impacts_chirho.borrow_mut();
        let mut counts_chirho = self.sample_counts_chirho.borrow_mut();
        for i_chirho in impacts_chirho.iter_mut() {
            *i_chirho = 0.5;
        }
        for c_chirho in counts_chirho.iter_mut() {
            *c_chirho = 0;
        }
    }
}

impl VariableOrderingChirho for ImpactBasedChirho {
    fn select_variable_chirho(
        &self,
        domain_sizes_chirho: &[usize],
        assigned_chirho: &[bool],
    ) -> Option<usize> {
        let impacts_chirho = self.impacts_chirho.borrow();
        let mut best_idx_chirho: Option<usize> = None;
        let mut best_impact_chirho: f64 = -1.0;

        for idx_chirho in 0..self.num_vars_chirho {
            if !assigned_chirho[idx_chirho] && domain_sizes_chirho[idx_chirho] > 0 {
                // Scale impact by domain size (prefer high impact + small domain)
                let effective_impact_chirho =
                    impacts_chirho[idx_chirho] / (domain_sizes_chirho[idx_chirho] as f64).sqrt();

                if effective_impact_chirho > best_impact_chirho {
                    best_impact_chirho = effective_impact_chirho;
                    best_idx_chirho = Some(idx_chirho);
                }
            }
        }

        best_idx_chirho
    }
}

/// Trait for value ordering heuristics.
///
/// Value ordering determines which value to try first when assigning a
/// variable. While value ordering doesn't affect whether a solution exists,
/// it can significantly speed up finding the first solution.
pub trait ValueOrderingChirho {
    /// Orders the values for a variable.
    ///
    /// # Arguments
    ///
    /// * `var_idx_chirho` - The variable being assigned
    /// * `values_chirho` - The available values
    ///
    /// # Returns
    ///
    /// Indices into `values_chirho` in the order to try them.
    fn order_values_chirho(
        &self,
        var_idx_chirho: usize,
        values_chirho: &[NumericInfoChirho],
    ) -> Vec<usize>;
}

/// Minimum value first ordering.
///
/// Try smaller values before larger ones. Useful when looking for
/// minimum-cost solutions.
#[derive(Clone, Debug, Default)]
pub struct MinValueChirho;

impl MinValueChirho {
    /// Creates a new min-value ordering.
    pub fn new_chirho() -> Self {
        Self
    }
}

impl ValueOrderingChirho for MinValueChirho {
    fn order_values_chirho(
        &self,
        _var_idx_chirho: usize,
        values_chirho: &[NumericInfoChirho],
    ) -> Vec<usize> {
        let mut indices_chirho: Vec<usize> = (0..values_chirho.len()).collect();
        indices_chirho.sort_by(|&a_chirho, &b_chirho| {
            let val_a_chirho = values_chirho[a_chirho]
                .as_interval_chirho()
                .map(|i_chirho| i_chirho.lo_chirho)
                .unwrap_or(f64::MAX);
            let val_b_chirho = values_chirho[b_chirho]
                .as_interval_chirho()
                .map(|i_chirho| i_chirho.lo_chirho)
                .unwrap_or(f64::MAX);
            val_a_chirho.partial_cmp(&val_b_chirho).unwrap_or(std::cmp::Ordering::Equal)
        });
        indices_chirho
    }
}

/// Maximum value first ordering.
///
/// Try larger values before smaller ones. Useful when looking for
/// maximum-value solutions.
#[derive(Clone, Debug, Default)]
pub struct MaxValueChirho;

impl MaxValueChirho {
    /// Creates a new max-value ordering.
    pub fn new_chirho() -> Self {
        Self
    }
}

impl ValueOrderingChirho for MaxValueChirho {
    fn order_values_chirho(
        &self,
        _var_idx_chirho: usize,
        values_chirho: &[NumericInfoChirho],
    ) -> Vec<usize> {
        let mut indices_chirho: Vec<usize> = (0..values_chirho.len()).collect();
        indices_chirho.sort_by(|&a_chirho, &b_chirho| {
            let val_a_chirho = values_chirho[a_chirho]
                .as_interval_chirho()
                .map(|i_chirho| i_chirho.lo_chirho)
                .unwrap_or(f64::MIN);
            let val_b_chirho = values_chirho[b_chirho]
                .as_interval_chirho()
                .map(|i_chirho| i_chirho.lo_chirho)
                .unwrap_or(f64::MIN);
            val_b_chirho.partial_cmp(&val_a_chirho).unwrap_or(std::cmp::Ordering::Equal)
        });
        indices_chirho
    }
}

/// Middle-out value ordering.
///
/// Try values near the middle of the domain first. Can be effective
/// when solutions are likely to have "balanced" values.
#[derive(Clone, Debug, Default)]
pub struct MiddleOutChirho;

impl MiddleOutChirho {
    /// Creates a new middle-out ordering.
    pub fn new_chirho() -> Self {
        Self
    }
}

impl ValueOrderingChirho for MiddleOutChirho {
    fn order_values_chirho(
        &self,
        _var_idx_chirho: usize,
        values_chirho: &[NumericInfoChirho],
    ) -> Vec<usize> {
        if values_chirho.is_empty() {
            return vec![];
        }

        // Find median value
        let mut vals_chirho: Vec<f64> = values_chirho
            .iter()
            .filter_map(|v_chirho| v_chirho.as_interval_chirho().map(|i_chirho| i_chirho.lo_chirho))
            .collect();
        vals_chirho.sort_by(|a_chirho, b_chirho| {
            a_chirho.partial_cmp(b_chirho).unwrap_or(std::cmp::Ordering::Equal)
        });

        let median_chirho = if vals_chirho.is_empty() {
            0.0
        } else {
            vals_chirho[vals_chirho.len() / 2]
        };

        // Sort by distance from median
        let mut indices_chirho: Vec<usize> = (0..values_chirho.len()).collect();
        indices_chirho.sort_by(|&a_chirho, &b_chirho| {
            let val_a_chirho = values_chirho[a_chirho]
                .as_interval_chirho()
                .map(|i_chirho| i_chirho.lo_chirho)
                .unwrap_or(0.0);
            let val_b_chirho = values_chirho[b_chirho]
                .as_interval_chirho()
                .map(|i_chirho| i_chirho.lo_chirho)
                .unwrap_or(0.0);
            let dist_a_chirho = (val_a_chirho - median_chirho).abs();
            let dist_b_chirho = (val_b_chirho - median_chirho).abs();
            dist_a_chirho.partial_cmp(&dist_b_chirho).unwrap_or(std::cmp::Ordering::Equal)
        });
        indices_chirho
    }
}

/// Search with restarts.
///
/// This wraps another search strategy and restarts from the beginning
/// after a certain number of backtracks, potentially with learned nogoods
/// or different randomization.
///
/// # Algorithm
///
/// Luby sequence restarts: restart after 1, 1, 2, 1, 1, 2, 4, 1, 1, 2, 1, 1, 2, 4, 8, ...
/// backtracks (scaled by a base factor).
///
/// # References
///
/// Luby, M., Sinclair, A., & Zuckerman, D. (1993). *Optimal speedup of
/// Las Vegas algorithms*. Information Processing Letters.
#[derive(Clone, Debug)]
pub struct RestartSearchChirho {
    /// Base number of backtracks before first restart.
    base_cutoff_chirho: usize,
    /// Current position in Luby sequence.
    luby_index_chirho: RefCell<usize>,
    /// Total restarts performed.
    restart_count_chirho: RefCell<usize>,
    /// Nogoods learned across restarts.
    learned_nogoods_chirho: RefCell<Vec<HashSet<String>>>,
}

impl RestartSearchChirho {
    /// Creates a restart search with the given base cutoff.
    pub fn new_chirho(base_cutoff_chirho: usize) -> Self {
        Self {
            base_cutoff_chirho,
            luby_index_chirho: RefCell::new(1),
            restart_count_chirho: RefCell::new(0),
            learned_nogoods_chirho: RefCell::new(Vec::new()),
        }
    }

    /// Returns the current backtrack cutoff for restart.
    pub fn current_cutoff_chirho(&self) -> usize {
        let idx_chirho = *self.luby_index_chirho.borrow();
        self.base_cutoff_chirho * Self::luby_chirho(idx_chirho)
    }

    /// Returns the number of restarts performed.
    pub fn restart_count_chirho(&self) -> usize {
        *self.restart_count_chirho.borrow()
    }

    /// Triggers a restart, advancing the Luby sequence.
    pub fn restart_chirho(&self) {
        *self.luby_index_chirho.borrow_mut() += 1;
        *self.restart_count_chirho.borrow_mut() += 1;
    }

    /// Records a nogood learned during search.
    pub fn add_nogood_chirho(&self, nogood_chirho: HashSet<String>) {
        self.learned_nogoods_chirho.borrow_mut().push(nogood_chirho);
    }

    /// Returns all learned nogoods.
    pub fn nogoods_chirho(&self) -> Vec<HashSet<String>> {
        self.learned_nogoods_chirho.borrow().clone()
    }

    /// Resets the restart search state.
    pub fn reset_chirho(&self) {
        *self.luby_index_chirho.borrow_mut() = 1;
        *self.restart_count_chirho.borrow_mut() = 0;
        self.learned_nogoods_chirho.borrow_mut().clear();
    }

    /// Computes the i-th value in the Luby sequence.
    ///
    /// Luby sequence: 1, 1, 2, 1, 1, 2, 4, 1, 1, 2, 1, 1, 2, 4, 8, ...
    fn luby_chirho(i_chirho: usize) -> usize {
        if i_chirho == 0 {
            return 1;
        }

        let mut k_chirho: u32 = 1;
        while (1usize << k_chirho) <= i_chirho + 1 {
            k_chirho += 1;
        }
        k_chirho -= 1;

        let power_chirho = 1usize << k_chirho;
        if i_chirho + 1 == power_chirho {
            power_chirho / 2
        } else {
            Self::luby_chirho(i_chirho - power_chirho / 2)
        }
    }
}

impl Default for RestartSearchChirho {
    fn default() -> Self {
        Self::new_chirho(100)
    }
}

/// Search statistics for analysis.
#[derive(Clone, Debug, Default)]
pub struct SearchStatsChirho {
    /// Number of nodes explored.
    pub nodes_chirho: usize,
    /// Number of backtracks.
    pub backtracks_chirho: usize,
    /// Number of restarts.
    pub restarts_chirho: usize,
    /// Number of nogoods learned.
    pub nogoods_learned_chirho: usize,
    /// Number of propagations.
    pub propagations_chirho: usize,
}

impl SearchStatsChirho {
    /// Creates empty search statistics.
    pub fn new_chirho() -> Self {
        Self::default()
    }

    /// Resets all statistics.
    pub fn reset_chirho(&mut self) {
        self.nodes_chirho = 0;
        self.backtracks_chirho = 0;
        self.restarts_chirho = 0;
        self.nogoods_learned_chirho = 0;
        self.propagations_chirho = 0;
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_amb_creation_chirho() {
        let amb_chirho = AmbChirho::new_chirho(vec![
            NumericInfoChirho::exact_chirho(1.0),
            NumericInfoChirho::exact_chirho(2.0),
            NumericInfoChirho::exact_chirho(3.0),
        ]);

        assert_eq!(amb_chirho.choice_count_chirho(), 3);
    }

    #[test]
    fn test_amb_range_chirho() {
        let amb_chirho = AmbChirho::range_chirho(1, 5);
        assert_eq!(amb_chirho.choice_count_chirho(), 5);
    }

    #[test]
    fn test_amb_iteration_chirho() {
        let amb_chirho = AmbChirho::range_chirho(1, 3);

        let first_chirho = amb_chirho.current_chirho().unwrap();
        assert!((first_chirho.as_interval_chirho().unwrap().lo_chirho - 1.0).abs() < 1e-10);

        assert!(amb_chirho.next_chirho());
        let second_chirho = amb_chirho.current_chirho().unwrap();
        assert!((second_chirho.as_interval_chirho().unwrap().lo_chirho - 2.0).abs() < 1e-10);

        assert!(amb_chirho.next_chirho());
        assert!(!amb_chirho.next_chirho()); // Exhausted
    }

    #[test]
    fn test_simple_search_chirho() {
        let search_chirho = BacktrackingSearchChirho::new_chirho();

        // Find x, y where x + y = 5 and x, y ∈ {1, 2, 3, 4}
        let ambs_chirho = vec![
            AmbChirho::range_chirho(1, 4), // x
            AmbChirho::range_chirho(1, 4), // y
        ];

        let result_chirho = search_chirho.search_simple_chirho(&ambs_chirho, |values_chirho| {
            if values_chirho.len() < 2 {
                true // Partial assignment, allow
            } else {
                let x_chirho = values_chirho[0].as_interval_chirho().unwrap().lo_chirho;
                let y_chirho = values_chirho[1].as_interval_chirho().unwrap().lo_chirho;
                (x_chirho + y_chirho - 5.0).abs() < 1e-10
            }
        });

        match result_chirho {
            SearchResultChirho::SolutionChirho(solution_chirho) => {
                let x_chirho = solution_chirho[0].as_interval_chirho().unwrap().lo_chirho;
                let y_chirho = solution_chirho[1].as_interval_chirho().unwrap().lo_chirho;
                assert!((x_chirho + y_chirho - 5.0).abs() < 1e-10);
            }
            _ => panic!("Expected solution"),
        }
    }

    #[test]
    fn test_no_solution_chirho() {
        let search_chirho = BacktrackingSearchChirho::new_chirho();

        // Find x where x > 10 and x ∈ {1, 2, 3}
        let ambs_chirho = vec![AmbChirho::range_chirho(1, 3)];

        let result_chirho = search_chirho.search_simple_chirho(&ambs_chirho, |values_chirho| {
            if values_chirho.is_empty() {
                true
            } else {
                let x_chirho = values_chirho[0].as_interval_chirho().unwrap().lo_chirho;
                x_chirho > 10.0
            }
        });

        assert!(matches!(
            result_chirho,
            SearchResultChirho::NoSolutionChirho
        ));
    }

    #[test]
    fn test_dd_search_basic_chirho() {
        let search_chirho = DependencyDirectedSearchChirho::new_chirho();

        // Find x, y where x + y = 5 and x, y ∈ {1, 2, 3, 4}
        let ambs_chirho = vec![
            ("x".to_string(), AmbChirho::range_chirho(1, 4)),
            ("y".to_string(), AmbChirho::range_chirho(1, 4)),
        ];

        let result_chirho = search_chirho.search_chirho(&ambs_chirho, |values_chirho| {
            if values_chirho.len() < 2 {
                Ok(())
            } else {
                let x_chirho = values_chirho[0].1.as_interval_chirho().unwrap().lo_chirho;
                let y_chirho = values_chirho[1].1.as_interval_chirho().unwrap().lo_chirho;
                if (x_chirho + y_chirho - 5.0).abs() < 1e-10 {
                    Ok(())
                } else {
                    let mut conflict_chirho = HashSet::new();
                    conflict_chirho.insert("x".to_string());
                    conflict_chirho.insert("y".to_string());
                    Err(conflict_chirho)
                }
            }
        });

        match result_chirho {
            SearchResultChirho::SolutionChirho(solution_chirho) => {
                let x_chirho = solution_chirho[0].as_interval_chirho().unwrap().lo_chirho;
                let y_chirho = solution_chirho[1].as_interval_chirho().unwrap().lo_chirho;
                assert!((x_chirho + y_chirho - 5.0).abs() < 1e-10);
            }
            _ => panic!("Expected solution"),
        }
    }

    #[test]
    fn test_dd_search_with_irrelevant_variable_chirho() {
        let search_chirho = DependencyDirectedSearchChirho::new_chirho();

        // z is irrelevant to the constraint x + y = 5
        // DD backtracking should skip trying all values of z
        let ambs_chirho = vec![
            ("x".to_string(), AmbChirho::range_chirho(1, 3)),
            ("y".to_string(), AmbChirho::range_chirho(1, 3)),
            ("z".to_string(), AmbChirho::range_chirho(1, 100)), // Many choices, but irrelevant
        ];

        let result_chirho = search_chirho.search_chirho(&ambs_chirho, |values_chirho| {
            if values_chirho.len() < 2 {
                Ok(())
            } else {
                let x_chirho = values_chirho[0].1.as_interval_chirho().unwrap().lo_chirho;
                let y_chirho = values_chirho[1].1.as_interval_chirho().unwrap().lo_chirho;
                if (x_chirho + y_chirho - 4.0).abs() < 1e-10 {
                    Ok(())
                } else {
                    // Only x and y are involved in the conflict
                    let mut conflict_chirho = HashSet::new();
                    conflict_chirho.insert("x".to_string());
                    conflict_chirho.insert("y".to_string());
                    Err(conflict_chirho)
                }
            }
        });

        match result_chirho {
            SearchResultChirho::SolutionChirho(_) => {
                // Should have some jumps due to z being irrelevant
                // The exact number depends on search order
                println!("Jumps: {}", search_chirho.jump_count_chirho());
                println!("Backtracks: {}", search_chirho.backtrack_count_chirho());
            }
            _ => panic!("Expected solution"),
        }
    }

    #[test]
    fn test_dd_search_no_solution_chirho() {
        let search_chirho = DependencyDirectedSearchChirho::new_chirho();

        // No solution: x + y = 100 with x, y ∈ {1, 2, 3}
        let ambs_chirho = vec![
            ("x".to_string(), AmbChirho::range_chirho(1, 3)),
            ("y".to_string(), AmbChirho::range_chirho(1, 3)),
        ];

        let result_chirho = search_chirho.search_chirho(&ambs_chirho, |values_chirho| {
            if values_chirho.len() < 2 {
                Ok(())
            } else {
                let x_chirho = values_chirho[0].1.as_interval_chirho().unwrap().lo_chirho;
                let y_chirho = values_chirho[1].1.as_interval_chirho().unwrap().lo_chirho;
                if (x_chirho + y_chirho - 100.0).abs() < 1e-10 {
                    Ok(())
                } else {
                    let mut conflict_chirho = HashSet::new();
                    conflict_chirho.insert("x".to_string());
                    conflict_chirho.insert("y".to_string());
                    Err(conflict_chirho)
                }
            }
        });

        assert!(matches!(
            result_chirho,
            SearchResultChirho::NoSolutionChirho
        ));

        // Should have learned some nogoods
        assert!(!search_chirho.nogoods_chirho().is_empty());
    }

    // =========================================================================
    // Tests for Search Heuristics (GAP-007)
    // =========================================================================

    #[test]
    fn test_first_fail_selects_smallest_domain_chirho() {
        let heuristic_chirho = FirstFailChirho::new_chirho();

        let domain_sizes_chirho = vec![3, 1, 5, 2];
        let assigned_chirho = vec![false, false, false, false];

        // Should select index 1 (smallest domain = 1)
        let selected_chirho =
            heuristic_chirho.select_variable_chirho(&domain_sizes_chirho, &assigned_chirho);
        assert_eq!(selected_chirho, Some(1));
    }

    #[test]
    fn test_first_fail_skips_assigned_chirho() {
        let heuristic_chirho = FirstFailChirho::new_chirho();

        let domain_sizes_chirho = vec![3, 1, 5, 2];
        let assigned_chirho = vec![false, true, false, false]; // Index 1 is assigned

        // Should select index 3 (next smallest = 2)
        let selected_chirho =
            heuristic_chirho.select_variable_chirho(&domain_sizes_chirho, &assigned_chirho);
        assert_eq!(selected_chirho, Some(3));
    }

    #[test]
    fn test_first_fail_all_assigned_chirho() {
        let heuristic_chirho = FirstFailChirho::new_chirho();

        let domain_sizes_chirho = vec![3, 1, 5];
        let assigned_chirho = vec![true, true, true];

        let selected_chirho =
            heuristic_chirho.select_variable_chirho(&domain_sizes_chirho, &assigned_chirho);
        assert_eq!(selected_chirho, None);
    }

    #[test]
    fn test_dom_wdeg_creation_chirho() {
        // 3 variables, 2 constraints
        let constraint_vars_chirho = vec![
            vec![0, 1], // constraint 0 involves vars 0 and 1
            vec![1, 2], // constraint 1 involves vars 1 and 2
        ];

        let heuristic_chirho = DomWdegChirho::new_chirho(3, constraint_vars_chirho);

        // Variable 1 has wdeg = 2 (involved in both constraints)
        assert!((heuristic_chirho.weighted_degree_chirho(1) - 2.0).abs() < 1e-10);
        // Variable 0 has wdeg = 1
        assert!((heuristic_chirho.weighted_degree_chirho(0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_dom_wdeg_failure_recording_chirho() {
        let constraint_vars_chirho = vec![vec![0, 1], vec![1, 2]];
        let heuristic_chirho = DomWdegChirho::new_chirho(3, constraint_vars_chirho);

        // Record failure on constraint 0
        heuristic_chirho.record_failure_chirho(0);

        // Weight should increase
        assert!((heuristic_chirho.weighted_degree_chirho(0) - 2.0).abs() < 1e-10);
        assert!((heuristic_chirho.weighted_degree_chirho(1) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_dom_wdeg_selection_chirho() {
        let constraint_vars_chirho = vec![
            vec![0, 1],
            vec![1, 2],
            vec![0, 1, 2], // var 1 is in all constraints
        ];
        let heuristic_chirho = DomWdegChirho::new_chirho(3, constraint_vars_chirho);

        // All domains same size
        let domain_sizes_chirho = vec![3, 3, 3];
        let assigned_chirho = vec![false, false, false];

        // Should select var with highest wdeg (var 1 with wdeg=3)
        // ratio = 3/3 = 1 for var 1, 3/2 = 1.5 for var 0, 3/2 = 1.5 for var 2
        let selected_chirho =
            heuristic_chirho.select_variable_chirho(&domain_sizes_chirho, &assigned_chirho);
        assert_eq!(selected_chirho, Some(1));
    }

    #[test]
    fn test_impact_based_creation_chirho() {
        let heuristic_chirho = ImpactBasedChirho::new_chirho(3);

        // Initial impact should be 0.5
        assert!((heuristic_chirho.impact_chirho(0) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_impact_based_recording_chirho() {
        let heuristic_chirho = ImpactBasedChirho::new_chirho(3);

        // Record high impact for var 0
        heuristic_chirho.record_impact_chirho(0, 0.9);

        // First recording replaces initial estimate: (0.5 * 0 + 0.9) / 1 = 0.9
        assert!((heuristic_chirho.impact_chirho(0) - 0.9).abs() < 1e-10);

        // Second recording gives running average: (0.9 * 1 + 0.3) / 2 = 0.6
        heuristic_chirho.record_impact_chirho(0, 0.3);
        assert!((heuristic_chirho.impact_chirho(0) - 0.6).abs() < 1e-10);
    }

    #[test]
    fn test_min_value_ordering_chirho() {
        let ordering_chirho = MinValueChirho::new_chirho();

        let values_chirho = vec![
            NumericInfoChirho::exact_chirho(30.0),
            NumericInfoChirho::exact_chirho(10.0),
            NumericInfoChirho::exact_chirho(20.0),
        ];

        let order_chirho = ordering_chirho.order_values_chirho(0, &values_chirho);

        // Should be sorted: 10 (idx 1), 20 (idx 2), 30 (idx 0)
        assert_eq!(order_chirho, vec![1, 2, 0]);
    }

    #[test]
    fn test_max_value_ordering_chirho() {
        let ordering_chirho = MaxValueChirho::new_chirho();

        let values_chirho = vec![
            NumericInfoChirho::exact_chirho(30.0),
            NumericInfoChirho::exact_chirho(10.0),
            NumericInfoChirho::exact_chirho(20.0),
        ];

        let order_chirho = ordering_chirho.order_values_chirho(0, &values_chirho);

        // Should be sorted descending: 30 (idx 0), 20 (idx 2), 10 (idx 1)
        assert_eq!(order_chirho, vec![0, 2, 1]);
    }

    #[test]
    fn test_middle_out_ordering_chirho() {
        let ordering_chirho = MiddleOutChirho::new_chirho();

        let values_chirho = vec![
            NumericInfoChirho::exact_chirho(1.0),
            NumericInfoChirho::exact_chirho(5.0),
            NumericInfoChirho::exact_chirho(3.0), // median
            NumericInfoChirho::exact_chirho(7.0),
            NumericInfoChirho::exact_chirho(9.0),
        ];

        let order_chirho = ordering_chirho.order_values_chirho(0, &values_chirho);

        // First should be closest to median (5.0 in sorted order: 1,3,5,7,9)
        // idx 1 has value 5.0, which is the median
        assert_eq!(order_chirho[0], 1);
    }

    #[test]
    fn test_restart_search_luby_sequence_chirho() {
        let restart_chirho = RestartSearchChirho::new_chirho(10);

        // First cutoff is 10 * 1 = 10
        assert_eq!(restart_chirho.current_cutoff_chirho(), 10);

        restart_chirho.restart_chirho();
        // Second: 10 * 1 = 10
        assert_eq!(restart_chirho.current_cutoff_chirho(), 10);

        restart_chirho.restart_chirho();
        // Third: 10 * 2 = 20
        assert_eq!(restart_chirho.current_cutoff_chirho(), 20);

        restart_chirho.restart_chirho();
        // Fourth: 10 * 1 = 10
        assert_eq!(restart_chirho.current_cutoff_chirho(), 10);
    }

    #[test]
    fn test_restart_search_nogood_recording_chirho() {
        let restart_chirho = RestartSearchChirho::new_chirho(100);

        let mut nogood_chirho = HashSet::new();
        nogood_chirho.insert("x".to_string());
        nogood_chirho.insert("y".to_string());

        restart_chirho.add_nogood_chirho(nogood_chirho);

        assert_eq!(restart_chirho.nogoods_chirho().len(), 1);
    }

    #[test]
    fn test_restart_search_reset_chirho() {
        let restart_chirho = RestartSearchChirho::new_chirho(10);

        restart_chirho.restart_chirho();
        restart_chirho.restart_chirho();

        let mut nogood_chirho = HashSet::new();
        nogood_chirho.insert("x".to_string());
        restart_chirho.add_nogood_chirho(nogood_chirho);

        assert_eq!(restart_chirho.restart_count_chirho(), 2);
        assert_eq!(restart_chirho.nogoods_chirho().len(), 1);

        restart_chirho.reset_chirho();

        assert_eq!(restart_chirho.restart_count_chirho(), 0);
        assert!(restart_chirho.nogoods_chirho().is_empty());
        assert_eq!(restart_chirho.current_cutoff_chirho(), 10);
    }

    #[test]
    fn test_search_stats_chirho() {
        let mut stats_chirho = SearchStatsChirho::new_chirho();

        stats_chirho.nodes_chirho = 100;
        stats_chirho.backtracks_chirho = 20;
        stats_chirho.restarts_chirho = 3;

        assert_eq!(stats_chirho.nodes_chirho, 100);
        assert_eq!(stats_chirho.backtracks_chirho, 20);

        stats_chirho.reset_chirho();

        assert_eq!(stats_chirho.nodes_chirho, 0);
        assert_eq!(stats_chirho.backtracks_chirho, 0);
    }
}
