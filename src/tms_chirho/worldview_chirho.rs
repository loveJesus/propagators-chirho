// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Worldviews for hypothetical reasoning.
//!
//! A worldview represents a set of active premises under which we evaluate
//! beliefs. By forking worldviews and assuming different premises, we can
//! explore hypothetical scenarios.
//!
//! # Example
//!
//! ```
//! use propagators_chirho::WorldviewChirho;
//!
//! // Start with an empty worldview
//! let base_chirho = WorldviewChirho::new_chirho();
//!
//! // Fork and assume "sunny"
//! let sunny_world_chirho = base_chirho.assume_chirho("sunny".to_string());
//!
//! // Fork again and assume "rainy" instead
//! let rainy_world_chirho = base_chirho.assume_chirho("rainy".to_string());
//!
//! // Each worldview has different active premises
//! assert!(sunny_world_chirho.has_premise_chirho("sunny"));
//! assert!(!sunny_world_chirho.has_premise_chirho("rainy"));
//! ```
//!
//! # References
//!
//! - Radul, A., & Sussman, G. J. (2009). *The Art of the Propagator*, Section 8.
//!   <https://dspace.mit.edu/handle/1721.1/44215>

use std::collections::HashSet;

use super::tms_chirho::PremiseSetChirho;

/// A worldview representing a set of active premises.
///
/// Worldviews are used for hypothetical reasoning. Each worldview can be
/// thought of as a "possible world" where certain assumptions hold.
///
/// # Operations
///
/// - **assume**: Add a premise to create a new worldview
/// - **retract**: Remove a premise to create a new worldview
/// - **fork**: Create a copy that can be modified independently
///
/// # Example
///
/// ```
/// use propagators_chirho::WorldviewChirho;
///
/// let base_chirho = WorldviewChirho::new_chirho();
///
/// // Create a worldview where it's sunny
/// let sunny_chirho = base_chirho
///     .assume_chirho("sunny".to_string())
///     .assume_chirho("warm".to_string());
///
/// assert!(sunny_chirho.has_premise_chirho("sunny"));
/// assert!(sunny_chirho.has_premise_chirho("warm"));
///
/// // Retract an assumption
/// let cloudy_chirho = sunny_chirho.retract_chirho("sunny");
/// assert!(!cloudy_chirho.has_premise_chirho("sunny"));
/// assert!(cloudy_chirho.has_premise_chirho("warm"));
/// ```
#[derive(Clone, Debug, Default)]
pub struct WorldviewChirho {
    /// The set of currently active premises.
    active_premises_chirho: PremiseSetChirho,
}

impl WorldviewChirho {
    /// Creates a new empty worldview.
    ///
    /// An empty worldview has no active premises, so only unconditional
    /// beliefs are valid.
    pub fn new_chirho() -> Self {
        Self {
            active_premises_chirho: HashSet::new(),
        }
    }

    /// Creates a worldview with the given premises.
    pub fn with_premises_chirho(premises_chirho: PremiseSetChirho) -> Self {
        Self {
            active_premises_chirho: premises_chirho,
        }
    }

    /// Returns a reference to the active premises.
    pub fn active_premises_chirho(&self) -> &PremiseSetChirho {
        &self.active_premises_chirho
    }

    /// Returns `true` if the given premise is active.
    pub fn has_premise_chirho(&self, premise_chirho: &str) -> bool {
        self.active_premises_chirho.contains(premise_chirho)
    }

    /// Returns `true` if this worldview has no active premises.
    pub fn is_empty_chirho(&self) -> bool {
        self.active_premises_chirho.is_empty()
    }

    /// Returns the number of active premises.
    pub fn premise_count_chirho(&self) -> usize {
        self.active_premises_chirho.len()
    }

    /// Creates a new worldview with the given premise added.
    ///
    /// This is non-destructive—the original worldview is unchanged.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::WorldviewChirho;
    ///
    /// let base_chirho = WorldviewChirho::new_chirho();
    /// let extended_chirho = base_chirho.assume_chirho("hypothesis".to_string());
    ///
    /// assert!(extended_chirho.has_premise_chirho("hypothesis"));
    /// assert!(!base_chirho.has_premise_chirho("hypothesis"));
    /// ```
    pub fn assume_chirho(&self, premise_chirho: String) -> Self {
        let mut new_premises_chirho = self.active_premises_chirho.clone();
        new_premises_chirho.insert(premise_chirho);
        Self {
            active_premises_chirho: new_premises_chirho,
        }
    }

    /// Creates a new worldview with the given premise removed.
    ///
    /// This is non-destructive—the original worldview is unchanged.
    ///
    /// # Example
    ///
    /// ```
    /// use propagators_chirho::WorldviewChirho;
    ///
    /// let world_chirho = WorldviewChirho::new_chirho()
    ///     .assume_chirho("a".to_string())
    ///     .assume_chirho("b".to_string());
    ///
    /// let without_a_chirho = world_chirho.retract_chirho("a");
    ///
    /// assert!(!without_a_chirho.has_premise_chirho("a"));
    /// assert!(without_a_chirho.has_premise_chirho("b"));
    /// ```
    pub fn retract_chirho(&self, premise_chirho: &str) -> Self {
        let mut new_premises_chirho = self.active_premises_chirho.clone();
        new_premises_chirho.remove(premise_chirho);
        Self {
            active_premises_chirho: new_premises_chirho,
        }
    }

    /// Creates a fork (copy) of this worldview.
    ///
    /// This is equivalent to `clone()` but semantically represents
    /// creating a branch for exploration.
    pub fn fork_chirho(&self) -> Self {
        self.clone()
    }

    /// Returns `true` if all premises in the given set are active.
    pub fn satisfies_chirho(&self, required_chirho: &PremiseSetChirho) -> bool {
        required_chirho.is_subset(&self.active_premises_chirho)
    }

    /// Merges another worldview into this one (union of premises).
    pub fn merge_chirho(&self, other_chirho: &Self) -> Self {
        let mut merged_chirho = self.active_premises_chirho.clone();
        merged_chirho.extend(other_chirho.active_premises_chirho.clone());
        Self {
            active_premises_chirho: merged_chirho,
        }
    }

    /// Returns the intersection with another worldview.
    pub fn intersect_chirho(&self, other_chirho: &Self) -> Self {
        let intersection_chirho: PremiseSetChirho = self
            .active_premises_chirho
            .intersection(&other_chirho.active_premises_chirho)
            .cloned()
            .collect();
        Self {
            active_premises_chirho: intersection_chirho,
        }
    }
}

impl std::fmt::Display for WorldviewChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.active_premises_chirho.is_empty() {
            write!(f_chirho, "Worldview(∅)")
        } else {
            let premises_chirho: Vec<_> = self.active_premises_chirho.iter().collect();
            write!(f_chirho, "Worldview({:?})", premises_chirho)
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_empty_worldview_chirho() {
        let worldview_chirho = WorldviewChirho::new_chirho();
        assert!(worldview_chirho.is_empty_chirho());
        assert_eq!(worldview_chirho.premise_count_chirho(), 0);
    }

    #[test]
    fn test_assume_chirho() {
        let worldview_chirho = WorldviewChirho::new_chirho().assume_chirho("a".to_string());

        assert!(worldview_chirho.has_premise_chirho("a"));
        assert!(!worldview_chirho.has_premise_chirho("b"));
    }

    #[test]
    fn test_retract_chirho() {
        let worldview_chirho = WorldviewChirho::new_chirho()
            .assume_chirho("a".to_string())
            .assume_chirho("b".to_string())
            .retract_chirho("a");

        assert!(!worldview_chirho.has_premise_chirho("a"));
        assert!(worldview_chirho.has_premise_chirho("b"));
    }

    #[test]
    fn test_fork_chirho() {
        let base_chirho = WorldviewChirho::new_chirho().assume_chirho("shared".to_string());

        let fork1_chirho = base_chirho.fork_chirho().assume_chirho("a".to_string());
        let fork2_chirho = base_chirho.fork_chirho().assume_chirho("b".to_string());

        assert!(fork1_chirho.has_premise_chirho("shared"));
        assert!(fork1_chirho.has_premise_chirho("a"));
        assert!(!fork1_chirho.has_premise_chirho("b"));

        assert!(fork2_chirho.has_premise_chirho("shared"));
        assert!(!fork2_chirho.has_premise_chirho("a"));
        assert!(fork2_chirho.has_premise_chirho("b"));
    }

    #[test]
    fn test_satisfies_chirho() {
        let worldview_chirho = WorldviewChirho::new_chirho()
            .assume_chirho("a".to_string())
            .assume_chirho("b".to_string());

        let required1_chirho: PremiseSetChirho = vec!["a".to_string()].into_iter().collect();
        let required2_chirho: PremiseSetChirho =
            vec!["a".to_string(), "c".to_string()].into_iter().collect();

        assert!(worldview_chirho.satisfies_chirho(&required1_chirho));
        assert!(!worldview_chirho.satisfies_chirho(&required2_chirho));
    }

    #[test]
    fn test_merge_chirho() {
        let wv1_chirho = WorldviewChirho::new_chirho().assume_chirho("a".to_string());
        let wv2_chirho = WorldviewChirho::new_chirho().assume_chirho("b".to_string());

        let merged_chirho = wv1_chirho.merge_chirho(&wv2_chirho);

        assert!(merged_chirho.has_premise_chirho("a"));
        assert!(merged_chirho.has_premise_chirho("b"));
    }
}
