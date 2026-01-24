// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Truth Maintenance System (TMS) for dependency-directed reasoning.
//!
//! A Truth Maintenance System tracks the justifications for beliefs, enabling:
//!
//! - **Dependency tracking**: Know why each value was derived
//! - **Assumption retraction**: Remove beliefs when premises fail
//! - **Nogood detection**: Find which premises cause contradictions
//!
//! # References
//!
//! - de Kleer, J. (1986). *An Assumption-based TMS*.
//!   Artificial Intelligence, 28(2), 127-162.
//!
//! - Radul, A., & Sussman, G. J. (2009). *The Art of the Propagator*, Section 7.
//!   <https://dspace.mit.edu/handle/1721.1/44215>

use std::collections::HashSet;
use std::fmt;

use crate::interval_chirho::NumericInfoChirho;

/// A set of premise identifiers.
pub type PremiseSetChirho = HashSet<String>;

/// A value with its supporting premises.
///
/// Supported values track which assumptions were needed to derive them.
/// This enables dependency-directed backtracking when contradictions occur.
///
/// # Example
///
/// ```
/// use propagators_chirho::{SupportedChirho, NumericInfoChirho};
///
/// // A value supported by premise "sensor_a"
/// let supported_chirho = SupportedChirho::with_premises_chirho(
///     NumericInfoChirho::exact_chirho(25.0),
///     vec!["sensor_a".to_string()].into_iter().collect()
/// );
///
/// assert!(supported_chirho.premises_chirho().contains("sensor_a"));
/// ```
#[derive(Clone, Debug)]
pub struct SupportedChirho<T> {
    /// The value being supported.
    value_chirho: T,
    /// The premises required for this value.
    premises_chirho: PremiseSetChirho,
}

impl<T> SupportedChirho<T> {
    /// Creates a supported value with no premises (unconditional).
    pub fn unconditional_chirho(value_chirho: T) -> Self {
        Self {
            value_chirho,
            premises_chirho: HashSet::new(),
        }
    }

    /// Creates a supported value with the given premises.
    pub fn with_premises_chirho(value_chirho: T, premises_chirho: PremiseSetChirho) -> Self {
        Self {
            value_chirho,
            premises_chirho,
        }
    }

    /// Returns a reference to the value.
    pub fn value_chirho(&self) -> &T {
        &self.value_chirho
    }

    /// Returns a reference to the premises.
    pub fn premises_chirho(&self) -> &PremiseSetChirho {
        &self.premises_chirho
    }

    /// Returns `true` if this value is unconditionally true (no premises).
    pub fn is_unconditional_chirho(&self) -> bool {
        self.premises_chirho.is_empty()
    }

    /// Returns `true` if all premises in the given set are satisfied.
    pub fn is_valid_in_chirho(&self, active_premises_chirho: &PremiseSetChirho) -> bool {
        self.premises_chirho.is_subset(active_premises_chirho)
    }
}

impl<T: Clone> SupportedChirho<T> {
    /// Combines this supported value with another, merging their premises.
    pub fn combine_with_chirho(&self, other_chirho: &Self) -> Self
    where
        T: Clone,
    {
        let mut combined_premises_chirho = self.premises_chirho.clone();
        combined_premises_chirho.extend(other_chirho.premises_chirho.clone());
        Self {
            value_chirho: self.value_chirho.clone(),
            premises_chirho: combined_premises_chirho,
        }
    }
}

/// A belief with its supporting premises and source.
///
/// Beliefs are used in Truth Maintenance Systems to track multiple
/// possible values for a cell, each with different justifications.
///
/// # Example
///
/// ```
/// use propagators_chirho::{BeliefChirho, NumericInfoChirho};
///
/// let belief_chirho = BeliefChirho::with_premises_chirho(
///     NumericInfoChirho::exact_chirho(25.0),
///     vec!["thermometer".to_string()].into_iter().collect(),
///     "temperature reading"
/// );
///
/// println!("Value: {}", belief_chirho.value_chirho());
/// println!("Source: {}", belief_chirho.source_chirho());
/// ```
#[derive(Clone, Debug)]
pub struct BeliefChirho {
    /// The believed value.
    value_chirho: NumericInfoChirho,
    /// Premises supporting this belief.
    premises_chirho: PremiseSetChirho,
    /// Human-readable source description.
    source_chirho: String,
}

impl BeliefChirho {
    /// Creates an unconditional belief (always true).
    pub fn unconditional_chirho(value_chirho: NumericInfoChirho, source_chirho: &str) -> Self {
        Self {
            value_chirho,
            premises_chirho: HashSet::new(),
            source_chirho: source_chirho.to_string(),
        }
    }

    /// Creates a belief with the given premises.
    pub fn with_premises_chirho(
        value_chirho: NumericInfoChirho,
        premises_chirho: PremiseSetChirho,
        source_chirho: &str,
    ) -> Self {
        Self {
            value_chirho,
            premises_chirho,
            source_chirho: source_chirho.to_string(),
        }
    }

    /// Returns the believed value.
    pub fn value_chirho(&self) -> NumericInfoChirho {
        self.value_chirho
    }

    /// Returns the supporting premises.
    pub fn premises_chirho(&self) -> &PremiseSetChirho {
        &self.premises_chirho
    }

    /// Returns the source description.
    pub fn source_chirho(&self) -> &str {
        &self.source_chirho
    }

    /// Returns `true` if this belief is valid given active premises.
    pub fn is_valid_in_chirho(&self, active_premises_chirho: &PremiseSetChirho) -> bool {
        self.premises_chirho.is_subset(active_premises_chirho)
    }
}

impl fmt::Display for BeliefChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f_chirho,
            "{} (from: {}, premises: {:?})",
            self.value_chirho, self.source_chirho, self.premises_chirho
        )
    }
}

/// A cell that maintains multiple beliefs with their justifications.
///
/// Unlike a regular cell that holds a single value, a TMS cell holds
/// multiple beliefs and can return different values depending on which
/// premises are currently active.
///
/// # Example
///
/// ```
/// use propagators_chirho::{TmsCellChirho, BeliefChirho, NumericInfoChirho, WorldviewChirho};
///
/// let cell_chirho = TmsCellChirho::new_chirho("temperature");
///
/// // Add beliefs from different sources
/// cell_chirho.add_belief_chirho(BeliefChirho::with_premises_chirho(
///     NumericInfoChirho::exact_chirho(25.0),
///     vec!["sensor_a".to_string()].into_iter().collect(),
///     "sensor A reading"
/// ));
///
/// cell_chirho.add_belief_chirho(BeliefChirho::with_premises_chirho(
///     NumericInfoChirho::exact_chirho(26.0),
///     vec!["sensor_b".to_string()].into_iter().collect(),
///     "sensor B reading"
/// ));
///
/// // Query under different worldviews
/// let worldview_a_chirho = WorldviewChirho::new_chirho().assume_chirho("sensor_a".to_string());
/// let value_a_chirho = cell_chirho.content_in_worldview_chirho(&worldview_a_chirho);
/// ```
pub struct TmsCellChirho {
    /// Cell name for debugging.
    name_chirho: String,
    /// All beliefs held by this cell.
    beliefs_chirho: std::cell::RefCell<Vec<BeliefChirho>>,
}

impl TmsCellChirho {
    /// Creates a new TMS cell with the given name.
    pub fn new_chirho(name_chirho: &str) -> Self {
        Self {
            name_chirho: name_chirho.to_string(),
            beliefs_chirho: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// Returns the cell name.
    pub fn name_chirho(&self) -> &str {
        &self.name_chirho
    }

    /// Adds a belief to the cell.
    pub fn add_belief_chirho(&self, belief_chirho: BeliefChirho) {
        self.beliefs_chirho.borrow_mut().push(belief_chirho);
    }

    /// Returns all beliefs in the cell.
    pub fn beliefs_chirho(&self) -> Vec<BeliefChirho> {
        self.beliefs_chirho.borrow().clone()
    }

    /// Returns the content valid in the given worldview.
    ///
    /// Merges all beliefs whose premises are satisfied by the worldview.
    pub fn content_in_worldview_chirho(
        &self,
        worldview_chirho: &crate::worldview_chirho::WorldviewChirho,
    ) -> NumericInfoChirho {
        let active_chirho = worldview_chirho.active_premises_chirho();
        let mut result_chirho = NumericInfoChirho::nothing_chirho();

        for belief_chirho in self.beliefs_chirho.borrow().iter() {
            if belief_chirho.is_valid_in_chirho(active_chirho) {
                result_chirho = result_chirho.merge_chirho(&belief_chirho.value_chirho());
            }
        }

        result_chirho
    }

    /// Finds nogoods (contradictory premise sets).
    ///
    /// Returns premise sets that, when all active, lead to contradiction.
    pub fn find_nogoods_chirho(&self) -> Vec<PremiseSetChirho> {
        let mut nogoods_chirho = Vec::new();
        let beliefs_chirho = self.beliefs_chirho.borrow();

        // Check each pair of beliefs for contradictions
        for i_chirho in 0..beliefs_chirho.len() {
            for j_chirho in (i_chirho + 1)..beliefs_chirho.len() {
                let merged_chirho = beliefs_chirho[i_chirho]
                    .value_chirho()
                    .merge_chirho(&beliefs_chirho[j_chirho].value_chirho());

                if merged_chirho.is_contradiction_chirho() {
                    // The union of their premises is a nogood
                    let mut nogood_chirho = beliefs_chirho[i_chirho].premises_chirho().clone();
                    nogood_chirho.extend(beliefs_chirho[j_chirho].premises_chirho().clone());
                    nogoods_chirho.push(nogood_chirho);
                }
            }
        }

        nogoods_chirho
    }

    /// Returns `true` if any belief leads to contradiction with others.
    pub fn has_contradiction_chirho(&self) -> bool {
        !self.find_nogoods_chirho().is_empty()
    }
}

impl fmt::Debug for TmsCellChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        f_chirho
            .debug_struct("TmsCell")
            .field("name", &self.name_chirho)
            .field("beliefs", &self.beliefs_chirho.borrow().len())
            .finish()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::worldview_chirho::WorldviewChirho;

    #[test]
    fn test_supported_value_chirho() {
        let supported_chirho = SupportedChirho::with_premises_chirho(
            42.0,
            vec!["p1".to_string()].into_iter().collect(),
        );

        assert!(supported_chirho.premises_chirho().contains("p1"));
        assert!(!supported_chirho.is_unconditional_chirho());
    }

    #[test]
    fn test_belief_creation_chirho() {
        let belief_chirho = BeliefChirho::with_premises_chirho(
            NumericInfoChirho::exact_chirho(25.0),
            vec!["sensor".to_string()].into_iter().collect(),
            "test",
        );

        assert!(belief_chirho.premises_chirho().contains("sensor"));
        assert_eq!(belief_chirho.source_chirho(), "test");
    }

    #[test]
    fn test_tms_cell_worldview_chirho() {
        let cell_chirho = TmsCellChirho::new_chirho("x");

        cell_chirho.add_belief_chirho(BeliefChirho::with_premises_chirho(
            NumericInfoChirho::exact_chirho(5.0),
            vec!["a".to_string()].into_iter().collect(),
            "source_a",
        ));

        cell_chirho.add_belief_chirho(BeliefChirho::with_premises_chirho(
            NumericInfoChirho::exact_chirho(10.0),
            vec!["b".to_string()].into_iter().collect(),
            "source_b",
        ));

        // Worldview with only "a"
        let worldview_a_chirho = WorldviewChirho::new_chirho().assume_chirho("a".to_string());
        let content_a_chirho = cell_chirho.content_in_worldview_chirho(&worldview_a_chirho);
        let interval_a_chirho = content_a_chirho.as_interval_chirho().unwrap();
        assert!((interval_a_chirho.lo_chirho - 5.0).abs() < 1e-10);

        // Worldview with only "b"
        let worldview_b_chirho = WorldviewChirho::new_chirho().assume_chirho("b".to_string());
        let content_b_chirho = cell_chirho.content_in_worldview_chirho(&worldview_b_chirho);
        let interval_b_chirho = content_b_chirho.as_interval_chirho().unwrap();
        assert!((interval_b_chirho.lo_chirho - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_nogood_detection_chirho() {
        let cell_chirho = TmsCellChirho::new_chirho("x");

        // Add contradictory beliefs
        cell_chirho.add_belief_chirho(BeliefChirho::with_premises_chirho(
            NumericInfoChirho::interval_chirho(0.0, 5.0),
            vec!["a".to_string()].into_iter().collect(),
            "source_a",
        ));

        cell_chirho.add_belief_chirho(BeliefChirho::with_premises_chirho(
            NumericInfoChirho::interval_chirho(10.0, 15.0),
            vec!["b".to_string()].into_iter().collect(),
            "source_b",
        ));

        let nogoods_chirho = cell_chirho.find_nogoods_chirho();
        assert!(!nogoods_chirho.is_empty());
        assert!(nogoods_chirho[0].contains("a"));
        assert!(nogoods_chirho[0].contains("b"));
    }
}
