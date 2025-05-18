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
        /// The kind of quantifier that threw this error.
        kind: QuantorKind,
        /// The index of the first failing element.
        index: usize
    },
    /// Returned when no elements are given.
    EmptyInput {
        /// The kind of quantifier that threw this error.
        kind: QuantorKind
    },
    /// Returned when no element satisfies the predicate in an `exists` check.
    NoMatch {
        /// The kind of quantifier that threw this error.
        kind: QuantorKind
    },
    /// Returned when `none` or `exactly_one` fails.
    UnexpectedMatch {
        /// The kind of quantifier that threw this error.
        kind: QuantorKind,
        /// The index of the violating element.
        index: usize
    },
    /// Returned when not all elements are equal in `all_equal`.
    NotAllEqual {
        /// The kind of quantifier that threw this error.
        kind: QuantorKind,
        /// The index of the first element that is not equal to the other, previously checked elements.
        index: usize
    },
    /// Returned when a pair of adjacent elements fail a `pairwise` predicate.
    PairwiseFailed {
        /// The kind of quantifier that threw this error.
        kind: QuantorKind,
        /// The index of the first element in the failing pair.
        index: usize
    },
    /// Returned when a `forallexists` condition fails.
    ForAllExistsFailed {
        /// The kind of quantifier that threw this error.
        kind: QuantorKind,
        /// The index of the outer (left) element that failed.
        outer_index: usize
    },
    /// Returned when no left-side element satisfies the `existsforall` condition.
    ExistsForAllFailed {
        /// The kind of quantifier that threw this error.
        kind: QuantorKind,
        /// The index of the outer (left) element that failed.
        outer_index: usize
    },
    /// Returned when the number of matches does not equal the expected count.
    ExactlyNFailed {
        /// The kind of quantifier that threw this error.
        kind: QuantorKind,
        /// Number of matches found.
        found: usize,
        /// Number of matches expected.
        expected: usize,
    },
    /// A catch-all error with a static message.
    Custom(&'static str),
}

/// Represents the type of quantifier used in a logical check.
///
/// Used in error variants to indicate which quantifier produced the failure,
/// and for introspection via [`QuantorError::kind()`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantorKind {
    /// Universal quantifier (`forall`): all elements must satisfy the predicate.
    Forall,
    /// Existential quantifier (`exists`): at least one element must satisfy the predicate.
    Exists,
    /// Negated existential quantifier (`none`): no elements may satisfy the predicate.
    None,
    /// True if exactly one element satisfies the predicate.
    ExactlyOne,
    /// True if the number of matching elements equals the given count.
    ExactlyN,
    /// True if all elements are equal (via `PartialEq`).
    AllEqual,
    /// True if every adjacent pair satisfies the predicate.
    Pairwise,
    /// Nested quantifier: for every element in `A`, some element in `B` satisfies a predicate.
    ForAllExists,
    /// Nested quantifier: some element in `A` satisfies a predicate for all elements in `B`.
    ExistsForAll,
    /// Fallback for custom or user-defined logic.
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
            PredicateFailed { kind, index } => write!(f, "Predicate failed for element at index {} of quantifier {}.", index, kind),
            EmptyInput { kind } => write!(f, "Empty input for quantifier {}.", kind),
            NoMatch { kind } => write!(f, "No element satisfied the predicate for quantifier {}.", kind),
            UnexpectedMatch { kind, index } => write!(f, "Unexpected match found at index {} of quantifier {}.", index, kind),
            NotAllEqual { kind, index } => write!(f, "Element at index {} of quantifier {} does not match the first element.", index, kind),
            PairwiseFailed { kind, index } => write!(f, "Predicate failed for adjacent pair starting at index {} for quantifier {}.", index, kind),
            ForAllExistsFailed { kind, outer_index } => write!(f, "Element at index {} in the outer collection failed to match any right-hand value for quantifier {}.", outer_index, kind),
            ExistsForAllFailed { kind, outer_index } => write!(f, "Element at index {} in the left-hand collection failed the universal condition for quantifier {}.", outer_index, kind),
            ExactlyNFailed { kind, found, expected } => write!(f, "Expected {} elements to match, found {} for quantifier {}.", expected, found, kind),
            Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl fmt::Display for QuantorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            QuantorKind::Forall => "forall",
            QuantorKind::Exists => "exists",
            QuantorKind::None => "none",
            QuantorKind::ExactlyOne => "exactly_one",
            QuantorKind::Pairwise => "pairwise",
            QuantorKind::ExactlyN => "exactly_n",
            QuantorKind::AllEqual => "all_equal",
            QuantorKind::ForAllExists => "forallexists",
            QuantorKind::ExistsForAll => "existsforall",
            QuantorKind::Custom => "custom",
        };
        write!(f, "{}", name)
    }
}

impl QuantorError {
    /// Returns `true` if the quantifier failed due to a predicate mismatch.
    ///
    /// Useful for identifying simple predicate failures, such as those from `forall` or `exactly_one`.
    ///
    /// ## Returns
    /// - `true` if the error variant is [`QuantorError::PredicateFailed`].
    /// - `false` otherwise.
    ///
    /// ## Example
    /// ```
    /// use quantor::{forall, error::QuantorResultExt};
    ///
    /// let nums = vec![1, 2, 3];
    /// let result = forall(&nums, |x| *x < 3);
    ///
    /// assert!(result.is_err());
    /// assert!(result.unwrap_err().is_predicate_failed());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_predicate_failed(&self) -> bool {
        matches!(self, QuantorError::PredicateFailed {..})
    }

    /// Returns `true` if the quantifier failed because no element matched the predicate.
    ///
    /// Typically used with `exists` or `existsforall` where at least one match is expected.
    ///
    /// ## Returns
    /// - `true` if the error variant is [`QuantorError::NoMatch`].
    /// - `false` otherwise.
    ///
    /// ## Example
    /// ```
    /// use quantor::{exists, error::QuantorResultExt};
    ///
    /// let nums = [1, 2, 3];
    /// let result = exists(&nums, |x| *x > 10);
    ///
    /// assert!(result.is_err());
    /// assert!(result.unwrap_err().is_no_match());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_no_match(&self) -> bool {
        matches!(self, QuantorError::NoMatch { .. })
    }

    /// Returns the [`QuantorKind`] associated with this error.
    ///
    /// Allows inspection of which quantifier failed, regardless of the specific error variant.
    ///
    /// ## Returns
    /// - A [`QuantorKind`] value corresponding to the quantifier that produced the error.
    /// - [`QuantorKind::Custom`] for generic errors.
    ///
    /// ## Example
    /// ```
    /// use quantor::{forall, error::{QuantorKind, QuantorResultExt}};
    ///
    /// let nums = [1, 2, 3];
    /// let result = forall(&nums, |x| *x > 3);
    ///
    /// assert_eq!(result.unwrap_err().kind(), QuantorKind::Forall);
    /// ```
    #[inline]
    #[must_use]
    pub fn kind(&self) -> QuantorKind {
        match self {
            QuantorError::PredicateFailed { .. } => QuantorKind::Forall,
            QuantorError::EmptyInput { kind } => *kind,
            QuantorError::NoMatch { .. } => QuantorKind::Exists,
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
            Err(QuantorError::PredicateFailed { index, .. }) => Some(*index),
            Err(QuantorError::UnexpectedMatch { index, .. }) => Some(*index),
            Err(QuantorError::PairwiseFailed { index, .. }) => Some(*index),
            Err(QuantorError::ForAllExistsFailed { outer_index, .. }) => Some(*outer_index),
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