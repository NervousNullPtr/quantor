//! # quantor: Errors
//!
//! This module defines the [`QuantorError`] enum used across all fallible quantifier functions,
//! such as `forall`, `none`, `existsforall`, `pairwise`, and others.
//!
//! Each error represents a logical violation of a quantifier expectation and includes
//! enough detail to aid in diagnostics or debugging. Most errors include the index of the
//! failing element or pair to allow precise localization of the failure.
//!
//! The accompanying extension trait [`QuantorResultExt`] provides convenience methods for
//! extracting diagnostic information from `Result<(), QuantorError>` values, which are returned
//! by all fallible quantifier functions.

use std::fmt;

/// Error type returned by fallible quantifier evaluations in `quantor`.
#[derive(PartialEq, Eq)]
pub enum QuantorError {
    /// Returned when a predicate fails during a `forall` check.
    PredicateFailed {
        /// The index of the first failing element.
        index: usize
    },
    /// Returned when no element satisfies the predicate in an `exists` check.
    NoMatch,
    /// Returned when `none` or `exactly_one` fails.
    UnexpectedMatch {
        /// The index of the violating element.
        index: usize
    },
    /// Returned when not all elements are equal in `all_equal`.
    NotAllEqual {
        /// The index of the first element that is not equal to the other, previously checked elements.
        index: usize
    },
    /// Returned when a pair of adjacent elements fail a `pairwise` predicate.
    PairwiseFailed {
        /// The index of the first element in the failing pair.
        index: usize
    },
    /// Returned when a `forallexists` condition fails.
    ForAllExistsFailed {
        /// The index of the outer (left) element that failed.
        outer_index: usize
    },
    /// Returned when no left-side element satisfies the `existsforall` condition.
    ExistsForAllFailed {
        /// The index of the outer (left) element that failed.
        outer_index: usize
    },
    /// Returned when the number of matches does not equal the expected count.
    ExactlyNFailed {
        /// Number of matches found.
        found: usize,
        /// Number of matches expected.
        expected: usize,
    },
    /// A catch-all error with a static message.
    Custom(&'static str),
}

/// Represents the kind of quantifier or logic that failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantorKind {
    Forall,
    Exists,
    None,
    ExactlyOne,
    ExactlyN,
    AllEqual,
    Pairwise,
    ForAllExists,
    ExistsForAll,
    Custom,
}

/// Extension trait for `Result<(), QuantorError>` to simplify diagnostics.
///
/// This trait provides methods like `failing_index` to allow inspection of where a
/// quantifier check failed, making it easier to debug or react programmatically.
///
/// All core quantifiers return `Result<(), QuantorError>`, so these helpers
/// are ideal for writing ergonomic test assertions or logging failure details.
pub trait QuantorResultExt {
    /// Returns the failing index for quantifiers that include index information,
    /// such as `forall`, `none`, `pairwise`, `all_equal`, and `existsforall`.
    /// ## Returns
    /// - `Some(index)` if the underlying [`QuantorError`] variant includes a specific index indicating
    ///   which element caused the quantifier to fail.
    /// - `None` if the error variant does not include index information (e.g. `NoMatch`, `Custom`).
    /// ## Example
    /// ```
    /// use quantor::{forall, error::QuantorResultExt};
    /// 
    /// let nums = vec![2, 4, 5, 6];
    /// let result = forall(&nums, |x| x % 2 == 0);
    ///
    /// assert!(result.is_err());
    ///
    /// if let Some(index) = result.failing_index() {
    ///     assert_eq!(2, index); // `5` fails the even check
    /// }
    /// ```
    fn failing_index(&self) -> Option<usize>;
    /// Returns the number of elements that matched the predicate,
    /// if available from the underlying [`QuantorError`] variant.
    ///
    /// This is primarily useful for quantifiers that care about cardinality,
    /// such as `exactly_n`, `at_least`, or `at_most`.
    ///
    /// ## Returns
    /// - `Some(count)` if the error contains a match count (e.g. `ExactlyNFailed`).
    /// - `None` for other kinds of errors that do not track match counts.
    ///
    /// ## Example
    /// ```
    /// use quantor::{quantifiers::exactly_n, error::QuantorResultExt};
    ///
    /// let nums = vec![2, 4, 6]; // 3 even numbers
    /// let result = exactly_n(&nums, 2, |x| x % 2 == 0);
    ///
    /// assert!(result.is_err());
    ///
    /// if let Some(count) = result.match_count() {
    ///     assert_eq!(count, 3); // we matched 3, expected only 2
    /// }
    /// ```
    fn match_count(&self) -> Option<usize>;
}

impl fmt::Display for QuantorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use QuantorError::*;

        match self {
            PredicateFailed { index } => write!(f, "Predicate failed for element at index {}.", index),
            NoMatch => write!(f, "No element satisfied the predicate."),
            UnexpectedMatch { index } => write!(f, "Unexpected match found at index {}.", index),
            NotAllEqual { index } => write!(f, "Element at index {} does not match the first element.", index),
            PairwiseFailed { index } => write!(f, "Predicate failed for adjacent pair starting at index {}.", index),
            ForAllExistsFailed { outer_index } => write!(f, "Element at index {} in the outer collection failed to match any right-hand value.", outer_index),
            ExistsForAllFailed { outer_index } => write!(f, "Element at index {} in the left-hand collection failed the universal condition.", outer_index),
            ExactlyNFailed { found, expected } => write!(f, "Expected {} elements to match, found {}.", expected, found),
            Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl QuantorError {
    #[inline]
    #[must_use]
    pub fn is_predicate_failed(&self) -> bool {
        matches!(self, QuantorError::PredicateFailed {..})
    }

    #[inline]
    #[must_use]
    pub fn is_no_match(&self) -> bool {
        matches!(self, QuantorError::NoMatch)
    }

    #[inline]
    #[must_use]
    pub fn kind(&self) -> QuantorKind {
        match self {
            QuantorError::PredicateFailed { .. } => QuantorKind::Forall,
            QuantorError::NoMatch => QuantorKind::Exists,
            QuantorError::UnexpectedMatch { .. } => QuantorKind::None,
            QuantorError::NotAllEqual { .. } => QuantorKind::AllEqual,
            QuantorError::PairwiseFailed { .. } => QuantorKind::Pairwise,
            QuantorError::ForAllExistsFailed { .. } => QuantorKind::ForAllExists,
            QuantorError::ExistsForAllFailed { .. } => QuantorKind::ExistsForAll,
            QuantorError::ExactlyNFailed { .. } => QuantorKind::ExactlyN,
            QuantorError::Custom(_) => QuantorKind::Custom,
        }
    }
}


impl std::error::Error for QuantorError {}

impl From<&'static str> for QuantorError {
    fn from(msg: &'static str) -> Self {
        QuantorError::Custom(msg)
    }
}

impl From<String> for QuantorError {
    fn from(msg: String) -> Self {
        QuantorError::Custom(Box::leak(msg.into_boxed_str()))
    }
}

impl fmt::Debug for QuantorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl QuantorResultExt for Result<(), QuantorError> {
    #[inline]
    fn failing_index(&self) -> Option<usize> {
        match self {
            Err(QuantorError::PredicateFailed { index }) => Some(*index),
            Err(QuantorError::UnexpectedMatch { index }) => Some(*index),
            Err(QuantorError::PairwiseFailed { index }) => Some(*index),
            Err(QuantorError::ForAllExistsFailed { outer_index }) => Some(*outer_index),
            _ => None,
        }
    }

    #[inline]
    fn match_count(&self) -> Option<usize> {
        match self {
            Err(QuantorError::ExactlyNFailed { found, .. }) => Some(*found),
            _ => None
        }
    }
}