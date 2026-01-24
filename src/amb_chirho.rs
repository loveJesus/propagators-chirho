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
//! # References
//!
//! - McCarthy, J. (1963). *A Basis for a Mathematical Theory of Computation*.
//!
//! - Radul, A., & Sussman, G. J. (2009). *The Art of the Propagator*, Section 9.
//!   <https://dspace.mit.edu/handle/1721.1/44215>

use std::cell::RefCell;

use crate::interval_chirho::NumericInfoChirho;

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
}
