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
//! - **Dependency-directed backtracking**: Focus search on relevant premises
//!
//! # Feature Flags
//!
//! - `tms-full`: Enables full TMS with justification tracking and minimal nogood computation.
//!   This adds some overhead but provides complete dependency information.
//!
//! # References
//!
//! - de Kleer, J. (1986). *An Assumption-based TMS*.
//!   Artificial Intelligence, 28(2), 127-162.
//!
//! - Radul, A., & Sussman, G. J. (2009). *The Art of the Propagator*, Section 7.
//!   <https://dspace.mit.edu/handle/1721.1/44215>
//!
//! - Stallman, R. M., & Sussman, G. J. (1977). *Forward Reasoning and
//!   Dependency-Directed Backtracking*. Artificial Intelligence, 9(2), 135-196.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::core_chirho::interval_chirho::NumericInfoChirho;

// Unique ID generator for justifications
static JUSTIFICATION_ID_COUNTER_CHIRHO: AtomicU64 = AtomicU64::new(0);

fn next_justification_id_chirho() -> u64 {
    JUSTIFICATION_ID_COUNTER_CHIRHO.fetch_add(1, Ordering::Relaxed)
}

/// A justification records how a belief was derived.
///
/// Each justification has:
/// - A unique ID for tracking
/// - The premises it depends on directly
/// - The antecedent justifications it was derived from
/// - A human-readable rule name
///
/// # Example
///
/// ```
/// use propagators_chirho::JustificationChirho;
///
/// // A justification from a direct premise
/// let direct_chirho = JustificationChirho::from_premise_chirho("sensor_a");
///
/// // A derived justification from applying a rule
/// let derived_chirho = JustificationChirho::from_rule_chirho(
///     "temperature_conversion",
///     vec![direct_chirho.clone()],
/// );
/// ```
#[derive(Clone, Debug)]
pub struct JustificationChirho {
    /// Unique identifier for this justification.
    id_chirho: u64,
    /// Direct premises this justification depends on.
    premises_chirho: PremiseSetChirho,
    /// IDs of antecedent justifications.
    antecedents_chirho: Vec<u64>,
    /// Name of the rule or source that created this.
    rule_chirho: String,
}

impl JustificationChirho {
    /// Creates a justification from a direct premise.
    pub fn from_premise_chirho(premise_chirho: &str) -> Self {
        let mut premises_chirho = HashSet::new();
        premises_chirho.insert(premise_chirho.to_string());
        Self {
            id_chirho: next_justification_id_chirho(),
            premises_chirho,
            antecedents_chirho: Vec::new(),
            rule_chirho: format!("premise:{}", premise_chirho),
        }
    }

    /// Creates a justification from applying a rule to antecedent justifications.
    pub fn from_rule_chirho(
        rule_chirho: &str,
        antecedents_chirho: Vec<JustificationChirho>,
    ) -> Self {
        // Collect all premises from antecedents
        let mut premises_chirho = HashSet::new();
        let mut antecedent_ids_chirho = Vec::new();

        for antecedent_chirho in antecedents_chirho {
            premises_chirho.extend(antecedent_chirho.premises_chirho.clone());
            antecedent_ids_chirho.push(antecedent_chirho.id_chirho);
        }

        Self {
            id_chirho: next_justification_id_chirho(),
            premises_chirho,
            antecedents_chirho: antecedent_ids_chirho,
            rule_chirho: rule_chirho.to_string(),
        }
    }

    /// Creates an unconditional (always true) justification.
    pub fn unconditional_chirho(rule_chirho: &str) -> Self {
        Self {
            id_chirho: next_justification_id_chirho(),
            premises_chirho: HashSet::new(),
            antecedents_chirho: Vec::new(),
            rule_chirho: rule_chirho.to_string(),
        }
    }

    /// Returns the unique ID.
    pub fn id_chirho(&self) -> u64 {
        self.id_chirho
    }

    /// Returns the premises this justification depends on.
    pub fn premises_chirho(&self) -> &PremiseSetChirho {
        &self.premises_chirho
    }

    /// Returns the antecedent justification IDs.
    pub fn antecedents_chirho(&self) -> &[u64] {
        &self.antecedents_chirho
    }

    /// Returns the rule name.
    pub fn rule_chirho(&self) -> &str {
        &self.rule_chirho
    }

    /// Returns true if this justification requires no premises.
    pub fn is_unconditional_chirho(&self) -> bool {
        self.premises_chirho.is_empty()
    }
}

/// A set of premise identifiers.
pub type PremiseSetChirho = HashSet<String>;

/// A store for nogood (contradictory) premise sets.
///
/// Nogoods are minimal sets of premises that, when all assumed together,
/// lead to a contradiction. The store tracks these and provides:
///
/// - Adding new nogoods
/// - Checking if a worldview contains a known nogood
/// - Computing minimal nogoods from a set
///
/// # Example
///
/// ```
/// use propagators_chirho::{NogoodStoreChirho, PremiseSetChirho};
///
/// let mut store_chirho = NogoodStoreChirho::new_chirho();
///
/// // Record that {a, b} is contradictory
/// let nogood_chirho: PremiseSetChirho = vec!["a".to_string(), "b".to_string()]
///     .into_iter().collect();
/// store_chirho.add_nogood_chirho(nogood_chirho);
///
/// // Check if a worldview contains this nogood
/// let worldview_chirho: PremiseSetChirho = vec!["a".to_string(), "b".to_string(), "c".to_string()]
///     .into_iter().collect();
/// assert!(store_chirho.contains_nogood_chirho(&worldview_chirho));
/// ```
#[derive(Clone, Debug, Default)]
pub struct NogoodStoreChirho {
    /// All known nogoods.
    nogoods_chirho: Vec<PremiseSetChirho>,
}

impl NogoodStoreChirho {
    /// Creates an empty nogood store.
    pub fn new_chirho() -> Self {
        Self {
            nogoods_chirho: Vec::new(),
        }
    }

    /// Adds a nogood to the store.
    ///
    /// The store automatically filters out non-minimal nogoods.
    pub fn add_nogood_chirho(&mut self, nogood_chirho: PremiseSetChirho) {
        if nogood_chirho.is_empty() {
            return; // Empty set can't be a nogood
        }

        // Check if this is subsumed by an existing nogood
        for existing_chirho in &self.nogoods_chirho {
            if existing_chirho.is_subset(&nogood_chirho) {
                return; // A smaller nogood already exists
            }
        }

        // Remove any nogoods that this one subsumes
        self.nogoods_chirho
            .retain(|existing_chirho| !nogood_chirho.is_subset(existing_chirho));

        self.nogoods_chirho.push(nogood_chirho);
    }

    /// Returns true if the given premise set contains any known nogood.
    pub fn contains_nogood_chirho(&self, premises_chirho: &PremiseSetChirho) -> bool {
        self.nogoods_chirho
            .iter()
            .any(|nogood_chirho| nogood_chirho.is_subset(premises_chirho))
    }

    /// Returns the first nogood contained in the premise set, if any.
    pub fn find_nogood_chirho(
        &self,
        premises_chirho: &PremiseSetChirho,
    ) -> Option<&PremiseSetChirho> {
        self.nogoods_chirho
            .iter()
            .find(|nogood_chirho| nogood_chirho.is_subset(premises_chirho))
    }

    /// Returns all nogoods contained in the premise set.
    pub fn find_all_nogoods_chirho(
        &self,
        premises_chirho: &PremiseSetChirho,
    ) -> Vec<&PremiseSetChirho> {
        self.nogoods_chirho
            .iter()
            .filter(|nogood_chirho| nogood_chirho.is_subset(premises_chirho))
            .collect()
    }

    /// Returns all stored nogoods.
    pub fn nogoods_chirho(&self) -> &[PremiseSetChirho] {
        &self.nogoods_chirho
    }

    /// Returns the number of stored nogoods.
    pub fn len_chirho(&self) -> usize {
        self.nogoods_chirho.len()
    }

    /// Returns true if no nogoods are stored.
    pub fn is_empty_chirho(&self) -> bool {
        self.nogoods_chirho.is_empty()
    }

    /// Suggests a premise to retract to avoid a nogood.
    ///
    /// Prefers premises that appear in the most nogoods (heuristic for
    /// maximum constraint relief).
    pub fn suggest_retraction_chirho(&self, premises_chirho: &PremiseSetChirho) -> Option<String> {
        let contained_nogoods_chirho = self.find_all_nogoods_chirho(premises_chirho);
        if contained_nogoods_chirho.is_empty() {
            return None;
        }

        // Count how often each premise appears in the contained nogoods
        let mut counts_chirho: HashMap<&str, usize> = HashMap::new();
        for nogood_chirho in contained_nogoods_chirho {
            for premise_chirho in nogood_chirho {
                *counts_chirho.entry(premise_chirho.as_str()).or_insert(0) += 1;
            }
        }

        // Return the premise that appears most often
        counts_chirho
            .into_iter()
            .max_by_key(|(_, count_chirho)| *count_chirho)
            .map(|(premise_chirho, _)| premise_chirho.to_string())
    }
}

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
        worldview_chirho: &super::worldview_chirho::WorldviewChirho,
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

/// A TMS-aware propagator network.
///
/// This network maintains TMS cells and tracks the dependency graph
/// between values. When contradictions are detected, it can identify
/// the minimal set of premises responsible.
///
/// # Features
///
/// - Cells with multiple beliefs under different premises
/// - Nogood store for tracking contradictions
/// - Dependency-directed reasoning
///
/// # Example
///
/// ```
/// use propagators_chirho::{TmsNetworkChirho, NumericInfoChirho};
///
/// let mut network_chirho = TmsNetworkChirho::new_chirho();
///
/// // Create cells
/// let temp_chirho = network_chirho.make_cell_chirho("temperature");
///
/// // Add beliefs from different sources
/// network_chirho.add_belief_chirho(
///     temp_chirho,
///     NumericInfoChirho::exact_chirho(25.0),
///     vec!["sensor_a".to_string()],
///     "sensor A"
/// );
/// ```
pub struct TmsNetworkChirho {
    /// All cells in the network.
    cells_chirho: Vec<std::rc::Rc<TmsCellChirho>>,
    /// Global nogood store.
    nogoods_chirho: std::cell::RefCell<NogoodStoreChirho>,
    /// Justification graph for full TMS mode.
    #[cfg(feature = "tms-full")]
    justifications_chirho: std::cell::RefCell<HashMap<u64, JustificationChirho>>,
}

impl TmsNetworkChirho {
    /// Creates a new TMS network.
    pub fn new_chirho() -> Self {
        Self {
            cells_chirho: Vec::new(),
            nogoods_chirho: std::cell::RefCell::new(NogoodStoreChirho::new_chirho()),
            #[cfg(feature = "tms-full")]
            justifications_chirho: std::cell::RefCell::new(HashMap::new()),
        }
    }

    /// Creates a new cell and returns its index.
    pub fn make_cell_chirho(&mut self, name_chirho: &str) -> usize {
        let cell_chirho = std::rc::Rc::new(TmsCellChirho::new_chirho(name_chirho));
        self.cells_chirho.push(cell_chirho);
        self.cells_chirho.len() - 1
    }

    /// Returns a reference to a cell by index.
    pub fn cell_chirho(&self, index_chirho: usize) -> Option<&std::rc::Rc<TmsCellChirho>> {
        self.cells_chirho.get(index_chirho)
    }

    /// Returns the number of cells.
    pub fn cell_count_chirho(&self) -> usize {
        self.cells_chirho.len()
    }

    /// Adds a belief to a cell.
    pub fn add_belief_chirho(
        &self,
        cell_index_chirho: usize,
        value_chirho: NumericInfoChirho,
        premises_chirho: Vec<String>,
        source_chirho: &str,
    ) {
        if let Some(cell_chirho) = self.cells_chirho.get(cell_index_chirho) {
            let premise_set_chirho: PremiseSetChirho = premises_chirho.into_iter().collect();
            let belief_chirho =
                BeliefChirho::with_premises_chirho(value_chirho, premise_set_chirho, source_chirho);
            cell_chirho.add_belief_chirho(belief_chirho);

            // Check for new nogoods
            self.update_nogoods_chirho(cell_index_chirho);
        }
    }

    /// Updates the nogood store based on a cell's beliefs.
    fn update_nogoods_chirho(&self, cell_index_chirho: usize) {
        if let Some(cell_chirho) = self.cells_chirho.get(cell_index_chirho) {
            let new_nogoods_chirho = cell_chirho.find_nogoods_chirho();
            let mut store_chirho = self.nogoods_chirho.borrow_mut();
            for nogood_chirho in new_nogoods_chirho {
                store_chirho.add_nogood_chirho(nogood_chirho);
            }
        }
    }

    /// Returns the nogood store.
    pub fn nogoods_chirho(&self) -> std::cell::Ref<'_, NogoodStoreChirho> {
        self.nogoods_chirho.borrow()
    }

    /// Checks if the given worldview contains a known nogood.
    pub fn worldview_is_contradictory_chirho(
        &self,
        worldview_chirho: &super::worldview_chirho::WorldviewChirho,
    ) -> bool {
        self.nogoods_chirho
            .borrow()
            .contains_nogood_chirho(worldview_chirho.active_premises_chirho())
    }

    /// Suggests a premise to retract to resolve contradictions.
    pub fn suggest_retraction_chirho(
        &self,
        worldview_chirho: &super::worldview_chirho::WorldviewChirho,
    ) -> Option<String> {
        self.nogoods_chirho
            .borrow()
            .suggest_retraction_chirho(worldview_chirho.active_premises_chirho())
    }

    /// Queries all cells under a given worldview.
    pub fn query_all_chirho(
        &self,
        worldview_chirho: &super::worldview_chirho::WorldviewChirho,
    ) -> Vec<NumericInfoChirho> {
        self.cells_chirho
            .iter()
            .map(|cell_chirho| cell_chirho.content_in_worldview_chirho(worldview_chirho))
            .collect()
    }

    /// Records a justification (only available with tms-full feature).
    #[cfg(feature = "tms-full")]
    pub fn record_justification_chirho(&self, justification_chirho: JustificationChirho) {
        self.justifications_chirho
            .borrow_mut()
            .insert(justification_chirho.id_chirho(), justification_chirho);
    }

    /// Retrieves a justification by ID (only available with tms-full feature).
    #[cfg(feature = "tms-full")]
    pub fn get_justification_chirho(&self, id_chirho: u64) -> Option<JustificationChirho> {
        self.justifications_chirho.borrow().get(&id_chirho).cloned()
    }

    /// Returns all justifications (only available with tms-full feature).
    #[cfg(feature = "tms-full")]
    pub fn all_justifications_chirho(&self) -> Vec<JustificationChirho> {
        self.justifications_chirho
            .borrow()
            .values()
            .cloned()
            .collect()
    }
}

impl Default for TmsNetworkChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl fmt::Debug for TmsNetworkChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        f_chirho
            .debug_struct("TmsNetwork")
            .field("cells", &self.cells_chirho.len())
            .field("nogoods", &self.nogoods_chirho.borrow().len_chirho())
            .finish()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use crate::tms_chirho::worldview_chirho::WorldviewChirho;

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

    #[test]
    fn test_justification_from_premise_chirho() {
        let just_chirho = JustificationChirho::from_premise_chirho("sensor_a");
        assert!(just_chirho.premises_chirho().contains("sensor_a"));
        assert!(just_chirho.antecedents_chirho().is_empty());
        assert!(!just_chirho.is_unconditional_chirho());
    }

    #[test]
    fn test_justification_from_rule_chirho() {
        let j1_chirho = JustificationChirho::from_premise_chirho("p1");
        let j2_chirho = JustificationChirho::from_premise_chirho("p2");

        let derived_chirho = JustificationChirho::from_rule_chirho(
            "sum",
            vec![j1_chirho.clone(), j2_chirho.clone()],
        );

        assert!(derived_chirho.premises_chirho().contains("p1"));
        assert!(derived_chirho.premises_chirho().contains("p2"));
        assert_eq!(derived_chirho.antecedents_chirho().len(), 2);
    }

    #[test]
    fn test_nogood_store_basic_chirho() {
        let mut store_chirho = NogoodStoreChirho::new_chirho();
        assert!(store_chirho.is_empty_chirho());

        let nogood_chirho: PremiseSetChirho =
            vec!["a".to_string(), "b".to_string()].into_iter().collect();
        store_chirho.add_nogood_chirho(nogood_chirho);

        assert_eq!(store_chirho.len_chirho(), 1);
    }

    #[test]
    fn test_nogood_store_subsumption_chirho() {
        let mut store_chirho = NogoodStoreChirho::new_chirho();

        // Add {a, b}
        let small_chirho: PremiseSetChirho =
            vec!["a".to_string(), "b".to_string()].into_iter().collect();
        store_chirho.add_nogood_chirho(small_chirho);

        // Add {a, b, c} - should be ignored (subsumed by {a, b})
        let large_chirho: PremiseSetChirho =
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
                .into_iter()
                .collect();
        store_chirho.add_nogood_chirho(large_chirho);

        assert_eq!(store_chirho.len_chirho(), 1);
    }

    #[test]
    fn test_nogood_store_contains_chirho() {
        let mut store_chirho = NogoodStoreChirho::new_chirho();
        let nogood_chirho: PremiseSetChirho =
            vec!["a".to_string(), "b".to_string()].into_iter().collect();
        store_chirho.add_nogood_chirho(nogood_chirho);

        let superset_chirho: PremiseSetChirho =
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
                .into_iter()
                .collect();
        assert!(store_chirho.contains_nogood_chirho(&superset_chirho));

        let unrelated_chirho: PremiseSetChirho =
            vec!["x".to_string(), "y".to_string()].into_iter().collect();
        assert!(!store_chirho.contains_nogood_chirho(&unrelated_chirho));
    }

    #[test]
    fn test_nogood_suggest_retraction_chirho() {
        let mut store_chirho = NogoodStoreChirho::new_chirho();

        // {a, b} and {a, c} are nogoods - a appears in both
        store_chirho
            .add_nogood_chirho(vec!["a".to_string(), "b".to_string()].into_iter().collect());
        store_chirho
            .add_nogood_chirho(vec!["a".to_string(), "c".to_string()].into_iter().collect());

        let premises_chirho: PremiseSetChirho =
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
                .into_iter()
                .collect();

        let suggestion_chirho = store_chirho.suggest_retraction_chirho(&premises_chirho);
        assert_eq!(suggestion_chirho, Some("a".to_string()));
    }

    #[test]
    fn test_tms_network_basic_chirho() {
        let mut network_chirho = TmsNetworkChirho::new_chirho();

        let cell_chirho = network_chirho.make_cell_chirho("x");
        assert_eq!(network_chirho.cell_count_chirho(), 1);

        network_chirho.add_belief_chirho(
            cell_chirho,
            NumericInfoChirho::exact_chirho(5.0),
            vec!["p1".to_string()],
            "source1",
        );

        let worldview_chirho = WorldviewChirho::new_chirho().assume_chirho("p1".to_string());
        let values_chirho = network_chirho.query_all_chirho(&worldview_chirho);
        assert_eq!(values_chirho.len(), 1);
    }

    #[test]
    fn test_tms_network_contradiction_chirho() {
        let mut network_chirho = TmsNetworkChirho::new_chirho();
        let cell_chirho = network_chirho.make_cell_chirho("x");

        // Add contradictory beliefs
        network_chirho.add_belief_chirho(
            cell_chirho,
            NumericInfoChirho::interval_chirho(0.0, 5.0),
            vec!["a".to_string()],
            "source_a",
        );
        network_chirho.add_belief_chirho(
            cell_chirho,
            NumericInfoChirho::interval_chirho(10.0, 15.0),
            vec!["b".to_string()],
            "source_b",
        );

        // Check that the contradiction is detected
        let worldview_chirho = WorldviewChirho::new_chirho()
            .assume_chirho("a".to_string())
            .assume_chirho("b".to_string());

        assert!(network_chirho.worldview_is_contradictory_chirho(&worldview_chirho));

        // Check retraction suggestion
        let suggestion_chirho = network_chirho.suggest_retraction_chirho(&worldview_chirho);
        assert!(suggestion_chirho.is_some());
    }
}
